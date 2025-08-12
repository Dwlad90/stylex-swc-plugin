/*!
CSS filter function parser.
Mirrors: packages/style-value-parser/src/css-types/filter-function.js exactly
*/

use crate::{
  css_types::{common_types::{number_or_percentage_parser, NumberOrPercentage}, Angle, Length},
  token_parser::{TokenParser, Tokens},
  token_types::SimpleToken,
};
use std::fmt::{self, Display};

/// Base FilterFunction class - mirrors JavaScript FilterFunction class exactly
#[derive(Debug, Clone, PartialEq)]
pub enum FilterFunction {
  Blur(BlurFilterFunction),
  Brightness(BrightnessFilterFunction),
  Contrast(ContrastFilterFunction),
  Grayscale(GrayscaleFilterFunction),
  HueRotate(HueRotateFilterFunction),
  Invert(InvertFilterFunction),
  Opacity(OpacityFilterFunction),
  Saturate(SaturateFilterFunction),
  Sepia(SepiaFilterFunction),
}

/// BlurFilterFunction class - mirrors JavaScript BlurFilterFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct BlurFilterFunction {
  pub radius: Length,
}

/// BrightnessFilterFunction class - mirrors JavaScript BrightnessFilterFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct BrightnessFilterFunction {
  pub percentage: f64,
}

/// ContrastFilterFunction class - mirrors JavaScript ContrastFilterFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct ContrastFilterFunction {
  pub amount: f64,
}

/// GrayscaleFilterFunction class - mirrors JavaScript GrayscaleFilterFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct GrayscaleFilterFunction {
  pub amount: f64,
}

/// HueRotateFilterFunction class - mirrors JavaScript HueRotateFilterFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct HueRotateFilterFunction {
  pub angle: Angle,
}

/// InvertFilterFunction class - mirrors JavaScript InvertFilterFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct InvertFilterFunction {
  pub amount: f64,
}

/// OpacityFilterFunction class - mirrors JavaScript OpacityFilterFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct OpacityFilterFunction {
  pub amount: f64,
}

/// SaturateFilterFunction class - mirrors JavaScript SaturateFilterFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct SaturateFilterFunction {
  pub amount: f64,
}

/// SepiaFilterFunction class - mirrors JavaScript SepiaFilterFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct SepiaFilterFunction {
  pub amount: f64,
}

impl FilterFunction {
  /// Static parser method - mirrors JavaScript FilterFunction.parser exactly
  pub fn parser() -> TokenParser<FilterFunction> {
    TokenParser::one_of(vec![
      BlurFilterFunction::parser().map(FilterFunction::Blur, Some("blur")),
      BrightnessFilterFunction::parser().map(FilterFunction::Brightness, Some("brightness")),
      ContrastFilterFunction::parser().map(FilterFunction::Contrast, Some("contrast")),
      GrayscaleFilterFunction::parser().map(FilterFunction::Grayscale, Some("grayscale")),
      HueRotateFilterFunction::parser().map(FilterFunction::HueRotate, Some("hue_rotate")),
      InvertFilterFunction::parser().map(FilterFunction::Invert, Some("invert")),
      OpacityFilterFunction::parser().map(FilterFunction::Opacity, Some("opacity")),
      SaturateFilterFunction::parser().map(FilterFunction::Saturate, Some("saturate")),
      SepiaFilterFunction::parser().map(FilterFunction::Sepia, Some("sepia")),
    ])
  }
}

impl BlurFilterFunction {
  pub fn new(radius: Length) -> Self {
    Self { radius }
  }

  /// Mirrors JavaScript BlurFilterFunction.parser exactly
  pub fn parser() -> TokenParser<BlurFilterFunction> {
    let function_token = Tokens::function()
      .where_fn(|token| {
        if let SimpleToken::Function(name) = token {
          name == "blur"
        } else {
          false
        }
      }, Some("blur_function"));

    function_token
      .flat_map(|_| Length::parser(), Some("radius"))
      .flat_map(|radius| {
        Tokens::close_paren().map(move |_| BlurFilterFunction::new(radius.clone()), Some("close"))
      }, Some("close"))
  }
}

impl BrightnessFilterFunction {
  pub fn new(percentage: f64) -> Self {
    Self { percentage }
  }

