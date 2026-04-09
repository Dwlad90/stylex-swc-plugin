use crate::shared::utils::common::round_to_decimal_places;
use crate::shared::utils::css::normalizers::base::base_normalizer;
use crate::shared::utils::css::validators::unprefixed_custom_properties::unprefixed_custom_properties_validator;
use crate::shared::{structures::state_manager::StateManager, utils::common::dashify};
use stylex_constants::constants::common::{
  COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES,
  COLOR_RELATIVE_VALUES_LISTED_NORMALIZED_PROPERTY_VALUES, CSS_CONTENT_FUNCTIONS,
  CSS_CONTENT_KEYWORDS,
};
use stylex_constants::constants::long_hand_logical::LONG_HAND_LOGICAL;
use stylex_constants::constants::long_hand_physical::LONG_HAND_PHYSICAL;
use stylex_constants::constants::messages::LINT_UNCLOSED_FUNCTION;
use stylex_constants::constants::number_properties::NUMBER_PROPERTY_SUFFIXIES;
use stylex_constants::constants::priorities::{
  AT_RULE_PRIORITIES, CAMEL_CASE_PRIORITIES, PSEUDO_CLASS_PRIORITIES, PSEUDO_ELEMENT_PRIORITY,
};
use stylex_constants::constants::shorthands_of_longhands::SHORTHANDS_OF_LONGHANDS;
use stylex_constants::constants::shorthands_of_shorthands::SHORTHANDS_OF_SHORTHANDS;
use stylex_constants::constants::unitless_number_properties::UNITLESS_NUMBER_PROPERTIES;
use stylex_css::css::generate_ltr::generate_ltr;
use stylex_css::css::generate_rtl::generate_rtl;
use stylex_css::css::normalizers::whitespace_normalizer::whitespace_normalizer;
use stylex_regex::regex::{
  ANCESTOR_SELECTOR, ANY_SIBLING_SELECTOR, CLEAN_CSS_VAR, DESCENDANT_SELECTOR, MANY_SPACES,
  PSEUDO_PART_REGEX, SIBLING_AFTER_SELECTOR, SIBLING_BEFORE_SELECTOR,
};
use stylex_structures::pair::Pair;
use stylex_structures::stylex_state_options::StyleXStateOptions;
use stylex_types::structures::injectable_style::InjectableStyle;

use stylex_macros::stylex_panic;
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

#[allow(unused_imports)]
pub(crate) use stylex_css::values::common::split_value_required;

#[allow(unused_imports)]
pub(crate) use stylex_css::values::common::split_value;

const THUMB_VARIANTS: [&str; 3] = [
  "::-webkit-slider-thumb",
  "::-moz-range-thumb",
  "::-ms-thumb",
];

pub(crate) fn build_nested_css_rule(
  class_name: &str,
  decls: impl AsRef<str>,
  pseudos: &mut [String],
  at_rules: &mut [String],
  const_rules: &mut [String],
) -> String {
  let decls = decls.as_ref();
  let pseudo = pseudos
    .iter()
    .filter(|&p| p != "::thumb")
    .collect::<Vec<&String>>();
  let pseudo_strs: Vec<&str> = pseudo.iter().map(|s| s.as_str()).collect();
  let pseudo = pseudo_strs.join("");

  let mut combined_at_rules = Vec::with_capacity(at_rules.len() + const_rules.len());

  combined_at_rules.extend_from_slice(at_rules);
  combined_at_rules.extend_from_slice(const_rules);

  // Bump specificity of stylex.when selectors
  let has_where = pseudo.contains(":where(");
  let extra_class_for_where = if has_where {
    format!(".{}", class_name)
  } else {
    String::default()
  };

  let mut selector_for_at_rules = format!(
    ".{class_name}{extra_class_for_where}{}{pseudo}",
    combined_at_rules
      .iter()
      .map(|_| format!(".{}", class_name))
      .collect::<Vec<String>>()
      .join(""),
  );

  if pseudos.contains(&"::thumb".to_string()) {
    selector_for_at_rules = THUMB_VARIANTS
      .iter()
      .map(|suffix| format!("{}{}", selector_for_at_rules, suffix))
      .collect::<Vec<String>>()
      .join(", ");
  }

  combined_at_rules.iter().fold(
    format!("{}{{{}}}", selector_for_at_rules, decls),
    |acc, at_rule| format!("{}{{{}}}", at_rule, acc),
  )
}

