use std::{collections::VecDeque, rc::Rc};

use indexmap::{IndexMap, IndexSet};

use crate::shared::{
  constants::common::COMPILED_KEY,
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, injectable_style::InjectableStyleKind,
  },
  structures::{
    functions::FunctionMap,
    pre_rule::{CompiledResult, ComputedStyle, PreRule, PreRules},
    state::EvaluationState,
    state_manager::StateManager,
    types::{ClassPathsInNamespace, FlatCompiledStyles},
  },
  utils::{
    ast::convertors::expr_to_str, common::create_short_hash,
    core::flatten_raw_style_object::flatten_raw_style_object, validators::validate_namespace,
  },
};

pub(crate) fn stylex_create_set(
  namespaces: &EvaluateResultValue,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  functions: &FunctionMap,
) -> (
  IndexMap<String, Rc<FlatCompiledStyles>>,
  IndexMap<String, Rc<InjectableStyleKind>>,
  IndexMap<String, Rc<ClassPathsInNamespace>>,
) {
  let mut resolved_namespaces = IndexMap::new();
  let mut injected_styles_map = IndexMap::new();
  let mut namespace_to_class_paths = IndexMap::new();

  for (namespace_name, namespace) in namespaces.as_map().unwrap() {
    validate_namespace(namespace, &[], traversal_state);

    let mut class_paths_in_namespace: ClassPathsInNamespace = IndexMap::new();

    let mut key_path = vec![];

    let mut seen_properties = IndexSet::<String>::new();

    let mut flattened_namespace =
      flatten_raw_style_object(namespace, &mut key_path, state, traversal_state, functions)
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
        let key = if traversal_state.options.enable_minified_keys && !key.starts_with("--") {
          let hashed_key = create_short_hash(&format!("<>{}", key));
          if traversal_state.options.debug {
            format!("{}-k{}", key, hashed_key)
          } else {
            format!("k{}", hashed_key)
          }
        } else {
          key.clone()
        };

        let compiled_value = match value {
          PreRules::PreRuleSet(rule_set) => rule_set.compiled(traversal_state),
          PreRules::StylesPreRule(styles_pre_rule) => styles_pre_rule.compiled(traversal_state),
          PreRules::NullPreRule(rule_set) => rule_set.compiled(traversal_state),
        };
        (key, compiled_value)
      })
      .collect::<Vec<(String, CompiledResult)>>();

    let mut namespace_obj: FlatCompiledStyles = IndexMap::new();

    for (key, value) in compiled_namespace_tuples {
      match value {
        CompiledResult::ComputedStyles(class_name_tuples) => {
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
            Rc::new(FlatCompiledStylesValue::String(class_name.clone())),
          );

          for ComputedStyle(class_name, injectable_styles, _) in class_name_tuples.iter() {
            injected_styles_map
              .entry(class_name.clone())
              .or_insert_with(|| Rc::new(InjectableStyleKind::Regular(injectable_styles.clone())));
          }
        }
        _ => {
          namespace_obj.insert(key.clone(), Rc::new(FlatCompiledStylesValue::Null));
        }
      }
    }

    let resolved_namespace_name = expr_to_str(namespace_name, traversal_state, functions);

    namespace_obj.insert(
      COMPILED_KEY.to_owned(),
      Rc::new(FlatCompiledStylesValue::Bool(true)),
    );

    resolved_namespaces.insert(resolved_namespace_name.clone(), Rc::new(namespace_obj));

    namespace_to_class_paths.insert(resolved_namespace_name, Rc::new(class_paths_in_namespace));
  }

  (
    resolved_namespaces,
    injected_styles_map,
    namespace_to_class_paths,
  )
}