  /// Mirrors JavaScript BrightnessFilterFunction.parser exactly
  pub fn parser() -> TokenParser<BrightnessFilterFunction> {
    let function_token = Tokens::function()
      .where_fn(|token| {
        if let SimpleToken::Function(name) = token {
          name == "brightness"
        } else {
          false
        }
      }, Some("brightness_function"));

    let number_or_percentage_to_number = number_or_percentage_parser().map(
      |v| match v {
        NumberOrPercentage::Number(n) => n.value as f64,
        NumberOrPercentage::Percentage(p) => p.value as f64 / 100.0,
      },
      Some("to_number"),
    );

    function_token
      .flat_map(move |_| number_or_percentage_to_number.clone(), Some("percentage"))
      .flat_map(|percentage| {
        Tokens::close_paren().map(move |_| BrightnessFilterFunction::new(percentage), Some("close"))
      }, Some("close"))
  }
}

impl ContrastFilterFunction {
  pub fn new(amount: f64) -> Self {
    Self { amount }
  }

  /// Mirrors JavaScript ContrastFilterFunction.parser exactly
  pub fn parser() -> TokenParser<ContrastFilterFunction> {
    let function_token = Tokens::function()
      .where_fn(|token| {
        if let SimpleToken::Function(name) = token {
          name == "contrast"
        } else {
          false
        }
      }, Some("contrast_function"));

    let number_or_percentage_to_number = number_or_percentage_parser().map(
      |v| match v {
        NumberOrPercentage::Number(n) => n.value as f64,
        NumberOrPercentage::Percentage(p) => p.value as f64 / 100.0,
      },
      Some("to_number"),
    );

    function_token
      .flat_map(move |_| {
        number_or_percentage_to_number.clone()
      }, Some("amount"))
      .flat_map(|amount| {
        Tokens::close_paren().map(move |_| ContrastFilterFunction::new(amount), Some("close"))
      }, Some("close"))
  }
}

impl GrayscaleFilterFunction {
  pub fn new(amount: f64) -> Self {
    Self { amount }
  }

  /// Mirrors JavaScript GrayscaleFilterFunction.parser exactly
  pub fn parser() -> TokenParser<GrayscaleFilterFunction> {
    let function_token = Tokens::function()
      .where_fn(|token| {
        if let SimpleToken::Function(name) = token {
          name == "grayscale"
        } else {
          false
        }
      }, Some("grayscale_function"));

    let number_or_percentage_to_number = number_or_percentage_parser().map(
      |v| match v {
        NumberOrPercentage::Number(n) => n.value as f64,
        NumberOrPercentage::Percentage(p) => p.value as f64 / 100.0,
      },
      Some("to_number"),
    );

    function_token
      .flat_map(move |_| {
        number_or_percentage_to_number.clone()
      }, Some("amount"))
      .flat_map(|amount| {
        Tokens::close_paren().map(move |_| GrayscaleFilterFunction::new(amount), Some("close"))
      }, Some("close"))
  }
}

impl HueRotateFilterFunction {
  pub fn new(angle: Angle) -> Self {
    Self { angle }
  }

  /// Mirrors JavaScript HueRotateFilterFunction.parser exactly
  pub fn parser() -> TokenParser<HueRotateFilterFunction> {
    let function_token = Tokens::function()
      .where_fn(|token| {
        if let SimpleToken::Function(name) = token {
          name == "hue-rotate"
        } else {
          false
        }
      }, Some("hue_rotate_function"));

    function_token
      .flat_map(|_| Angle::parser(), Some("angle"))
      .flat_map(|angle| {
        Tokens::close_paren().map(move |_| HueRotateFilterFunction::new(angle.clone()), Some("close"))
      }, Some("close"))
  }
}

impl InvertFilterFunction {
  pub fn new(amount: f64) -> Self {
    Self { amount }
  }

  /// Mirrors JavaScript InvertFilterFunction.parser exactly
  pub fn parser() -> TokenParser<InvertFilterFunction> {
    let function_token = Tokens::function()
      .where_fn(|token| {
        if let SimpleToken::Function(name) = token {
          name == "invert"
        } else {
          false
        }
      }, Some("invert_function"));

    let number_or_percentage_to_number = number_or_percentage_parser().map(
      |v| match v {
        NumberOrPercentage::Number(n) => n.value as f64,
        NumberOrPercentage::Percentage(p) => p.value as f64 / 100.0,
      },
      Some("to_number"),
    );

    function_token
      .flat_map(move |_| {
        number_or_percentage_to_number.clone()
      }, Some("amount"))
      .flat_map(|amount| {
        Tokens::close_paren().map(move |_| InvertFilterFunction::new(amount), Some("close"))
      }, Some("close"))
  }
}

