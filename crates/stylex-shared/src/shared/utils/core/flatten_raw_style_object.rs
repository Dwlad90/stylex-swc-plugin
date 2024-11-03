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

pub(crate) fn flatten_raw_style_object(
  style: &[KeyValueProp],
  pseudos: &mut Vec<String>,
  at_rules: &mut Vec<String>,
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
      flattened.insert(
        key.clone(),
        PreRules::PreIncludedStylesRule(PreIncludedStylesRule::new(
          property.value.as_ref().clone(),
        )),
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

          let pre_rule_value = if values.len() == 1 {
            PreRuleValue::String(values[0].clone())
          } else {
            PreRuleValue::Vec(values)
          };

          let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
            property.as_str(),
            pre_rule_value,
            Some(pseudos.clone()),
            Some(at_rules.clone()),
          ));
          flattened.insert(property, pre_rule);
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
        let handled_tpl = handle_tpl_to_expression(tpl, traversal_state, fns);
        let result = expr_tpl_to_string(handled_tpl.as_tpl().unwrap(), state, traversal_state, fns);

        let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
          css_property_key.as_str(),
          PreRuleValue::String(result),
          Some(pseudos.clone()),
          Some(at_rules.clone()),
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

          let inner_flattened = flatten_raw_style_object(
            &[property_cloned],
            pseudos,
            at_rules,
            state,
            traversal_state,
            fns,
          );

          flattened.extend(inner_flattened);
        } else {
          panic!("{}", NON_STATIC_VALUE)
        }
      }
      Expr::Bin(bin) => {
        let result = transform_bin_expr_to_number(bin, state, traversal_state, fns);

        let mut property_cloned = property.clone();
        property_cloned.value = Box::new(number_to_expression(result));

        let inner_flattened = flatten_raw_style_object(
          &[property_cloned],
          pseudos,
          at_rules,
          state,
          traversal_state,
          fns,
        );

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
                let mut pseudos_to_pass_down = pseudos.clone();
                let mut at_rules_to_pass_down = at_rules.clone();

                if condition.starts_with(':') {
                  pseudos_to_pass_down.push(condition.clone());
                } else if condition.starts_with('@') {
                  at_rules_to_pass_down.push(condition.clone());
                }

                inner_key_value.key = PropName::Str(quote_str!(css_property_key.clone()));

                let pairs = flatten_raw_style_object(
                  &[inner_key_value],
                  &mut pseudos_to_pass_down,
                  &mut at_rules_to_pass_down,
                  state,
                  traversal_state,
                  fns,
                );

                for (property, pre_rule) in pairs {
                  equivalent_pairs
                    .entry(property)
                    .or_default()
                    .insert(condition.clone(), pre_rule);
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
            traversal_state,
            fns,
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
