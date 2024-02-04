use std::collections::HashMap;

use indexmap::IndexMap;
use swc_core::ecma::ast::{Id, KeyValueProp, VarDeclarator};

use crate::{
    shared::{
        structures::{
            injectable_style::{InjectableStyle, InjectableStyleBase}, meta_data::MetaData, pre_rule::{CompiledResult, PreRule, PreRules}, stylex_options::StyleXOptions, stylex_state_options::StyleXStateOptions
        },
        utils::{css::flatten_raw_style_object, validators::validate_and_return_property},
    },
};

pub(crate) fn stylex_create(
    namespace_name: &str,
    property: &KeyValueProp,
    prefix: &str,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
    options: &StyleXStateOptions,
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
        options,
    );
    println!("!!!!__ flattened_namespace: {:#?}", flattened_namespace);

    let compiled_namespace_tuples = flattened_namespace
        .iter()
        .map(|(key, value)| match value {
            PreRules::PreRuleSet(rule_set) => (key.to_string(), rule_set.clone().compiled(prefix)),
            PreRules::StylesPreRule(styles_pre_rule) => {
                (key.to_string(), styles_pre_rule.clone().compiled(prefix))
            }
            PreRules::NullPreRule(rule_set) => (key.to_string(), rule_set.clone().compiled(prefix)),
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
                    CompiledResult::Null(null) => CompiledResult::Null(null.clone()),
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
                    let class_name = computed_style.0.clone();

                    println!("!!!!__ class_name to join: {:#?}", class_name);

                    return class_name;
                })
                .collect::<Vec<String>>()
                .join(" ");

            namespace_obj.insert(key.clone(), Some(class_name.clone()));

            println!(
                "!!!!__ class_name_tuples: {:#?}, namespace_obj: {:#?}",
                class_name_tuples, namespace_obj
            );

            for item in &class_name_tuples {
                let class_name = item.0.clone();
                let injectable_styles = item.1.clone();
                if !injected_styles_map.contains_key(class_name.as_str()) {
                    injected_styles_map.insert(class_name.clone(), injectable_styles.clone());
                }
            }
        } else {
            println!("!!!!__ value: {:#?}", value);
            namespace_obj.insert(key.clone(), Option::None);
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

pub(crate) fn evaluate_style_x_create_arg(){

}