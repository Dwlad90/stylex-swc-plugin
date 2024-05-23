use indexmap::IndexMap;

use crate::shared::{
  constants::common::COMPILED_KEY,
  enums::FlatCompiledStylesValue,
  structures::{
    evaluate_result::EvaluateResultValue,
    functions::FunctionMap,
    injectable_style::InjectableStyle,
    pre_rule::{CompiledResult, PreRule, PreRules},
    state_manager::StateManager,
    types::FlatCompiledStyles,
  },
  utils::{
    common::expr_to_str, css::stylex::flatten_raw_style_object::flatten_raw_style_object,
    validators::validate_namespace,
  },
};

pub(crate) fn stylex_create_set(
  namespaces: &EvaluateResultValue,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> (
  IndexMap<String, Box<FlatCompiledStyles>>,
  IndexMap<String, Box<InjectableStyle>>,
) {
  let mut resolved_namespaces: IndexMap<String, Box<FlatCompiledStyles>> = IndexMap::new();
  let mut injected_styles_map: IndexMap<String, Box<InjectableStyle>> = IndexMap::new();

  for (namespace_name, namespace) in namespaces.as_map().unwrap() {
    validate_namespace(namespace, &[]);

    let mut pseudos = vec![];
    let mut at_rules = vec![];

    let flattened_namespace =
      flatten_raw_style_object(namespace, &mut pseudos, &mut at_rules, state, functions);

    let compiled_namespace_tuples = flattened_namespace
      .iter()
      .map(|(key, value)| match value {
        PreRules::PreRuleSet(rule_set) => (key.to_string(), rule_set.clone().compiled(state)),
        PreRules::StylesPreRule(styles_pre_rule) => {
          (key.to_string(), styles_pre_rule.clone().compiled(state))
        }
        PreRules::NullPreRule(rule_set) => (key.to_string(), rule_set.clone().compiled(state)),
        PreRules::PreIncludedStylesRule(pre_included_tyles_rule) => (
          key.to_string(),
          pre_included_tyles_rule.clone().compiled(state),
        ),
      })
      .collect::<Vec<(String, CompiledResult)>>();

    let compiled_namespace = compiled_namespace_tuples
      .iter()
      .map(|(key, value)| {
        (
          key.to_string(),
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
      .collect::<IndexMap<String, CompiledResult>>();

    let mut namespace_obj: FlatCompiledStyles = IndexMap::new();

    for key in compiled_namespace.keys() {
      let value = compiled_namespace.get(key).unwrap();

      if let Some(included_styles) = value.as_included_style() {
        namespace_obj.insert(
          key.clone(),
          Box::new(FlatCompiledStylesValue::IncludedStyle(
            included_styles.clone(),
          )),
        );
      } else if let Some(styles) = value.as_computed_styles() {
        let class_name_tuples = styles.clone();

        let class_name = &class_name_tuples
          .iter()
          .map(|computed_style| computed_style.0.clone())
          .collect::<Vec<String>>()
          .join(" ");

        namespace_obj.insert(
          key.clone(),
          Box::new(FlatCompiledStylesValue::String(class_name.clone())),
        );

        for item in &class_name_tuples {
          let class_name = item.0.clone();
          let injectable_styles = item.1.clone();
          if !injected_styles_map.contains_key(class_name.as_str()) {
            injected_styles_map.insert(class_name.clone(), Box::new(injectable_styles.clone()));
          }
        }
      } else {
        namespace_obj.insert(key.clone(), Box::new(FlatCompiledStylesValue::Null));
      }
    }

    let resolved_namespace_name = expr_to_str(namespace_name, state, functions);

    let mut namespace_obj = namespace_obj.clone();

    namespace_obj.insert(
      COMPILED_KEY.to_string(),
      Box::new(FlatCompiledStylesValue::Bool(true)),
    );

    resolved_namespaces.insert(resolved_namespace_name.clone(), Box::new(namespace_obj));
  }

  (resolved_namespaces, injected_styles_map)
}
