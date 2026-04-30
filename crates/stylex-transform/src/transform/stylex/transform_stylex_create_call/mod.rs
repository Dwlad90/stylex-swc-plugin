mod dynamic_style_functions;
mod helpers;
mod runtime_function_map;
use dynamic_style_functions::apply_dynamic_style_functions;
use helpers::*;
use runtime_function_map::build_runtime_function_map;
use std::{
  fmt::Write,
  rc::Rc,
  sync::{Arc, LazyLock},
};
use stylex_macros::stylex_panic;
use stylex_path_resolver::package_json::PackageJsonExtended;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use swc_core::{
  common::{DUMMY_SP, SyntaxContext, comments::Comments},
  ecma::{
    ast::{
      ArrowExpr, BinaryOp, BlockStmtOrExpr, Bool, CallExpr, Decl, Expr, Lit, ModuleItem, ParenExpr,
      Pat, Prop, PropName, PropOrSpread, Stmt, UnaryOp, VarDecl, VarDeclKind,
    },
    utils::drop_span,
  },
};

use crate::{
  shared::{
    structures::{
      functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType, StylexExprFn},
      pre_rule::PreRuleValue,
      state::EvaluationState,
      state_manager::ImportKind,
      types::{
        FlatCompiledStyles, FunctionMapIdentifiers, FunctionMapMemberExpression,
        InjectableStylesMap,
      },
    },
    transformers::{
      stylex_create::stylex_create_set, stylex_default_marker,
      stylex_first_that_works::stylex_first_that_works, stylex_keyframes::get_keyframes_fn,
      stylex_position_try::get_position_try_fn,
    },
    utils::{
      ast::convertors::{
        convert_atom_to_string, convert_expr_to_str, convert_key_value_to_str,
        convert_lit_to_string, create_null_expr, create_string_expr,
      },
      common::{
        downcast_style_options_to_state_manager, get_key_values_from_object, normalize_expr,
      },
      core::{
        add_source_map_data::add_source_map_data,
        dev_class_name::{convert_to_test_styles, inject_dev_class_names},
        evaluate_stylex_create_arg::evaluate_stylex_create_arg,
        flat_map_expanded_shorthands::flat_map_expanded_shorthands,
        js_to_ast::{NestedStringObject, convert_object_to_ast, remove_objects_with_spreads},
      },
      log::build_code_frame_error::{build_code_frame_error, build_code_frame_error_and_panic},
      validators::{is_create_call, validate_stylex_create},
    },
  },
  transform::StyleXTransform,
};
use stylex_ast::ast::factories::{
  create_array_expression, create_bin_expr, create_cond_expr, create_expr_or_spread,
  create_key_value_prop, create_object_expression, create_prop_from_name,
  create_string_var_declarator, create_var_declarator,
};
use stylex_constants::constants::{
  api_names::{
    STYLEX_CREATE, STYLEX_DEFAULT_MARKER, STYLEX_FIRST_THAT_WORKS, STYLEX_KEYFRAMES,
    STYLEX_POSITION_TRY, STYLEX_WHEN,
  },
  common::COMPILED_KEY,
  messages::{EXPECTED_COMPILED_STYLES, non_static_value},
};
use stylex_css::utils::when as stylex_when;
use stylex_enums::{counter_mode::CounterMode, style_resolution::StyleResolution};
use stylex_regex::regex::VAR_EXTRACTION_REGEX;
use stylex_structures::{
  dynamic_style::DynamicStyle, order_pair::OrderPair, stylex_state_options::StyleXStateOptions,
  top_level_expression::TopLevelExpression, uid_generator::UidGenerator,
};
use stylex_types::{
  enums::data_structures::injectable_style::InjectableStyleKind,
  structures::injectable_style::InjectableStyle,
};

