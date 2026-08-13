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
use crate::utils::pseudo::{is_pseudo_class, is_pseudo_element, is_pseudo_selector};
use stylex_constants::constants::{
  common::{COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES, COLOR_RELATIVE_VALUE_FUNCTIONS},
  long_hand_logical::LONG_HAND_LOGICAL,
  long_hand_physical::LONG_HAND_PHYSICAL,
  messages::{
    LINT_RULE_BREAKING_TOKEN, LINT_UNCLOSED_COMMENT, LINT_UNCLOSED_FUNCTION, LINT_UNCLOSED_STRING,
  },
  number_properties::NUMBER_PROPERTY_SUFFIXIES,
  priorities::{AT_RULE_PRIORITIES, PSEUDO_CLASS_PRIORITIES, PSEUDO_ELEMENT_PRIORITY},
  shorthands_of_longhands::SHORTHANDS_OF_LONGHANDS,
  shorthands_of_shorthands::SHORTHANDS_OF_SHORTHANDS,
  unitless_number_properties::UNITLESS_NUMBER_PROPERTIES,
};
use stylex_macros::stylex_panic;
use stylex_regex::regex::{
  ANCESTOR_SELECTOR, ANY_SIBLING_SELECTOR, CLEAN_CSS_VAR, DESCENDANT_SELECTOR, PSEUDO_PART_REGEX,
  SIBLING_AFTER_SELECTOR, SIBLING_BEFORE_SELECTOR,
};
use stylex_structures::{pair::Pair, stylex_state_options::StyleXStateOptions};
use stylex_types::structures::injectable_style::InjectableStyle;
use stylex_utils::{number::to_js_string, string::dashify};
use swc_core::{
  atoms::Atom,
  common::{BytePos, input::StringInput, source_map::SmallPos},
  css::{
    ast::{Function, FunctionName, Ident, Stylesheet},
    codegen::{
      CodeGenerator, CodegenConfig, Emit,
      writer::basic::{BasicCssWriter, BasicCssWriterConfig},
    },
    parser::{error::Error, parse_string_input, parser::ParserConfig},
    visit::{Visit, VisitWith},
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
  // Classification is `is_pseudo_element`, so entries must already be
  // normalized to the modern double-colon form; a legacy `:before` would be
  // mis-sorted into the pseudo-class group.
  // Pseudo-classes first (pseudo-elements are excluded, which also drops
  // `::thumb`)...
  for pseudo in pseudos.iter().filter(|pseudo| !is_pseudo_element(pseudo)) {
    result.push_str(pseudo);
  }
  // ...then pseudo-elements, still skipping the expanded `::thumb`.
  for pseudo in pseudos
    .iter()
    .filter(|pseudo| is_pseudo_element(pseudo) && pseudo.as_str() != "::thumb")
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
      if is_pseudo_element(part) {
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
  if is_pseudo_element(key) {
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

  // This function prices pseudo classes only; pseudo elements are priced by
  // `get_pseudo_element_priority`. The bare colon this replaced also matched
  // `::before`, which was unreachable because `get_priority` probes elements
  // first — so the narrowing changes no output, and it holds whether or not
  // that ordering survives.
  if is_pseudo_class(key) {
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

fn normalize_spacing_only_value(value: &str, normalize_commas: bool) -> String {
  let mut normalized = String::with_capacity(value.len());
  let mut previous_char: Option<char> = None;
  let mut quote: Option<char> = None;
  let mut escaped = false;
  let mut is_comment = false;
  let mut chars = value.chars().peekable();

  while let Some(ch) = chars.next() {
    if is_comment {
      normalized.push(ch);

      if ch == '*' && chars.peek() == Some(&'/') {
        normalized.push('/');
        chars.next();
        previous_char = Some('/');
        is_comment = false;
      } else {
        previous_char = Some(ch);
      }

      continue;
    }

    if let Some(current_quote) = quote {
      normalized.push(ch);

      if escaped {
        escaped = false;
      } else if ch == '\\' {
        escaped = true;
      } else if ch == current_quote {
        quote = None;
      }

      previous_char = Some(ch);
      continue;
    }

    if ch == '/' && chars.peek() == Some(&'*') {
      normalized.push('/');
      normalized.push('*');
      chars.next();
      previous_char = Some('*');
      is_comment = true;
      continue;
    }

    if ch == '\'' || ch == '"' {
      normalized.push(ch);
      previous_char = Some(ch);
      quote = Some(ch);
      continue;
    }

    if ch.is_whitespace() {
      while chars.next_if(|next| next.is_whitespace()).is_some() {}

      match (previous_char, chars.peek()) {
        // Leading and trailing whitespace, and whitespace hugging a paren. A
        // stray leading/trailing space would change the hash the class name is
        // derived from, so it must not survive here.
        (None, _) | (_, None) | (Some('('), _) | (_, Some(')')) => continue,
        (Some(','), _) | (_, Some(',')) if normalize_commas => continue,
        _ => {},
      }

      normalized.push(' ');
      previous_char = Some(' ');
      continue;
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

/// Structural facts about a raw CSS property value, all gathered in one pass.
///
/// All four checks share the same string/comment tokenizer, so scanning once
/// keeps them from drifting apart and does the work of several passes over every
/// declaration the compiler sees.
#[derive(Debug, Default, PartialEq, Eq)]
struct ValueStructure {
  /// A quoted string was opened and never closed.
  has_unclosed_string: bool,
  /// A `(` was never matched by a `)`.
  has_unclosed_function: bool,
  /// A `/*` was never matched by a `*/`.
  has_unclosed_comment: bool,
  /// A `{`, `}` or `;` occurs outside of strings and comments. Emitting such a
  /// value verbatim would terminate the rule the compiler is generating and
  /// splice arbitrary CSS into the stylesheet, so it must never take the
  /// "preserve unknown syntax" fallback in [`normalize_css_property_value`].
  has_rule_breaking_token: bool,
}

impl ValueStructure {
  /// Returns `true` when the value can be emitted into the generated stylesheet
  /// verbatim without being able to escape its own declaration.
  ///
  /// Only the two paths that bypass SWC's codegen — the relative-colour/spacing
  /// only path and the "preserve unknown syntax" fallback — need this check.
  /// Values that round-trip through SWC are re-serialized from an AST and are
  /// inert by construction.
  fn is_inert(&self) -> bool {
    !self.has_rule_breaking_token && !self.has_unclosed_comment
  }
}

fn scan_value_structure(css_property_value: &str) -> ValueStructure {
  let value = css_property_value.as_bytes();
  let mut structure = ValueStructure::default();
  let mut quote: Option<u8> = None;
  let mut is_comment = false;
  let mut paren_depth: usize = 0;
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
      None if byte == b')' => {
        paren_depth = paren_depth.saturating_sub(1);
      },
      None if matches!(byte, b'{' | b'}' | b';') => {
        structure.has_rule_breaking_token = true;
      },
      _ => {},
    }

    index += 1;
  }

  structure.has_unclosed_string = quote.is_some();
  structure.has_unclosed_function = paren_depth > 0;
  // A comment left open swallows every rule emitted after this declaration, so
  // it is as rule-breaking as a stray `}`.
  structure.has_unclosed_comment = is_comment && quote.is_none();

  structure
}

/// Placeholder declaration name used to parse a value independently of the
/// grammar SWC associates with the real CSS property. Deliberately not a real
/// property so SWC always falls back to the generic component-value grammar.
const GENERIC_PROPERTY_NAME: &str = "stylexValue";

/// Byte overhead of the `* { ` / `: ` / ` }` wrapper added by [`build_css_rule`].
const CSS_RULE_WRAPPER_LEN: usize = 8;

/// Builds the throwaway CSS rule handed to SWC's parser, and embedded verbatim
/// in parse-error messages.
///
/// Pseudo-selectors (`:hover`) are already rule-shaped and are emitted as
/// `<selector> <value>`; every other property is wrapped in a `* { … }` block.
fn build_css_rule(property: &str, css_property_value: &str, is_pseudo: bool) -> String {
  if is_pseudo {
    let mut rule = String::with_capacity(property.len() + css_property_value.len() + 1);
    rule.push_str(property);
    rule.push(' ');
    rule.push_str(css_property_value);
    rule
  } else {
    let mut rule =
      String::with_capacity(property.len() + css_property_value.len() + CSS_RULE_WRAPPER_LEN);
    rule.push_str("* { ");
    rule.push_str(property);
    rule.push_str(": ");
    rule.push_str(css_property_value);
    rule.push_str(" }");
    rule
  }
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
  // A value that is nothing but a number has no CSS grammar to normalize, and
  // round-tripping it through the parser is what loses it: SWC holds an integer
  // as `i64`, so anything past 2^63 comes back saturated. Re-spell it directly.
  //
  // Checked before the function scans below, which cannot match a bare number
  // and would only be discarded.
  if let Some(number) = parse_bare_number(css_property_value) {
    return strip_leading_zero(&to_js_string(number));
  }

  let should_normalize_spacing_only = COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES
    .iter()
    .any(|css_fnc| contains_css_function_call(css_property_value, css_fnc))
    || contains_relative_color_function(css_property_value);

  // The single colon is deliberate: this only decides how `build_css_rule`
  // spells the error text — a selector wrapping its value, or a declaration
  // inside `* { ... }`. Both kinds of pseudo are selectors, so narrowing this
  // to `::` would print every pseudo class as a declaration.
  let is_pseudo = is_pseudo_selector(css_property);
  let structure = scan_value_structure(css_property_value);

  if structure.has_unclosed_function {
    // Report the error against the author's actual property name; `--x` has no
    // grammar of its own, so `color` stands in for it.
    let error_property = if css_property.starts_with("--") {
      "color"
    } else {
      css_property
    };
    let css_rule = build_css_rule(error_property, css_property_value, is_pseudo);

    stylex_panic!("{}, css rule: {}", LINT_UNCLOSED_FUNCTION, css_rule);
  }

  if structure.has_unclosed_string {
    stylex_panic!("{}", LINT_UNCLOSED_STRING);
  }

  if structure.has_unclosed_comment {
    stylex_panic!("{}", LINT_UNCLOSED_COMMENT);
  }

  if should_normalize_spacing_only {
    // This path bypasses SWC's codegen and emits the author's value verbatim, so
    // it needs the same structural guard as the unknown-syntax fallback below.
    if !structure.is_inert() {
      let css_rule = build_css_rule(css_property, css_property_value, is_pseudo);
      stylex_panic!("{}, css rule: {}", LINT_RULE_BREAKING_TOKEN, css_rule);
    }

    return normalize_spacing_only_value(css_property_value, false);
  }

  // Values are parsed independently of the CSS property's own grammar, so that
  // a property-specific gap in SWC's grammar cannot reject a valid value.
  let parse_property = if is_pseudo {
    css_property
  } else {
    GENERIC_PROPERTY_NAME
  };
  let css_rule = build_css_rule(parse_property, css_property_value, is_pseudo);

  let (parsed_css, errors) = swc_parse_css(css_rule.as_str());

  // A value SWC cannot parse is only preserved when it is *structurally* inert.
  // Anything that could terminate the generated rule (`}`, `;`, `{`) is still
  // rejected, otherwise `height: "1px solid } color: red"` would escape its own
  // declaration and inject arbitrary CSS into the stylesheet.
  if !errors.is_empty() && (is_pseudo || !structure.is_inert()) {
    handle_css_parse_errors(&errors, &css_rule);
  }

  // SWC parser returns errors via the separate `errors` list above,
  // so the `Err` branch is practically unreachable.
  let parsed_css_property_value = parsed_css.unwrap_or_else(
    #[cfg_attr(coverage_nightly, coverage(off))]
    |err| swc_css_parse_unreachable(&err.message()),
  );

  unprefixed_custom_properties_validator(&parsed_css_property_value);

  if !errors.is_empty() {
    // Generic values may use syntax newer than SWC's CSS grammar (`calc-size()`,
    // future functions). Normalize the original value's spacing rather than
    // rejecting syntax the compiler simply does not know yet.
    let normalized_spacing = normalize_spacing_only_value(css_property_value, true);
    return normalize_spacing(&normalized_spacing);
  }

  let parsed_ast = base_normalizer(
    parsed_css_property_value,
    options.enable_font_size_px_to_rem,
    Some(css_property),
  );

  // Collected before codegen, which lowercases the names it emits. A value with
  // no `(` has no function to restore, so the extra traversal is skipped.
  let authored_function_names = if css_property_value.contains('(') {
    collect_function_names(&parsed_ast)
  } else {
    Vec::new()
  };

  let stringified = stringify(&parsed_ast);
  let value = extract_css_value(&stringified);
  let normalized_spacing = normalize_spacing(value);
  let negative_leading_zero_restored = restore_negative_leading_zero(&normalized_spacing);

  let names_restored =
    restore_function_names(&negative_leading_zero_restored, &authored_function_names);

  restore_js_number_spelling(&names_restored)
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

/// Collects every function name in the value, in the order they appear.
///
/// The parser keeps the name the author wrote; codegen is what lowercases it,
/// so the authored spelling has to be taken from the AST beforehand.
fn collect_function_names(ast: &Stylesheet) -> Vec<Atom> {
  struct FunctionNameCollector {
    names: Vec<Atom>,
  }

  impl Visit for FunctionNameCollector {
    fn visit_function(&mut self, func: &Function) {
      if let FunctionName::Ident(name) = &func.name {
        self.names.push(name.value.clone());
      }

      // A nested function is emitted after its parent, so visiting the parent
      // first keeps `names` in the same order the string scan will find them.
      func.visit_children_with(self);
    }
  }

  let mut collector = FunctionNameCollector { names: Vec::new() };
  ast.visit_with(&mut collector);

  collector.names
}

/// Restores the function names the author wrote (e.g. `translatey(0)` back to
/// `translateY(0)`).
///
/// SWC's codegen lowercases function names when minifying — CSS function names
/// are case-insensitive — which would rewrite `translateY(0)` to
/// `translatey(0)` and change every hash derived from it. `authored` supplies
/// the original spellings in source order.
///
/// Matching is case-insensitive and searches forward, so a name the AST does
/// not account for (a `url()` body, say) leaves the scan aligned rather than
/// shifting every later name onto the wrong function.
///
/// Quoted strings and `url()` bodies are copied through untouched, so a name
/// that only appears inside one is left alone.
pub(crate) fn restore_function_names(value: &str, authored: &[Atom]) -> String {
  if !value.contains('(') {
    return value.to_string();
  }

  let mut result = String::with_capacity(value.len());
  // Where the identifier currently being read starts *in `result`*, so a match
  // can be rewritten in place once its `(` arrives.
  let mut ident_start: Option<usize> = None;
  let mut in_quote: Option<char> = None;
  let mut escaped = false;
  let mut url_depth: usize = 0;
  let mut next_authored = 0;

  for (idx, ch) in value.char_indices() {
    if let Some(quote) = in_quote {
      result.push(ch);

      if escaped {
        escaped = false;
      } else if ch == '\\' {
        escaped = true;
      } else if ch == quote {
        in_quote = None;
      }

      continue;
    }

    if url_depth > 0 {
      result.push(ch);

      match ch {
        '(' => url_depth += 1,
        ')' => url_depth -= 1,
        '"' | '\'' => in_quote = Some(ch),
        _ => {},
      }

      continue;
    }

    match ch {
      '"' | '\'' => {
        in_quote = Some(ch);
        ident_start = None;
        result.push(ch);
      },
      '(' => {
        if is_url_function(value, idx) {
          url_depth = 1;
        } else if let Some(start) = ident_start
          && let Some(offset) = authored[next_authored..]
            .iter()
            .position(|name| name.eq_ignore_ascii_case(&result[start..]))
        {
          let name = &authored[next_authored + offset];
          result.truncate(start);
          result.push_str(name);
          next_authored += offset + 1;
        }

        ident_start = None;
        result.push(ch);
      },
      _ if ch.is_alphanumeric() || ch == '-' || ch == '_' => {
        if ident_start.is_none() {
          ident_start = Some(result.len());
        }

        result.push(ch);
      },
      _ => {
        ident_start = None;
        result.push(ch);
      },
    }
  }

  result
}

/// The value as an `f64` when it is a bare number and nothing else.
///
/// Deliberately stricter than `f64::from_str`, which also accepts `inf`,
/// `infinity` and `NaN` — none of which are CSS numbers.
fn parse_bare_number(value: &str) -> Option<f64> {
  if number_token_end(value.as_bytes(), 0, None)? != value.len() {
    return None;
  }

  value.parse::<f64>().ok()
}

/// Re-spells every number in the value the way JS `String(Number)` does.
///
/// SWC's codegen rewrites numbers when minifying, folding trailing zeros into
/// an exponent: `1000` becomes `1e3`, `123000` becomes `123e3`, and
/// `1.0000000000000001e+21` becomes `10000000000000001e5`. None of those are
/// spellings a style value ever has on the way in, and each one changes the
/// hash built from it.
///
/// Every number reaching here has already been through that rewrite, so
/// re-rendering each one from its value restores the single spelling a JS
/// number has. The leading zero is then dropped again, which is a
/// normalization the value is meant to carry.
///
/// Quoted strings and `url()` bodies are copied through untouched.
pub(crate) fn restore_js_number_spelling(value: &str) -> String {
  let bytes = value.as_bytes();
  let mut result = String::with_capacity(value.len());
  let mut index = 0;
  let mut in_quote: Option<u8> = None;
  let mut escaped = false;
  let mut url_depth: usize = 0;

  while index < bytes.len() {
    let byte = bytes[index];

    if let Some(quote) = in_quote {
      index = copy_char(value, index, &mut result);

      if escaped {
        escaped = false;
      } else if byte == b'\\' {
        escaped = true;
      } else if byte == quote {
        in_quote = None;
      }

      continue;
    }

    if url_depth > 0 {
      index = copy_char(value, index, &mut result);

      match byte {
        b'(' => url_depth += 1,
        b')' => url_depth -= 1,
        b'"' | b'\'' => in_quote = Some(byte),
        _ => {},
      }

      continue;
    }

    if byte == b'"' || byte == b'\'' {
      in_quote = Some(byte);
      index = copy_char(value, index, &mut result);
      continue;
    }

    if byte == b'(' && is_url_function(value, index) {
      url_depth = 1;
      index = copy_char(value, index, &mut result);
      continue;
    }

    // The preceding *character*, not the preceding byte: a byte-wise `last()`
    // reads a UTF-8 continuation byte after a non-ASCII ident char, which the
    // guard would not recognise as an ident and would re-spell the digits that
    // follow (`名前007` -> `名前7`).
    match number_token_end(bytes, index, result.chars().next_back()) {
      Some(end) => {
        let number = parse_number_token(&value[index..end]);

        result.push_str(&strip_leading_zero(&to_js_string(number)));
        index = end;
      },
      None => {
        index = copy_char(value, index, &mut result);
      },
    }
  }

  result
}

/// Copies the character starting at `index` and returns the next index.
///
/// Byte-wise copying would split a multi-byte character; every byte this
/// scanner branches on is ASCII, so whole characters can be carried across
/// untouched.
fn copy_char(value: &str, index: usize, result: &mut String) -> usize {
  let char_len = value[index..].chars().next().map_or(1, char::len_utf8);

  result.push_str(&value[index..index + char_len]);

  index + char_len
}

/// The end of the number token starting at `index`, or `None` when nothing
/// there is one.
///
/// A number only starts where a value can: never partway through an identifier
/// (`translate3d`, `名前007`), a hex colour (`#123`), or a dashed name (`--x1`).
/// The exponent is only taken when digits follow it, so the `e` of `1em` stays
/// with the unit.
///
/// `previous` is the preceding character, so `is_alphanumeric` covers the
/// non-ASCII characters a CSS identifier is also allowed to contain.
fn number_token_end(bytes: &[u8], index: usize, previous: Option<char>) -> Option<usize> {
  if previous.is_some_and(|previous| {
    previous.is_alphanumeric() || matches!(previous, '#' | '-' | '_' | '\\')
  }) {
    return None;
  }

  let mut end = index;

  if matches!(bytes.get(end), Some(b'-' | b'+')) {
    end += 1;
  }

  let digits_before_point = take_digits(bytes, &mut end);

  if bytes.get(end) == Some(&b'.') {
    end += 1;
    let digits_after_point = take_digits(bytes, &mut end);

    if !digits_before_point && !digits_after_point {
      return None;
    }
  } else if !digits_before_point {
    return None;
  }

  // `1e3` is one number; the `e` of `1em` belongs to the unit.
  if matches!(bytes.get(end), Some(b'e' | b'E')) {
    let mut exponent_end = end + 1;

    if matches!(bytes.get(exponent_end), Some(b'-' | b'+')) {
      exponent_end += 1;
    }

    if take_digits(bytes, &mut exponent_end) {
      end = exponent_end;
    }
  }

  Some(end)
}

/// The token as an `f64`.
///
/// [`number_token_end`] only ever yields a literal `f64` can parse, so the
/// default is unreachable; taking it keeps this total rather than adding a
/// branch no input can take.
fn parse_number_token(token: &str) -> f64 {
  token.parse::<f64>().unwrap_or_default()
}

fn take_digits(bytes: &[u8], end: &mut usize) -> bool {
  let start = *end;

  while matches!(bytes.get(*end), Some(byte) if byte.is_ascii_digit()) {
    *end += 1;
  }

  *end > start
}

/// Drops the zero before the decimal point (`0.5` -> `.5`), the same
/// normalization the minified spelling carried.
///
/// A negative decimal keeps its zero (`-0.24`), matching
/// [`restore_negative_leading_zero`].
fn strip_leading_zero(number: &str) -> String {
  match number.strip_prefix("0.") {
    Some(rest) => format!(".{}", rest),
    None => number.to_string(),
  }
}

/// Whether the `(` at `open_paren_index` opens a `url()` function, whose body
/// is not CSS-tokenized and so carries no function names of its own.
pub(crate) fn is_url_function(value: &str, open_paren_index: usize) -> bool {
  let mut preceding = value[..open_paren_index].chars().rev();

  for expected in ['l', 'r', 'u'] {
    if !preceding
      .next()
      .is_some_and(|ch| ch.eq_ignore_ascii_case(&expected))
    {
      return false;
    }
  }

  !preceding
    .next()
    .is_some_and(|ch| ch.is_alphanumeric() || matches!(ch, '-' | '_' | '\\'))
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
      .replace_all(result.as_str(), |caps: &fancy_regex::Captures<str>| {
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