impl OpacityFilterFunction {
  pub fn new(amount: f64) -> Self {
    Self { amount }
  }

  /// Mirrors JavaScript OpacityFilterFunction.parser exactly
  pub fn parser() -> TokenParser<OpacityFilterFunction> {
    let function_token = Tokens::function()
      .where_fn(|token| {
        if let SimpleToken::Function(name) = token {
          name == "opacity"
        } else {
          false
        }
      }, Some("opacity_function"));

    let number_or_percentage_to_number = number_or_percentage_parser().map(
      |v| match v {
        NumberOrPercentage::Number(n) => n.value as f64,
        NumberOrPercentage::Percentage(p) => p.value as f64 / 100.0,
      },
      Some("to_number"),
    );

    function_token
      .flat_map(move |_| {
        number_or_percentage_to_number.clone()
      }, Some("amount"))
      .flat_map(|amount| {
        Tokens::close_paren().map(move |_| OpacityFilterFunction::new(amount), Some("close"))
      }, Some("close"))
  }
}

impl SaturateFilterFunction {
  pub fn new(amount: f64) -> Self {
    Self { amount }
  }

  /// Mirrors JavaScript SaturateFilterFunction.parser exactly
  pub fn parser() -> TokenParser<SaturateFilterFunction> {
    let function_token = Tokens::function()
      .where_fn(|token| {
        if let SimpleToken::Function(name) = token {
          name == "saturate"
        } else {
          false
        }
      }, Some("saturate_function"));

    let number_or_percentage_to_number = number_or_percentage_parser().map(
      |v| match v {
        NumberOrPercentage::Number(n) => n.value as f64,
        NumberOrPercentage::Percentage(p) => p.value as f64 / 100.0,
      },
      Some("to_number"),
    );

    function_token
      .flat_map(move |_| {
        number_or_percentage_to_number.clone()
      }, Some("amount"))
      .flat_map(|amount| {
        Tokens::close_paren().map(move |_| SaturateFilterFunction::new(amount), Some("close"))
      }, Some("close"))
  }
}

impl SepiaFilterFunction {
  pub fn new(amount: f64) -> Self {
    Self { amount }
  }

  /// Mirrors JavaScript SepiaFilterFunction.parser exactly
  pub fn parser() -> TokenParser<SepiaFilterFunction> {
    let function_token = Tokens::function()
      .where_fn(|token| {
        if let SimpleToken::Function(name) = token {
          name == "sepia"
        } else {
          false
        }
      }, Some("sepia_function"));

    let number_or_percentage_to_number = number_or_percentage_parser().map(
      |v| match v {
        NumberOrPercentage::Number(n) => n.value as f64,
        NumberOrPercentage::Percentage(p) => p.value as f64 / 100.0,
      },
      Some("to_number"),
    );

    function_token
      .flat_map(move |_| {
        number_or_percentage_to_number.clone()
      }, Some("amount"))
      .flat_map(|amount| {
        Tokens::close_paren().map(move |_| SepiaFilterFunction::new(amount), Some("close"))
      }, Some("close"))
  }
}

// Display implementations to match JavaScript toString() methods
impl Display for FilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      FilterFunction::Blur(blur) => blur.fmt(f),
      FilterFunction::Brightness(brightness) => brightness.fmt(f),
      FilterFunction::Contrast(contrast) => contrast.fmt(f),
      FilterFunction::Grayscale(grayscale) => grayscale.fmt(f),
      FilterFunction::HueRotate(hue_rotate) => hue_rotate.fmt(f),
      FilterFunction::Invert(invert) => invert.fmt(f),
      FilterFunction::Opacity(opacity) => opacity.fmt(f),
      FilterFunction::Saturate(saturate) => saturate.fmt(f),
      FilterFunction::Sepia(sepia) => sepia.fmt(f),
    }
  }
}

impl Display for BlurFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "blur({})", self.radius)
  }
}

impl Display for BrightnessFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "brightness({})", self.percentage)
  }
}

impl Display for ContrastFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "contrast({})", self.amount)
  }
}

impl Display for GrayscaleFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "grayscale({})", self.amount)
  }
}

impl Display for HueRotateFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "hue-rotate({})", self.angle)
  }
}

impl Display for InvertFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "invert({})", self.amount)
  }
}

impl Display for OpacityFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "opacity({})", self.amount)
  }
}

impl Display for SaturateFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "saturate({})", self.amount)
  }
}

impl Display for SepiaFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "sepia({})", self.amount)
  }
}
