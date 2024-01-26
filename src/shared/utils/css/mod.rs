pub(crate) mod normalizers;
pub(crate) mod tests;
pub(crate) mod validators;

use core::panic;
use std::{collections::HashMap, sync::Arc};

use crate::{
    shared::{
        constants::{
            self, common::ILLEGAL_PROP_ARRAY_VALUE, long_hand_logical::LONG_HAND_LOGICAL,
            long_hand_physical::LONG_HAND_PHYSICAL, number_properties::NUMBER_PROPERTY_SUFFIXIES,
            priorities::PRIORITIES, shorthands_of_longhands::SHORTHANDS_OF_LONGHANDS,
            shorthands_of_shorthands::SHORTHANDS_OF_SHORTHANDS,
            unitless_number_properties::UNITLESS_NUMBER_PROPERTIES,
        },
        structures::{
            injectable_style::InjectableStyle,
            pair::Pair,
            pre_rule::{self, PreRule, PreRules, StylesPreRule},
            pre_rule_set::PreRuleSet,
        },
        utils::common::{expr_tpl_to_string, get_key_str, handle_tpl_to_expression, hash_css},
    },
    StylexConfig,
};
use colored::Colorize;
use convert_case::{Case, Casing};

use indexmap::IndexMap;
use regex::Regex;
use serde::de::value;
use swc_core::{
    // base::Compiler,
    common::{input::StringInput, source_map::Pos, BytePos, DUMMY_SP},
    css::{
        ast::{Ident, Stylesheet},
        codegen::{
            writer::basic::{BasicCssWriter, BasicCssWriterConfig},
            CodeGenerator, CodegenConfig, Emit,
        },
        parser::{error::Error, parse_string_input, parser::ParserConfig},
    },
    ecma::ast::{Expr, Id, KeyValueProp, Prop, PropName, PropOrSpread, Str, VarDeclarator},
};

use self::{
    normalizers::{base::base_normalizer, convert_font_size_to_rem::convert_font_size_to_rem},
    validators::unprefixed_custom_properties::unprefixed_custom_properties_validator,
};

use super::common::{
    get_expr_from_var_decl, get_string_val_from_lit, get_var_decl_by_ident, number_to_expression,
    transform_bin_expr_to_number,
};

pub(crate) fn convert_style_to_class_name(
    obj_entry: (&str, &Vec<&PreRules>),
    pseudos: &mut Vec<String>,
    at_rules: &mut Vec<String>,
    prefix: &str,
) -> (String, String, InjectableStyle) {
    let (css_property, css_property_values) = obj_entry;

    let css_property_key = css_property.to_string().to_case(Case::Kebab);

    let mut sorted_pseudos = pseudos.clone();
    sorted_pseudos.sort();

    let mut sorted_at_rules = at_rules.clone();
    sorted_at_rules.sort();

    let at_rule_hash_string = sorted_at_rules.join("");
    let pseudo_hash_string = sorted_pseudos.join("");

    let modifier_hash_string = format!("{}{}", at_rule_hash_string, pseudo_hash_string);

    let modifier_hash_string = if modifier_hash_string.is_empty() {
        "null".to_string()
    } else {
        modifier_hash_string
    };

    let (css_property_value, css_style) = if css_property_values.len() > 1 {
        let mut css_styles = vec![];

        let css_property_value = css_property_values
            .iter()
            .map(|css_property_value| {
                let value = match css_property_value {
                    PreRules::PreRuleSet(rule_set) => rule_set.get_value(),
                    PreRules::StylesPreRule(styles_pre_rule) => styles_pre_rule.get_value(),
                    PreRules::NullPreRule(null_pre_rule) => null_pre_rule.get_value(),
                };

                println!("!!!!__ value: {:?}", value);

                let transformed_css = get_css_propty_value_to_transform(css_property, value);
                css_styles.push(format!("{}:{}", css_property_key, transformed_css));

                transformed_css
            })
            .collect::<Vec<String>>()
            .join(", ");

        let css_style = format!("{{{}}}", css_styles.join(";"));

        (css_property_value.clone(), css_style)
    } else {
        let first_item = match css_property_values.get(0) {
            Some(item) => {
                println!("\n\n!!!!__ item: {:?}\n\n", item);
                let value = match item {
                    PreRules::PreRuleSet(rule_set) => rule_set.get_value(),
                    PreRules::StylesPreRule(styles_pre_rule) => styles_pre_rule.get_value(),
                    PreRules::NullPreRule(null_pre_rule) => null_pre_rule.get_value(),
                };

                value
            }
            None => Some("{}".to_string()),
        };
        println!(
            "\n\n!!!!__ first_item: {:?}, css_property_values: {:?} \n\n",
            first_item, css_property_values
        );

        let css_property_value = get_css_propty_value_to_transform(css_property, first_item);
        let css_style = format!("{{{}:{}}}", css_property_key, css_property_value);

        (css_property_value.clone(), css_style)
    };

    let value_to_hash = format!(
        "<>{}{}{}",
        css_property_key, css_property_value, modifier_hash_string
    );

    let class_name_hashed = format!("{}{}", prefix, hash_css(value_to_hash.as_str()));

    let css_rules = generate_rule(
        class_name_hashed.as_str(),
        css_property_key.as_str(),
        css_property_values,
        pseudos,
        at_rules,
    );

    println!("\n\n!!css_rules {:?} \n\n\n", css_rules);
    (css_style, class_name_hashed, css_rules)
}

