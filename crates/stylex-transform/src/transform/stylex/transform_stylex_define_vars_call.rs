use std::rc::Rc;

use rustc_hash::{FxHashMap, FxHashSet};
use stylex_constants::constants::messages::SPREAD_NOT_SUPPORTED;
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::{
  common::comments::Comments,
  ecma::ast::{
    BlockStmtOrExpr, CallExpr, Callee, Expr, Lit, MemberProp, Pat, Prop, PropName, PropOrSpread,
  },
};

use crate::shared::{
  structures::{
    functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType},
    state_manager::ImportKind,
    theme_ref::ThemeRef,
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  transformers::{
    stylex_define_vars::stylex_define_vars, stylex_keyframes::get_keyframes_fn,
    stylex_position_try::get_position_try_fn, stylex_types::get_types_fn,
  },
  utils::{
    common::gen_file_based_identifier,
    core::js_to_expr::{NestedStringObject, convert_object_to_ast},
    js::evaluate::evaluate,
    log::build_code_frame_error::build_code_frame_error,
    validators::{find_and_validate_stylex_define_vars, is_define_vars_call},
  },
};
use stylex_constants::constants::messages::{
  cannot_generate_hash, cyclic_define_vars_reference, invalid_define_vars_function_value,
  non_static_value, non_style_object, unknown_define_vars_reference,
};

use crate::StyleXTransform;
use stylex_structures::top_level_expression::TopLevelExpression;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_define_vars(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_define_vars = is_define_vars_call(call, &self.state);

    if is_define_vars {
      let stylex_create_theme_top_level_expr =
        match find_and_validate_stylex_define_vars(call, &mut self.state) {
          Some(expr) => expr,
          #[cfg_attr(coverage_nightly, coverage(off))]
          None => stylex_panic!("defineVars(): Could not find the top-level variable declaration."),
        };

      let TopLevelExpression(_, _, var_id) = stylex_create_theme_top_level_expr;

      let first_arg = call.args.first().map(|first_arg| match &first_arg.spread {
        #[cfg_attr(coverage_nightly, coverage(off))]
        Some(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
        None => first_arg.expr.clone(),
      })?;

      let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
      let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

      let keyframes_fn = get_keyframes_fn();
      let types_fn = get_types_fn();
      let position_try_fn = get_position_try_fn();

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::Keyframes) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
          );
        }
      }

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::Types) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(types_fn.clone())),
          );
        }
      }

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::PositionTry) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
          );
        }
      }

      for name in &self.state.stylex_import {
        let member_expression = member_expressions.entry(name.clone()).or_default();

        member_expression.insert(
          "keyframes".into(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );

        member_expression.insert(
          "positionTry".into(),
          Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
        );

        let identifier = identifiers
          .entry(name.get_import_str().into())
          .or_insert_with(|| Box::new(FunctionConfigType::Map(FxHashMap::default())));

        if let Some(identifier_map) = identifier.as_map_mut() {
          identifier_map.insert("types".into(), types_fn.clone());
        }
      }

      self
        .state
        .apply_stylex_env(&mut identifiers, &mut member_expressions);

      // Compute file_name, export_name, and export_id BEFORE evaluation so the
      // ThemeRef self-reference proxy can be built and injected into identifiers.
      let file_name = match self
        .state
        .get_filename_for_hashing(&mut FxHashMap::default())
      {
        Some(name) => name,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", cannot_generate_hash("defineVars")),
      };

      let export_name = match var_id.map(|decl| decl.to_string()) {
        Some(name) => name,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!(
          "defineVars(): The export variable could not be found. Ensure the call is bound to a named export."
        ),
      };

      self.state.export_id = Some(gen_file_based_identifier(&file_name, &export_name, None));

      // Static analysis: validate arrow function values and build the dependency
      // graph so cycles and unknown references can be caught before evaluation.
      let all_keys = collect_object_keys(&first_arg);
      let dependency_map = build_dependency_map(&first_arg, &export_name, &all_keys);
      assert_no_define_vars_cycles(&dependency_map);

      // Inject a lazy ThemeRef factory under the export variable name so that
      // arrow function bodies can resolve `exportName.property` to `var(--hash)`
      // during normal evaluation.  Using ThemeRefMapper instead of a shared
      // mutable proxy keeps ownership clean and avoids Rc<RefCell> threading.
      let fn_ = file_name.clone();
      let en_ = export_name.clone();
      let pfx_ = self.state.options.class_name_prefix.to_string();
      let theme_ref_factory: Rc<dyn Fn() -> ThemeRef + 'static> =
        Rc::new(move || ThemeRef::new(fn_.clone(), en_.clone(), pfx_.clone()));

      identifiers.insert(
        export_name.as_str().into(),
        Box::new(FunctionConfigType::Regular(FunctionConfig {
          fn_ptr: FunctionType::ThemeRefMapper(theme_ref_factory),
          takes_path: false,
        })),
      );

      let function_map: Box<FunctionMap> = Box::new(FunctionMap {
        identifiers,
        member_expressions,
        disable_imports: false,
      });

      let evaluated_arg = evaluate(&first_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          &non_static_value("defineVars"),
          &mut self.state,
        )
      );

      let value = match evaluated_arg.value {
        Some(value) => {
          assert!(
            value
              .as_expr()
              .map(|expr| expr.is_object())
              .unwrap_or(false),
            "{}",
            build_code_frame_error(
              &Expr::Call(call.clone()),
              &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
              &non_style_object("defineVars"),
              &mut self.state,
            )
          );
          value
        },
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", non_static_value("defineVars")),
      };

      let (variables_obj, injected_styles_sans_keyframes) =
        stylex_define_vars(&value, &mut self.state);

      let mut injected_styles = self.state.other_injected_css_rules.clone();
      injected_styles.extend(injected_styles_sans_keyframes);

      let result_ast =
        convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(variables_obj));

      self
        .state
        .register_styles(call, &injected_styles, &result_ast, None);

      Some(result_ast)
    } else {
      None
    }
  }
}

