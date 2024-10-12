use std::collections::VecDeque;

use indexmap::{IndexMap, IndexSet};

use crate::shared::{
  constants::common::COMPILED_KEY,
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
  },
  structures::{
    functions::FunctionMap,
    injectable_style::InjectableStyle,
    pre_rule::{CompiledResult, ComputedStyle, PreRule, PreRules},
    state_manager::StateManager,
    types::{ClassPathsInNamespace, FlatCompiledStyles},
  },
  utils::{
    ast::convertors::expr_to_str, core::flatten_raw_style_object::flatten_raw_style_object,
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
  IndexMap<String, Box<ClassPathsInNamespace>>,
) {
  let mut resolved_namespaces: IndexMap<String, Box<FlatCompiledStyles>> = IndexMap::new();
  let mut injected_styles_map: IndexMap<String, Box<InjectableStyle>> = IndexMap::new();
  let mut namespace_to_class_paths: IndexMap<String, Box<ClassPathsInNamespace>> = IndexMap::new();

  for (namespace_name, namespace) in namespaces.as_map().unwrap() {
    let mut class_paths_in_namespace: ClassPathsInNamespace = IndexMap::new();

    validate_namespace(namespace, &[]);

    let mut key_path = vec![];

    let mut seen_properties = IndexSet::<String>::new();

    let mut flattened_namespace =
      flatten_raw_style_object(namespace, &mut key_path, state, functions)
        .into_iter()
        .rev()
        .fold(VecDeque::new(), |mut arr, curr| {
          if !seen_properties.contains(&curr.0) {
            seen_properties.insert(curr.0.clone());
            arr.push_front(curr);
          }
          arr
        });

    let compiled_namespace_tuples = flattened_namespace
      .iter_mut()
      .map(|(key, value)| {
        let key = key.clone();

        let compiled_value = match value {
          PreRules::PreRuleSet(rule_set) => rule_set.compiled(state),
          PreRules::StylesPreRule(styles_pre_rule) => styles_pre_rule.compiled(state),
          PreRules::NullPreRule(rule_set) => rule_set.compiled(state),
          PreRules::PreIncludedStylesRule(pre_included_styles_rule) => {
            pre_included_styles_rule.compiled(state)
          }
        };
        (key, compiled_value)
      })
      .collect::<Vec<(String, CompiledResult)>>();

    let mut namespace_obj: FlatCompiledStyles = IndexMap::new();

    for (key, value) in compiled_namespace_tuples {
      if let Some(included_styles) = value.as_included_style() {
        // stylex.include calls are passed through as-is.
        namespace_obj.insert(
          key.clone(),
          Box::new(FlatCompiledStylesValue::IncludedStyle(
            included_styles.clone(),
          )),
        );
      } else if let Some(class_name_tuples) = value.as_computed_styles() {
        for ComputedStyle(_class_name, _, classes_to_original_path) in class_name_tuples.iter() {
          class_paths_in_namespace.extend(classes_to_original_path.clone());
        }

        let class_name = class_name_tuples
          .iter()
          .map(|ComputedStyle(name, _, _)| name.as_str())
          .collect::<Vec<&str>>()
          .join(" ");

        namespace_obj.insert(
          key.clone(),
          Box::new(FlatCompiledStylesValue::String(class_name.clone())),
        );

        for ComputedStyle(class_name, injectable_styles, _) in class_name_tuples.iter() {
          injected_styles_map
            .entry(class_name.clone())
            .or_insert_with(|| Box::new(injectable_styles.clone()));
        }
      } else {
        namespace_obj.insert(key.clone(), Box::new(FlatCompiledStylesValue::Null));
      }
    }
    let resolved_namespace_name = expr_to_str(namespace_name, state, functions);

    namespace_obj.insert(
      COMPILED_KEY.to_owned(),
      Box::new(FlatCompiledStylesValue::Bool(true)),
    );

    resolved_namespaces.insert(resolved_namespace_name.clone(), Box::new(namespace_obj));

    namespace_to_class_paths.insert(resolved_namespace_name, Box::new(class_paths_in_namespace));
  }

  (
    resolved_namespaces,
    injected_styles_map,
    namespace_to_class_paths,
  )
}
