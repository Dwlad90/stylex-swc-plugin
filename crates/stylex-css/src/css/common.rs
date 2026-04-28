use crate::css::{
  generate_ltr::generate_ltr,
  generate_rtl::generate_rtl,
  normalizers::{base::base_normalizer, extract_css_value, normalize_spacing},
  validators::unprefixed_custom_properties::unprefixed_custom_properties_validator,
};
use stylex_constants::constants::{
  common::{
    COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES,
    COLOR_RELATIVE_VALUES_LISTED_NORMALIZED_PROPERTY_VALUES,
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

  for pseudo in pseudos.iter().filter(|pseudo| pseudo.as_str() != "::thumb") {
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

fn contains_css_function_call(value: &str, function_name: &str) -> bool {
  let value_bytes = value.as_bytes();
  let function_bytes = function_name.as_bytes();

  if function_bytes.is_empty() || value_bytes.len() <= function_bytes.len() {
    return false;
  }

  for i in 0..=(value_bytes.len() - function_bytes.len() - 1) {
    if value_bytes[i..i + function_bytes.len()].eq_ignore_ascii_case(function_bytes)
      && value_bytes[i + function_bytes.len()] == b'('
    {
      return true;
    }
  }

  false
}

/// Panics with a formatted CSS parse error.  Replaces unclosed-function
/// messages with a friendlier lint message.  The non-paren error path
/// is defensive — SWC rarely reports non-paren errors for declaration
/// values — so the entire function is excluded from coverage.
#[cfg_attr(coverage_nightly, coverage(off))]
fn handle_css_parse_errors(errors: &[Error], css_rule: &str) -> ! {
  let mut error_message = errors[0].message().to_string();
  if error_message.ends_with("expected ')'") || error_message.ends_with("expected '('") {
    error_message = LINT_UNCLOSED_FUNCTION.to_string();
  }
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
  if COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES
    .iter()
    .chain(COLOR_RELATIVE_VALUES_LISTED_NORMALIZED_PROPERTY_VALUES.iter())
    .any(|css_fnc| contains_css_function_call(css_property_value, css_fnc))
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
  let normalized = normalize_spacing(value);

  convert_css_function_to_camel_case(&normalized)
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

  format!("{}{}", camel_case_name, args)
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
pub fn normalize_css_property_name(prop: &str) -> String {
  if prop.starts_with("--") {
    return prop.to_string();
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
    push_css_decl(&mut out, normalized_key.as_str(), pair.value.as_str());
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
