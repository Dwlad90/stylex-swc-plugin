use indexmap::IndexMap;
use regex::Regex;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{Expr, KeyValueProp, Prop, PropName, PropOrSpread, Str},
};

use crate::shared::{
  constants::messages::{ILLEGAL_PROP_ARRAY_VALUE, ILLEGAL_PROP_VALUE, NON_STATIC_VALUE},
  enums::misc::VarDeclAction,
  regex::INCLUDED_IDENT_REGEX,
  structures::{
    functions::FunctionMap,
    null_pre_rule::NullPreRule,
    order_pair::OrderPair,
    pre_included_styles_rule::PreIncludedStylesRule,
    pre_rule::{PreRuleValue, PreRules, StylesPreRule},
    pre_rule_set::PreRuleSet,
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

pub(crate) fn flatten_raw_style_object(
  style: &[KeyValueProp],
  pseudos: &mut Vec<String>,
  at_rules: &mut Vec<String>,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> IndexMap<String, PreRules> {
  let mut flattened: IndexMap<String, PreRules> = IndexMap::new();

  for property in style.iter() {
    let key = get_key_str(property);

    let key_regex = Regex::new(r"var\(--[a-z0-9]+\)").unwrap();
    let css_property_key = if key_regex.is_match(&key) {
      key[4..key.len() - 1].to_string()
    } else {
      key.clone()
    };

    if INCLUDED_IDENT_REGEX.is_match(key.as_str()) {
      flattened.insert(
        key.clone(),
        PreRules::PreIncludedStylesRule(PreIncludedStylesRule::new(*property.value.clone())),
      );

      continue;
    }

    match property.value.as_ref() {
      Expr::Array(property_array) => {
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
                  &state.options,
                );

                for OrderPair(property, val) in pairs.iter() {
                  let property = property.to_string();

                  if let Some(val) = val {
                    if equivalent_pairs.get(&property).is_none() {
                      let inner_map = vec![val.clone()];
                      equivalent_pairs.insert(property, inner_map);
                    } else {
                      let inner_map = equivalent_pairs.get_mut(&property).unwrap();
                      inner_map.push(val.clone());
                    }
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
            flattened.insert(property.clone(), PreRules::NullPreRule(NullPreRule::new()));
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

            let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
              property.as_str(),
              pre_rule_value,
              Some(pseudos.clone()),
              Some(at_rules.clone()),
            ));
            flattened.insert(property.clone(), pre_rule);
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
            &state.options,
          );

          for OrderPair(property, pre_rule) in pairs.iter() {
            let property = property.to_string();

            if let Some(pair_value) = pre_rule {
              let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
                property.as_str(),
                PreRuleValue::String(pair_value.to_string()),
                Some(pseudos.clone()),
                Some(at_rules.clone()),
              ));

              flattened.insert(property, pre_rule);
            } else {
              flattened.insert(property, PreRules::NullPreRule(NullPreRule::new()));
            }
          }
        }
      }
      Expr::Tpl(tpl) => {
        let handled_tpl = handle_tpl_to_expression(tpl, state, functions);
        let result = expr_tpl_to_string(handled_tpl.as_tpl().unwrap(), state, functions);

        let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
          css_property_key.as_str(),
          PreRuleValue::String(result),
          Some(pseudos.clone()),
          Some(at_rules.clone()),
        ));

        flattened.insert(css_property_key, pre_rule);
      }
      Expr::Ident(ident) => {
        let ident = get_var_decl_by_ident(ident, state, functions, VarDeclAction::Reduce);

        match ident {
          Some(var_decl) => {
            let var_decl_expr = get_expr_from_var_decl(&var_decl);

            let mut property_cloned = property.clone();
            property_cloned.value = Box::new(var_decl_expr.clone());

            let inner_flattened =
              flatten_raw_style_object(&[property_cloned], pseudos, at_rules, state, functions);

            flattened.extend(inner_flattened);
          }
          None => {
            panic!("{}", NON_STATIC_VALUE)
          }
        }
      }
      Expr::Bin(bin) => {
        let result = transform_bin_expr_to_number(bin, state);

        let mut property_cloned = property.clone();
        property_cloned.value = Box::new(number_to_expression(result));

        let inner_flattened =
          flatten_raw_style_object(&[property_cloned], pseudos, at_rules, state, functions);

        flattened.extend(inner_flattened)
      }
      Expr::Call(_) => panic!("{}", NON_STATIC_VALUE),
      Expr::Object(obj) => {
        if !key.starts_with(':') && !key.starts_with('@') {
          if obj.props.is_empty() {
            return flattened;
          }
          let mut equivalent_pairs: IndexMap<String, IndexMap<String, PreRules>> = IndexMap::new();

          obj.props.clone().into_iter().for_each(|prop| match prop {
            PropOrSpread::Prop(mut prop) => {
              transform_shorthand_to_key_values(&mut prop);

              match prop.as_ref() {
                Prop::KeyValue(key_value) => {
                  let mut inner_key_value: KeyValueProp = key_value.clone();

                  let condition = get_key_str(&inner_key_value);
                  let mut pseudos_to_pass_down = pseudos.clone();
                  let mut at_rules_to_pass_down = at_rules.clone();

                  if condition.starts_with(':') {
                    pseudos_to_pass_down.push(condition.clone());
                  } else if condition.starts_with('@') {
                    at_rules_to_pass_down.push(condition.clone());
                  }

                  inner_key_value.key = PropName::Str(Str {
                    span: DUMMY_SP,
                    value: css_property_key.clone().into(),
                    raw: None,
                  });

                  let pairs = flatten_raw_style_object(
                    &[inner_key_value],
                    &mut pseudos_to_pass_down,
                    &mut at_rules_to_pass_down,
                    state,
                    functions,
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
                }
                _ => panic!("{}", NON_STATIC_VALUE),
              }
            }
            _ => panic!("{}", NON_STATIC_VALUE),
          });
          for (property, obj) in equivalent_pairs.iter() {
            let sorted_keys: Vec<&String> = obj.keys().collect();

            let mut rules: Vec<PreRules> = Vec::new();
            for condition in sorted_keys {
              rules.push(obj[condition].clone());
            }

            // If there are many conditions with `null` values, we will collapse them into a single `null` value.
            // `PreRuleSet::create` takes care of that for us.
            flattened.insert(property.clone(), PreRuleSet::create(rules));
          }
        } else {
          let mut pseudos_to_pass_down = pseudos.clone();
          let mut at_rules_to_pass_down = at_rules.clone();

          if key.starts_with(':') {
            pseudos_to_pass_down.push(key.clone());
          } else if key.starts_with('@') {
            at_rules_to_pass_down.push(key.clone());
          }

          let inner_key_value = get_key_values_from_object(obj);

          let pairs = flatten_raw_style_object(
            &inner_key_value.into_iter().collect::<Vec<KeyValueProp>>(),
            &mut pseudos_to_pass_down,
            &mut at_rules_to_pass_down,
            state,
            functions,
          );

          for (property, pre_rule) in pairs {
            flattened.insert(format!("{}_{}", key, property), pre_rule);
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
