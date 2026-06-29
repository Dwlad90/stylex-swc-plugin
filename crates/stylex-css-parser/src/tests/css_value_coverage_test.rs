// Additional coverage tests for css_value.rs.
// Targets branches not exercised by the existing suite, notably:
//   - as_number(): Percentage and Dimension arms, and the None arm  (lines 67-69)
//   - as_percentage(): None arm                                      (line 77)
//   - as_string(): None arm                                          (line 85)
//   - as_angle(): None arm                                           (line 92)
//   - as_color(): None arm                                           (line 100)
//   - as_dimension(): None arm                                       (line 109)
//   - is_string(), is_ident(), is_function(), is_sequence(),
//     is_angle(), is_color()                                         (lines 142-162)
//   - has_unit(): None arm for non-Dimension values                  (line 173)
//   - Display impl: String, Sequence, Angle, Color, Length,
//     Token, None variants                                           (lines 227-252)
//   - From<SimpleToken> for CssValue: String arm                     (line 306)

use super::*;

// ---------------------------------------------------------------------------
// as_number(): uncovered arms
// ---------------------------------------------------------------------------

#[test]
fn as_number_returns_some_for_percentage() {
  // CssValue::Percentage arm (line 67)
  let val = CssValue::percentage(75.0);
  assert_eq!(val.as_number(), Some(75.0));
}

#[test]
fn as_number_returns_some_for_dimension() {
  // CssValue::Dimension arm (line 68)
  let val = CssValue::dimension(10.0, "px");
  assert_eq!(val.as_number(), Some(10.0));
}

#[test]
fn as_number_returns_none_for_ident() {
  // _ => None arm (line 69)
  let val = CssValue::ident("auto");
  assert_eq!(val.as_number(), None);
}

#[test]
fn as_number_returns_none_for_none_variant() {
  // _ => None arm (line 69)
  assert_eq!(CssValue::None.as_number(), None);
}

// ---------------------------------------------------------------------------
// as_percentage(): None arm
// ---------------------------------------------------------------------------

#[test]
fn as_percentage_returns_none_for_number() {
  // _ => None arm (line 77)
  let val = CssValue::number(42.0);
  assert_eq!(val.as_percentage(), None);
}

#[test]
fn as_percentage_returns_none_for_dimension() {
  let val = CssValue::dimension(10.0, "px");
  assert_eq!(val.as_percentage(), None);
}

// ---------------------------------------------------------------------------
// as_string(): None arm
// ---------------------------------------------------------------------------

#[test]
fn as_string_returns_none_for_number() {
  // _ => None arm (line 85)
  let val = CssValue::number(42.0);
  assert!(val.as_string().is_none());
}

#[test]
fn as_string_returns_none_for_none_variant() {
  assert!(CssValue::None.as_string().is_none());
}

// ---------------------------------------------------------------------------
// as_angle(): Some arm and None arm
// ---------------------------------------------------------------------------

#[test]
fn as_angle_returns_some_for_angle_variant() {
  // CssValue::Angle(a) => Some(a) arm (line 92)
  use crate::css_types::Angle;
  let angle = Angle::new(45.0, "deg");
  let val = CssValue::Angle(angle.clone());
  let result = val.as_angle();
  assert!(
    result.is_some(),
    "as_angle() should return Some for CssValue::Angle"
  );
  assert_eq!(result.unwrap().value, angle.value);
}

#[test]
fn as_angle_returns_none_for_number() {
  // _ => None arm (line 93)
  assert!(CssValue::number(42.0).as_angle().is_none());
}

#[test]
fn as_angle_returns_none_for_ident() {
  assert!(CssValue::ident("auto").as_angle().is_none());
}

// ---------------------------------------------------------------------------
// as_color(): Some arm and None arm
// ---------------------------------------------------------------------------

#[test]
fn as_color_returns_some_for_color_variant() {
  // CssValue::Color(c) => Some(c) arm (line 100)
  use crate::css_types::color::{Color, NamedColor};
  let color = Color::Named(NamedColor::new("blue".to_string()));
  let val = CssValue::Color(color);
  let result = val.as_color();
  assert!(
    result.is_some(),
    "as_color() should return Some for CssValue::Color"
  );
}

#[test]
fn as_color_returns_none_for_number() {
  // _ => None arm (line 101)
  assert!(CssValue::number(0.0).as_color().is_none());
}

#[test]
fn as_color_returns_none_for_string() {
  assert!(CssValue::string("red").as_color().is_none());
}

// ---------------------------------------------------------------------------
// as_dimension(): None arm
// ---------------------------------------------------------------------------

#[test]
fn as_dimension_returns_none_for_number() {
  // _ => None arm (line 109)
  assert!(CssValue::number(42.0).as_dimension().is_none());
}

