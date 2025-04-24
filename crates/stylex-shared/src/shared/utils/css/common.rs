use core::panic;

use crate::shared::{
  constants::{
    common::{
      COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES,
      COLOR_RELATIVE_VALUES_LISTED_NORMALIZED_PROPERTY_VALUES, CSS_CONTENT_FUNCTIONS,
      CSS_CONTENT_KEYWORDS,
    },
    long_hand_logical::LONG_HAND_LOGICAL,
    long_hand_physical::LONG_HAND_PHYSICAL,
    messages::LINT_UNCLOSED_FUNCTION,
    number_properties::NUMBER_PROPERTY_SUFFIXIES,
    priorities::{
      AT_RULE_PRIORITIES, CAMEL_CASE_PRIORITIES, PSEUDO_CLASS_PRIORITIES, PSEUDO_ELEMENT_PRIORITY,
    },
    shorthands_of_longhands::SHORTHANDS_OF_LONGHANDS,
    shorthands_of_shorthands::SHORTHANDS_OF_SHORTHANDS,
    unitless_number_properties::UNITLESS_NUMBER_PROPERTIES,
  },
  regex::{CLEAN_CSS_VAR, MANY_SPACES},
  structures::{
    injectable_style::InjectableStyle, pair::Pair, state_manager::StateManager,
    stylex_state_options::StyleXStateOptions,
  },
  utils::css::{
    normalizers::{base::base_normalizer, whitespace_normalizer::whitespace_normalizer},
    validators::unprefixed_custom_properties::unprefixed_custom_properties_validator,
  },
};

use rustc_hash::FxHashMap;
use swc_core::{
  common::{BytePos, input::StringInput, source_map::SmallPos},
  css::{
    ast::{Ident, Stylesheet},
    codegen::{
      CodeGenerator, CodegenConfig, Emit,
      writer::basic::{BasicCssWriter, BasicCssWriterConfig},
    },
    parser::{error::Error, parse_string_input, parser::ParserConfig},
  },
};

use crate::shared::{
  constants::cursor_flip::CURSOR_FLIP, regex::LENGTH_UNIT_TESTER_REGEX,
  structures::pre_rule::PreRules,
};

use super::parser::parse_css;

fn logical_to_physical(input: &str) -> &str {
  match input {
    "start" => "left",
    "end" => "right",
    _ => input,
  }
}

fn property_to_ltr(pair: (&str, &str)) -> Pair {
  if pair.0 == "background-position" {
    let words: Vec<&str> = pair.1.split_whitespace().collect();
    let new_val = words
      .iter()
      .map(|&word| match word {
        "start" => "left",
        "end" => "right",
        _ => word,
      })
      .collect::<Vec<&str>>()
      .join(" ");
    return Pair::new(pair.0.to_string(), new_val);
  };

  let result = match pair.0 {
    "margin-start" => ("margin-left", pair.1),
    "margin-end" => ("margin-right", pair.1),
    "padding-start" => ("padding-left", pair.1),
    "padding-end" => ("padding-right", pair.1),
    "border-start" => ("border-left", pair.1),
    "border-end" => ("border-right", pair.1),
    "border-start-width" => ("border-left-width", pair.1),
    "border-end-width" => ("border-right-width", pair.1),
    "border-start-color" => ("border-left-color", pair.1),
    "border-end-color" => ("border-right-color", pair.1),
    "border-start-style" => ("border-left-style", pair.1),
    "border-end-style" => ("border-right-style", pair.1),
    "border-top-start-radius" => ("border-top-left-radius", pair.1),
    "border-bottom-start-radius" => ("border-bottom-left-radius", pair.1),
    "border-top-end-radius" => ("border-top-right-radius", pair.1),
    "border-bottom-end-radius" => ("border-bottom-right-radius", pair.1),
    "float" => (pair.0, logical_to_physical(pair.1)),
    "clear" => (pair.0, logical_to_physical(pair.1)),
    "start" => ("left", pair.1),
    "end" => ("right", pair.1),

    _ => pair,
  };

  Pair::new(result.0.to_string(), result.1.to_string())
}

pub(crate) fn generate_ltr(pair: &Pair) -> Pair {
  property_to_ltr((pair.key.as_str(), pair.value.as_str()))
}

fn flip_sign(value: &str) -> String {
  if value == "0" {
    value.to_string()
  } else if value.starts_with('-') {
    value.strip_prefix('-').unwrap().to_string()
  } else {
    format!("-{}", value)
  }
}

fn is_unit(input: &str) -> bool {
  LENGTH_UNIT_TESTER_REGEX.is_match(input)
}

fn flip_shadow(value: &str) -> Option<String> {
  let defs: Vec<&str> = value.split(',').collect();
  let mut built_defs = Vec::new();

  for def in defs {
    let mut parts = def
      .split_whitespace()
      .map(|x| x.to_string())
      .collect::<Vec<String>>();

    // NOTE: temporary solution, need to implement unit parser
    let index = if is_unit(parts[0].as_str()) { 0 } else { 1 };

    if index < parts.len() {
      let flipped = flip_sign(&parts[index]);
      parts[index] = flipped;
    }
    built_defs.push(parts.join(" "));
  }

  let rtl = built_defs.join(", ");
  if rtl != value { Some(rtl) } else { None }
}

