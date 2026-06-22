use std::borrow::Cow;

use crate::css::{
  generate_ltr::generate_ltr,
  generate_rtl::generate_rtl,
  normalizers::{
    base::{base_normalizer, restore_negative_leading_zero},
    extract_css_value, normalize_spacing,
  },
  validators::unprefixed_custom_properties::unprefixed_custom_properties_validator,
};
use stylex_constants::constants::{
  common::{COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES, COLOR_RELATIVE_VALUE_FUNCTIONS},
  long_hand_logical::LONG_HAND_LOGICAL,
  long_hand_physical::LONG_HAND_PHYSICAL,
  messages::{LINT_UNCLOSED_FUNCTION, LINT_UNCLOSED_STRING},
  number_properties::NUMBER_PROPERTY_SUFFIXIES,
  priorities::{
    AT_RULE_PRIORITIES, CAMEL_CASE_PRIORITIES, PSEUDO_CLASS_PRIORITIES, PSEUDO_ELEMENT_PRIORITY,
  },
  shorthands_of_longhands::SHORTHANDS_OF_LONGHANDS,
  shorthands_of_shorthands::SHORTHANDS_OF_SHORTHANDS,
  unitless_number_properties::UNITLESS_NUMBER_PROPERTIES,
};
use stylex_macros::stylex_panic;
use stylex_regex::regex::{
  ANCESTOR_SELECTOR, ANY_SIBLING_SELECTOR, CLEAN_CSS_VAR, DESCENDANT_SELECTOR, MANY_SPACES,
  PSEUDO_PART_REGEX, SIBLING_AFTER_SELECTOR, SIBLING_BEFORE_SELECTOR,
};
use stylex_structures::{pair::Pair, stylex_state_options::StyleXStateOptions};
use stylex_types::structures::injectable_style::InjectableStyle;
use stylex_utils::string::dashify;
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

const THUMB_VARIANTS: [&str; 3] = [
  "::-webkit-slider-thumb",
  "::-moz-range-thumb",
  "::-ms-thumb",
];

/// Wraps a CSS declaration string in nested at-rules and pseudo selectors,
/// producing a complete CSS rule string.
pub fn build_nested_css_rule(
  class_name: &str,
  decls: String,
  pseudos: &mut [String],
  at_rules: &mut [String],
  const_rules: &mut [String],
) -> String {
  let has_thumb = pseudos.iter().any(|pseudo| pseudo == "::thumb");
  let has_where = pseudos
    .iter()
    .any(|pseudo| pseudo != "::thumb" && pseudo.contains(":where("));
  let wrapper_count = at_rules.len() + const_rules.len();
  let selector_len = selector_len(class_name, pseudos, has_where, wrapper_count);
  let thumb_selector_len = if has_thumb {
    THUMB_VARIANTS
      .iter()
      .map(|suffix| selector_len + suffix.len())
      .sum::<usize>()
      + ", ".len() * (THUMB_VARIANTS.len() - 1)
  } else {
    selector_len
  };
  let wrapper_len = at_rules
    .iter()
    .chain(const_rules.iter())
    .map(|rule| rule.len() + 2)
    .sum::<usize>();

  let mut result = String::with_capacity(wrapper_len + thumb_selector_len + decls.len() + 2);

  for rule in at_rules.iter().chain(const_rules.iter()).rev() {
    result.push_str(rule);
    result.push('{');
  }

  if has_thumb {
    for (index, suffix) in THUMB_VARIANTS.iter().enumerate() {
      if index > 0 {
        result.push_str(", ");
      }
      push_selector(&mut result, class_name, pseudos, has_where, wrapper_count);
      result.push_str(suffix);
    }
  } else {
    push_selector(&mut result, class_name, pseudos, has_where, wrapper_count);
  }

  result.push('{');
  result.push_str(&decls);
  result.push('}');

  for _ in 0..wrapper_count {
    result.push('}');
  }

  result
}

fn selector_len(
  class_name: &str,
  pseudos: &[String],
  has_where: bool,
  wrapper_count: usize,
) -> usize {
  let class_selector_len = 1 + class_name.len();
  let pseudo_len = pseudos
    .iter()
    .filter(|pseudo| pseudo.as_str() != "::thumb")
    .map(String::len)
    .sum::<usize>();

  class_selector_len
    + usize::from(has_where) * class_selector_len
    + wrapper_count * class_selector_len
    + pseudo_len
}

