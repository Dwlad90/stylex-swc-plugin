use indexmap::IndexMap;
use regex::Regex;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{Expr, KeyValueProp, Prop, PropName, PropOrSpread, Str},
};

use crate::shared::{
  constants,
  enums::VarDeclAction,
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
    common::{
      expr_tpl_to_string, get_expr_from_var_decl, get_key_str, get_key_values_from_object,
      get_string_val_from_lit, get_var_decl_by_ident, handle_tpl_to_expression,
      number_to_expression, transform_bin_expr_to_number,
    },
    css::common::flat_map_expanded_shorthands,
  },
};

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
    let css_property_key = if key_regex.is_match(&key.clone()) {
      key[4..key.len() - 1].to_string()
    } else {
      key.clone()
    };

    let value = property.value.as_ref();
    if INCLUDED_IDENT_REGEX.is_match(key.as_str()) {
      flattened.insert(
        key.clone(),
        PreRules::PreIncludedStylesRule(PreIncludedStylesRule::new(value.clone())),
      );

      continue;
    }

    match value {
      Expr::Array(property_array) => {
        let mut equivalent_pairs: IndexMap<String, Vec<String>> = IndexMap::new();

        property_array.elems.iter().for_each(|each_val| {
          match each_val {
            Option::Some(property) => match property.expr.as_ref() {
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

                // let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
                //     css_property_key.clone(),
                //     PreRuleValue::String(get_string_val_from_lit(property_lit)),
                //     pseudos.clone(),
                //     at_rules.clone(),
                // ));

                // flattened.insert(css_property_key.clone(), pre_rule);
              }
              _ => panic!("{}", constants::messages::ILLEGAL_PROP_ARRAY_VALUE),
            },
            _ => {}
          }
        });

        for (property, values) in equivalent_pairs {
          // Remove nulls and deduplicate
          let mut values = values
            .clone()
            .into_iter()
            .filter(|v| !v.is_empty())
            .collect::<Vec<String>>();

          values.dedup();

          if values.is_empty() {
            flattened.insert(property.clone(), PreRules::NullPreRule(NullPreRule::new()));
          } else {
            let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
              property.clone(),
              PreRuleValue::Vec(values),
              pseudos.clone(),
              at_rules.clone(),
            ));

            flattened.insert(css_property_key.clone(), pre_rule);
          }
        }
      }
      Expr::Lit(property_lit) => {
        if !css_property_key.starts_with(":") && !css_property_key.starts_with("@") {
          let value = get_string_val_from_lit(property_lit);

          // let a = css_property_key.clone();
          // let a = a.as_ref();

          // let b = value.clone();

          // let b = b.as_str();
          // let b = Option::Some(b);

          dbg!(&css_property_key, &value);
          // panic!();

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

            dbg!("{:#?}", &pseudos);

            if let Some(pair_value) = pre_rule {
              let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
                property.clone(),
                PreRuleValue::String(pair_value.to_string()),
                pseudos.clone(),
                at_rules.clone(),
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
          css_property_key.clone(),
          PreRuleValue::String(result),
          pseudos.clone(),
          at_rules.clone(),
        ));

        flattened.insert(css_property_key, pre_rule);
      }
      Expr::Ident(ident) => {
        let ident = get_var_decl_by_ident(ident, state, functions, VarDeclAction::Reduce);

        match ident {
          Some(var_decl) => {
            let var_decl_expr = get_expr_from_var_decl(&var_decl);

            let mut k = property.clone();
            k.value = Box::new(var_decl_expr);

            let inner_flattened =
              flatten_raw_style_object(&vec![k], pseudos, at_rules, state, functions);

            println!("!!before_updated_flattened: {:?}", flattened);
            // println!("!!updated_flattened: {:?}", updated_flattened);

            flattened.extend(inner_flattened);
          }
          None => {
            panic!("{}", constants::messages::NON_STATIC_VALUE)
          }
        }
      }
      Expr::Bin(bin) => {
        let result = transform_bin_expr_to_number(bin, state);

        let mut k = property.clone();
        k.value = Box::new(number_to_expression(result as f64).unwrap());

        let inner_flattened =
          flatten_raw_style_object(&vec![k], pseudos, at_rules, state, functions);

        flattened.extend(inner_flattened)
      }
      Expr::Call(_) => panic!("{}", constants::messages::NON_STATIC_VALUE),
      Expr::Object(obj) => {
        if !key.starts_with(":") && !key.starts_with("@") {
          if obj.props.is_empty() {
            println!("!!obj.props.is_empty(): {:?}", flattened);
            return flattened;
          }
          let mut equivalent_pairs: IndexMap<String, IndexMap<String, PreRules>> = IndexMap::new();

          obj.props.iter().for_each(|prop| match prop {
            PropOrSpread::Prop(prop) => match prop.as_ref() {
              Prop::KeyValue(key_value) => {
                let mut inner_key_value: KeyValueProp = key_value.clone();
                // validate_conditional_styles(&inner_key_value);

                let condition = get_key_str(&inner_key_value);
                let mut pseudos_to_pass_down = pseudos.clone();
                let mut at_rules_to_pass_down = at_rules.clone();

                if condition.starts_with(":") {
                  dbg!("{:?}", &pseudos_to_pass_down);
                  pseudos_to_pass_down.push(condition.clone());
                } else if condition.starts_with("@") {
                  at_rules_to_pass_down.push(condition.clone());
                }

                inner_key_value.key = PropName::Str(Str {
                  span: DUMMY_SP,
                  value: css_property_key.clone().into(),
                  raw: Option::None,
                });

                println!(
                  "!!condition: {:#?}, inner_key, {:#?}",
                  condition, inner_key_value.key
                );

                let pairs = flatten_raw_style_object(
                  &vec![inner_key_value],
                  &mut pseudos_to_pass_down,
                  &mut at_rules_to_pass_down,
                  state,
                  functions,
                );

                println!("!!pairs: {:#?}", pairs);
                // equivalent_pairs.extend(pairs);
                // for (key, value) in pairs {
                //     equivalent_pairs.insert(key, value);
                // }
                for (property, pre_rule) in pairs {
                  // if let Some(pre_rule) = pre_rule.downcast_ref::<PreIncludedStylesRule>() {
                  //     // NOT POSSIBLE, but needed for Flow
                  //     panic!("stylex.include can only be used at the top-level");
                  // }
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
              _ => panic!("{}", constants::messages::NON_STATIC_VALUE),
            },
            _ => panic!("{}", constants::messages::NON_STATIC_VALUE),
          });
          for (property, obj) in equivalent_pairs.iter() {
            let sorted_keys: Vec<&String> = obj.keys().collect();
            // sorted_keys.sort(); // Uncomment this line if you want to sort the keys

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

          if key.starts_with(":") {
            pseudos_to_pass_down.push(key.clone());
          } else if key.starts_with("@") {
            at_rules_to_pass_down.push(key.clone());
          }

          let inner_key_value = get_key_values_from_object(obj);

          let pairs = flatten_raw_style_object(
            &inner_key_value,
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
        dbg!(value);
        panic!("{}", constants::messages::ILLEGAL_PROP_VALUE)
      }
    };
  }

  flattened
}
