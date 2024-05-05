use core::panic;

use crate::shared::{
  constants::{
    self,
    long_hand_logical::LONG_HAND_LOGICAL,
    long_hand_physical::LONG_HAND_PHYSICAL,
    messages,
    number_properties::NUMBER_PROPERTY_SUFFIXIES,
    priorities::{AT_RULE_PRIORITIES, PSEUDO_CLASS_PRIORITIES, PSEUDO_ELEMENT_PRIORITY},
    shorthands_of_longhands::SHORTHANDS_OF_LONGHANDS,
    shorthands_of_shorthands::SHORTHANDS_OF_SHORTHANDS,
    unitless_number_properties::UNITLESS_NUMBER_PROPERTIES,
  },
  structures::{
    injectable_style::InjectableStyle, pair::Pair, pre_rule::PreRuleValue,
    state_manager::StateManager, stylex_state_options::StyleXStateOptions,
  },
  utils::{
    common::{create_hash, dashify},
    css::{
      common::{generate_ltr, generate_rtl},
      normalizers::{base::base_normalizer, whitespace_normalizer::whitespace_normalizer},
      validators::unprefixed_custom_properties::unprefixed_custom_properties_validator,
    },
  },
};
use convert_case::{Case, Casing};
// use lightningcss::{
//   printer::PrinterOptions,
//   stylesheet::{ParserOptions, StyleSheet},
// };

use regex::Regex;
use swc_core::{
  common::{input::StringInput, source_map::Pos, BytePos},
  css::{
    ast::{Ident, Stylesheet},
    codegen::{
      writer::basic::{BasicCssWriter, BasicCssWriterConfig},
      CodeGenerator, CodegenConfig, Emit,
    },
    parser::{error::Error, parse_string_input, parser::ParserConfig},
  },
};

pub(crate) fn convert_style_to_class_name(
  obj_entry: (&str, &PreRuleValue),
  pseudos: &mut [String],
  at_rules: &mut [String],
  prefix: &str,
  state: &StateManager,
) -> (String, String, InjectableStyle) {
  let (key, raw_value) = obj_entry;
  dbg!(obj_entry);

  let dashed_key = if key.starts_with("--") {
    key.to_string()
  } else {
    dashify(key).to_case(Case::Kebab)
  };

  dbg!(&dashed_key);

  let sorted_pseudos = &mut pseudos.to_vec();
  sorted_pseudos.sort();

  let sorted_at_rules = &mut at_rules.to_vec();
  sorted_at_rules.sort();

  let at_rule_hash_string = sorted_at_rules.join("");
  let pseudo_hash_string = sorted_pseudos.join("");

  let modifier_hash_string = format!("{}{}", at_rule_hash_string, pseudo_hash_string);

  let modifier_hash_string = if modifier_hash_string.is_empty() {
    "null".to_string()
  } else {
    modifier_hash_string
  };

  dbg!(&raw_value);
  let value = match raw_value {
    PreRuleValue::String(value) => PreRuleValue::String(transform_value(key, value, state)),
    PreRuleValue::Vec(vec) => PreRuleValue::Vec(
      vec
        .iter()
        .map(|each_value| transform_value(key, each_value.as_str(), state))
        .collect::<Vec<String>>(),
    ),
    PreRuleValue::Expr(_) => panic!("{}", constants::messages::ILLEGAL_PROP_VALUE),
    PreRuleValue::Null => panic!("{}", constants::messages::ILLEGAL_PROP_VALUE),
  };

  let value = match value.clone() {
    PreRuleValue::String(value) => vec![value],
    PreRuleValue::Vec(values) => {
      if values
        .iter()
        .any(|value| value.starts_with("var(") && value.ends_with(')'))
      {
        variable_fallbacks(values)
      } else {
        values
      }
    }
    PreRuleValue::Expr(_) | PreRuleValue::Null => {
      panic!("{}", constants::messages::ILLEGAL_PROP_VALUE)
    }
  };

  let string_to_hash = format!(
    "<>{}{}{}",
    dashed_key,
    value.join(", "),
    modifier_hash_string
  );

  dbg!(&at_rule_hash_string, &pseudo_hash_string);
  dbg!(&dashed_key, &value, &modifier_hash_string);
  dbg!(string_to_hash.clone());

  let class_name_hashed = format!("{}{}", prefix, create_hash(string_to_hash.as_str()));

  let css_rules = generate_rule(
    class_name_hashed.as_str(),
    dashed_key.as_str(),
    &value,
    pseudos,
    at_rules,
  );

  dbg!(key, &value, &class_name_hashed, &css_rules);

  (key.to_string(), class_name_hashed, css_rules)
}