fn push_selector(
  result: &mut String,
  class_name: &str,
  pseudos: &[String],
  has_where: bool,
  wrapper_count: usize,
) {
  result.push('.');
  result.push_str(class_name);

  if has_where {
    result.push('.');
    result.push_str(class_name);
  }

  for _ in 0..wrapper_count {
    result.push('.');
    result.push_str(class_name);
  }

  // Pseudo-elements (::before, ::after, etc.) must come after pseudo-classes
  // in the selector. e.g. `.class:hover::before` not `.class::before:hover`.
  // Pseudo-classes first (`::`-prefixed entries are excluded, which also
  // drops `::thumb`)...
  for pseudo in pseudos.iter().filter(|pseudo| !pseudo.starts_with("::")) {
    result.push_str(pseudo);
  }
  // ...then pseudo-elements, still skipping the expanded `::thumb`.
  for pseudo in pseudos
    .iter()
    .filter(|pseudo| pseudo.starts_with("::") && pseudo.as_str() != "::thumb")
  {
    result.push_str(pseudo);
  }
}

/// Generates a complete `InjectableStyle` (LTR + optional RTL rule + priority)
/// from a CSS class name, property key, values, pseudos, at-rules and options.
pub fn generate_css_rule(
  class_name: &str,
  key: &str,
  values: &[String],
  pseudos: &mut [String],
  at_rules: &mut [String],
  const_rules: &mut [String],
  options: &StyleXStateOptions,
) -> InjectableStyle {
  let decl_capacity = values
    .iter()
    .map(|value| key.len() + value.len() + 1)
    .sum::<usize>()
    + values.len().saturating_sub(1);
  let mut ltr_decls = String::with_capacity(decl_capacity);
  let mut rtl_decls = String::with_capacity(decl_capacity);

  for value in values {
    let pair = Pair::new(key, value.as_str());
    let ltr_pair = generate_ltr(&pair, options);
    push_css_decl(
      &mut ltr_decls,
      ltr_pair.key.as_ref(),
      ltr_pair.value.as_ref(),
    );

    if let Some(rtl_pair) = generate_rtl(&pair, options) {
      push_css_decl(
        &mut rtl_decls,
        rtl_pair.key.as_ref(),
        rtl_pair.value.as_ref(),
      );
    }
  }

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
    + pseudos.iter().map(|p| get_priority(p)).sum::<f64>()
    + at_rules.iter().map(|a| get_priority(a)).sum::<f64>()
    + const_rules.iter().map(|c| get_priority(c)).sum::<f64>();

  InjectableStyle {
    priority: Some(priority),
    rtl: rtl_rule,
    ltr: ltr_rule,
  }
}

