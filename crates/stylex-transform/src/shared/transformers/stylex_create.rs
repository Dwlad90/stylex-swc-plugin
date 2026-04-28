use std::{collections::VecDeque, rc::Rc};

use indexmap::{IndexMap, IndexSet};
use stylex_macros::stylex_panic;

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
  },
  structures::{
    functions::FunctionMap,
    pre_rule::{CompiledResult, ComputedStyle, PreRule, PreRules},
    state::EvaluationState,
    state_manager::StateManager,
    types::{
      ClassPathsInNamespace, ClassPathsMap, FlatCompiledStyles, InjectableStylesMap, RuleKey,
      StylesObjectMap,
    },
  },
  utils::{
    ast::convertors::convert_expr_to_str, core::flatten_raw_style_object::flatten_raw_style_object,
    validators::validate_namespace,
  },
};
use stylex_constants::constants::{
  common::COMPILED_KEY,
  messages::{EXPRESSION_IS_NOT_A_STRING, VALUES_MUST_BE_OBJECT},
};
use stylex_types::enums::data_structures::injectable_style::InjectableStyleKind;
use stylex_utils::hash::create_short_hash;

pub(crate) fn stylex_create_set(
  namespaces: &EvaluateResultValue,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  functions: &FunctionMap,
) -> (StylesObjectMap, InjectableStylesMap, ClassPathsMap) {
  let mut resolved_namespaces = IndexMap::new();
  let mut injected_styles_map = IndexMap::new();
  let mut namespace_to_class_paths = IndexMap::new();

  for (namespace_name, namespace) in match namespaces.as_map() {
    Some(map) => map,
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("{}", VALUES_MUST_BE_OBJECT),
  } {
    validate_namespace(namespace, &[], traversal_state);

    let mut class_paths_in_namespace: ClassPathsInNamespace = IndexMap::new();

    let mut seen_properties = IndexSet::<String>::new();

    let mut flattened_namespace =
      flatten_raw_style_object(namespace, state, traversal_state, functions)
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
          let mut unique_class_names = IndexSet::new();

          for ComputedStyle(class_name, _, _) in class_name_tuples.iter() {
            unique_class_names.insert(class_name.as_str());
          }

          let class_name = unique_class_names
            .iter()
            .cloned()
            .collect::<Vec<&str>>()
            .join(" ");

          namespace_obj.insert(
            key,
            if !class_name.is_empty() {
              Rc::new(FlatCompiledStylesValue::String(class_name))
            } else {
              Rc::new(FlatCompiledStylesValue::Null)
            },
          );

          for ComputedStyle(class_name, injectable_styles, classes_to_original_path) in
            class_name_tuples.into_iter()
          {
            class_paths_in_namespace.extend(
              classes_to_original_path
                .into_iter()
                .map(|(class_name, original_path)| (class_name.into_string(), original_path)),
            );
            injected_styles_map
              .entry(RuleKey::from(class_name.into_string()))
              .or_insert_with(move || Rc::new(InjectableStyleKind::Regular(injectable_styles)));
          }
        },
        _ => {
          namespace_obj.insert(key, Rc::new(FlatCompiledStylesValue::Null));
        },
      }
    }

    let resolved_namespace_name =
      match convert_expr_to_str(namespace_name, traversal_state, functions) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", EXPRESSION_IS_NOT_A_STRING),
      };

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
