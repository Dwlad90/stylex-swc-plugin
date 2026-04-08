use rustc_hash::{FxHashMap, FxHashSet};
use stylex_constants::constants::{
  api_names::STYLEX_DEFINE_VARS,
  messages::{
    cyclic_define_vars_reference, invalid_define_vars_function_value, non_static_value,
    unknown_define_vars_reference,
  },
};
use stylex_macros::stylex_panic;
use swc_core::{
  atoms::Atom,
  ecma::{
    ast::{
      ArrowExpr, BlockStmtOrExpr, CallExpr, Expr, KeyValueProp, Lit, MemberExpr, MemberProp,
      ObjectLit, Pat, Prop, PropOrSpread,
    },
    visit::{Visit, VisitWith},
  },
};

use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::{
    ast::helpers::{namespace_name_from_prop_key, prop_as_key_value, prop_contains_arrow},
    js::evaluate::evaluate,
    log::build_code_frame_error::build_code_frame_error,
  },
};

/// Walks the `defineVars` object once and collects:
/// (1) the set of top-level keys, and
/// (2) the dependency map `key -> set of same-group keys referenced in its arrow body`.
///
/// Also validates:
/// - Arrow functions must have zero parameters.
/// - Arrow functions must use an expression body, not a block body.
/// - Referenced same-group keys must exist (panics with `unknown_define_vars_reference`).
///
/// Returns `(all_keys, dependency_map)`. Fuses what used to be two separate passes.
pub(super) fn collect_keys_and_dependencies(
  expr: &Expr,
  export_name: &str,
) -> (FxHashSet<Atom>, FxHashMap<Atom, FxHashSet<Atom>>) {
  let mut all_keys: FxHashSet<Atom> = FxHashSet::default();
  let mut dep_map: FxHashMap<Atom, FxHashSet<Atom>> = FxHashMap::default();

  let obj = match expr.as_object() {
    Some(o) => o,
    None => return (all_keys, dep_map),
  };

  // First pass over top-level props: collect keys + validate + buffer arrow refs.
  // We need `all_keys` populated before the unknown-ref check, so we do a small
  // two-step over the same prop list. Each step is O(props.len()).
  let mut arrow_props: Vec<(Atom, &ArrowExpr)> = Vec::with_capacity(obj.props.len());

  for prop in &obj.props {
    let Some(kv) = prop_as_key_value(prop) else {
      continue;
    };

    let Some(key) = namespace_name_from_prop_key(&kv.key) else {
      continue;
    };
    all_keys.insert(key.clone());

    if let Expr::Arrow(arrow) = kv.value.as_ref() {
      // Validate: zero-argument arrow functions only. An empty `params` vector OR
      // a vector containing only `Pat::Invalid` placeholders both count as zero args.
      if arrow.params.iter().any(|p| !matches!(p, Pat::Invalid(_))) {
        stylex_panic!("{}", invalid_define_vars_function_value());
      }
      // Validate: expression body only (no block statements).
      if let BlockStmtOrExpr::BlockStmt(_) = arrow.body.as_ref() {
        stylex_panic!("{}", invalid_define_vars_function_value());
      }
      arrow_props.push((key, arrow));
    }
  }

  for (key, arrow) in arrow_props {
    let BlockStmtOrExpr::Expr(body_expr) = arrow.body.as_ref() else {
      continue; // Already validated above.
    };

    let mut collector = DependencyVisitor {
      export_name,
      deps: FxHashSet::default(),
    };
    body_expr.visit_with(&mut collector);

    if collector.deps.is_empty() {
      continue;
    }

    for dep in &collector.deps {
      if !all_keys.contains(dep) {
        stylex_panic!("{}", unknown_define_vars_reference(&key, dep));
      }
    }

    dep_map.insert(key, collector.deps);
  }

  (all_keys, dep_map)
}

/// SWC `Visit` implementation that walks any expression sub-tree and records
/// every `exportName.key` member access (both `obj.prop` and `obj["prop"]`).
/// Uses `Visit` (read-only) for completeness: SWC automatically dispatches to
/// `visit_children_with` for sub-nodes we don't override, so all expression
/// kinds (Cond, Logical, Array, Seq, TaggedTpl, New, …) are covered.
struct DependencyVisitor<'a> {
  export_name: &'a str,
  deps: FxHashSet<Atom>,
}

impl<'a> Visit for DependencyVisitor<'a> {
  fn visit_member_expr(&mut self, member: &MemberExpr) {
    if let Expr::Ident(obj_ident) = member.obj.as_ref()
      && obj_ident.sym.as_ref() == self.export_name
    {
      match &member.prop {
        MemberProp::Ident(prop_ident) => {
          self.deps.insert(prop_ident.sym.clone());
        },
        MemberProp::Computed(computed) => {
          if let Expr::Lit(Lit::Str(s)) = computed.expr.as_ref()
            && let Some(s) = s.value.as_str()
          {
            self.deps.insert(Atom::from(s));
          }
        },
        MemberProp::PrivateName(_) => {},
      }
    }
    member.visit_children_with(self);
  }
}

/// DFS-based cycle detection on the dependency graph.
/// Panics with `cyclic_define_vars_reference` if a cycle is found.
pub(super) fn assert_no_define_vars_cycles(dependency_map: &FxHashMap<Atom, FxHashSet<Atom>>) {
  let mut visited: FxHashSet<Atom> = FxHashSet::default();
  let mut in_stack: FxHashSet<Atom> = FxHashSet::default();

  // Sort keys for deterministic error messages across platforms.
  let mut keys: Vec<&Atom> = dependency_map.keys().collect();
  keys.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));

  for key in keys {
    if !visited.contains(key) {
      let mut stack: Vec<Atom> = Vec::new();
      if detect_cycle(key, dependency_map, &mut visited, &mut in_stack, &mut stack) {
        let back_edge = match stack.last() {
          Some(e) => e.clone(),
          #[cfg_attr(coverage_nightly, coverage(off))]
          None => return,
        };
        if let Some(cycle_start) = stack.iter().position(|k| k == &back_edge) {
          let cycle_path: Vec<&str> = stack[cycle_start..].iter().map(|s| s.as_ref()).collect();
          stylex_panic!("{}", cyclic_define_vars_reference(&cycle_path.join(" -> ")));
        }
      }
    }
  }
}