/// Calculates priority for compound pseudo selectors (e.g. `:hover::after`).
fn get_compound_pseudo_priority(key: &str) -> Option<f64> {
  let parts: Vec<&str> = PSEUDO_PART_REGEX
    .find_iter(key)
    .flatten()
    .map(|m| m.as_str())
    .collect();

  // Only handle chains of simple pseudo-classes and pseudo-elements.
  // Opt out if there's zero/one part or any functional pseudo-class.
  if parts.len() <= 1 || parts.iter().any(|p| p.contains('(')) {
    return None;
  }

  let total = parts
    .iter()
    .map(|part| {
      if part.starts_with("::") {
        PSEUDO_ELEMENT_PRIORITY
      } else {
        **PSEUDO_CLASS_PRIORITIES.get(*part).unwrap_or(&&40.0)
      }
    })
    .sum();

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

/// Returns the CSS priority for a given key (property name, at-rule, or
/// pseudo).
pub fn get_priority(key: &str) -> f64 {
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

/// Parses a CSS source string into an SWC `Stylesheet` AST.
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

/// A byte that may appear in a CSS identifier (used to ensure a matched
/// function name is not a suffix of a longer identifier).
fn is_ident_byte(byte: u8) -> bool {
  byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_'
}

fn contains_css_function_call(value: &str, function_name: &str) -> bool {
  find_css_function_call(value, function_name, |_, _| true)
}

fn find_css_function_call(
  value: &str,
  function_name: &str,
  mut matches_after_open_paren: impl FnMut(&[u8], usize) -> bool,
) -> bool {
  let value_bytes = value.as_bytes();
  let function_bytes = function_name.as_bytes();

  if function_bytes.is_empty() || value_bytes.len() <= function_bytes.len() {
    return false;
  }

  let mut quote: Option<u8> = None;
  let mut is_comment = false;
  let mut index = 0;

  while index < value_bytes.len() {
    let byte = value_bytes[index];

    if quote.is_none() {
      if is_comment {
        if byte == b'*' && value_bytes.get(index + 1) == Some(&b'/') {
          is_comment = false;
          index += 2;
          continue;
        }

        index += 1;
        continue;
      }

      if byte == b'/' && value_bytes.get(index + 1) == Some(&b'*') {
        is_comment = true;
        index += 2;
        continue;
      }
    }

    match quote {
      Some(current_quote) if byte == current_quote && !is_escaped(value_bytes, index) => {
        quote = None;
      },
      Some(_) => {},
      None if (byte == b'\'' || byte == b'"') && !is_escaped(value_bytes, index) => {
        quote = Some(byte);
      },
      None
        if index + function_bytes.len() < value_bytes.len()
          && value_bytes[index..index + function_bytes.len()]
            .eq_ignore_ascii_case(function_bytes)
          && value_bytes[index + function_bytes.len()] == b'('
          && (index == 0 || !is_ident_byte(value_bytes[index - 1]))
          && matches_after_open_paren(value_bytes, index + function_bytes.len() + 1) =>
      {
        return true;
      },
      _ => {},
    }

    index += 1;
  }

  false
}

/// Detects CSS relative color syntax, e.g. `rgb(from red r g b)`.
///
/// A relative color function is any color function (see
/// `COLOR_RELATIVE_VALUE_FUNCTIONS`) whose first argument is the `from`
/// keyword. SWC's CSS parser cannot parse this form, so such values are
/// normalized via spacing only instead of being parsed and re-serialized.
fn contains_relative_color_function(value: &str) -> bool {
  COLOR_RELATIVE_VALUE_FUNCTIONS
    .iter()
    .any(|name| has_relative_color_call(value, name))
}

/// Returns `true` if `value` contains `function_name` immediately followed by
/// `(` and, after any whitespace, the `from` keyword (the relative color
/// marker).
fn has_relative_color_call(value: &str, function_name: &str) -> bool {
  const FROM: &[u8] = b"from";

  find_css_function_call(value, function_name, |value_bytes, mut cursor| {
    // Skip whitespace after `(`, then look for the `from` keyword followed by a
    // whitespace boundary.
    while cursor < value_bytes.len() && value_bytes[cursor].is_ascii_whitespace() {
      cursor += 1;
    }

    cursor + FROM.len() < value_bytes.len()
      && value_bytes[cursor..cursor + FROM.len()].eq_ignore_ascii_case(FROM)
      && value_bytes[cursor + FROM.len()].is_ascii_whitespace()
  })
}

fn normalize_spacing_only_value(value: &str) -> String {
  let value = MANY_SPACES.replace_all(value, " ");
  let mut normalized = String::with_capacity(value.len());
  let mut previous_char: Option<char> = None;
  let mut chars = value.chars().peekable();

  while let Some(ch) = chars.next() {
    if ch == ' ' {
      match (previous_char, chars.peek()) {
        (Some('('), _) | (_, Some(')')) => continue,
        _ => {},
      }
    }

    normalized.push(ch);
    previous_char = Some(ch);
  }

  normalized
}

fn is_escaped(value: &[u8], index: usize) -> bool {
  let mut backslash_count = 0;
  let mut cursor = index;

  while cursor > 0 && value[cursor - 1] == b'\\' {
    backslash_count += 1;
    cursor -= 1;
  }

  backslash_count % 2 == 1
}

fn detect_unclosed_strings(css_property_value: &str) {
  let value = css_property_value.as_bytes();
  let mut quote: Option<u8> = None;
  let mut is_comment = false;
  let mut index = 0;

  while index < value.len() {
    let byte = value[index];

    if quote.is_none() {
      if is_comment {
        if byte == b'*' && value.get(index + 1) == Some(&b'/') {
          is_comment = false;
          index += 2;
          continue;
        }

        index += 1;
        continue;
      }

      if byte == b'/' && value.get(index + 1) == Some(&b'*') {
        is_comment = true;
        index += 2;
        continue;
      }
    }

    match quote {
      Some(current_quote) if byte == current_quote && !is_escaped(value, index) => {
        quote = None;
      },
      None if (byte == b'\'' || byte == b'"') && !is_escaped(value, index) => {
        quote = Some(byte);
      },
      _ => {},
    }

    index += 1;
  }

  if quote.is_some() {
    stylex_panic!("{}", LINT_UNCLOSED_STRING);
  }
}

fn has_unclosed_function(css_property_value: &str) -> bool {
  let value = css_property_value.as_bytes();
  let mut quote: Option<u8> = None;
  let mut is_comment = false;
  let mut paren_depth = 0;
  let mut index = 0;

  while index < value.len() {
    let byte = value[index];

    if quote.is_none() {
      if is_comment {
        if byte == b'*' && value.get(index + 1) == Some(&b'/') {
          is_comment = false;
          index += 2;
          continue;
        }

        index += 1;
        continue;
      }

      if byte == b'/' && value.get(index + 1) == Some(&b'*') {
        is_comment = true;
        index += 2;
        continue;
      }
    }

    match quote {
      Some(current_quote) if byte == current_quote && !is_escaped(value, index) => {
        quote = None;
      },
      Some(_) => {},
      None if (byte == b'\'' || byte == b'"') && !is_escaped(value, index) => {
        quote = Some(byte);
      },
      None if byte == b'(' => {
        paren_depth += 1;
      },
      None if byte == b')' && paren_depth > 0 => {
        paren_depth -= 1;
      },
      _ => {},
    }

    index += 1;
  }

  paren_depth > 0
}

/// Panics with a formatted CSS parse error.
fn handle_css_parse_errors(errors: &[Error], css_rule: &str) -> ! {
  let error_message = errors[0].message().to_string();
  stylex_panic!("{}, css rule: {}", error_message, css_rule)
}

/// SWC's CSS parser reports errors via a separate `errors` list. The `Err`
/// branch of the parse result is therefore practically unreachable.
#[cfg_attr(coverage_nightly, coverage(off))]
fn swc_css_parse_unreachable(msg: &str) -> ! {
  stylex_panic!("{}", msg)
}

/// CSS codegen on a well-formed AST never produces an `Err` in practice.
#[cfg_attr(coverage_nightly, coverage(off))]
fn css_codegen_unreachable(e: std::fmt::Error) -> ! {
  stylex_panic!("CSS codegen emit failed: {}", e)
}

/// Normalizes a CSS property value by parsing, normalizing, and re-serializing
/// it.
pub fn normalize_css_property_value(
  css_property: &str,
  css_property_value: &str,
  options: &StyleXStateOptions,
) -> String {
  let should_normalize_spacing_only = COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES
    .iter()
    .any(|css_fnc| contains_css_function_call(css_property_value, css_fnc))
    || contains_relative_color_function(css_property_value);

  let is_css_variable = css_property.starts_with("--");

  let css_property_for_parsing = if is_css_variable {
    "color"
  } else {
    css_property
  };

  let css_rule = if css_property_for_parsing.starts_with(':') {
    let mut rule =
      String::with_capacity(css_property_for_parsing.len() + css_property_value.len() + 1);
    rule.push_str(css_property_for_parsing);
    rule.push(' ');
    rule.push_str(css_property_value);
    rule
  } else {
    let mut rule =
      String::with_capacity(css_property_for_parsing.len() + css_property_value.len() + 8);
    rule.push_str("* { ");
    rule.push_str(css_property_for_parsing);
    rule.push_str(": ");
    rule.push_str(css_property_value);
    rule.push_str(" }");
    rule
  };

  let is_unclosed_function = has_unclosed_function(css_property_value);

  if is_unclosed_function {
    stylex_panic!("{}, css rule: {}", LINT_UNCLOSED_FUNCTION, css_rule);
  }

  detect_unclosed_strings(css_property_value);

  if should_normalize_spacing_only {
    return normalize_spacing_only_value(css_property_value);
  }

  let (parsed_css, errors) = swc_parse_css(css_rule.as_str());

  if !errors.is_empty() {
    handle_css_parse_errors(&errors, &css_rule);
  }

  // SWC parser returns errors via the separate `errors` list above,
  // so the `Err` branch is practically unreachable.
  let parsed_css_property_value = parsed_css.unwrap_or_else(
    #[cfg_attr(coverage_nightly, coverage(off))]
    |err| swc_css_parse_unreachable(&err.message()),
  );

  unprefixed_custom_properties_validator(&parsed_css_property_value);

  let parsed_ast = base_normalizer(
    parsed_css_property_value,
    options.enable_font_size_px_to_rem,
    Some(css_property),
  );

  let stringified = stringify(&parsed_ast);
  let value = extract_css_value(&stringified);
  let normalized_spacing = normalize_spacing(value);
  let negative_leading_zero_restored = restore_negative_leading_zero(&normalized_spacing);

  convert_css_function_to_camel_case(&negative_leading_zero_restored)
}

/// Returns the numeric suffix for a CSS property (`"px"`, `"ms"`, `""`, etc.).
pub fn get_number_suffix(key: &str) -> &'static str {
  if UNITLESS_NUMBER_PROPERTIES.contains(key) || key.starts_with("--") {
    return "";
  }

  match NUMBER_PROPERTY_SUFFIXIES.get(key) {
    Some(suffix) => suffix,
    None => "px",
  }
}

/// Extracts the string value from a CSS `Ident` AST node.
pub fn get_value_from_ident(ident: &Ident) -> String {
  ident.value.to_string()
}

/// Converts a CSS function name to its camelCase equivalent (e.g.
/// `oklab(…)` → `oklab(…)`) when a known mapping exists.
fn convert_css_function_to_camel_case(function: &str) -> String {
  let Some(items) = function.find('(') else {
    return function.to_string();
  };

  let (name, args) = function.split_at(items);

  let Some(camel_case_name) = CAMEL_CASE_PRIORITIES.get(name) else {
    return function.to_string();
  };

  let mut result = String::with_capacity(camel_case_name.len() + args.len());
  result.push_str(camel_case_name);
  result.push_str(args);
  result
}

/// Serializes an SWC `Stylesheet` AST back to a minified CSS string.
pub fn stringify(node: &Stylesheet) -> String {
  let mut buf = String::with_capacity(256);
  let wr = BasicCssWriter::new(&mut buf, None, BasicCssWriterConfig::default());
  let mut codegen = CodeGenerator::new(wr, CodegenConfig { minify: true });

  // CSS codegen on a valid AST never fails in practice.
  Emit::emit(&mut codegen, node).unwrap_or_else(
    #[cfg_attr(coverage_nightly, coverage(off))]
    |e| css_codegen_unreachable(e),
  );

  drop(codegen);

  let mut result = buf.replace('\'', "");

  if result.contains("--\\") {
    /*
     * In CSS, identifiers (including element names, classes, and IDs in
     * selectors) can contain only the characters [a-zA-Z0-9] and ISO 10646
     * characters U+00A0 and higher, plus the hyphen (-) and the underscore
     * (_); they cannot start with a digit, two hyphens, or a hyphen followed
     * by a digit.
     *
     * https://stackoverflow.com/a/27882887/6717252
     *
     * HACK: Replace `--\3{number}` with `--{number}` to simulate original
     * behavior of StyleX
     */

    let clean = CLEAN_CSS_VAR
      .replace_all(result.as_str(), |caps: &fancy_regex::Captures| {
        caps
          .get(1)
          .map_or(String::default(), |m| m.as_str().to_string())
      })
      .to_string();
    result = clean;
  }

  result
}

/// Converts a camelCase CSS property name to its hyphenated form.
///
/// Custom properties (`--*`) are returned as-is. Vendor-prefixed properties
/// (e.g. `MsTransition`, `WebkitTapHighlightColor`) are converted to their
/// standard hyphenated forms (`-ms-transition`, `-webkit-tap-highlight-color`).
pub fn normalize_css_property_name(prop: &str) -> Cow<'_, str> {
  if prop.starts_with("--") {
    return Cow::Borrowed(prop);
  }
  dashify(prop)
}

/// Serializes a list of key-value pairs into an inline CSS style string.
///
/// Each pair is formatted as `property:value` and joined with `;`.
pub fn inline_style_to_css_string(pairs: &[Pair]) -> String {
  let capacity = pairs
    .iter()
    .map(|pair| pair.key.len() + pair.value.len() + 1)
    .sum::<usize>()
    + pairs.len().saturating_sub(1);
  let mut out = String::with_capacity(capacity);

  for pair in pairs {
    let normalized_key = normalize_css_property_name(&pair.key);
    push_css_decl(&mut out, normalized_key.as_ref(), pair.value.as_str());
  }

  out
}

fn push_css_decl(out: &mut String, key: &str, value: &str) {
  if !out.is_empty() {
    out.push(';');
  }

  out.push_str(key);
  out.push(':');
  out.push_str(value);
}