// pub(crate) fn generate_ltr(pair: Pair) -> Pair {
//     eprintln!(
//         "{}",
//         Colorize::yellow("!!!! generate_ltr not implemented yet !!!!")
//     );
//     pair
// }

// pub(crate) fn generate_rtl(pair: Pair) -> Option<Pair> {
//     eprintln!(
//         "{}",
//         Colorize::yellow("!!!! generate_rtl not implemented yet !!!!")
//     );
//     Option::None
// }
const THUMB_VARIANTS: [&str; 3] = [
  "::-webkit-slider-thumb",
  "::-moz-range-thumb",
  "::-ms-thumb",
];

pub(crate) fn generate_css_rule(
  class_name: &str,
  decls: String,
  pseudos: &mut [String],
  at_rules: &mut [String],
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
  values: &Vec<String>,
  pseudos: &mut [String],
  at_rules: &mut [String],
) -> InjectableStyle {
  let mut pairs: Vec<Pair> = vec![];

  for value in values {
    pairs.push(Pair {
      key: key.to_string(),
      value: value.clone(),
    });
  }

  // match values {
  //   PreRuleValue::String(value) => pairs.push(Pair {
  //     key: key.to_string(),
  //     value: value.clone(),
  //   }),
  //   PreRuleValue::Vec(values) => values.iter().for_each(|value| {
  //     pairs.push(Pair {
  //       key: key.to_string(),
  //       value: value.clone(),
  //     })
  //   }),
  //   PreRuleValue::Expr(_) => panic!("{}", constants::messages::ILLEGAL_PROP_VALUE),
  //   PreRuleValue::Null => panic!("{}", constants::messages::ILLEGAL_PROP_VALUE),
  // }

  let ltr_pairs: Vec<Pair> = pairs
    .iter()
    .map(|pair| generate_ltr(pair.clone()))
    .collect::<Vec<Pair>>();

  let rtl_pairs: Vec<Pair> = pairs
    .iter()
    .filter_map(|pair| generate_rtl(pair.clone()))
    .collect::<Vec<Pair>>();

  let ltr_decls = ltr_pairs
    .iter()
    .map(|pair| format!("{}:{}", pair.key, pair.value))
    .collect::<Vec<String>>()
    .join(";");

  let rtl_decls = rtl_pairs
    .iter()
    .map(|pair| format!("{}:{}", pair.key, pair.value))
    .collect::<Vec<String>>()
    .join(";");

  dbg!(&ltr_decls, &rtl_decls);

  let ltr_rule = generate_css_rule(class_name, ltr_decls, pseudos, at_rules);
  let rtl_rule = if rtl_decls.is_empty() {
    Option::None
  } else {
    Option::Some(generate_css_rule(class_name, rtl_decls, pseudos, at_rules))
  };

  dbg!(&ltr_rule, &rtl_rule);

  let priority = get_priority(key)
    + pseudos.iter().map(|p| get_priority(p)).sum::<f32>()
    + at_rules.iter().map(|a| get_priority(a)).sum::<f32>();

  // dbg!(
  //     get_priority(key),
  //     &pseudos,
  //     pseudos.iter().map(|p| get_priority(p)).sum::<f32>(),
  //     at_rules.iter().map(|a| get_priority(a)).sum::<f32>()
  // );

  InjectableStyle {
    priority: Option::Some(priority),
    rtl: rtl_rule,
    ltr: ltr_rule,
  }
}