/// Collects all top-level string keys from a `defineVars` object literal.
fn collect_object_keys(expr: &Expr) -> FxHashSet<String> {
  let obj = match expr.as_object() {
    Some(o) => o,
    None => return FxHashSet::default(),
  };

  obj
    .props
    .iter()
    .filter_map(|prop| match prop {
      PropOrSpread::Prop(p) => match p.as_ref() {
        Prop::KeyValue(kv) => prop_name_to_string(&kv.key),
        _ => None,
      },
      _ => None,
    })
    .collect()
}

/// Walks a `defineVars` object literal and builds a dependency map:
/// `key -> set of same-group keys that key's arrow function references`.
///
/// Also validates:
/// - Arrow functions must have zero parameters.
/// - Arrow functions must use an expression body, not a block body.
/// - Referenced same-group keys must exist (panics otherwise).
fn build_dependency_map(
  expr: &Expr,
  export_name: &str,
  all_keys: &FxHashSet<String>,
) -> FxHashMap<String, FxHashSet<String>> {
  let obj = match expr.as_object() {
    Some(o) => o,
    None => return FxHashMap::default(),
  };

  let mut dep_map: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();

  for prop in &obj.props {
    let kv = match prop {
      PropOrSpread::Prop(p) => match p.as_ref() {
        Prop::KeyValue(kv) => kv,
        _ => continue,
      },
      _ => continue,
    };

    let arrow = match kv.value.as_ref() {
      Expr::Arrow(a) => a,
      _ => continue,
    };

    let key = match prop_name_to_string(&kv.key) {
      Some(k) => k,
      None => continue,
    };

    // Validate: zero-argument arrow functions only.
    let has_params = arrow.params.iter().any(|p| !matches!(p, Pat::Invalid(_)));
    if !arrow.params.is_empty() && has_params {
      stylex_panic!("{}", invalid_define_vars_function_value());
    }

    // Validate: expression body only (no block statements).
    let body_expr = match arrow.body.as_ref() {
      BlockStmtOrExpr::Expr(e) => e,
      BlockStmtOrExpr::BlockStmt(_) => {
        stylex_panic!("{}", invalid_define_vars_function_value());
      },
    };

    // Collect which same-group keys this arrow function references.
    let deps = collect_arrow_fn_deps(body_expr, export_name);

    if deps.is_empty() {
      continue;
    }

    for dep in &deps {
      if !all_keys.contains(dep) {
        stylex_panic!("{}", unknown_define_vars_reference(&key, dep));
      }
    }

    dep_map.insert(key, deps);
  }

  dep_map
}

/// Recursively walks an expression and collects the names of all same-group
/// vars referenced as `exportName.key` member expressions.
fn collect_arrow_fn_deps(expr: &Expr, export_name: &str) -> FxHashSet<String> {
  let mut deps = FxHashSet::default();
  collect_deps_recursive(expr, export_name, &mut deps);
  deps
}