/// Recursive DFS helper for cycle detection. Returns `true` if a cycle is found.
fn detect_cycle(
  node: &Atom,
  dependency_map: &FxHashMap<Atom, FxHashSet<Atom>>,
  visited: &mut FxHashSet<Atom>,
  in_stack: &mut FxHashSet<Atom>,
  stack: &mut Vec<Atom>,
) -> bool {
  visited.insert(node.clone());
  in_stack.insert(node.clone());
  stack.push(node.clone());

  if let Some(deps) = dependency_map.get(node) {
    let mut sorted_deps: Vec<&Atom> = deps.iter().collect();
    sorted_deps.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));

    for dep in sorted_deps {
      if !visited.contains(dep) {
        if detect_cycle(dep, dependency_map, visited, in_stack, stack) {
          return true;
        }
      } else if in_stack.contains(dep) {
        stack.push(dep.clone());
        return true;
      }
    }
  }

  in_stack.remove(node);
  stack.pop();
  false
}

/// Walks the evaluated `defineVars` object and expands zero-param arrow function
/// values by evaluating their bodies (mirrors `normalizeDefineVarsObject` from the
/// TypeScript implementation). Nested arrow functions (i.e. arrows appearing
/// anywhere below the top-level property value) are rejected — only the top-level
/// `key: () => …` shape is supported, matching the TypeScript behaviour where
/// `normalizeDefineVarsValue` is called recursively with `allowCSSType = false`
/// and a function value at depth > 0 triggers `invalidDefineVarsFunctionValue`.
///
/// Returns the input `value` unchanged when no rewriting is necessary, avoiding
/// the per-property clone on the warm path.
pub(super) fn normalize_define_vars_functions(
  value: EvaluateResultValue,
  state: &mut StateManager,
  function_map: &FunctionMap,
  call: &CallExpr,
  first_arg: &Expr,
) -> EvaluateResultValue {
  // Borrow the object literal without cloning until we know we have rewriting to do.
  let needs_rewrite = match value.as_expr().and_then(|e| e.as_object()) {
    Some(obj) => obj.props.iter().any(prop_contains_arrow),
    None => return value,
  };
  if !needs_rewrite {
    return value;
  }

  let obj = match value.as_expr().and_then(|e| e.as_object()) {
    Some(o) => o.clone(),
    None => return value,
  };

  let mut props: Vec<PropOrSpread> = Vec::with_capacity(obj.props.len());

  for prop in &obj.props {
    // Short-circuit any prop that we know we will copy unchanged.
    let Some(kv) = prop_as_key_value(prop) else {
      props.push(prop.clone());
      continue;
    };

    let new_value_expr: Expr = match kv.value.as_ref() {
      Expr::Arrow(arrow) if arrow.params.is_empty() => {
        let body_expr = match arrow.body.as_ref() {
          BlockStmtOrExpr::Expr(e) => e.as_ref(),
          BlockStmtOrExpr::BlockStmt(_) => {
            stylex_panic!("{}", invalid_define_vars_function_value());
          },
        };
        let result = evaluate(body_expr, state, function_map);
        if !result.confident {
          let deopt = result.deopt.clone().unwrap_or_else(|| first_arg.clone());
          stylex_panic!(
            "{}",
            build_code_frame_error(
              &Expr::Call(call.clone()),
              &deopt,
              &non_static_value(STYLEX_DEFINE_VARS),
              state,
            )
          );
        }
        match result.value {
          Some(EvaluateResultValue::Expr(expr)) => {
            // Reject nested arrows in the evaluated body too.
            assert_no_nested_arrows(&expr);
            expr
          },
          #[cfg_attr(coverage_nightly, coverage(off))]
          _ => stylex_panic!("{}", non_static_value(STYLEX_DEFINE_VARS)),
        }
      },
      Expr::Arrow(_) => {
        // Arrow with params — should have been caught by collect_keys_and_dependencies,
        // but guard here in case the walker missed it.
        stylex_panic!("{}", invalid_define_vars_function_value());
      },
      other => {
        // Reject nested arrows that appear inside non-arrow top-level values.
        assert_no_nested_arrows(other);
        other.clone()
      },
    };

    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: kv.key.clone(),
      value: Box::new(new_value_expr),
    }))));
  }

  EvaluateResultValue::Expr(Expr::Object(ObjectLit {
    span: obj.span,
    props,
  }))
}

/// Panics with `invalid_define_vars_function_value` if any `Expr::Arrow` appears
/// inside `expr` (after evaluation). Mirrors TypeScript's recursive call to
/// `normalizeDefineVarsValue` with `allowCSSType = false`, which rejects
/// `typeof value === 'function'` at depth > 0.
fn assert_no_nested_arrows(expr: &Expr) {
  if let Expr::Object(obj) = expr {
    for prop in &obj.props {
      if let PropOrSpread::Prop(p) = prop
        && let Prop::KeyValue(kv) = p.as_ref()
      {
        if matches!(kv.value.as_ref(), Expr::Arrow(_)) {
          stylex_panic!("{}", invalid_define_vars_function_value());
        }
        assert_no_nested_arrows(&kv.value);
      }
    }
  }
}
