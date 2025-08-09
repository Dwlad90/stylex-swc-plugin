use std::str::FromStr;

use anyhow::bail;

use crate::parser::Parser;
#[derive(Clone, Debug, PartialEq)]
pub enum BlendMode {
  Normal,
  Multiply,
  Screen,
  Overlay,
  Darken,
  Lighten,
  ColorDodge,
  ColorBurn,
  HardLight,
  SoftLight,
  Difference,
  Exclusion,
  Hue,
  Saturation,
  Color,
  Luminosity,
}

impl BlendMode {
  // Helper method to convert BlendMode to string representation
  fn as_str(&self) -> &'static str {
    match self {
      BlendMode::Normal => "normal",
      BlendMode::Multiply => "multiply",
      BlendMode::Screen => "screen",
      BlendMode::Overlay => "overlay",
      BlendMode::Darken => "darken",
      BlendMode::Lighten => "lighten",
      BlendMode::ColorDodge => "color-dodge",
      BlendMode::ColorBurn => "color-burn",
      BlendMode::HardLight => "hard-light",
      BlendMode::SoftLight => "soft-light",
      BlendMode::Difference => "difference",
      BlendMode::Exclusion => "exclusion",
      BlendMode::Hue => "hue",
      BlendMode::Saturation => "saturation",
      BlendMode::Color => "color",
      BlendMode::Luminosity => "luminosity",
    }
  }
}

impl std::fmt::Display for BlendMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl From<BlendMode> for String {
  fn from(blend_mode: BlendMode) -> Self {
    blend_mode.as_str().to_string()
  }
}

impl FromStr for BlendMode {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "normal" => Ok(BlendMode::Normal),
      "multiply" => Ok(BlendMode::Multiply),
      "screen" => Ok(BlendMode::Screen),
      "overlay" => Ok(BlendMode::Overlay),
      "darken" => Ok(BlendMode::Darken),
      "lighten" => Ok(BlendMode::Lighten),
      "color-dodge" => Ok(BlendMode::ColorDodge),
      "color-burn" => Ok(BlendMode::ColorBurn),
      "hard-light" => Ok(BlendMode::HardLight),
      "soft-light" => Ok(BlendMode::SoftLight),
      "difference" => Ok(BlendMode::Difference),
      "exclusion" => Ok(BlendMode::Exclusion),
      "hue" => Ok(BlendMode::Hue),
      "saturation" => Ok(BlendMode::Saturation),
      "color" => Ok(BlendMode::Color),
      "luminosity" => Ok(BlendMode::Luminosity),
      _ => bail!("Invalid blend mode: {}", s),
    }
  }
}

impl From<String> for BlendMode {
  fn from(s: String) -> Self {
    s.parse()
      .unwrap_or_else(|_| panic!("Invalid blend mode: {}", s))
  }
}

impl BlendMode {
  pub fn parse<'a>() -> Parser<'a, BlendMode> {
    Parser::one_of(vec![
      Parser::<'a, BlendMode>::string("normal").map(|_| BlendMode::Normal),
      Parser::<'a, BlendMode>::string("multiply").map(|_| BlendMode::Multiply),
      Parser::<'a, BlendMode>::string("screen").map(|_| BlendMode::Screen),
      Parser::<'a, BlendMode>::string("overlay").map(|_| BlendMode::Overlay),
      Parser::<'a, BlendMode>::string("darken").map(|_| BlendMode::Darken),
      Parser::<'a, BlendMode>::string("lighten").map(|_| BlendMode::Lighten),
      Parser::<'a, BlendMode>::string("color-dodge").map(|_| BlendMode::ColorDodge),
      Parser::<'a, BlendMode>::string("color-burn").map(|_| BlendMode::ColorBurn),
      Parser::<'a, BlendMode>::string("hard-light").map(|_| BlendMode::HardLight),
      Parser::<'a, BlendMode>::string("soft-light").map(|_| BlendMode::SoftLight),
      Parser::<'a, BlendMode>::string("difference").map(|_| BlendMode::Difference),
      Parser::<'a, BlendMode>::string("exclusion").map(|_| BlendMode::Exclusion),
      Parser::<'a, BlendMode>::string("hue").map(|_| BlendMode::Hue),
      Parser::<'a, BlendMode>::string("saturation").map(|_| BlendMode::Saturation),
      Parser::<'a, BlendMode>::string("color").map(|_| BlendMode::Color),
      Parser::<'a, BlendMode>::string("luminosity").map(|_| BlendMode::Luminosity),
    ])
  }
}

// pub fn blend_mode<'a>() -> Parser<'a, String> {
//   Parser::one_of(vec![
//     Parser::<'a, String>::string("normal"),
//     Parser::<'a, String>::string("multiply"),
//     Parser::<'a, String>::string("screen"),
//     Parser::<'a, String>::string("overlay"),
//     Parser::<'a, String>::string("darken"),
//     Parser::<'a, String>::string("lighten"),
//     Parser::<'a, String>::string("color-dodge"),
//     Parser::<'a, String>::string("color-burn"),
//     Parser::<'a, String>::string("hard-light"),
//     Parser::<'a, String>::string("soft-light"),
//     Parser::<'a, String>::string("difference"),
//     Parser::<'a, String>::string("exclusion"),
//     Parser::<'a, String>::string("hue"),
//     Parser::<'a, String>::string("saturation"),
//     Parser::<'a, String>::string("color"),
//     Parser::<'a, String>::string("luminosity"),
//   ])
// }
