use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use swc_core::ecma::ast::{BinaryOp, Expr, Id, KeyValueProp, VarDeclarator};

use crate::shared::{
    constants::{
        self, common::ILLEGAL_PROP_ARRAY_VALUE, long_hand_logical::LONG_HAND_LOGICAL,
        long_hand_physical::LONG_HAND_PHYSICAL, priorities::PRIORITIES,
        shorthands_of_longhands::SHORTHANDS_OF_LONGHANDS,
        shorthands_of_shorthands::SHORTHANDS_OF_SHORTHANDS,
    },
    structures::{
        included_style::IncludedStyle,
        injectable_style::{self, InjectableStyle},
        pre_rule::{CompiledResult, ComputedStyle, PreRule, PreRules, Styles},
    },
    utils::{
        common::get_key_str,
        css::{convert_style_to_class_name, flatten_raw_style_object, get_priority},
        validators::validate_and_return_property,
    },
};

use super::injectable_style::InjectableStyleBase;

#[derive(Debug, Serialize, Deserialize, Clone)]

pub(crate) struct MetaData {
    class_name: String,
    style: InjectableStyleBase,
    priority: u16,
}

impl MetaData {
    pub(crate) fn fabric(
        namespace_name: &str,
        property: &KeyValueProp,
        prefix: &str,
        declarations: &Vec<VarDeclarator>,
        var_dec_count_map: &mut HashMap<Id, i8>,
    ) -> (
        IndexMap<String, IndexMap<String, Option<String>>>,
        IndexMap<String, InjectableStyle>,
    ) {
        validate_and_return_property(property);

        let mut resolved_namespaces: IndexMap<String, IndexMap<String, Option<String>>> =
        IndexMap::new();
        let mut injected_styles_map: IndexMap<String, InjectableStyle> = IndexMap::new();

        // let css_property = get_key_str(property);

        let mut pseudos = vec![];
        let mut at_rules = vec![];

        let flattened_namespace = flatten_raw_style_object(
            property,
            declarations,
            var_dec_count_map,
            &mut pseudos,
            &mut at_rules,
        );
        println!("!!!!__ flattened_namespace: {:#?}", flattened_namespace);

        let compiled_namespace_tuples = flattened_namespace
            .iter()
            .map(|(key, value)| match value {
                PreRules::PreRuleSet(rule_set) => {
                    (key.to_string(), rule_set.clone().compiled(prefix))
                }
                PreRules::StylesPreRule(styles_pre_rule) => {
                    (key.to_string(), styles_pre_rule.clone().compiled(prefix))
                }
                _ => todo!("compiled_namespace_tuples not implemented"),//(key.to_string(), CompiledResult::ComputedStyles(vec![])),
            })
            .collect::<Vec<(String, CompiledResult)>>();

        println!(
            "!!!!__ compiled_namespace_tuples: {:#?}",
            compiled_namespace_tuples
        );

        let compiled_namespace = compiled_namespace_tuples
            .iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    match value {
                        CompiledResult::ComputedStyles(styles) => {
                            CompiledResult::ComputedStyles(styles.clone())
                        }
                        _ => todo!("handle other cases"),
                    },
                )
            })
            .collect::<IndexMap<String, CompiledResult>>();

        println!("!!!!__ compiled_namespace: {:#?}", compiled_namespace);

        let mut namespace_obj: IndexMap<String, Option<String>> = IndexMap::new();
        for key in compiled_namespace.keys() {
            let value = compiled_namespace.get(key).unwrap();

            // ...

            if let Some(included_styles) = value.as_included_style() {
                todo!("handle included style")
                // namespace_obj.insert(key.clone(), Some(included_styles));
            } else if let Some(styles) = value.as_computed_styles() {
                // let mut class_name_tuples = styles
                //     .iter()
                //     .map(|item| item)
                //     .collect::<Vec<ComputedStyle>>();

                let class_name_tuples = styles.clone();

                let class_name = &class_name_tuples
                    .iter()
                    .map(|computed_style| {
                        if let Some(class_name) = computed_style.0.clone() {
                            println!("!!!!__ class_name to join: {:#?}", class_name);
                            return class_name;
                        }

                        "".to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(" ");

                namespace_obj.insert(key.clone(), Some(class_name.clone()));

                println!("!!!!__ class_name_tuples: {:#?}", class_name_tuples);

                for item in &class_name_tuples {
                    let class_name = item.0.clone().unwrap();
                    let injectable_styles: InjectableStyle = item.1.clone().unwrap();

                    if !injected_styles_map.contains_key(class_name.as_str()) {
                        injected_styles_map.insert(class_name.clone(), injectable_styles.clone());
                    }
                }
            }
        }

        resolved_namespaces.insert(namespace_name.to_string(), namespace_obj);
        (resolved_namespaces, injected_styles_map)
        // let values: Vec<String> = flattened_namespace.values().map(|v| v.get_value().to_string()).collect();

        // for vec in flattened_namespace.values() {
        //     for pre_rule in vec {
        //         values.push(pre_rule);
        //     }
        // }

        // let (css_style, class_name_hashed, rules) = convert_style_to_class_name(
        //     (&css_property, &values),
        //     &mut pseudos,
        //     &mut at_rules,
        //     prefix,
        // );

        // println!("!!!!__ rules: {:?}", rules);

        // MetaData {
        //     class_name: class_name_hashed.clone(),
        //     style: InjectableStyleBase {
        //         // priority: Option::None, //MetaData::set_priority(&css_property),
        //         rtl: None,
        //         ltr: format!(".{}{}", class_name_hashed, css_style),
        //     },
        //     priority: MetaData::set_priority(&css_property),
        // }
    }
    pub(crate) fn from_injected_styles_map(
        injected_styles_map: IndexMap<String, InjectableStyle>,
    ) -> Vec<Self> {
        injected_styles_map
            .into_iter()
            .map(|(class_name, injectable_style)| MetaData {
                class_name: class_name.clone(),
                style: InjectableStyleBase {
                    ltr: injectable_style.ltr,
                    rtl: injectable_style.rtl,
                },
                priority: injectable_style.priority.unwrap(),
            })
            .collect()
    }

    pub(crate) fn _get_style(&self) -> &InjectableStyleBase {
        &self.style
    }

    pub(crate) fn get_css(&self) -> String {
        format!(".{}{}", self.class_name, self.style.ltr)
    }

    pub(crate) fn get_class_name(&self) -> &str {
        &self.class_name
    }

    pub(crate) fn get_priority(&self) -> &u16 {
        &self.priority
    }

    fn set_priority(key: &str) -> u16 {
        get_priority(key)
    }
}
