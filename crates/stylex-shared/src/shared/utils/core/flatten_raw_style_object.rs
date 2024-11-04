use indexmap::IndexMap;
use swc_core::ecma::{
  ast::{Expr, KeyValueProp, Prop, PropName, PropOrSpread},
  utils::quote_str,
};

use crate::shared::{
  constants::messages::{ILLEGAL_PROP_ARRAY_VALUE, ILLEGAL_PROP_VALUE, NON_STATIC_VALUE},
  enums::misc::VarDeclAction,
  regex::{CSS_PROPERTY_KEY, INCLUDED_IDENT_REGEX},
  structures::{
    functions::FunctionMap,
    null_pre_rule::NullPreRule,
    order_pair::OrderPair,
    pre_included_styles_rule::PreIncludedStylesRule,
    pre_rule::{PreRuleValue, PreRules, StylesPreRule},
    pre_rule_set::PreRuleSet,
    state::EvaluationState,
    state_manager::StateManager,
  },
  utils::{
    ast::convertors::{
      expr_tpl_to_string, handle_tpl_to_expression, number_to_expression,
      transform_bin_expr_to_number, transform_shorthand_to_key_values,
    },
    common::{
      get_expr_from_var_decl, get_key_str, get_key_values_from_object, get_string_val_from_lit,
      get_var_decl_by_ident,
    },
  },
};

use super::flat_map_expanded_shorthands::flat_map_expanded_shorthands;

fn normalize_key_path(key_path: Vec<String>, key: &str, property: String) -> Vec<String> {
  if key_path.contains(&key.to_string()) {
    key_path
      .into_iter()
      .map(|k| if k == key { property.clone() } else { k })
      .collect()
  } else {
    let mut new_key_path = key_path.clone();
    new_key_path.push(property);
    new_key_path
  }
}