#[test]
fn as_dimension_returns_none_for_percentage() {
  assert!(CssValue::percentage(50.0).as_dimension().is_none());
}

// ---------------------------------------------------------------------------
// is_string(), is_ident(), is_function(), is_sequence(), is_angle(), is_color()
// ---------------------------------------------------------------------------

#[test]
fn is_string_true_for_string_variant() {
  // line 142
  assert!(CssValue::string("hello").is_string());
}

#[test]
fn is_string_false_for_ident() {
  assert!(!CssValue::ident("auto").is_string());
}

#[test]
fn is_ident_true_for_ident_variant() {
  // line 146
  assert!(CssValue::ident("inherit").is_ident());
}

#[test]
fn is_ident_false_for_string() {
  assert!(!CssValue::string("inherit").is_ident());
}

#[test]
fn is_function_true_for_function_variant() {
  // line 150
  let func = CssValue::function("rgb", vec![]);
  assert!(func.is_function());
}

#[test]
fn is_function_false_for_ident() {
  assert!(!CssValue::ident("rgb").is_function());
}

#[test]
fn is_sequence_true_for_sequence_variant() {
  // line 154
  let seq = CssValue::sequence(vec![CssValue::number(1.0)]);
  assert!(seq.is_sequence());
}

#[test]
fn is_sequence_false_for_number() {
  assert!(!CssValue::number(1.0).is_sequence());
}

#[test]
fn is_angle_true_for_angle_variant() {
  // line 158
  use crate::css_types::Angle;
  let angle: CssValue = Angle::new(45.0, "deg").into();
  assert!(angle.is_angle());
}

#[test]
fn is_angle_false_for_dimension() {
  assert!(!CssValue::dimension(45.0, "deg").is_angle());
}

#[test]
fn is_color_true_for_color_variant() {
  // line 162
  use crate::css_types::color::{Color, NamedColor};
  let color: CssValue = Color::Named(NamedColor::new("blue".to_string())).into();
  assert!(color.is_color());
}

#[test]
fn is_color_false_for_string() {
  assert!(!CssValue::string("blue").is_color());
}

// ---------------------------------------------------------------------------
// has_unit(): _ => false arm for non-Dimension values
// ---------------------------------------------------------------------------

#[test]
fn has_unit_returns_false_for_number() {
  // _ => false arm (line 173)
  assert!(!CssValue::number(42.0).has_unit("px"));
}

#[test]
fn has_unit_returns_false_for_percentage() {
  assert!(!CssValue::percentage(50.0).has_unit("%"));
}

#[test]
fn has_unit_returns_false_for_wrong_unit() {
  let dim = CssValue::dimension(10.0, "em");
  assert!(!dim.has_unit("px"));
}

// ---------------------------------------------------------------------------
// Display impl: uncovered variants
// ---------------------------------------------------------------------------

#[test]
fn display_string_variant_wraps_in_quotes() {
  // CssValue::String (line 227)
  let val = CssValue::String("hello".to_string());
  assert_eq!(val.to_string(), "\"hello\"");
}

#[test]
fn display_function_with_args_formats_correctly() {
  // lines 229-237
  let val = CssValue::function(
    "rgb",
    vec![
      CssValue::number(255.0),
      CssValue::number(0.0),
      CssValue::number(0.0),
    ],
  );
  assert_eq!(val.to_string(), "rgb(255, 0, 0)");
}

#[test]
fn display_function_single_arg_no_comma() {
  // Exercises the i == 0 path (no comma) inside the Function Display loop (line 232)
  let val = CssValue::function("var", vec![CssValue::ident("--color")]);
  assert_eq!(val.to_string(), "var(--color)");
}

#[test]
fn display_function_zero_args() {
  // Empty args list — the loop body is never entered (line 231 loop with 0 iters)
  let val = CssValue::function("none", vec![]);
  assert_eq!(val.to_string(), "none()");
}

#[test]
fn display_sequence_single_item_no_space() {
  // CssValue::Sequence with one item — no leading space (lines 239-246, i == 0)
  let val = CssValue::sequence(vec![CssValue::number(1.0)]);
  assert_eq!(val.to_string(), "1");
}

#[test]
fn display_sequence_multiple_items_space_separated() {
  // lines 239-246: i > 0 path adds a space
  let val = CssValue::sequence(vec![
    CssValue::dimension(1.0, "px"),
    CssValue::ident("solid"),
    CssValue::ident("red"),
  ]);
  assert_eq!(val.to_string(), "1px solid red");
}

#[test]
fn display_angle_variant() {
  // CssValue::Angle (line 248)
  use crate::css_types::Angle;
  let val: CssValue = Angle::new(90.0, "deg").into();
  assert_eq!(val.to_string(), "90deg");
}