pub(crate) fn generate_css_rule(
  class_name: &str,
  key: &str,
  values: &[String],
  pseudos: &mut [String],
  at_rules: &mut [String],
  const_rules: &mut [String],
  options: &StyleXStateOptions,
) -> InjectableStyle {
  let mut pairs: Vec<Pair> = vec![];

  for value in values {
    pairs.push(Pair::new(key.to_string(), value.clone()));
  }

  let ltr_pairs: Vec<Pair> = pairs
    .iter()
    .map(|pair| generate_ltr(pair, options))
    .collect::<Vec<Pair>>();

  let rtl_pairs: Vec<Pair> = pairs
    .iter()
    .filter_map(|pair| generate_rtl(pair, options))
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

  let ltr_rule = build_nested_css_rule(class_name, ltr_decls, pseudos, at_rules, const_rules);
  let rtl_rule = if rtl_decls.is_empty() {
    None
  } else {
    Some(build_nested_css_rule(
      class_name,
      rtl_decls,
      pseudos,
      at_rules,
      const_rules,
    ))
  };

  let priority = get_priority(key)
    + pseudos.iter().map(get_priority).sum::<f64>()
    + at_rules.iter().map(get_priority).sum::<f64>()
    + const_rules.iter().map(get_priority).sum::<f64>();

  InjectableStyle {
    priority: Some(priority),
    rtl: rtl_rule,
    ltr: ltr_rule,
  }
}

// Helper to calculate compound pseudo priority (e.g. :hover::after => sum of parts)
fn get_compound_pseudo_priority(key: &str) -> Option<f64> {
  let mut parts: Vec<String> = vec![];

  for mat in PSEUDO_PART_REGEX.find_iter(key).flatten() {
    parts.push(mat.as_str().to_string());
  }

  // Only handle chains of simple pseudo-classes and pseudo-elements.
  // Opt out if there's zero/one part or any functional pseudo-class.
  if parts.len() <= 1 || parts.iter().any(|p| p.contains('(')) {
    return None;
  }

  let mut total: f64 = 0.0;

  for part in parts.iter() {
    total += if part.starts_with("::") {
      PSEUDO_ELEMENT_PRIORITY
    } else {
      **PSEUDO_CLASS_PRIORITIES.get(part.as_str()).unwrap_or(&&40.0)
    };
  }

  Some(total)
}

fn get_at_rule_priority(key: &str) -> Option<f64> {
  if key.starts_with("--") {
    return Some(1.0);
  }

  if key.starts_with("@supports") {
    return AT_RULE_PRIORITIES.get("@supports").map(|v| **v);
  }

  if key.starts_with("@media") {
    return AT_RULE_PRIORITIES.get("@media").map(|v| **v);
  }

  if key.starts_with("@container") {
    return AT_RULE_PRIORITIES.get("@container").map(|v| **v);
  }

  None
}

fn get_pseudo_element_priority(key: &str) -> Option<f64> {
  if key.starts_with("::") {
    return Some(PSEUDO_ELEMENT_PRIORITY);
  }

  None
}

fn get_pseudo_class_priority(key: &str) -> Option<f64> {
  let pseudo_base = |p: &str| -> f64 { **PSEUDO_CLASS_PRIORITIES.get(p).unwrap_or(&&40.0) / 100.0 };

  // Check ancestor selector
  if let Ok(Some(captures)) = ANCESTOR_SELECTOR.captures(key)
    && let Some(pseudo) = captures.get(1)
  {
    return Some(10.0 + pseudo_base(pseudo.as_str()));
  }

  // Check descendant selector
  if let Ok(Some(captures)) = DESCENDANT_SELECTOR.captures(key)
    && let Some(pseudo) = captures.get(1)
  {
    return Some(15.0 + pseudo_base(pseudo.as_str()));
  }

  // Check any sibling selector (must come before individual sibling selectors)
  if let Ok(Some(captures)) = ANY_SIBLING_SELECTOR.captures(key)
    && let (Some(pseudo1), Some(pseudo2)) = (captures.get(1), captures.get(2))
  {
    return Some(20.0 + pseudo_base(pseudo1.as_str()).max(pseudo_base(pseudo2.as_str())));
  }

  // Check sibling before selector
  if let Ok(Some(captures)) = SIBLING_BEFORE_SELECTOR.captures(key)
    && let Some(pseudo) = captures.get(1)
  {
    return Some(30.0 + pseudo_base(pseudo.as_str()));
  }

  // Check sibling after selector
  if let Ok(Some(captures)) = SIBLING_AFTER_SELECTOR.captures(key)
    && let Some(pseudo) = captures.get(1)
  {
    return Some(40.0 + pseudo_base(pseudo.as_str()));
  }

  if key.starts_with(':') {
    let prop: &str = key.split('(').next().unwrap_or(key);

    return Some(**PSEUDO_CLASS_PRIORITIES.get(prop).unwrap_or(&&40.0));
  }

  None
}

fn get_default_priority(key: &str) -> Option<f64> {
  if SHORTHANDS_OF_SHORTHANDS.contains(key) {
    return Some(1000.0);
  }

  if SHORTHANDS_OF_LONGHANDS.contains(key) {
    return Some(2000.0);
  }

  if LONG_HAND_LOGICAL.contains(key) {
    return Some(3000.0);
  }

  if LONG_HAND_PHYSICAL.contains(key) {
    return Some(4000.0);
  }

  None
}

