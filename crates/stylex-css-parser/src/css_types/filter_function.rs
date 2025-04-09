use crate::css_types::{angle::Angle, color::Color, length::Length};
use crate::parser::Parser;
use std::fmt;

use super::common_types::Percentage;

pub trait FilterFunction: fmt::Debug + fmt::Display {
  fn clone_box(&self) -> Box<dyn FilterFunction>;
}

// Add a blanket implementation for all types that implement FilterFunction + Clone
impl<T: 'static + Clone + fmt::Debug + fmt::Display> FilterFunction for T {
  fn clone_box(&self) -> Box<dyn FilterFunction> {
    Box::new(self.clone())
  }
}

#[derive(Debug, Clone)]
pub struct BlurFilterFunction {
  radius: Length,
}

impl BlurFilterFunction {
  pub fn new(radius: Length) -> Self {
    Self { radius }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, Length, String, ()>(
      Some(Parser::<'a, String>::string("blur(")),
      Some(Length::parse().surrounded_by(Parser::<'a, String>::whitespace().optional(), None)),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, radius, _, _) = values.expect("Expected values to be present");
      BlurFilterFunction::new(radius.unwrap())
    })
  }
}

impl fmt::Display for BlurFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "blur({})", self.radius)
  }
}

#[derive(Debug, Clone)]
pub struct BrightnessFilterFunction {
  percentage: f32,
}

impl BrightnessFilterFunction {
  pub fn new(percentage: f32) -> Self {
    Self { percentage }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, f32, String, ()>(
      Some(Parser::<'a, String>::string("brightness(")),
      Some(
        Parser::one_of(vec![
          Percentage::parse().map(|p| p.unwrap().value / 100.0),
          Parser::<'a, f32>::float().where_fn(|n| *n >= 0.0),
        ])
        .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, percentage, _, _) = values.expect("Expected values to be present");
      BrightnessFilterFunction::new(percentage.unwrap())
    })
  }
}

impl fmt::Display for BrightnessFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "brightness({})", self.percentage)
  }
}

#[derive(Debug, Clone)]
pub struct ContrastFilterFunction {
  amount: f32,
}

impl ContrastFilterFunction {
  pub fn new(amount: f32) -> Self {
    Self { amount }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, f32, String, ()>(
      Some(Parser::<'a, String>::string("contrast(")),
      Some(
        Parser::one_of(vec![
          Percentage::parse().map(|p| p.unwrap().value / 100.0),
          Parser::<'a, f32>::float().where_fn(|n| *n >= 0.0),
        ])
        .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, amount, _, _) = values.expect("Expected values to be present");
      ContrastFilterFunction::new(amount.unwrap())
    })
  }
}

impl fmt::Display for ContrastFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "contrast({})", self.amount)
  }
}

#[derive(Debug, Clone)]
pub struct DropShadowFilterFunction {
  offset_x: Length,
  offset_y: Length,
  blur_radius: Length,
  color: Option<Color>,
}

impl DropShadowFilterFunction {
  pub fn new(
    offset_x: Length,
    offset_y: Length,
    blur_radius: Length,
    color: Option<Color>,
  ) -> Self {
    Self {
      offset_x,
      offset_y,
      blur_radius,
      color,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<
      String,
      (Length, Length, Option<Length>, Option<Color>),
      String,
      (),
    >(
      Some(Parser::<'a, String>::string("drop-shadow(")),
      Some(
        Parser::<'a, ()>::sequence::<Length, Length, Length, Color>(
          Some(Length::parse()),
          Some(Length::parse()),
          Some(Length::parse().optional()),
          Some(Color::parse().optional()),
        )
        .to_parser()
        .separated_by(Parser::<'a, ()>::whitespace())
        .surrounded_by(Parser::<'a, String>::whitespace().optional(), None)
        .map(|values| {
          let values = values.unwrap();
          if values.len() != 1 {
            panic!("Expected exactly one sequence of values");
          }
          let (offset_x, offset_y, blur_radius, color) = &values[0];
          (
            offset_x.clone().unwrap(),
            offset_y.clone().unwrap(),
            blur_radius.clone(),
            color.clone(),
          )
        }),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, params, _, _) = values.expect("Expected values to be present");
      let (offset_x, offset_y, blur_radius, color) = params.unwrap();
      DropShadowFilterFunction::new(
        offset_x,
        offset_y,
        blur_radius.unwrap_or_else(|| Length::new(0.0, Some("px".to_string()))),
        color,
      )
    })
  }
}

impl fmt::Display for DropShadowFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut parts = Vec::new();

    parts.push(self.offset_x.to_string());
    parts.push(self.offset_y.to_string());

    if self.blur_radius.value != 0.0 {
      parts.push(self.blur_radius.to_string());
    }

    if let Some(color) = &self.color {
      parts.push(color.to_string());
    }

    write!(f, "drop-shadow({})", parts.join(" "))
  }
}

#[derive(Debug, Clone)]
pub struct GrayscaleFilterFunction {
  amount: f32,
}

impl GrayscaleFilterFunction {
  pub fn new(amount: f32) -> Self {
    Self { amount }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, f32, String, ()>(
      Some(Parser::<'a, String>::string("grayscale(")),
      Some(
        Parser::one_of(vec![
          Percentage::parse().map(|p| p.unwrap().value / 100.0),
          Parser::<'a, f32>::float().where_fn(|n| *n >= 0.0),
        ])
        .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, amount, _, _) = values.expect("Expected values to be present");
      GrayscaleFilterFunction::new(amount.unwrap())
    })
  }
}

