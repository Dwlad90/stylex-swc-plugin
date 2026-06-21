use swc_core::{
  common::DUMMY_SP,
  css::{
    ast::{
      ComponentValue, Declaration, DeclarationName, Dimension, Function, Ident, Length,
      ListOfComponentValues, Number, Stylesheet,
    },
    visit::{VisitMut, VisitMutWith},
  },
};

use stylex_constants::constants::common::ROOT_FONT_SIZE;
use stylex_utils::string::dashify;

struct CssFolder {
  enable_font_size_px_to_rem: bool,
  parent_key: Option<&'static str>,
  is_function_arg: bool,
  current_property: Option<String>,
}

impl CssFolder {
  fn convert_font_size_to_rem_normalizer(&mut self, declaration: &mut Declaration) {
    if let DeclarationName::Ident(ident) = &declaration.name
      && (ident.value == "fontSize" || self.parent_key == Some("fontSize"))
    {
      self.parent_key = Some("fontSize");
      declaration.value.visit_mut_children_with(self);
      self.parent_key = None;
    }
  }
}

impl VisitMut for CssFolder {
  fn visit_mut_list_of_component_values(&mut self, list: &mut ListOfComponentValues) {
    list.visit_mut_children_with(self);
  }

  fn visit_mut_declaration(&mut self, declaration: &mut Declaration) {
    kebab_case_normalizer(declaration);

    if self.enable_font_size_px_to_rem {
      self.convert_font_size_to_rem_normalizer(declaration);
    }

    declaration.visit_mut_children_with(self);
  }

  fn visit_mut_dimension(&mut self, dimension: &mut Dimension) {
    timing_normalizer(dimension);
    zero_dimension_normalizer(
      dimension,
      self.is_function_arg,
      self.current_property.as_deref(),
    );

    dimension.visit_mut_children_with(self);
  }

  fn visit_mut_length(&mut self, length: &mut Length) {
    if self.parent_key == Some("fontSize")
      && length.unit.value.eq("px")
      && length.value.value != 0.0
    {
      let span = length.span;
      length.value = Number {
        value: length.value.value / ROOT_FONT_SIZE as f64,
        raw: None,
        span,
      };
      length.unit = Ident {
        value: "rem".into(),
        raw: None,
        span,
      };
      length.span = DUMMY_SP;
    };
  }

  fn visit_mut_function(&mut self, func: &mut Function) {
    self.is_function_arg = true;

    // NOTE: only last css function value should be visited.
    // SWC always parses at least one component value inside a function,
    // so `last_mut()` practically always returns `Some`.
    if let Some(last) = func.value.last_mut() {
      last.visit_mut_children_with(self);
    }

    self.is_function_arg = false;
  }
}

fn timing_normalizer(dimension: &mut Dimension) {
  if let Dimension::Time(time) = dimension {
    if !time.unit.eq("ms") || time.value.value < 10.0 {
      return;
    }

    time.value = Number {
      value: time.value.value / 1000.0,
      raw: None,
      span: DUMMY_SP,
    };

    time.unit = Ident {
      span: DUMMY_SP,
      value: "s".into(),
      raw: None,
    };
  }
}

fn kebab_case_normalizer(declaration: &mut Declaration) {
  let should_normalize = match &declaration.name {
    DeclarationName::Ident(ident) => {
      ident.value == "transitionProperty" || ident.value == "willChange"
    },
    DeclarationName::DashedIdent(_) => false,
  };

  if !should_normalize {
    return;
  }

  for value in &mut declaration.value {
    if let ComponentValue::Ident(ident) = value
      && !ident.value.starts_with("--")
    {
      ident.value = dashify(ident.value.as_str()).into_owned().into();
    }
  }
}

/// Applies base CSS normalizations (timing units, zero dimensions,
/// camelCase → kebab-case, and optional font-size px→rem conversion)
/// to a parsed `Stylesheet` AST.
pub fn base_normalizer(
  mut ast: Stylesheet,
  enable_font_size_px_to_rem: bool,
  current_property: Option<&str>,
) -> Stylesheet {
  let mut folder = CssFolder {
    enable_font_size_px_to_rem,
    parent_key: None,
    is_function_arg: false,
    current_property: current_property.map(|p| p.to_string()),
  };
  ast.visit_mut_with(&mut folder);
  ast
}

/// Shared zero value for all dimension types.
fn zero_value() -> Number {
  Number {
    value: 0.0,
    raw: None,
    span: DUMMY_SP,
  }
}

