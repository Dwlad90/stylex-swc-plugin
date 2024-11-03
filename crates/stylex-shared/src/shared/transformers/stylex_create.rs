use std::rc::Rc;

use indexmap::IndexMap;

use crate::shared::{
  constants::common::COMPILED_KEY,
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
  },
  structures::{
    functions::FunctionMap,
    injectable_style::InjectableStyle,
    pre_rule::{CompiledResult, ComputedStyle, PreRule, PreRules},
    state::EvaluationState,
    state_manager::StateManager,
    types::FlatCompiledStyles,
  },
  utils::{
    ast::convertors::expr_to_str, core::flatten_raw_style_object::flatten_raw_style_object,
    validators::validate_namespace,
  },
};

pub(crate) fn stylex_create_set(
  namespaces: &EvaluateResultValue,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  functions: &FunctionMap,
) -> (
  IndexMap<String, Box<FlatCompiledStyles>>,
  IndexMap<String, Rc<InjectableStyle>>,
) {
  let mut resolved_namespaces: IndexMap<String, Box<FlatCompiledStyles>> = IndexMap::new();
  let mut injected_styles_map: IndexMap<String, Rc<InjectableStyle>> = IndexMap::new();

  for (namespace_name, namespace) in namespaces.as_map().unwrap() {
    validate_namespace(namespace, &[]);

    let mut pseudos: Vec<String> = vec![];
    let mut at_rules: Vec<String> = vec![];

    let mut flattened_namespace = flatten_raw_style_object(
      namespace,
      &mut pseudos,
      &mut at_rules,
      state,
      traversal_state,
      functions,
    );

    let compiled_namespace_tuples = flattened_namespace
      .iter_mut()
      .map(|(key, value)| match value {
        PreRules::PreRuleSet(rule_set) => (key.to_string(), rule_set.compiled(traversal_state)),
        PreRules::StylesPreRule(styles_pre_rule) => {
          (key.to_string(), styles_pre_rule.compiled(traversal_state))
        }
        PreRules::NullPreRule(rule_set) => (key.to_string(), rule_set.compiled(traversal_state)),
        PreRules::PreIncludedStylesRule(pre_included_tyles_rule) => (
          key.to_string(),
          pre_included_tyles_rule.compiled(traversal_state),
        ),
      })
      .collect::<Vec<(String, CompiledResult)>>();

    let compiled_namespace = compiled_namespace_tuples
      .iter()
      .map(|(key, value)| {
        (
          key.as_str(),
          match value {
            CompiledResult::ComputedStyles(styles) => {
              CompiledResult::ComputedStyles(styles.clone())
            }
            CompiledResult::Null => CompiledResult::Null,
            CompiledResult::IncludedStyle(include_styles) => {
              CompiledResult::IncludedStyle(include_styles.clone())
            }
          },
        )
      })
      .collect::<IndexMap<&str, CompiledResult>>();

    let mut namespace_obj: FlatCompiledStyles = IndexMap::new();

    for key in compiled_namespace.keys() {
      let value = compiled_namespace.get(key).unwrap();

      if let Some(included_styles) = value.as_included_style() {
        namespace_obj.insert(
          (*key).to_string(),
          Box::new(FlatCompiledStylesValue::IncludedStyle(
            included_styles.clone(),
          )),
        );
      } else if let Some(class_name_tuples) = value.as_computed_styles() {
        let class_name = class_name_tuples
          .iter()
          .map(|ComputedStyle(name, _)| name.as_str())
          .collect::<Vec<&str>>()
          .join(" ");

        namespace_obj.insert(
          (*key).to_string(),
          Box::new(FlatCompiledStylesValue::String(class_name.clone())),
        );

        for ComputedStyle(class_name, injectable_styles) in class_name_tuples.iter() {
          injected_styles_map
            .entry(class_name.clone())
            .or_insert_with(|| Rc::new(injectable_styles.clone()));
        }
      } else {
        namespace_obj.insert((*key).to_string(), Box::new(FlatCompiledStylesValue::Null));
      }
    }
    let resolved_namespace_name = expr_to_str(namespace_name, traversal_state, functions);

    namespace_obj.insert(
      COMPILED_KEY.to_owned(),
      Box::new(FlatCompiledStylesValue::Bool(true)),
    );

    resolved_namespaces.insert(resolved_namespace_name, Box::new(namespace_obj));
  }

  (resolved_namespaces, injected_styles_map)
}