pub(crate) fn flatten_raw_style_object(
  style: &[KeyValueProp],
  key_path: &mut Vec<String>,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> IndexMap<String, PreRules> {
  let mut flattened: IndexMap<String, PreRules> = IndexMap::new();

  for property in style.iter() {
    let key = get_key_str(property);

    let css_property_key = if CSS_PROPERTY_KEY.is_match(&key) {
      key[4..key.len() - 1].to_string()
    } else {
      key.clone()
    };

    if INCLUDED_IDENT_REGEX.is_match(key.as_str()) {
      let pre_rule =
        PreRules::PreIncludedStylesRule(PreIncludedStylesRule::new(*property.value.clone()));
      insert_or_update_rule_with_shifting_index(&mut flattened, &key, pre_rule);

      continue;
    }

    match property.value.as_ref() {
      Expr::Array(property_array) => {
        // Step 1: Expand properties to its constituent parts
        // Collect the various values for each value in the array
        // that belongs to the same property.

        let mut equivalent_pairs: IndexMap<String, Vec<String>> = IndexMap::new();

        property_array.elems.iter().for_each(|each_val| {
          if let Some(property) = each_val {
            match property.expr.as_ref() {
              Expr::Lit(property_lit) => {
                let pairs = flat_map_expanded_shorthands(
                  (
                    css_property_key.clone(),
                    match get_string_val_from_lit(property_lit) {
                      Some(val) => PreRuleValue::String(val),
                      None => PreRuleValue::Null,
                    },
                  ),
                  &traversal_state.options,
                );

                for OrderPair(property, val) in pairs.iter() {
                  let property = property.to_string();

                  if let Some(val) = val {
                    equivalent_pairs
                      .entry(property)
                      .or_default()
                      .push(val.clone());
                  }
                }
              }
              _ => panic!("{}", ILLEGAL_PROP_ARRAY_VALUE),
            }
          }
        });

        for (property, values) in equivalent_pairs {
          // Remove nulls and deduplicate
          let mut values = values
            .into_iter()
            .filter(|v| !v.is_empty())
            .collect::<Vec<String>>();

          values.dedup();

          if values.is_empty() {
            let pre_rule = PreRules::NullPreRule(NullPreRule::new());

            insert_or_update_rule_with_shifting_index(&mut flattened, &property, pre_rule);
          } else {
            let pre_rule_value = if let Some(first_value) = values.first() {
              if values.len() == 1 {
                PreRuleValue::String(first_value.clone())
              } else {
                PreRuleValue::Vec(values.clone())
              }
            } else {
              PreRuleValue::Null // Default value when `values` is empty.
            };

            let normalized_key_path =
              normalize_key_path(key_path.clone(), key.as_str(), property.clone());

            let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
              property.as_str(),
              pre_rule_value,
              Some(normalized_key_path),
            ));
            insert_or_update_rule_with_shifting_index(&mut flattened, &property, pre_rule);
          }
        }
      }
      Expr::Lit(property_lit) => {
        if !css_property_key.starts_with(':') && !css_property_key.starts_with('@') {
          let value = get_string_val_from_lit(property_lit);

          let pairs = flat_map_expanded_shorthands(
            (
              css_property_key,
              match value {
                Some(val) => PreRuleValue::String(val),
                None => PreRuleValue::Null,
              },
            ),
            &traversal_state.options,
          );

          for OrderPair(property, pre_rule) in pairs.iter() {
            let property = property.to_string();

            if let Some(pair_value) = pre_rule {
              let normalized_key_path =
                normalize_key_path(key_path.clone(), key.as_str(), property.clone());

              let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
                property.as_str(),
                PreRuleValue::String(pair_value.to_string()),
                Some(normalized_key_path),
              ));

              insert_or_update_rule_with_shifting_index(&mut flattened, &property, pre_rule);
            } else {
              let pre_rule = PreRules::NullPreRule(NullPreRule::new());
              insert_or_update_rule_with_shifting_index(&mut flattened, &property, pre_rule);
            }
          }
        }
      }
      Expr::Tpl(tpl) => {
        let handled_tpl = handle_tpl_to_expression(tpl, traversal_state, fns);
        let result = expr_tpl_to_string(handled_tpl.as_tpl().unwrap(), state, traversal_state, fns);

        let normalized_key_path =
          normalize_key_path(key_path.clone(), key.as_str(), css_property_key.clone());

        let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
          css_property_key.as_str(),
          PreRuleValue::String(result),
          Some(normalized_key_path),
        ));

        flattened.insert(css_property_key, pre_rule);
      }
      Expr::Ident(ident) => {
        if let Some(var_decl) =
          get_var_decl_by_ident(ident, traversal_state, fns, VarDeclAction::Reduce)
        {
          let var_decl_expr = get_expr_from_var_decl(&var_decl);

          let mut property_cloned = property.clone();
          property_cloned.value = Box::new(var_decl_expr.clone());

          let inner_flattened =
            flatten_raw_style_object(&[property_cloned], key_path, state, traversal_state, fns);

          flattened.extend(inner_flattened);
        } else {
          panic!("{}", NON_STATIC_VALUE)
        }
      }
      Expr::Bin(bin) => {
        let result = transform_bin_expr_to_number(bin, state, traversal_state, fns);

        let mut property_cloned = property.clone();
        property_cloned.value = Box::new(number_to_expression(result));

        let inner_flattened =
          flatten_raw_style_object(&[property_cloned], key_path, state, traversal_state, fns);

        flattened.extend(inner_flattened)
      }
      Expr::Call(_) => panic!("{}", NON_STATIC_VALUE),
      Expr::Object(obj) => {
        if !key.starts_with(':') && !key.starts_with('@') {
          if obj.props.is_empty() {
            return flattened;
          }
          let mut equivalent_pairs: IndexMap<String, IndexMap<String, PreRules>> = IndexMap::new();

          for prop in obj.clone().props.iter_mut() {
            if let PropOrSpread::Prop(ref mut prop) = prop {
              transform_shorthand_to_key_values(prop);

              if let Prop::KeyValue(key_value) = prop.as_ref() {
                let mut inner_key_value: KeyValueProp = key_value.clone();

                let condition = get_key_str(&inner_key_value);

                inner_key_value.key = PropName::Str(quote_str!(css_property_key.clone()));

                let mut key_path = if !key_path.is_empty() {
                  let mut new_key_path = key_path.clone();
                  new_key_path.push(condition.clone());
                  new_key_path
                } else {
                  vec![key.clone(), condition.clone()]
                };

                let pairs = flatten_raw_style_object(
                  &[inner_key_value],
                  &mut key_path,
                  state,
                  traversal_state,
                  fns,
                );

                for (property, pre_rule) in pairs {
                  if equivalent_pairs.get(&property).is_none() {
                    let mut inner_map = IndexMap::new();
                    inner_map.insert(condition.clone(), pre_rule);
                    equivalent_pairs.insert(property, inner_map);
                  } else {
                    let inner_map = equivalent_pairs.get_mut(&property).unwrap();
                    inner_map.insert(condition.clone(), pre_rule);
                  }
                }
              } else {
                panic!("{}", NON_STATIC_VALUE);
              }
            } else {
              panic!("{}", NON_STATIC_VALUE);
            }
          }

          for (property, obj) in equivalent_pairs.iter() {
            let sorted_keys: Vec<&String> = obj.keys().collect();

            let rules: Vec<PreRules> = sorted_keys
              .iter()
              .map(|condition| obj[*condition].clone())
              .collect();

            flattened.insert(property.clone(), PreRuleSet::create(rules));
          }
        } else {
          let inner_key_value = get_key_values_from_object(obj);

          let mut key_path = key_path.clone();
          key_path.push(key.clone());

          let pairs = flatten_raw_style_object(
            &inner_key_value.into_iter().collect::<Vec<KeyValueProp>>(),
            &mut key_path,
            state,
            traversal_state,
            fns,
          );

          for (property, pre_rule) in pairs {
            insert_or_update_rule_with_shifting_index(
              &mut flattened,
              format!("{}_{}", key, property).as_str(),
              pre_rule,
            );
          }
        }
      }
      _ => {
        panic!("{}", ILLEGAL_PROP_VALUE)
      }
    };
  }

  flattened
}

fn insert_or_update_rule_with_shifting_index(
  flattened: &mut IndexMap<String, PreRules>,
  property: &str,
  pre_rule: PreRules,
) {
  if flattened.get(property).is_some() {
    flattened.shift_remove_entry(property);
  };

  flattened.insert(property.to_string(), pre_rule);
}