/// Returns the appropriate unit for a zero-value dimension.
/// Most units are stripped (empty string); `fr` and `%` are preserved.
///
/// The `"%" => "%"` arm is theoretically correct but SWC's CSS parser
/// emits `0%` as a `Percentage` node (not a `Dimension`), so this arm
/// is unreachable via the normal visitor path — it is covered by direct
/// unit tests only.
pub(crate) fn zero_unit(unit: &Ident) -> Ident {
  let value = match unit.value.as_ref() {
    "fr" => "fr",
    "%" => "%",
    _ => "",
  };

  Ident {
    value: value.into(),
    raw: None,
    span: DUMMY_SP,
  }
}

fn zero_dimension_normalizer(
  dimension: &mut Dimension,
  is_function_arg: bool,
  current_property: Option<&str>,
) {
  // Skip normalization for CSS custom properties
  if let Some(prop) = current_property
    && prop.starts_with("--")
  {
    return;
  }

  if is_function_arg {
    return;
  }

  // Macro to eliminate repetition across dimension variants.
  // Each variant has a `.value` (Number) and `.unit` (Ident) field.
  macro_rules! normalize_zero {
    ($dim:expr) => {{
      if $dim.value.value != 0.0 {
        return;
      }
      $dim.value = zero_value();
      $dim.unit = zero_unit(&$dim.unit);
    }};
  }

  match dimension {
    Dimension::Length(d) => normalize_zero!(d),
    Dimension::Frequency(d) => normalize_zero!(d),
    Dimension::Resolution(d) => normalize_zero!(d),
    Dimension::Flex(d) => normalize_zero!(d),
    // SWC does not produce `UnknownDimension` from standard CSS parsing.
    Dimension::UnknownDimension(d) => normalize_zero!(d),
    Dimension::Angle(angle) => {
      if angle.value.value != 0.0 {
        return;
      }
      angle.value = zero_value();
      angle.unit = Ident {
        span: DUMMY_SP,
        value: "deg".into(),
        raw: None,
      };
    },
    Dimension::Time(time) => {
      if time.value.value != 0.0 {
        return;
      }
      time.value = zero_value();
      time.unit = Ident {
        span: DUMMY_SP,
        value: "s".into(),
        raw: None,
      };
    },
  }
}

/// Returns `true` when `c` is part of a number or identifier, meaning a
/// following `-` is an operator/suffix rather than the sign of a standalone
/// negative number (e.g. the `-` in `1px-.5px` or `a-.5`).
fn is_number_part(c: char) -> bool {
  c.is_alphanumeric() || c == '_' || c == '.'
}

/// Restores the leading zero that SWC's CSS minifier strips from negative
/// decimals (e.g. `-.24px` → `-0.24px`).
///
/// SWC's `minify_numeric` removes the leading zero from every decimal,
/// including negative ones. Should only strip leading zeros from positive values
/// in the `[0, 1)` range (via its `normalizeLeadingZero` normalizer) and leaves
/// negative decimals untouched.
/// Class names are hashed from the normalized value, so this divergence yields
/// mismatched class names between the two compilers. Restoring the leading zero
/// on negative decimals keeps them aligned.
///
/// Scans the string once over Unicode scalar values so multibyte characters
/// (e.g. emoji inside `content`) are preserved. Quoted strings are skipped so
/// values such as `content: "-.5"` are left untouched.
pub(crate) fn restore_negative_leading_zero(value: &str) -> String {
  if !value.contains("-.") {
    return value.to_string();
  }

  let mut result = String::with_capacity(value.len() + 2);
  let mut in_quote: Option<char> = None;
  let mut escaped = false;
  // The previous character, used to tell a sign `-` (start of a negative
  // number) from a `-` that follows a number or identifier.
  let mut prev: Option<char> = None;

  for (idx, cur) in value.char_indices() {
    if let Some(quote) = in_quote {
      result.push(cur);
      if escaped {
        escaped = false;
      } else if cur == '\\' {
        escaped = true;
      } else if cur == quote {
        in_quote = None;
      }
      prev = Some(cur);
      continue;
    }

    if cur == '"' || cur == '\'' {
      in_quote = Some(cur);
      result.push(cur);
      prev = Some(cur);
      continue;
    }

    // A sign-position `-` immediately followed by `.<digit>` lost its leading
    // zero to the minifier; restore it. `.` and digits are single-byte ASCII,
    // so inspecting the raw bytes after `-` is UTF-8 safe.
    if cur == '-' && !prev.is_some_and(is_number_part) {
      let rest = &value.as_bytes()[idx + 1..];
      if rest.first() == Some(&b'.') && rest.get(1).is_some_and(u8::is_ascii_digit) {
        result.push('-');
        result.push('0');
        prev = Some('-');
        continue;
      }
    }

    result.push(cur);
    prev = Some(cur);
  }

  result
}