fn shadows_flip(key: &str, val: &str) -> Option<Pair> {
  match key {
    "box-shadow" | "text-shadow" => {
      let rtl_val = flip_shadow(val);
      rtl_val.map(|rtl_val| Pair::new(key.to_string(), rtl_val))
    }
    _ => None,
  }
}

fn property_to_rtl(key: &str, val: &str) -> Option<Pair> {
  let logical_to_physical = [("start", "right"), ("end", "left")]
    .iter()
    .cloned()
    .collect::<FxHashMap<_, _>>();

  match key {
    "margin-start" => Some(Pair::new("margin-right".to_string(), val.to_string())),
    "margin-end" => Some(Pair::new("margin-left".to_string(), val.to_string())),
    "padding-start" => Some(Pair::new("padding-right".to_string(), val.to_string())),
    "padding-end" => Some(Pair::new("padding-left".to_string(), val.to_string())),
    "border-start" => Some(Pair::new("border-right".to_string(), val.to_string())),
    "border-end" => Some(Pair::new("border-left".to_string(), val.to_string())),
    "border-start-width" => Some(Pair::new("border-right-width".to_string(), val.to_string())),
    "border-end-width" => Some(Pair::new("border-lrft-width".to_string(), val.to_string())),

    "border-start-color" => Some(Pair::new("border-right-color".to_string(), val.to_string())),
    "border-end-color" => Some(Pair::new("border-lrft-colot".to_string(), val.to_string())),

    "border-start-style" => Some(Pair::new("border-right-style".to_string(), val.to_string())),
    "border-end-style" => Some(Pair::new("border-lrft-style".to_string(), val.to_string())),

    "border-top-satrt-radius" => Some(Pair::new(
      "border-top-right-radius".to_string(),
      val.to_string(),
    )),

    "border-bottom-start-radius" => Some(Pair::new(
      "border-bottom-right-radius".to_string(),
      val.to_string(),
    )),

    "border-top-end-radius" => Some(Pair::new(
      "border-top-left-radius".to_string(),
      val.to_string(),
    )),

    "border-bottom-end-radius" => Some(Pair::new(
      "border-bottom-left-radius".to_string(),
      val.to_string(),
    )),

    "float" | "clear" => logical_to_physical
      .get(val)
      .map(|&physical_val| Pair::new(key.to_string(), physical_val.to_string())),
    "start" => Some(Pair::new("right".to_string(), val.to_string())),
    "end" => Some(Pair::new("left".to_string(), val.to_string())),
    "background-position" => {
      let words: Vec<&str> = val.split_whitespace().collect();
      if words.contains(&"start") || words.contains(&"end") {
        let new_val: String = words
          .iter()
          .map(|&word| match word {
            "start" => "right",
            "end" => "left",
            _ => word,
          })
          .collect::<Vec<_>>()
          .join(" ");
        Some(Pair::new(key.to_string(), new_val))
      } else {
        None
      }
    }
    "cursor" => CURSOR_FLIP
      .get(val)
      .map(|val| Pair::new(key.to_string(), val.to_string())),
    _ => shadows_flip(key, val),
  }
}

fn _flip_value(value: &PreRules) -> Option<PreRules> {
  // Implement your logic here to flip the value
  // For now, I'm just returning the same value
  Some(value.clone())
}

pub(crate) fn generate_rtl(pair: &Pair) -> Option<Pair> {
  let result = property_to_rtl(pair.key.as_str(), pair.value.as_str());

  result
}

pub(crate) fn split_value_required(strng: Option<&str>) -> (String, String, String, String) {
  let values = split_value(strng);

  let top = values.0;
  let right = values.1.unwrap_or(top.clone());
  let bottom = values.2.unwrap_or(top.clone());
  let left = values.3.unwrap_or(right.clone());

  (top, right, bottom, left)
}

pub(crate) fn split_value(
  value: Option<&str>,
) -> (String, Option<String>, Option<String>, Option<String>) {
  let nodes = parse_css(value.unwrap_or_default());

  let top = nodes.first().cloned().unwrap_or(String::default());
  let right = nodes.get(1).cloned();
  let bottom = nodes.get(2).cloned();
  let left = nodes.get(3).cloned();

  (top, right, bottom, left)
}

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
  values: &[String],
  pseudos: &mut [String],
  at_rules: &mut [String],
) -> InjectableStyle {
  let mut pairs: Vec<Pair> = vec![];

  for value in values {
    pairs.push(Pair::new(key.to_string(), value.clone()));
  }

  let ltr_pairs: Vec<Pair> = pairs.iter().map(generate_ltr).collect::<Vec<Pair>>();

  let rtl_pairs: Vec<Pair> = pairs.iter().filter_map(generate_rtl).collect::<Vec<Pair>>();

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

  let ltr_rule = generate_css_rule(class_name, ltr_decls, pseudos, at_rules);
  let rtl_rule = if rtl_decls.is_empty() {
    None
  } else {
    Some(generate_css_rule(class_name, rtl_decls, pseudos, at_rules))
  };

  let priority = get_priority(key)
    + pseudos.iter().map(|p| get_priority(p)).sum::<f64>()
    + at_rules.iter().map(|a| get_priority(a)).sum::<f64>();

  InjectableStyle {
    priority: Some(priority),
    rtl: rtl_rule,
    ltr: ltr_rule,
  }
}