#[test]
fn display_color_variant() {
  // CssValue::Color (line 249)
  use crate::css_types::color::{Color, NamedColor};
  let val: CssValue = Color::Named(NamedColor::new("red".to_string())).into();
  assert_eq!(val.to_string(), "red");
}

#[test]
fn display_length_variant() {
  // CssValue::Length (line 250)
  use crate::css_types::Length;
  let val: CssValue = Length::new(10.0, "px".to_string()).into();
  assert_eq!(val.to_string(), "10px");
}

#[test]
fn display_token_variant_uses_debug_format() {
  // CssValue::Token (line 251)
  use crate::token_types::SimpleToken;
  let val = CssValue::Token(SimpleToken::Comma);
  let formatted = val.to_string();
  // The Debug format of SimpleToken::Comma is "Comma"
  assert!(
    formatted.contains("Comma"),
    "Token variant display should use Debug format; got: {formatted}"
  );
}

#[test]
fn display_none_variant() {
  // CssValue::None (line 252)
  assert_eq!(CssValue::None.to_string(), "none");
}

// ---------------------------------------------------------------------------
// From<SimpleToken> for CssValue: String arm (line 306)
// ---------------------------------------------------------------------------

#[test]
fn from_simple_token_string_becomes_css_value_string() {
  // SimpleToken::String => CssValue::String (line 306)
  use crate::token_types::SimpleToken;
  let token = SimpleToken::String("Arial".to_string());
  let val: CssValue = token.into();
  assert!(
    val.is_string(),
    "SimpleToken::String should convert to CssValue::String"
  );
  assert_eq!(val.as_string(), Some(&"Arial".to_string()));
}

#[test]
fn from_simple_token_dimension_becomes_css_value_dimension() {
  // SimpleToken::Dimension => CssValue::Dimension (line 305)
  use crate::token_types::SimpleToken;
  let token = SimpleToken::Dimension {
    value: 10.0,
    unit: "px".to_string(),
  };
  let val: CssValue = token.into();
  assert!(val.is_dimension());
  assert_eq!(val.as_dimension(), Some((10.0, &"px".to_string())));
}

#[test]
fn from_simple_token_percentage_becomes_css_value_percentage() {
  // SimpleToken::Percentage => CssValue::Percentage (line 304)
  use crate::token_types::SimpleToken;
  let token = SimpleToken::Percentage(0.5);
  let val: CssValue = token.into();
  assert!(val.is_percentage());
  assert_eq!(val.as_percentage(), Some(0.5));
}

// ---------------------------------------------------------------------------
// Display `?` error-propagation paths (Function & Sequence arms).
//
// The `write!(f, ...)?` operators inside the Function and Sequence Display arms
// only take their `Err` branch when the underlying writer fails. A real
// String-backed formatter never fails, so we drive `Display::fmt` through a
// `fmt::Write` adapter that returns `Err` after a configurable number of
// successful writes. Sweeping that threshold guarantees every `?` along the
// formatting path takes its `Err` branch at least once.
// ---------------------------------------------------------------------------

use std::fmt::{self, Write};

/// A writer that succeeds `remaining` times, then fails every subsequent write.
struct FailAfter {
  remaining: usize,
}

impl Write for FailAfter {
  fn write_str(&mut self, _s: &str) -> fmt::Result {
    if self.remaining == 0 {
      return Err(fmt::Error);
    }
    self.remaining -= 1;
    Ok(())
  }
}

#[test]
fn display_function_propagates_writer_errors_on_every_inner_write() {
  // Two args so the `i > 0` separator branch is exercised too.
  let value = CssValue::Function {
    name: "rgb".to_string(),
    args: vec![CssValue::Number(1.0), CssValue::Number(2.0)],
  };

  // Sweep the failure point across the whole formatting sequence so each
  // `write!(...)?` is, for some threshold, the operator that observes the error.
  let mut saw_err = false;
  let mut saw_ok = false;
  for remaining in 0..32 {
    let mut writer = FailAfter { remaining };
    match write!(writer, "{}", value) {
      Ok(()) => saw_ok = true,
      Err(_) => saw_err = true,
    }
  }
  assert!(saw_err, "a failing writer must surface a formatting error");
  assert!(saw_ok, "a permissive writer must complete formatting");
}

#[test]
fn display_sequence_propagates_writer_errors_on_every_inner_write() {
  // Two items so the `i > 0` separator branch is exercised too.
  let value = CssValue::Sequence(vec![CssValue::Number(1.0), CssValue::Number(2.0)]);

  let mut saw_err = false;
  let mut saw_ok = false;
  for remaining in 0..32 {
    let mut writer = FailAfter { remaining };
    match write!(writer, "{}", value) {
      Ok(()) => saw_ok = true,
      Err(_) => saw_err = true,
    }
  }
  assert!(saw_err, "a failing writer must surface a formatting error");
  assert!(saw_ok, "a permissive writer must complete formatting");
}
