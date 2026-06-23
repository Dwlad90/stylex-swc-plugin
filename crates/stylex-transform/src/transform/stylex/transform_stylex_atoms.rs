use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{Expr, Id, Module},
    visit::VisitMutWith,
  },
};

use crate::{
  StyleXTransform,
  shared::{
    enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
    structures::{
      state::EvaluationState,
      types::{FlatCompiledStyles, InjectableStylesMap},
    },
    transformers::stylex_create::stylex_create_set,
    utils::core::{
      dev_class_name::inject_dev_class_names,
      evaluate_stylex_create_arg::evaluate_stylex_create_arg,
      js_to_ast::{NestedStringObject, convert_object_to_ast},
    },
  },
};
use stylex_ast::ast::factories::{
  create_key_value_prop, create_object_expression, create_string_key_value_prop,
};
use stylex_atoms::transform::{
  AtomCompileResult, AtomFlatValue, Compile, InjectedAtomStyle, create_utility_styles_visitor,
};
use stylex_types::{
  enums::data_structures::injectable_style::InjectableStyleKind,
  structures::{injectable_style::InjectableStyle, style_key::RuleKey},
};

use super::transform_stylex_create_call::{build_runtime_function_map, hoist_expression};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  /// Runs the atoms transform pass over the module, rewriting inline atomic
  /// styles (`css.display.flex`, `css.color(value)`) into compiled style
  /// objects or hoisted dynamic-style calls.
  ///
  /// The `create_utility_styles_visitor` pass executes between the producer
  /// (`stylex.create`) and consumer (`stylex.props`) passes so that atoms are
  /// already compiled when `stylex.props` processes them.
  pub(crate) fn transform_atoms(&mut self, module: &mut Module) {
    if self.state.atom_imports.is_empty() {
      return;
    }

    let mut visitor = create_utility_styles_visitor(self);
    module.visit_mut_with(&mut visitor);
  }
}

impl<C> Compile for StyleXTransform<C>
where
  C: Comments,
{
  fn atom_imports(&self) -> &FxHashMap<Id, String> {
    &self.state.atom_imports
  }

  fn style_x_create_set(&mut self, property: &str, value: &str) -> Option<AtomCompileResult> {
    // Compile `{ __inline__: { [property]: value } }` exactly as `stylex.create`
    // would, reusing the full create pipeline.
    let inner = create_object_expression(vec![create_string_key_value_prop(property, value)]);
    let mut first_arg = create_object_expression(vec![create_key_value_prop("__inline__", inner)]);

    let function_map = build_runtime_function_map(self);

    // Save/restore rather than force `false`, so a future re-entrant caller that
    // was already mid-create is not silently knocked out of create mode. (Today
    // the atoms pass runs between the create and props passes, so the prior
    // value is always `false`.)
    let prev_in_stylex_create = self.state.in_stylex_create;
    self.state.in_stylex_create = true;
    let evaluated = evaluate_stylex_create_arg(&mut first_arg, &mut self.state, &function_map);

    // Bail out gracefully (leaving the original expression for runtime) instead
    // of panicking when the inline style is not statically evaluable. Restore the
    // `in_stylex_create` flag on this early-return path too, so a later pass is
    // not left in create mode.
    let Some(value_result) = evaluated.value else {
      self.state.in_stylex_create = prev_in_stylex_create;
      return None;
    };

    let (mut compiled, injected, _class_paths) = stylex_create_set(
      &value_result,
      &mut EvaluationState::new(),
      &mut self.state,
      &function_map,
    );

    if self.state.is_dev() && self.state.options.enable_dev_class_names {
      compiled = inject_dev_class_names(&compiled, &None, &self.state);
    }
    self.state.in_stylex_create = prev_in_stylex_create;

    let namespace: FlatCompiledStyles = compiled
      .get("__inline__")
      .map(|ns| (**ns).clone())
      .unwrap_or_default();

    let compiled_ast = convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(
      namespace.clone(),
    ));

    let compiled_flat = namespace
      .iter()
      .map(|(key, value)| {
        let flat_value = match value.as_ref() {
          FlatCompiledStylesValue::String(string) => AtomFlatValue::String(string.clone()),
          FlatCompiledStylesValue::Bool(bool_value) => AtomFlatValue::Bool(*bool_value),
          _ => AtomFlatValue::Null,
        };
        (key.clone(), flat_value)
      })
      .collect();

    let injected = injected
      .iter()
      .map(|(rule_key, kind)| {
        let (priority, ltr, rtl) = match kind.as_ref() {
          InjectableStyleKind::Regular(style) => (
            style.priority.unwrap_or(0.0),
            style.ltr.clone(),
            style.rtl.clone(),
          ),
          InjectableStyleKind::Const(style) => (
            style.priority.unwrap_or(0.0),
            style.ltr.clone(),
            style.rtl.clone(),
          ),
        };
        InjectedAtomStyle {
          class_name: rule_key.as_str().to_string(),
          priority,
          ltr,
          rtl,
        }
      })
      .collect();

    Some(AtomCompileResult {
      compiled_ast,
      compiled_flat,
      injected,
    })
  }

  fn register_styles(&mut self, injected: &[InjectedAtomStyle]) {
    if injected.is_empty() {
      return;
    }

    let mut style_map: InjectableStylesMap = IndexMap::new();

    for style in injected {
      let kind = match &style.rtl {
        Some(rtl) => {
          InjectableStyle::with_rtl(style.ltr.clone(), rtl.clone(), Some(style.priority))
        },
        None => InjectableStyle::regular(style.ltr.clone(), Some(style.priority)),
      };

      style_map.insert(RuleKey::from(style.class_name.clone()), kind);
    }

    self.state.register_atom_styles(&style_map);
  }

  fn hoist_expression(&mut self, expr: Expr) -> Expr {
    hoist_expression(expr, &mut self.state)
  }
}