pub(crate) fn get_priority(key: &str) -> f32 {
  if key.starts_with("--") {
    return 1.0;
  };

  if key.starts_with("@supports") {
    return **AT_RULE_PRIORITIES
      .get("@supports")
      .expect("No priority found");
  };

  if key.starts_with("@media") {
    return **AT_RULE_PRIORITIES.get("@media").expect("No priority found");
  };

  if key.starts_with("@container") {
    return **AT_RULE_PRIORITIES
      .get("@container")
      .expect("No priority found");
  };

  if key.starts_with("::") {
    return PSEUDO_ELEMENT_PRIORITY;
  };

  if key.starts_with(':') {
    let prop: &str = if key.starts_with(':') && key.contains('(') {
      let index = key.chars().position(|c| c == '(').unwrap();

      &key[0..index]
    } else {
      key
    };

    return **PSEUDO_CLASS_PRIORITIES.get(prop).unwrap_or(&&40.0);
  };

  if LONG_HAND_PHYSICAL.contains(key) {
    return 4000.0;
  }

  if LONG_HAND_LOGICAL.contains(key) {
    return 3000.0;
  }

  if SHORTHANDS_OF_LONGHANDS.contains(key) {
    return 2000.0;
  }

  if SHORTHANDS_OF_SHORTHANDS.contains(key) {
    return 1000.0;
  }

  3000.0
}

pub(crate) fn transform_value(key: &str, value: &str, state: &StateManager) -> String {
  let css_property_value = value.trim();

  let value = match &css_property_value.parse::<f64>() {
    Ok(value) => format!(
      "{0}{1}",
      ((value * 10000.0).round() / 10000.0),
      get_number_suffix(key)
    ),
    Err(_) => css_property_value.to_string(),
  };

  if key == "content" || key == "hyphenateCharacter" || key == "hyphenate-character" {
    let val = value.trim();
    if Regex::new(r"^attr\([a-zA-Z0-9-]+\)$")
      .unwrap()
      .is_match(val)
    {
      return val.to_string();
    }
    if !(val.starts_with('"') && val.ends_with('"') || val.starts_with('\'') && val.ends_with('\''))
    {
      return format!("\"{}\"", val);
    }

    return val.to_string();
  }

  dbg!(&key, &value);
  let result = normalize_css_property_value(key, value.as_ref(), &state.options);

  dbg!(&result);

  // panic!("Not implemented yet");
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
  options: &StyleXStateOptions,
) -> String {
  let css_property = if css_property.starts_with("--") {
    "color"
  } else {
    css_property
  };

  let css_rule = if css_property.starts_with(':') {
    format!("{0} {1}", css_property, css_property_value)
  } else {
    format!("* {{ {0}: {1} }}", css_property, css_property_value)
  };

  // let stylesheet = StyleSheet::parse(&css_rule, ParserOptions::default()).unwrap();

  // let a = stylesheet.to_css(PrinterOptions {
  //   minify: true,
  //   ..Default::default()
  // });
  // dbg!(&stylesheet, &a);

  let (parsed_css, errors) = swc_parse_css(css_rule.as_str());

  if !errors.is_empty() {
    let mut error_message = errors.first().unwrap().message().to_string();

    if error_message.ends_with("expected ')'") || error_message.ends_with("expected '('") {
      error_message = messages::LINT_UNCLOSED_FUNCTION.to_string();
    }

    panic!("{}", error_message)
  }

  let ast_normalized = match parsed_css {
    Ok(ast) => {
      dbg!(&css_rule);

      let (parsed_css_property_value, _) = swc_parse_css(css_rule.as_str());

      let validators: Vec<Validator> = vec![
        unprefixed_custom_properties_validator,
        // Add other validator functions here...
      ];

      let normalizers: Vec<Normalizer> = vec![
        base_normalizer,
        // Add other normalizer functions here...
      ];

      dbg!(&options.use_rem_for_font_size);

      for validator in validators {
        validator(ast.clone());
      }

      let mut parsed_ast = parsed_css_property_value.unwrap();
      dbg!(&parsed_ast);

      for normalizer in normalizers {
        parsed_ast = normalizer(parsed_ast, options.use_rem_for_font_size);
      }

      dbg!(&parsed_ast);
      // panic!("Not implemented yet");
      let result = stringify(&parsed_ast);
      // let b = a.unwrap().code;

      // dbg!(&result, &b);

      // dbg!(&result, &b, &result == &b);
      // result.to_case(Case::Kebab)
      whitespace_normalizer(result)
    }
    Err(err) => {
      panic!("{}", err.message())
    }
  };

  ast_normalized
}