pub(crate) fn generate_ltr(pair: Pair) -> Pair {
    eprintln!(
        "{}",
        Colorize::yellow("!!!! generate_ltr not implemented yet !!!!")
    );
    pair
}

pub(crate) fn generate_rtl(pair: Pair) -> Option<Pair> {
    eprintln!(
        "{}",
        Colorize::yellow("!!!! generate_rtl not implemented yet !!!!")
    );
    Option::None
}
const THUMB_VARIANTS: [&str; 3] = [
    "::-webkit-slider-thumb",
    "::-moz-range-thumb",
    "::-ms-thumb",
];

pub(crate) fn generate_css_rule(
    class_name: &str,
    decls: String,
    pseudos: &mut Vec<String>,
    at_rules: &mut Vec<String>,
) -> String {
    let pseudo = pseudos
        .iter()
        .filter(|&p| p != "::thumb")
        .collect::<Vec<&String>>();
    let pseudo_strs: Vec<&str> = pseudo.iter().map(|s| s.as_str()).collect();
    let pseudo = pseudo_strs.join("");
    let mut selector_for_at_rules = format!(
        ".{}{}{}",
        class_name,
        at_rules
            .iter()
            .map(|_| format!(".{}", class_name))
            .collect::<Vec<String>>()
            .join(""),
        pseudo
    );

    if pseudos.contains(&"::thumb".to_string()) {
        selector_for_at_rules = THUMB_VARIANTS
            .iter()
            .map(|suffix| format!("{}{}", selector_for_at_rules, suffix))
            .collect::<Vec<String>>()
            .join(", ");
    }

    at_rules.iter().fold(
        format!("{}{{{}}}", selector_for_at_rules, decls),
        |acc, at_rule| format!("{}{{{}}}", at_rule, acc),
    )
}

