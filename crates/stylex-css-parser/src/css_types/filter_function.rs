/*!
CSS filter function parser.
Mirrors: packages/style-value-parser/src/css-types/filter-function.js
*/

use crate::{
  css_types::common_types::number_or_percentage_parser,
  css_types::{Length, NumberOrPercentage},
  token_parser::TokenParser,
  token_types::SimpleToken,
};

/// A CSS filter function
#[derive(Debug, Clone, PartialEq)]
pub enum FilterFunction {
  Blur(Length),
  Brightness(f32),
  Contrast(f32),
  Grayscale(f32),
  HueRotate(super::angle::Angle),
  Invert(f32),
  Opacity(f32),
  Saturate(f32),
  Sepia(f32),
}

impl FilterFunction {
  pub fn parse() -> TokenParser<FilterFunction> {
    TokenParser::one_of(vec![
      Self::blur_parser(),
      Self::brightness_parser(),
      Self::contrast_parser(),
      Self::grayscale_parser(),
      Self::hue_rotate_parser(),
      Self::invert_parser(),
      Self::opacity_parser(),
      Self::saturate_parser(),
      Self::sepia_parser(),
    ])
  }

  fn fn_name(name: &str) -> TokenParser<String> {
    TokenParser::<String>::fn_name(name)
  }

  fn number_or_percentage_to_number() -> TokenParser<f32> {
    number_or_percentage_parser().map(
      |v| match v {
        NumberOrPercentage::Number(n) => n.value,
        NumberOrPercentage::Percentage(p) => p.value / 100.0,
      },
      Some("to_number"),
    )
  }

  fn blur_parser() -> TokenParser<FilterFunction> {
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
    Self::fn_name("blur")
      .flat_map(|_| Length::parser(), Some("len"))
      .flat_map(
        move |len| close.clone().map(move |_| len.clone(), Some(")")),
        Some("close"),
      )
      .map(FilterFunction::Blur, Some("to_blur"))
  }

  fn brightness_parser() -> TokenParser<FilterFunction> {
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
    Self::fn_name("brightness")
      .flat_map(|_| Self::number_or_percentage_to_number(), Some("val"))
      .flat_map(
        move |v| close.clone().map(move |_| v, Some(")")),
        Some("close"),
      )
      .map(FilterFunction::Brightness, Some("to_brightness"))
  }

  fn contrast_parser() -> TokenParser<FilterFunction> {
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
    Self::fn_name("contrast")
      .flat_map(|_| Self::number_or_percentage_to_number(), Some("val"))
      .flat_map(
        move |v| close.clone().map(move |_| v, Some(")")),
        Some("close"),
      )
      .map(FilterFunction::Contrast, Some("to_contrast"))
  }

  fn grayscale_parser() -> TokenParser<FilterFunction> {
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
    Self::fn_name("grayscale")
      .flat_map(|_| Self::number_or_percentage_to_number(), Some("val"))
      .flat_map(
        move |v| close.clone().map(move |_| v, Some(")")),
        Some("close"),
      )
      .map(FilterFunction::Grayscale, Some("to_grayscale"))
  }

  fn hue_rotate_parser() -> TokenParser<FilterFunction> {
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
    Self::fn_name("hue-rotate")
      .flat_map(|_| super::angle::Angle::parser(), Some("angle"))
      .flat_map(
        move |a| close.clone().map(move |_| a.clone(), Some(")")),
        Some("close"),
      )
      .map(FilterFunction::HueRotate, Some("to_hue_rotate"))
  }

  fn invert_parser() -> TokenParser<FilterFunction> {
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
    Self::fn_name("invert")
      .flat_map(|_| Self::number_or_percentage_to_number(), Some("val"))
      .flat_map(
        move |v| close.clone().map(move |_| v, Some(")")),
        Some("close"),
      )
      .map(FilterFunction::Invert, Some("to_invert"))
  }

  fn opacity_parser() -> TokenParser<FilterFunction> {
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
    Self::fn_name("opacity")
      .flat_map(|_| Self::number_or_percentage_to_number(), Some("val"))
      .flat_map(
        move |v| close.clone().map(move |_| v, Some(")")),
        Some("close"),
      )
      .map(FilterFunction::Opacity, Some("to_opacity"))
  }

  fn saturate_parser() -> TokenParser<FilterFunction> {
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
    Self::fn_name("saturate")
      .flat_map(|_| Self::number_or_percentage_to_number(), Some("val"))
      .flat_map(
        move |v| close.clone().map(move |_| v, Some(")")),
        Some("close"),
      )
      .map(FilterFunction::Saturate, Some("to_saturate"))
  }

  fn sepia_parser() -> TokenParser<FilterFunction> {
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
    Self::fn_name("sepia")
      .flat_map(|_| Self::number_or_percentage_to_number(), Some("val"))
      .flat_map(
        move |v| close.clone().map(move |_| v, Some(")")),
        Some("close"),
      )
      .map(FilterFunction::Sepia, Some("to_sepia"))
  }
}