pub(crate) fn get_priority(key: &str) -> f64 {
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
    let is_css_function = CSS_CONTENT_FUNCTIONS
      .iter()
      .any(|func| value.contains(func));

    let is_keyword = CSS_CONTENT_KEYWORDS.contains(&value.as_str());

    let double_quote_count = value.matches('"').count();
    let single_quote_count = value.matches('\'').count();

    let has_matching_quotes = double_quote_count >= 2 || single_quote_count >= 2;

    if is_css_function || is_keyword || has_matching_quotes {
      return value.to_string();
    }

    return format!("\"{}\"", value);
  }

  let result = normalize_css_property_value(key, value.as_ref(), &state.options);

  result
}

pub(crate) fn transform_value_cached(key: &str, value: &str, state: &mut StateManager) -> String {
  let cache_key: String = format!("{}:{}", key, value);

  let cache = state.css_property_seen.get(&cache_key);

  if let Some(result) = cache {
    return result.to_string();
  }

  let result = transform_value(key, value, state);

  state.css_property_seen.insert(cache_key, result.clone());

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

  (parse_string_input(input, None, config, &mut errors), errors)
}

pub(crate) fn normalize_css_property_value(
  css_property: &str,
  css_property_value: &str,
  options: &StyleXStateOptions,
) -> String {
  if COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES
    .iter()
    .chain(COLOR_RELATIVE_VALUES_LISTED_NORMALIZED_PROPERTY_VALUES.iter())
    .any(|css_fnc| {
      css_property_value.contains(format!("{}(", css_fnc).as_str())
        || css_property_value.to_lowercase().contains(css_fnc)
    })
  {
    return MANY_SPACES.replace_all(css_property_value, " ").to_string();
  }

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

  let (parsed_css, errors) = swc_parse_css(css_rule.as_str());

  if !errors.is_empty() {
    let mut error_message = errors.first().unwrap().message().to_string();

    if error_message.ends_with("expected ')'") || error_message.ends_with("expected '('") {
      error_message = LINT_UNCLOSED_FUNCTION.to_string();
    }

    panic!("{}, css rule: {}", error_message, css_rule)
  }

  let ast_normalized = match parsed_css {
    Ok(parsed_css_property_value) => {
      // let validators: Vec<Validator> = vec![
      //   unprefixed_custom_properties_validator,
      //   // Add other validator functions here...
      // ];

      // let normalizers: Vec<Normalizer> = vec![
      //   base_normalizer,
      //   // Add other normalizer functions here...
      // ];

      // for validator in validators {
      //   validator(ast.clone());
      // }

      unprefixed_custom_properties_validator(&parsed_css_property_value);

      let parsed_ast = base_normalizer(parsed_css_property_value, options.use_rem_for_font_size);

      // for normalizer in normalizers {
      //   parsed_ast = normalizer(parsed_ast, options.use_rem_for_font_size);
      // }

      let result = whitespace_normalizer(stringify(&parsed_ast));

      convert_css_function_to_camel_case(result.as_str())
    }
    Err(err) => {
      panic!("{}", err.message())
    }
  };

  ast_normalized
}

// type Normalizer = fn(Stylesheet, bool) -> Stylesheet;
// type Validator = fn(Stylesheet);

pub(crate) fn get_number_suffix(key: &str) -> String {
  if UNITLESS_NUMBER_PROPERTIES.contains(key) || key.starts_with("--") {
    return String::default();
  }

  let result = match NUMBER_PROPERTY_SUFFIXIES.get(key) {
    Some(suffix) => suffix,
    None => "px",
  };

  result.to_string()
}

pub(crate) fn get_value_from_ident(ident: &Ident) -> String {
  ident.value.to_string()
}

fn convert_css_function_to_camel_case(function: &str) -> String {
  let Some(items) = function.find('(') else {
    return function.to_string();
  };

  let (name, args) = function.split_at(items);

  let Some(camel_case_name) = CAMEL_CASE_PRIORITIES.get(name) else {
    return function.to_string();
  };

  format!("{}{}", camel_case_name, args)
}

pub fn stringify(node: &Stylesheet) -> String {
  let mut buf = String::new();
  let writer = BasicCssWriter::new(&mut buf, None, BasicCssWriterConfig::default());
  let mut codegen = CodeGenerator::new(writer, CodegenConfig { minify: true });

  Emit::emit(&mut codegen, node).unwrap();

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

    result = CLEAN_CSS_VAR
      .replace_all(buf.as_str(), |caps: &regex::Captures| {
        caps
          .get(1)
          .map_or(String::default(), |m| m.as_str().to_string())
      })
      .to_string();
  }

  result
}