pub(crate) fn get_priority(key: impl AsRef<str>) -> f64 {
  let key = key.as_ref();
  if let Some(at_rule_priority) = get_at_rule_priority(key) {
    return at_rule_priority;
  }

  if let Some(compound_priority) = get_compound_pseudo_priority(key) {
    return compound_priority;
  }

  if let Some(pseudo_element_priority) = get_pseudo_element_priority(key) {
    return pseudo_element_priority;
  }

  if let Some(pseudo_class_priority) = get_pseudo_class_priority(key) {
    return pseudo_class_priority;
  }

  if let Some(default_priority) = get_default_priority(key) {
    return default_priority;
  }

  3000.0
}

pub(crate) fn transform_value(
  key: impl AsRef<str>,
  value: impl AsRef<str>,
  state: &StateManager,
) -> String {
  let key = key.as_ref();
  let value = value.as_ref();
  let css_property_value = value.trim();

  let value = match &css_property_value.parse::<f64>() {
    Ok(value) => format!(
      "{0}{1}",
      round_to_decimal_places(*value, 4),
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

  normalize_css_property_value(key, value.as_ref(), &state.options)
}

pub(crate) fn transform_value_cached(
  key: impl AsRef<str>,
  value: impl AsRef<str>,
  state: &mut StateManager,
) -> String {
  let key = key.as_ref();
  let value = value.as_ref();
  let cache_key: String = format!("{}:{}", key, value);

  let cache = state.css_property_seen.get(&cache_key);

  if let Some(result) = cache {
    return result.to_string();
  }

  let result = transform_value(key, value, state);

  state.css_property_seen.insert(cache_key, result.clone());

  result
}

pub fn swc_parse_css(source: impl AsRef<str>) -> (Result<Stylesheet, Error>, Vec<Error>) {
  let source = source.as_ref();
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

  let is_css_variable = css_property.starts_with("--");

  let css_property_for_parsing = if is_css_variable {
    "color"
  } else {
    css_property
  };

  let css_rule = if css_property_for_parsing.starts_with(':') {
    format!("{0} {1}", css_property_for_parsing, css_property_value)
  } else {
    format!(
      "* {{ {0}: {1} }}",
      css_property_for_parsing, css_property_value
    )
  };

  let (parsed_css, errors) = swc_parse_css(css_rule.as_str());

  if !errors.is_empty() {
    let mut error_message = match errors.first() {
      Some(e) => e.message().to_string(),
      None => stylex_panic!("CSS parsing failed but no error details were available."),
    };

    if error_message.ends_with("expected ')'") || error_message.ends_with("expected '('") {
      error_message = LINT_UNCLOSED_FUNCTION.to_string();
    }

    stylex_panic!("{}, css rule: {}", error_message, css_rule)
  }

  match parsed_css {
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

      let parsed_ast = base_normalizer(
        parsed_css_property_value,
        options.enable_font_size_px_to_rem,
        Some(css_property),
      );

      // for normalizer in normalizers {
      //   parsed_ast = normalizer(parsed_ast, options.enable_font_size_px_to_rem);
      // }

      let result = whitespace_normalizer(stringify(&parsed_ast));

      convert_css_function_to_camel_case(result.as_str())
    },
    Err(err) => {
      stylex_panic!("{}", err.message())
    },
  }
}

// type Normalizer = fn(Stylesheet, bool) -> Stylesheet;
// type Validator = fn(Stylesheet);

pub(crate) fn get_number_suffix(key: impl AsRef<str>) -> String {
  let key = key.as_ref();
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

  match Emit::emit(&mut codegen, node) {
    Ok(_) => {},
    Err(e) => stylex_panic!("CSS codegen emit failed: {}", e),
  };

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
      .replace_all(buf.as_str(), |caps: &fancy_regex::Captures| {
        caps
          .get(1)
          .map_or(String::default(), |m| m.as_str().to_string())
      })
      .to_string();
  }

  result
}

/// Converts a camelCase CSS property name to its hyphenated form.
///
/// Custom properties (`--*`) are returned as-is. Vendor-prefixed properties
/// (e.g. `MsTransition`, `WebkitTapHighlightColor`) are converted to their
/// standard hyphenated forms (`-ms-transition`, `-webkit-tap-highlight-color`).
pub(crate) fn normalize_css_property_name(prop: impl AsRef<str>) -> String {
  let prop = prop.as_ref();
  if prop.starts_with("--") {
    return prop.to_string();
  }
  dashify(prop)
}

/// Serializes a list of key-value pairs into an inline CSS style string.
///
/// Each pair is formatted as `property:value` and joined with `;`.
pub(crate) fn inline_style_to_css_string(pairs: &[Pair]) -> String {
  pairs
    .iter()
    .map(|p| format!("{}:{}", normalize_css_property_name(&p.key), p.value))
    .collect::<Vec<_>>()
    .join(";")
}