fn collect_deps_recursive(expr: &Expr, export_name: &str, deps: &mut FxHashSet<String>) {
  match expr {
    Expr::Member(member) => {
      if let Expr::Ident(obj_ident) = member.obj.as_ref() {
        if obj_ident.sym.as_ref() == export_name {
          match &member.prop {
            MemberProp::Ident(prop_ident) => {
              deps.insert(prop_ident.sym.to_string());
            },
            MemberProp::Computed(computed) => {
              if let Expr::Lit(Lit::Str(s)) = computed.expr.as_ref() {
                if let Some(s) = s.value.as_str() {
                  deps.insert(s.to_string());
                }
              }
            },
            MemberProp::PrivateName(_) => {},
          }
        }
      }
      // Also recurse into the object in case of chained access.
      collect_deps_recursive(&member.obj, export_name, deps);
    },
    Expr::Tpl(tpl) => {
      for e in &tpl.exprs {
        collect_deps_recursive(e, export_name, deps);
      }
    },
    Expr::Call(call) => {
      for arg in &call.args {
        collect_deps_recursive(&arg.expr, export_name, deps);
      }
      if let Callee::Expr(callee) = &call.callee {
        collect_deps_recursive(callee, export_name, deps);
      }
    },
    Expr::Bin(bin) => {
      collect_deps_recursive(&bin.left, export_name, deps);
      collect_deps_recursive(&bin.right, export_name, deps);
    },
    Expr::Paren(paren) => collect_deps_recursive(&paren.expr, export_name, deps),
    Expr::Object(obj) => {
      for prop in &obj.props {
        if let PropOrSpread::Prop(p) = prop {
          if let Prop::KeyValue(kv) = p.as_ref() {
            collect_deps_recursive(&kv.value, export_name, deps);
          }
        }
      }
    },
    Expr::Arrow(arrow) => {
      if let BlockStmtOrExpr::Expr(body) = arrow.body.as_ref() {
        collect_deps_recursive(body, export_name, deps);
      }
    },
    _ => {},
  }
}

/// Extracts a string key from a `PropName`.
fn prop_name_to_string(prop_name: &PropName) -> Option<String> {
  match prop_name {
    PropName::Ident(ident) => Some(ident.sym.to_string()),
    PropName::Str(s) => s.value.as_str().map(str::to_string),
    PropName::Num(n) => Some(n.value.to_string()),
    _ => None,
  }
}

/// DFS-based cycle detection on the dependency graph.
/// Panics with a descriptive error message if a cycle is found.
fn assert_no_define_vars_cycles(dependency_map: &FxHashMap<String, FxHashSet<String>>) {
  let mut visited: FxHashSet<String> = FxHashSet::default();
  let mut in_stack: FxHashSet<String> = FxHashSet::default();

  // Sort keys for deterministic error messages across platforms.
  let mut keys: Vec<&String> = dependency_map.keys().collect();
  keys.sort();

  for key in keys {
    if !visited.contains(key) {
      let mut stack: Vec<String> = Vec::new();
      if detect_cycle(key, dependency_map, &mut visited, &mut in_stack, &mut stack) {
        let back_edge = match stack.last() {
          Some(e) => e.clone(),
          #[cfg_attr(coverage_nightly, coverage(off))]
          None => return,
        };
        if let Some(cycle_start) = stack.iter().position(|k| k == &back_edge) {
          let cycle_path: Vec<&str> = stack[cycle_start..].iter().map(|s| s.as_str()).collect();
          stylex_panic!("{}", cyclic_define_vars_reference(&cycle_path.join(" -> ")));
        }
      }
    }
  }
}

/// Recursive DFS helper for cycle detection. Returns `true` if a cycle is found.
fn detect_cycle(
  node: &str,
  dependency_map: &FxHashMap<String, FxHashSet<String>>,
  visited: &mut FxHashSet<String>,
  in_stack: &mut FxHashSet<String>,
  stack: &mut Vec<String>,
) -> bool {
  visited.insert(node.to_string());
  in_stack.insert(node.to_string());
  stack.push(node.to_string());

  if let Some(deps) = dependency_map.get(node) {
    let mut sorted_deps: Vec<&String> = deps.iter().collect();
    sorted_deps.sort();

    for dep in sorted_deps {
      if !visited.contains(dep.as_str()) {
        if detect_cycle(dep, dependency_map, visited, in_stack, stack) {
          return true;
        }
      } else if in_stack.contains(dep.as_str()) {
        stack.push(dep.to_string());
        return true;
      }
    }
  }

  in_stack.remove(node);
  stack.pop();
  false
}