/// Lazily-initialized Arc-wrapped map of stylex.when helper functions.
///
/// Thread-safety: Arc ensures safe sharing across threads; Lazy guarantees
/// one-time initialization. Lifecycle: Initialized on first access, immutable
/// thereafter. Contains pure, stateless transformation functions (ancestor,
/// descendant, etc.) that convert expressions to CSS selectors for relational
/// styling.
static STYLEX_WHEN_MAP: LazyLock<Arc<IndexMap<String, StylexExprFn>>> = LazyLock::new(|| {
  let mut map: IndexMap<String, StylexExprFn> = IndexMap::default();

  map.insert(
    "ancestor".to_string(),
    |expr: Expr, state: &mut dyn stylex_types::traits::StyleOptions| {
      let state = downcast_style_options_to_state_manager(state);
      let expr_str = match convert_expr_to_str(&expr, state, &FunctionMap::default()) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("stylex.when ancestor: expression is not a string"),
      };
      let result = match stylex_when::ancestor(&expr_str, Some(&state.options)) {
        Ok(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        Err(e) => stylex_panic!("stylex.when ancestor error: {}", e),
      };
      create_string_expr(&result)
    },
  );

  map.insert(
    "descendant".to_string(),
    |expr: Expr, state: &mut dyn stylex_types::traits::StyleOptions| {
      let state = downcast_style_options_to_state_manager(state);
      let expr_str = match convert_expr_to_str(&expr, state, &FunctionMap::default()) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("stylex.when descendant: expression is not a string"),
      };
      let result = match stylex_when::descendant(&expr_str, Some(&state.options)) {
        Ok(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        Err(e) => stylex_panic!("stylex.when descendant error: {}", e),
      };
      create_string_expr(&result)
    },
  );

  map.insert(
    "siblingBefore".to_string(),
    |expr: Expr, state: &mut dyn stylex_types::traits::StyleOptions| {
      let state = downcast_style_options_to_state_manager(state);
      let expr_str = match convert_expr_to_str(&expr, state, &FunctionMap::default()) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("stylex.when siblingBefore: expression is not a string"),
      };
      let result = match stylex_when::sibling_before(&expr_str, Some(&state.options)) {
        Ok(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        Err(e) => stylex_panic!("stylex.when siblingBefore error: {}", e),
      };
      create_string_expr(&result)
    },
  );

  map.insert(
    "siblingAfter".to_string(),
    |expr: Expr, state: &mut dyn stylex_types::traits::StyleOptions| {
      let state = downcast_style_options_to_state_manager(state);
      let expr_str = match convert_expr_to_str(&expr, state, &FunctionMap::default()) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("stylex.when siblingAfter: expression is not a string"),
      };
      let result = match stylex_when::sibling_after(&expr_str, Some(&state.options)) {
        Ok(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        Err(e) => stylex_panic!("stylex.when siblingAfter error: {}", e),
      };
      create_string_expr(&result)
    },
  );

  map.insert(
    "anySibling".to_string(),
    |expr: Expr, state: &mut dyn stylex_types::traits::StyleOptions| {
      let state = downcast_style_options_to_state_manager(state);
      let expr_str = match convert_expr_to_str(&expr, state, &FunctionMap::default()) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("stylex.when anySibling: expression is not a string"),
      };
      let result = match stylex_when::any_sibling(&expr_str, Some(&state.options)) {
        Ok(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        Err(e) => stylex_panic!("stylex.when anySibling error: {}", e),
      };
      create_string_expr(&result)
    },
  );

  Arc::new(map)
});

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_create(&mut self, call: &CallExpr) -> Option<Expr> {
    self.state.in_stylex_create = true;
    let mut package_json_seen: FxHashMap<String, PackageJsonExtended> = FxHashMap::default();

    let is_create_call = is_create_call(call, &self.state);

    let result = if is_create_call {
      validate_stylex_create(call, &mut self.state);

      let is_program_level = self
        .state
        .find_top_level_expr(
          call,
          |tpe: &TopLevelExpression| matches!(tpe.1, Expr::Array(_)),
          None,
        )
        .is_some();

      let mut first_arg = call.args.first()?.expr.clone();

      let mut resolved_namespaces: IndexMap<String, Box<FlatCompiledStyles>> = IndexMap::new();
      let function_map = build_runtime_function_map(self);

      let evaluated_arg =
        evaluate_stylex_create_arg(&mut first_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          evaluated_arg
            .reason
            .as_deref()
            .unwrap_or(&non_static_value(STYLEX_CREATE)),
          &mut self.state,
        )
      );

      let value = match evaluated_arg.value {
        Some(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", non_static_value(STYLEX_CREATE)),
      };

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          evaluated_arg
            .reason
            .as_deref()
            .unwrap_or(&non_static_value(STYLEX_CREATE)),
          &mut self.state,
        )
      );

      let mut injected_inherit_styles: InjectableStylesMap = IndexMap::default();

      if let Some(fns) = &evaluated_arg.fns {
        let dynamic_fns_names = fns
          .values()
          .flat_map(|(_, map)| {
            map.keys().map(|k| {
              let path = map.get(k).map(|p| p.path.clone()).unwrap_or_default();

              (k.clone(), path)
            })
          })
          .collect::<Vec<(String, Vec<String>)>>();

        for (variable_name, paths) in dynamic_fns_names {
          // Pseudo elements can only access css vars via inheritance
          let is_pseudo_element = paths.iter().any(|path| path.starts_with(':'));

          injected_inherit_styles.insert(
            variable_name.clone().into(),
            InjectableStyle::regular(
              create_property_rule(&variable_name, is_pseudo_element),
              Some(0f64),
            ),
          );
        }
      }

      let (mut compiled_styles, injected_styles_sans_keyframes, class_paths_per_namespace) =
        stylex_create_set(
          &value,
          &mut EvaluationState::new(),
          &mut self.state,
          &function_map,
        );

      for (namespace, properties) in compiled_styles.iter() {
        resolved_namespaces
          .entry(namespace.clone())
          .or_default()
          .extend(properties.iter().map(|(k, v)| (k.clone(), v.clone())));
      }

      let mut injected_styles = self.state.other_injected_css_rules.clone();

      injected_styles.extend(injected_styles_sans_keyframes);

      injected_styles.extend(injected_inherit_styles);

      let (var_name, parent_var_decl) = self.get_call_var_name(call);

      if self.state.is_debug() && self.state.options.enable_debug_data_prop {
        compiled_styles = add_source_map_data(
          &compiled_styles,
          call,
          &mut self.state,
          &mut package_json_seen,
          &function_map,
        );
      }

      if self.state.is_dev() && self.state.options.enable_dev_class_names {
        compiled_styles = inject_dev_class_names(&compiled_styles, &var_name, &self.state);
      }

      if self.state.is_test() {
        compiled_styles = convert_to_test_styles(&compiled_styles, &var_name, &self.state);
      }

      if is_program_level && let Some(var_name) = var_name.as_ref() {
        let styles_to_remember = remove_objects_with_spreads(&compiled_styles);

        self
          .state
          .style_map
          .insert(var_name.clone(), Rc::new(styles_to_remember));

        if let Some(parent_var_decl) = parent_var_decl {
          self
            .state
            .style_vars
            .insert(var_name.clone(), drop_span(parent_var_decl));
        } else {
          let call_expr = Expr::Call(call.clone());

          build_code_frame_error_and_panic(
            &Expr::Paren(ParenExpr {
              span: DUMMY_SP,
              expr: Box::new(call_expr.clone()),
            }),
            &call_expr,
            "Function type",
            &mut self.state,
          )
        }
      }

      let styles_ast =
        convert_object_to_ast(&NestedStringObject::FlatCompiledStyles(compiled_styles));

      let mut result_ast =
        path_replace_hoisted(styles_ast.clone(), is_program_level, &mut self.state);

      result_ast = apply_dynamic_style_functions(
        self,
        call,
        result_ast,
        evaluated_arg.fns,
        &class_paths_per_namespace,
        &injected_styles,
        is_program_level,
      );

      self.state.register_styles(
        call,
        &injected_styles,
        &result_ast,
        (!result_ast.eq(&styles_ast)).then_some(&styles_ast),
      );

      Some(result_ast)
    } else {
      None
    };

    self.state.in_stylex_create = false;

    result
  }
}