pub(crate) fn generate_rule(
    class_name: &str,
    key: &str,
    values: &Vec<&PreRules>,
    pseudos: &mut Vec<String>,
    at_rules: &mut Vec<String>,
) -> InjectableStyle {
    let mut pairs: Vec<Pair> = vec![];

    for value in values {
        pairs.push(Pair {
            key: key.to_string(),
            value: (*value).clone(), // Clone value
        });
    }

    let ltr_pairs: Vec<Pair> = pairs
        .iter()
        .map(|pair| generate_ltr(pair.clone()))
        .collect::<Vec<Pair>>();

    let rtl_pairs: Vec<Pair> = pairs
        .iter()
        .map(|pair| generate_rtl(pair.clone()))
        .filter(|pair| pair.is_some())
        .map(|pair| pair.unwrap())
        .collect::<Vec<Pair>>();

    let ltr_decls = ltr_pairs
        .iter()
        .map(|pair| {
            let value = match &pair.value {
                PreRules::PreRuleSet(rule_set) => rule_set.get_value(),
                PreRules::StylesPreRule(styles_pre_rule) => styles_pre_rule.get_value(),
                PreRules::NullPreRule(null_pre_rule) => null_pre_rule.get_value(),
            };

            format!("{}:{}", pair.key, value.unwrap())
        })
        .collect::<Vec<String>>()
        .join(";");

    let rtl_decls = rtl_pairs
        .iter()
        .map(|pair| {
            let value = match &pair.value {
                PreRules::PreRuleSet(rule_set) => rule_set.get_value(),
                PreRules::StylesPreRule(styles_pre_rule) => styles_pre_rule.get_value(),
                PreRules::NullPreRule(null_pre_rule) => null_pre_rule.get_value(),
            };

            format!("{}:{}", pair.key, value.unwrap())
        })
        .collect::<Vec<String>>()
        .join(";");

    let ltr_rule = generate_css_rule(class_name, ltr_decls, pseudos, at_rules);
    let rtl_rule = if rtl_decls.is_empty() {
        Option::None
    } else {
        Option::Some(generate_css_rule(class_name, rtl_decls, pseudos, at_rules))
    };

    let priority = get_priority(key)
        + pseudos.iter().map(|p| get_priority(p)).sum::<u16>()
        + at_rules.iter().map(|a| get_priority(a)).sum::<u16>();

    InjectableStyle {
        priority: Option::Some(priority),
        rtl: rtl_rule,
        ltr: ltr_rule,
    }
}

pub(crate) fn get_priority(key: &str) -> u16 {
    if key.starts_with("--") {
        return 1;
    };

    if key.starts_with("@supports") {
        return 30;
    };

    if key.starts_with("@media") {
        return 200;
    };

    if key.starts_with("@container") {
        return 300;
    };

    if key.starts_with("::") {
        return 5000;
    };

    let default_value = &3000;

    if key.starts_with(":") {
        let prop: &str = if key.starts_with(':') && key.contains("(") {
            let index = key.chars().position(|c| c == '(').unwrap();

            &key[0..index]
        } else {
            key
        };

        return **PRIORITIES.get(prop).unwrap_or(&default_value);
    };

    if LONG_HAND_PHYSICAL.contains(key) {
        return 4000;
    }

    if LONG_HAND_LOGICAL.contains(key) {
        return *default_value;
    }

    if SHORTHANDS_OF_LONGHANDS.contains(key) {
        return 2000;
    }

    if SHORTHANDS_OF_SHORTHANDS.contains(key) {
        return 1000;
    }

    return 3000;
}

fn get_css_propty_value_to_transform(
    css_property: &str,
    css_property_value: Option<String>,
) -> String {
    let css_property_value = match css_property_value {
        Some(value) => value,
        None => panic!("css_property_values is empty"),
    };

    transform_css_property_value_to_str(&css_property, &css_property_value)
}