type Normalizer = fn(Stylesheet, bool) -> Stylesheet;
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

pub(crate) fn get_value_from_ident(ident: &Ident) -> String {
  ident.value.to_string()
}

pub fn stringify(node: &Stylesheet) -> String {
/// Stringifies the [`Stylesheet`]
  dbg!(&node);

  let mut buf = String::new();
  let writer = BasicCssWriter::new(&mut buf, None, BasicCssWriterConfig::default());
  let mut codegen = CodeGenerator::new(writer, CodegenConfig { minify: true });

  // let mut codegen = CodeGenerator::new(writer, CodegenConfig { minify: false });

  codegen.emit(&node).unwrap();

  dbg!(&buf);

  let mut result = buf.replace('\'', "");

  if result.contains("--\\") {
    /*
     * In CSS, identifiers (including element names, classes, and IDs in selectors)
     * can contain only the characters [a-zA-Z0-9] and ISO 10646 characters U+00A0 and higher,
     * plus the hyphen (-) and the underscore (_);
     * they cannot start with a digit, two hyphens, or a hyphen followed by a digit.
     *
     * https://stackoverflow.com/a/27882887/6717252
     *
     * HACK: Replace `--\3{number}` with `--{number}` to simulate original behavior of StyleX
     */
    let re = Regex::new(r"\\3(\d) ").unwrap();

    result = re
      .replace_all(buf.as_str(), |caps: &regex::Captures| {
        caps
          .get(1)
          .map_or("".to_string(), |m| m.as_str().to_string())
      })
      .to_string();
  }

  dbg!(&result);

  result
}

// fn get_expanded_keys(stylex_config: &StyleXOptions) -> Vec<String> {}

fn variable_fallbacks(values: Vec<String>) -> Vec<String> {
  let first_var = values
    .iter()
    .position(|val| val.starts_with("var(") && val.ends_with(')'));

  let last_var = values
    .iter()
    .rev()
    .position(|val| val.starts_with("var(") && val.ends_with(')'))
    .map(|i| values.len() - 1 - i);

  let values_before_first_var = &values[0..first_var.unwrap_or(0)];

  let mut var_values: Vec<String> = values
    [first_var.unwrap_or(0)..last_var.unwrap_or(values.len()) + 1]
    .iter()
    .rev()
    .cloned()
    .collect::<Vec<String>>();

  let values_after_last_var = &values[last_var.unwrap_or(values.len()) + 1..];

  assert!(
    !var_values
      .iter()
      .any(|val| !val.starts_with("var(") || !val.ends_with(')')),
    "{}",
    messages::NON_CONTIGUOUS_VARS
  );

  var_values = var_values
    .iter()
    .map(|val| val[4..val.len() - 1].to_string())
    .collect::<Vec<String>>();

  let mut result = Vec::new();

  if !values_before_first_var.is_empty() {
    for val in values_before_first_var {
      let mut to_push = var_values.clone();

      to_push.push(val.to_string());

      result.push(compose_vars(to_push));
    }
  } else {
    result.push(compose_vars(var_values));
  }

  for val in values_after_last_var {
    result.push(val.to_string());
  }

  dbg!(&result);

  result
}

fn compose_vars(vars: Vec<String>) -> String {
  match vars.split_first() {
    Some((first, rest)) if !rest.is_empty() => {
      format!("var({},{})", first, compose_vars(rest.to_vec()))
    }
    Some((first, _)) if first.starts_with("--") => {
      format!("var({})", first)
    }
    Some((first, _)) => first.to_string(),
    None => String::new(),
  }
}