impl fmt::Display for GrayscaleFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "grayscale({})", self.amount)
  }
}

#[derive(Debug, Clone)]
pub struct HueRotateFilterFunction {
  angle: Angle,
}

impl HueRotateFilterFunction {
  pub fn new(angle: Angle) -> Self {
    Self { angle }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, Angle, String, ()>(
      Some(Parser::<'a, String>::string("hue-rotate(")),
      Some(Angle::parse()),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, angle, _, _) = values.expect("Expected values to be present");
      HueRotateFilterFunction::new(angle.unwrap())
    })
  }
}

impl fmt::Display for HueRotateFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "hue-rotate({})", self.angle)
  }
}

#[derive(Debug, Clone)]
pub struct InvertFilterFunction {
  amount: f32,
}

impl InvertFilterFunction {
  pub fn new(amount: f32) -> Self {
    Self { amount }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, f32, String, ()>(
      Some(Parser::<'a, String>::string("invert(")),
      Some(
        Parser::one_of(vec![
          Percentage::parse().map(|p| p.unwrap().value / 100.0),
          Parser::<'a, f32>::float().where_fn(|n| *n >= 0.0),
        ])
        .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, amount, _, _) = values.expect("Expected values to be present");
      InvertFilterFunction::new(amount.unwrap())
    })
  }
}

impl fmt::Display for InvertFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "invert({})", self.amount)
  }
}

#[derive(Debug, Clone)]
pub struct OpacityFilterFunction {
  amount: f32,
}

impl OpacityFilterFunction {
  pub fn new(amount: f32) -> Self {
    Self { amount }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, f32, String, ()>(
      Some(Parser::<'a, String>::string("opacity(")),
      Some(
        Parser::one_of(vec![
          Percentage::parse().map(|p| p.unwrap().value / 100.0),
          Parser::<'a, f32>::float().where_fn(|n| *n >= 0.0),
        ])
        .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, amount, _, _) = values.expect("Expected values to be present");
      OpacityFilterFunction::new(amount.unwrap())
    })
  }
}

impl fmt::Display for OpacityFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "opacity({})", self.amount)
  }
}

#[derive(Debug, Clone)]
pub struct SaturateFilterFunction {
  amount: f32,
}

impl SaturateFilterFunction {
  pub fn new(amount: f32) -> Self {
    Self { amount }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, f32, String, ()>(
      Some(Parser::<'a, String>::string("saturate(")),
      Some(
        Parser::one_of(vec![
          Percentage::parse().map(|p| p.unwrap().value / 100.0),
          Parser::<'a, f32>::float().where_fn(|n| *n >= 0.0),
        ])
        .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, amount, _, _) = values.expect("Expected values to be present");
      SaturateFilterFunction::new(amount.unwrap())
    })
  }
}

impl fmt::Display for SaturateFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "saturate({})", self.amount)
  }
}

#[derive(Debug, Clone)]
pub struct SepiaFilterFunction {
  amount: f32,
}

impl SepiaFilterFunction {
  pub fn new(amount: f32) -> Self {
    Self { amount }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, f32, String, ()>(
      Some(Parser::<'a, String>::string("sepia(")),
      Some(
        Parser::one_of(vec![
          Percentage::parse().map(|p| p.unwrap().value / 100.0),
          Parser::<'a, f32>::float().where_fn(|n| *n >= 0.0),
        ])
        .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, amount, _, _) = values.expect("Expected values to be present");
      SepiaFilterFunction::new(amount.unwrap())
    })
  }
}

impl fmt::Display for SepiaFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "sepia({})", self.amount)
  }
}

// Define a wrapper for Box<dyn FilterFunction> that implements Clone
#[derive(Debug)]
pub struct BoxedFilterFunction(Box<dyn FilterFunction>);

impl Clone for BoxedFilterFunction {
  fn clone(&self) -> Self {
    Self(self.0.clone_box())
  }
}

impl fmt::Display for BoxedFilterFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl BoxedFilterFunction {
  pub fn new(function: Box<dyn FilterFunction>) -> Self {
    Self(function)
  }
}

pub fn parse_filter_function<'a>() -> Parser<'a, BoxedFilterFunction> {
  Parser::one_of(vec![
    BlurFilterFunction::parse().map(|f| BoxedFilterFunction::new(Box::new(f.unwrap()))),
    BrightnessFilterFunction::parse().map(|f| BoxedFilterFunction::new(Box::new(f.unwrap()))),
    ContrastFilterFunction::parse().map(|f| BoxedFilterFunction::new(Box::new(f.unwrap()))),
    DropShadowFilterFunction::parse().map(|f| BoxedFilterFunction::new(Box::new(f.unwrap()))),
    GrayscaleFilterFunction::parse().map(|f| BoxedFilterFunction::new(Box::new(f.unwrap()))),
    HueRotateFilterFunction::parse().map(|f| BoxedFilterFunction::new(Box::new(f.unwrap()))),
    InvertFilterFunction::parse().map(|f| BoxedFilterFunction::new(Box::new(f.unwrap()))),
    OpacityFilterFunction::parse().map(|f| BoxedFilterFunction::new(Box::new(f.unwrap()))),
    SaturateFilterFunction::parse().map(|f| BoxedFilterFunction::new(Box::new(f.unwrap()))),
    SepiaFilterFunction::parse().map(|f| BoxedFilterFunction::new(Box::new(f.unwrap()))),
  ])
}