pub(crate) fn flatten_raw_style_object(
    property: &KeyValueProp,
    declarations: &Vec<VarDeclarator>,
    var_dec_count_map: &mut HashMap<Id, i8>,
    pseudos: &mut Vec<String>,
    at_rules: &mut Vec<String>,
) -> IndexMap<String, PreRules> {
    let key = get_key_str(property);

    let key_regex = Regex::new(r"var\(--[a-z0-9]+\)").unwrap();
    let css_property_key = if key_regex.is_match(&key.clone()) {
        key[4..key.len() - 1].to_string()
    } else {
        key.clone()
    };

    let mut pre_rules: IndexMap<String, PreRules> = IndexMap::new();

    match property.value.as_ref() {
        Expr::Array(property_array) => {
            property_array
                .elems
                .iter()
                .for_each(|propery| match propery {
                    Option::Some(property) => match property.expr.as_ref() {
                        Expr::Lit(property_lit) => {
                            let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
                                css_property_key.clone(),
                                get_string_val_from_lit(property_lit),
                                pseudos.clone(),
                                at_rules.clone(),
                            ));

                            pre_rules.insert(css_property_key.clone(), pre_rule);
                        }
                        _ => panic!("{}", ILLEGAL_PROP_ARRAY_VALUE),
                    },
                    _ => {}
                })
        }
        Expr::Lit(property_lit) => {
            let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
                css_property_key.clone(),
                get_string_val_from_lit(property_lit),
                pseudos.clone(),
                at_rules.clone(),
            ));

            pre_rules.insert(css_property_key, pre_rule);
        }
        Expr::Tpl(tpl) => {
            let handled_tpl = handle_tpl_to_expression(tpl, declarations, var_dec_count_map);
            let result = expr_tpl_to_string(
                handled_tpl.as_tpl().unwrap(),
                declarations,
                var_dec_count_map,
            );

            let pre_rule = PreRules::StylesPreRule(StylesPreRule::new(
                css_property_key.clone(),
                result,
                pseudos.clone(),
                at_rules.clone(),
            ));

            pre_rules.insert(css_property_key, pre_rule);
        }
        Expr::Ident(ident) => {
            let ident = get_var_decl_by_ident(ident, declarations, var_dec_count_map);

            match ident {
                Some(var_decl) => {
                    let var_decl_expr = get_expr_from_var_decl(var_decl);

                    let mut k = property.clone();
                    k.value = Box::new(var_decl_expr);

                    let flattened = flatten_raw_style_object(
                        &k,
                        declarations,
                        var_dec_count_map,
                        pseudos,
                        at_rules,
                    );

                    println!("!!before_updated_flattened: {:?}", flattened);
                    // println!("!!updated_flattened: {:?}", updated_flattened);

                    pre_rules.extend(flattened);
                }
                None => panic!("{}", constants::common::NON_STATIC_VALUE),
            }
        }
        Expr::Bin(bin) => {
            let result = transform_bin_expr_to_number(bin, declarations, var_dec_count_map);

            let mut k = property.clone();
            k.value = Box::new(number_to_expression(result as f64).unwrap());

            let flattened =
                flatten_raw_style_object(&k, declarations, var_dec_count_map, pseudos, at_rules);

            pre_rules.extend(flattened)
        }
        Expr::Call(_) => panic!("{}", constants::common::NON_STATIC_VALUE),
        Expr::Object(obj) => {
            if obj.props.is_empty() {
                println!("!!obj.props.is_empty(): {:?}", pre_rules);
                return pre_rules;
            }
            let mut equivalent_pairs: IndexMap<String, IndexMap<String, PreRules>> =
                IndexMap::new();

            obj.props.iter().for_each(|prop| match prop {
                PropOrSpread::Prop(prop) => match prop.as_ref() {
                    Prop::KeyValue(key_value) => {
                        println!("!!!!!css_property_key: {:?}", css_property_key);

                        let mut inner_key_value: KeyValueProp = key_value.clone();
                        validate_conditional_styles(&inner_key_value);

                        let condition = get_key_str(&inner_key_value);

                        // inner_key_value.key = css_property_key.clo;

                        if condition.starts_with(":") {
                            pseudos.push(condition.clone());
                        } else if condition.starts_with("@") {
                            at_rules.push(condition.clone());
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

                        let mut pairs = flatten_raw_style_object(
                            &inner_key_value,
                            declarations,
                            var_dec_count_map,
                            pseudos,
                            at_rules,
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
                    _ => panic!("{}", constants::common::NON_STATIC_VALUE),
                },
                _ => panic!("{}", constants::common::NON_STATIC_VALUE),
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
                pre_rules.insert(property.clone(), PreRuleSet::create(rules));
            }
        }
        _ => panic!("{}", constants::common::ILLEGAL_PROP_VALUE),
    };

    pre_rules
}

pub(crate) fn validate_conditional_styles(inner_key_value: &KeyValueProp) {
    let inner_key = get_key_str(inner_key_value);

    assert!(
        (inner_key.starts_with(":") || inner_key.starts_with("@") || inner_key == "default"),
        "{}",
        constants::common::INVALID_PSEUDO_OR_AT_RULE,
    );
}

pub(crate) fn transform_css_property_value_to_str(
    css_property: &str,
    css_property_values: &str,
) -> String {
    let css_property_value = css_property_values.trim();

    let value = match &css_property_value.parse::<f64>() {
        Ok(value) => format!(
            "{0}{1}",
            ((value * 10000.0).round() / 10000.0),
            get_number_suffix(css_property)
        ),
        Err(_) => css_property_value.to_string(),
    };

    if css_property == "content"
        || css_property == "hyphenateCharacter"
        || css_property == "hyphenate-character"
    {
        todo!()
    }

    let result =
        normalize_css_property_value(css_property, value.as_ref(), &StylexConfig::default());

    result
}
pub fn swc_parse_css(source: &str) -> (Result<Stylesheet, Error>, Vec<Error>) {
    let config = ParserConfig {
        allow_wrong_line_comments: false,
        css_modules: false,
        legacy_nesting: false,
        legacy_ie: false,
    };

    let input = StringInput::new(
        source,
        BytePos::from_usize(0),
        BytePos::from_usize(source.len()),
    );
    let mut errors: Vec<Error> = vec![];

    (
        parse_string_input(input, Option::None, config, &mut errors),
        errors,
    )
}

pub(crate) fn normalize_css_property_value(
    css_property: &str,
    css_property_value: &str,
    options: &StylexConfig,
) -> String {
    let css_rule = if css_property.starts_with(":") {
        format!("{0} {1}", css_property, css_property_value)
    } else {
        format!("* {{ {0}: {1}; }}", css_property, css_property_value)
    };

    let (parsed_css, errors) = swc_parse_css(css_rule.as_str());

    if errors.len() > 0 {
        let error_message = errors.get(0).unwrap().message().to_string();

        panic!("{}", error_message)
    }

    let ast_normalized = match parsed_css {
        Ok(ast) => {
            let (parsed_css_property_value, _) = swc_parse_css(css_property_value);

            let validators: Vec<Validator> = vec![
                unprefixed_custom_properties_validator,
                // Add other validator functions here...
            ];

            let mut normalizers: Vec<Normalizer> = vec![
                base_normalizer,
                // Add other normalizer functions here...
            ];

            if options.use_rem_for_font_size {
                normalizers.push(convert_font_size_to_rem);
            }

            for validator in validators {
                validator(ast.clone());
            }

            let mut parsed_ast = parsed_css_property_value.unwrap();

            for normalizer in normalizers {
                parsed_ast = normalizer(parsed_ast);
            }

            stringify(&parsed_ast)
        }
        Err(err) => {
            panic!("{}", err.message())
        }
    };

    ast_normalized
}

type Normalizer = fn(Stylesheet) -> Stylesheet;
type Validator = fn(Stylesheet);

pub(crate) fn get_number_suffix(css_property: &str) -> String {
    if UNITLESS_NUMBER_PROPERTIES.contains(css_property) {
        return "".to_string();
    }

    let result = match NUMBER_PROPERTY_SUFFIXIES.get(css_property) {
        Some(suffix) => suffix,
        None => "px",
    };

    result.to_string()
}

fn get_value_from_ident(ident: &Ident) -> String {
    ident.value.to_string()
}

/// Stringifies the [`Stylesheet`]
pub(crate) fn stringify(node: &Stylesheet) -> String {
    let mut buf = String::new();
    let writer = BasicCssWriter::new(&mut buf, None, BasicCssWriterConfig::default());
    let mut codegen = CodeGenerator::new(writer, CodegenConfig { minify: false });

    let _ = codegen.emit(&node);

    buf
}
