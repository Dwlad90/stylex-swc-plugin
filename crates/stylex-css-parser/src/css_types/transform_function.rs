/*!
CSS transform function parser.
*/

use crate::{
  css_types::{
    angle::Angle,
    common_types::{number_or_percentage_parser, NumberOrPercentage},
    length::Length,
    length_percentage::LengthPercentage,
    length_percentage_parser,
  },
  token_parser::TokenParser,
  token_types::SimpleToken,
  CssParseError,
};
use std::fmt::{self, Display};

/// A CSS transform function
#[derive(Debug, Clone, PartialEq)]
pub enum TransformFunction {
  Matrix(Matrix),
  Matrix3d(Matrix3d),
  Perspective(Perspective),
  Rotate(Rotate),
  RotateXYZ(RotateXYZ),
  Rotate3d(Rotate3d),
  Scale(Scale),
  Scale3d(Scale3d),
  ScaleAxis(ScaleAxis),
  Skew(Skew),
  SkewAxis(SkewAxis),
  Translate3d(Translate3d),
  Translate(Translate),
  TranslateAxis(TranslateAxis),
}

/// Matrix transform function
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
  pub a: f64,
  pub b: f64,
  pub c: f64,
  pub d: f64,
  pub tx: f64,
  pub ty: f64,
}

/// Matrix3d transform function
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix3d {
  pub args: [f64; 16],
}

/// Perspective transform function
#[derive(Debug, Clone, PartialEq)]
pub struct Perspective {
  pub length: Length,
}

/// Rotate transform function
#[derive(Debug, Clone, PartialEq)]
pub struct Rotate {
  pub angle: Angle,
}

/// RotateXYZ transform function (rotateX, rotateY, rotateZ)
#[derive(Debug, Clone, PartialEq)]
pub struct RotateXYZ {
  pub angle: Angle,
  pub axis: Axis,
}

/// Rotate3d transform function
#[derive(Debug, Clone, PartialEq)]
pub struct Rotate3d {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub angle: Angle,
}

/// Scale transform function
#[derive(Debug, Clone, PartialEq)]
pub struct Scale {
  pub sx: f64,
  pub sy: Option<f64>,
}

/// Scale3d transform function
#[derive(Debug, Clone, PartialEq)]
pub struct Scale3d {
  pub sx: f64,
  pub sy: f64,
  pub sz: f64,
}

/// ScaleAxis transform function (scaleX, scaleY, scaleZ)
#[derive(Debug, Clone, PartialEq)]
pub struct ScaleAxis {
  pub s: f64,
  pub axis: Axis,
}

/// Skew transform function
#[derive(Debug, Clone, PartialEq)]
pub struct Skew {
  pub ax: Angle,
  pub ay: Option<Angle>,
}

/// SkewAxis transform function (skewX, skewY)
#[derive(Debug, Clone, PartialEq)]
pub struct SkewAxis {
  pub a: Angle,
  pub axis: SkewAxis2D,
}

/// Translate transform function
#[derive(Debug, Clone, PartialEq)]
pub struct Translate {
  pub tx: LengthPercentage,
  pub ty: Option<LengthPercentage>,
}

/// Translate3d transform function
#[derive(Debug, Clone, PartialEq)]
pub struct Translate3d {
  pub tx: LengthPercentage,
  pub ty: LengthPercentage,
  pub tz: Length,
}

/// TranslateAxis transform function (translateX, translateY, translateZ)
#[derive(Debug, Clone, PartialEq)]
pub struct TranslateAxis {
  pub t: LengthPercentage,
  pub axis: Axis,
}

/// 3D axis enum
#[derive(Debug, Clone, PartialEq)]
pub enum Axis {
  X,
  Y,
  Z,
}

/// 2D axis enum for skew functions
#[derive(Debug, Clone, PartialEq)]
pub enum SkewAxis2D {
  X,
  Y,
}

// Helper to convert NumberOrPercentage to f64 (percentage becomes 0-1 range)
fn number_or_percentage_to_f64(n: NumberOrPercentage) -> f64 {
  match n {
    NumberOrPercentage::Number(n) => n.value.into(),
    NumberOrPercentage::Percentage(p) => (p.value / 100.0).into(),
  }
}

// Helper function to create a number parser
fn number_parser() -> TokenParser<f64> {
  TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number")).map(
    |t| {
      if let SimpleToken::Number(v) = t {
        v
      } else {
        0.0
      }
    },
    Some("to_f64"),
  )
}

impl Matrix {
  pub fn new(a: f64, b: f64, c: f64, d: f64, tx: f64, ty: f64) -> Self {
    Self { a, b, c, d, tx, ty }
  }

  pub fn parse() -> TokenParser<Matrix> {
    TokenParser::new(
      |tokens| {
        // Parse: matrix(a, b, c, d, tx, ty)

        // Expect Function("matrix")
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected matrix function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "matrix" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'matrix' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse 6 numbers separated by commas
        let mut numbers = Vec::new();
        for i in 0..6 {
          if i > 0 {
            // Skip optional whitespace before comma
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }

            // Expect comma
            let comma_token = tokens
              .consume_next_token()?
              .ok_or(CssParseError::ParseError {
                message: "Expected comma".to_string(),
              })?;
            if !matches!(comma_token, SimpleToken::Comma) {
              return Err(CssParseError::ParseError {
                message: format!("Expected comma, got {:?}", comma_token),
              });
            }
          }

          // Skip optional whitespace before each number (including first one after opening paren)
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }

          // Parse number
          let num_token = tokens
            .consume_next_token()?
            .ok_or(CssParseError::ParseError {
              message: "Expected number".to_string(),
            })?;

          if let SimpleToken::Number(value) = num_token {
            numbers.push(value);
          } else {
            return Err(CssParseError::ParseError {
              message: format!("Expected number, got {:?}", num_token),
            });
          }
        }

        // Skip whitespace before closing paren
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect closing paren
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected ')', got {:?}", close_token),
          });
        }

        Ok(Matrix::new(
          numbers[0], // a
          numbers[1], // b
          numbers[2], // c
          numbers[3], // d
          numbers[4], // tx
          numbers[5], // ty
        ))
      },
      "matrix_parser",
    )
  }
}

impl Matrix3d {
  pub fn new(args: [f64; 16]) -> Self {
    Self { args }
  }

  /// Parse matrix3d function with 16 comma-separated numbers
  pub fn parse() -> TokenParser<Matrix3d> {
    use crate::token_types::SimpleToken;

    TokenParser::new(
      |tokens| {
        // Expect "matrix3d" function
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(name)) if name == "matrix3d" => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected 'matrix3d' function".to_string(),
            })
          }
        }

        // Parse 16 comma-separated numbers with whitespace handling
        let mut values = [0.0; 16];
        for (i, value) in values.iter_mut().enumerate() {
          // Skip any whitespace before number
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }

          // Parse number
          match tokens.consume_next_token()? {
            Some(SimpleToken::Number(v)) => {
              *value = v;
            }
            _ => {
              return Err(CssParseError::ParseError {
                message: format!("Expected number at position {}", i + 1),
              })
            }
          }

          // Expect comma (except for the last value)
          if i < 15 {
            // Skip any whitespace before comma
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }

            match tokens.consume_next_token()? {
              Some(SimpleToken::Comma) => {}
              _ => {
                return Err(CssParseError::ParseError {
                  message: format!("Expected comma after value {}", i + 1),
                })
              }
            }
          }
        }

        // Skip any whitespace before closing paren
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect closing parenthesis
        match tokens.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected ')' after matrix3d values".to_string(),
            })
          }
        }

        Ok(Matrix3d::new(values))
      },
      "matrix3d",
    )
  }
}

impl Perspective {
  pub fn new(length: Length) -> Self {
    Self { length }
  }

  pub fn parse() -> TokenParser<Perspective> {
    let fn_name = TokenParser::<String>::fn_name("perspective");
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    fn_name
      .flat_map(move |_| Length::parser(), Some("length"))
      .flat_map(
        move |length| {
          close.clone().map(
            move |_| Perspective::new(length.clone()),
            Some("to_perspective"),
          )
        },
        Some("close"),
      )
  }
}

impl Rotate {
  pub fn new(angle: Angle) -> Self {
    Self { angle }
  }

  pub fn parse() -> TokenParser<Rotate> {
    let fn_name = TokenParser::<String>::fn_name("rotate");
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    fn_name
      .flat_map(move |_| Angle::parser(), Some("angle"))
      .flat_map(
        move |angle| {
          close
            .clone()
            .map(move |_| Rotate::new(angle.clone()), Some("to_rotate"))
        },
        Some("close"),
      )
  }
}

impl RotateXYZ {
  pub fn new(angle: Angle, axis: Axis) -> Self {
    Self { angle, axis }
  }

  pub fn parse() -> TokenParser<RotateXYZ> {
    let rotate_x = TokenParser::<String>::fn_name("rotateX").map(|_| Axis::X, Some("x_axis"));
    let rotate_y = TokenParser::<String>::fn_name("rotateY").map(|_| Axis::Y, Some("y_axis"));
    let rotate_z = TokenParser::<String>::fn_name("rotateZ").map(|_| Axis::Z, Some("z_axis"));

    let axis_parser = TokenParser::one_of(vec![rotate_x, rotate_y, rotate_z]);
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    axis_parser
      .flat_map(
        move |axis| {
          let axis_clone = axis.clone();
          Angle::parser().map(move |angle| (axis_clone.clone(), angle), Some("with_angle"))
        },
        Some("angle"),
      )
      .flat_map(
        move |(axis, angle)| {
          close.clone().map(
            move |_| RotateXYZ::new(angle.clone(), axis.clone()),
            Some("to_rotatexyz"),
          )
        },
        Some("close"),
      )
  }
}

impl Rotate3d {
  pub fn new(x: f64, y: f64, z: f64, angle: Angle) -> Self {
    Self { x, y, z, angle }
  }

  /// Parse rotate3d function with 3 numbers and 1 angle, comma-separated
  /// Example: rotate3d(1, 0, 0, 45deg)
  pub fn parse() -> TokenParser<Rotate3d> {
    use crate::token_types::SimpleToken;

    TokenParser::new(
      |tokens| {
        // Expect "rotate3d" function
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(name)) if name == "rotate3d" => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected 'rotate3d' function".to_string(),
            })
          }
        }

        // Skip any whitespace before x
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse x (first number)
        let x = (number_parser().run)(tokens)?;

        // Skip any whitespace before comma
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect comma
        match tokens.consume_next_token()? {
          Some(SimpleToken::Comma) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected comma after x value".to_string(),
            })
          }
        }

        // Skip any whitespace before y
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse y (second number)
        let y = (number_parser().run)(tokens)?;

        // Skip any whitespace before comma
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect comma
        match tokens.consume_next_token()? {
          Some(SimpleToken::Comma) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected comma after y value".to_string(),
            })
          }
        }

        // Skip any whitespace before z
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse z (third number)
        let z = (number_parser().run)(tokens)?;

        // Skip any whitespace before comma
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect comma
        match tokens.consume_next_token()? {
          Some(SimpleToken::Comma) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected comma after z value".to_string(),
            })
          }
        }

        // Skip any whitespace before angle
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse angle (fourth parameter)
        let angle = (Angle::parser().run)(tokens)?;

        // Skip any whitespace before closing paren
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect closing parenthesis
        match tokens.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected ')' after rotate3d values".to_string(),
            })
          }
        }

        Ok(Rotate3d::new(x, y, z, angle))
      },
      "rotate3d",
    )
  }
}

impl Scale {
  pub fn new(sx: f64, sy: Option<f64>) -> Self {
    Self { sx, sy }
  }

  pub fn parse() -> TokenParser<Scale> {
    TokenParser::new(
      |tokens| {
        // Parse: scale(sx) or scale(sx, sy)

        // Expect Function("scale")
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected scale function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "scale" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'scale' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse first number/percentage (sx)
        let sx = (number_or_percentage_parser().run)(tokens)?;

        // Try to parse optional comma + second number/percentage
        let sy = {
          // Skip whitespace
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }

          // Check if we have a comma
          if let Ok(Some(SimpleToken::Comma)) = tokens.peek() {
            // Consume comma
            tokens.consume_next_token()?;

            // Skip whitespace after comma
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }

            // Parse second number/percentage
            Some((number_or_percentage_parser().run)(tokens)?)
          } else {
            None
          }
        };

        // Skip whitespace before closing paren
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect closing parenthesis
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected closing parenthesis, got {:?}", close_token),
          });
        }

        let sx_f64 = number_or_percentage_to_f64(sx);
        let sy_f64 = sy.map(number_or_percentage_to_f64);

        Ok(Scale::new(sx_f64, sy_f64))
      },
      "scale_parser",
    )
  }
}

impl Scale3d {
  pub fn new(sx: f64, sy: f64, sz: f64) -> Self {
    Self { sx, sy, sz }
  }

  /// Parse scale3d function with 3 comma-separated numbers/percentages
  /// Example: scale3d(1.5, 2.0, 0.5)
  pub fn parse() -> TokenParser<Scale3d> {
    use crate::token_types::SimpleToken;

    TokenParser::new(
      |tokens| {
        // Expect "scale3d" function
        match tokens.consume_next_token()? {
          Some(SimpleToken::Function(name)) if name == "scale3d" => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected 'scale3d' function".to_string(),
            })
          }
        }

        // Skip any whitespace before sx
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse sx (first scale value)
        let sx = (number_or_percentage_parser().run)(tokens)?;

        // Skip any whitespace before comma
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect comma
        match tokens.consume_next_token()? {
          Some(SimpleToken::Comma) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected comma after sx value".to_string(),
            })
          }
        }

        // Skip any whitespace before sy
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse sy (second scale value)
        let sy = (number_or_percentage_parser().run)(tokens)?;

        // Skip any whitespace before comma
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect comma
        match tokens.consume_next_token()? {
          Some(SimpleToken::Comma) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected comma after sy value".to_string(),
            })
          }
        }

        // Skip any whitespace before sz
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Parse sz (third scale value)
        let sz = (number_or_percentage_parser().run)(tokens)?;

        // Skip any whitespace before closing paren
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect closing parenthesis
        match tokens.consume_next_token()? {
          Some(SimpleToken::RightParen) => {}
          _ => {
            return Err(CssParseError::ParseError {
              message: "Expected ')' after scale3d values".to_string(),
            })
          }
        }

        // Convert to f64 values
        let sx_f64 = number_or_percentage_to_f64(sx);
        let sy_f64 = number_or_percentage_to_f64(sy);
        let sz_f64 = number_or_percentage_to_f64(sz);

        Ok(Scale3d::new(sx_f64, sy_f64, sz_f64))
      },
      "scale3d",
    )
  }
}

impl ScaleAxis {
  pub fn new(s: f64, axis: Axis) -> Self {
    Self { s, axis }
  }

  pub fn parse() -> TokenParser<ScaleAxis> {
    let scale_x = TokenParser::<String>::fn_name("scaleX").map(|_| Axis::X, Some("x_axis"));
    let scale_y = TokenParser::<String>::fn_name("scaleY").map(|_| Axis::Y, Some("y_axis"));
    let scale_z = TokenParser::<String>::fn_name("scaleZ").map(|_| Axis::Z, Some("z_axis"));

    let axis_parser = TokenParser::one_of(vec![scale_x, scale_y, scale_z]);
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    axis_parser
      .flat_map(
        move |axis| {
          let axis_clone = axis.clone();
          number_or_percentage_parser().map(move |s| (axis_clone.clone(), s), Some("with_scale"))
        },
        Some("scale"),
      )
      .flat_map(
        move |(axis, s)| {
          let s_f64 = number_or_percentage_to_f64(s);
          close.clone().map(
            move |_| ScaleAxis::new(s_f64, axis.clone()),
            Some("to_scaleaxis"),
          )
        },
        Some("close"),
      )
  }
}

impl Skew {
  pub fn new(ax: Angle, ay: Option<Angle>) -> Self {
    Self { ax, ay }
  }

  pub fn parse() -> TokenParser<Skew> {
    let fn_name = TokenParser::<String>::fn_name("skew");
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    let second_angle = Angle::parser().optional();

    fn_name
      .flat_map(move |_| Angle::parser(), Some("ax"))
      .flat_map(
        move |ax| {
          let ax_clone = ax.clone();
          second_angle
            .clone()
            .map(move |ay_opt| (ax_clone.clone(), ay_opt), Some("ay"))
        },
        Some("with_ay"),
      )
      .flat_map(
        move |(ax, ay_opt)| {
          close.clone().map(
            move |_| Skew::new(ax.clone(), ay_opt.clone()),
            Some("to_skew"),
          )
        },
        Some("close"),
      )
  }
}

impl SkewAxis {
  pub fn new(a: Angle, axis: SkewAxis2D) -> Self {
    Self { a, axis }
  }

  pub fn parse() -> TokenParser<SkewAxis> {
    let skew_x = TokenParser::<String>::fn_name("skewX").map(|_| SkewAxis2D::X, Some("x_axis"));
    let skew_y = TokenParser::<String>::fn_name("skewY").map(|_| SkewAxis2D::Y, Some("y_axis"));

    let axis_parser = TokenParser::one_of(vec![skew_x, skew_y]);
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    axis_parser
      .flat_map(
        move |axis| {
          let axis_clone = axis.clone();
          Angle::parser().map(move |a| (axis_clone.clone(), a), Some("with_angle"))
        },
        Some("angle"),
      )
      .flat_map(
        move |(axis, a)| {
          close.clone().map(
            move |_| SkewAxis::new(a.clone(), axis.clone()),
            Some("to_skewaxis"),
          )
        },
        Some("close"),
      )
  }
}

impl Translate {
  pub fn new(tx: LengthPercentage, ty: Option<LengthPercentage>) -> Self {
    Self { tx, ty }
  }

  pub fn parse() -> TokenParser<Translate> {
    TokenParser::new(
      |tokens| {
        // Parse: translate(tx) or translate(tx, ty)

        // Expect Function("translate")
        let function_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected translate function".to_string(),
          })?;

        if let SimpleToken::Function(name) = function_token {
          if name != "translate" {
            return Err(CssParseError::ParseError {
              message: format!("Expected 'translate' function, got '{}'", name),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected Function token, got {:?}", function_token),
          });
        }

        // Parse first length/percentage (tx)
        let tx = (length_percentage_parser().run)(tokens)?;

        // Try to parse optional comma + second length/percentage
        let ty = {
          // Skip whitespace
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            tokens.consume_next_token()?;
          }

          // Check if we have a comma
          if let Ok(Some(SimpleToken::Comma)) = tokens.peek() {
            // Consume comma
            tokens.consume_next_token()?;

            // Skip whitespace after comma
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              tokens.consume_next_token()?;
            }

            // Parse second length/percentage
            Some((length_percentage_parser().run)(tokens)?)
          } else {
            None
          }
        };

        // Skip whitespace before closing paren
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          tokens.consume_next_token()?;
        }

        // Expect closing parenthesis
        let close_token = tokens
          .consume_next_token()?
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;

        if !matches!(close_token, SimpleToken::RightParen) {
          return Err(CssParseError::ParseError {
            message: format!("Expected closing parenthesis, got {:?}", close_token),
          });
        }

        Ok(Translate::new(tx, ty))
      },
      "translate_parser",
    )
  }
}

impl Translate3d {
  pub fn new(tx: LengthPercentage, ty: LengthPercentage, tz: Length) -> Self {
    Self { tx, ty, tz }
  }

  pub fn parse() -> TokenParser<Translate3d> {
    let fn_name = TokenParser::<String>::fn_name("translate3d");
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    fn_name
      .flat_map(|_| length_percentage_parser(), Some("tx"))
      .flat_map(
        |tx| {
          let tx_clone = tx.clone();
          length_percentage_parser().map(move |ty| (tx_clone.clone(), ty), Some("ty"))
        },
        Some("with_ty"),
      )
      .flat_map(
        |(tx, ty)| {
          let tx_clone = tx.clone();
          let ty_clone = ty.clone();
          Length::parser().map(
            move |tz| (tx_clone.clone(), ty_clone.clone(), tz),
            Some("tz"),
          )
        },
        Some("with_tz"),
      )
      .flat_map(
        move |(tx, ty, tz)| {
          close.map(
            move |_| Translate3d::new(tx.clone(), ty.clone(), tz.clone()),
            Some("to_translate3d"),
          )
        },
        Some("close"),
      )
  }
}

impl TranslateAxis {
  pub fn new(t: LengthPercentage, axis: Axis) -> Self {
    Self { t, axis }
  }

  pub fn parse() -> TokenParser<TranslateAxis> {
    let translate_x = TokenParser::<String>::fn_name("translateX").map(|_| Axis::X, Some("x_axis"));
    let translate_y = TokenParser::<String>::fn_name("translateY").map(|_| Axis::Y, Some("y_axis"));
    let translate_z = TokenParser::<String>::fn_name("translateZ").map(|_| Axis::Z, Some("z_axis"));

    let axis_parser = TokenParser::one_of(vec![translate_x, translate_y, translate_z]);
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    axis_parser
      .flat_map(
        move |axis| {
          let axis_clone = axis.clone();
          length_percentage_parser().map(move |t| (axis_clone.clone(), t), Some("with_translate"))
        },
        Some("translate"),
      )
      .flat_map(
        move |(axis, t)| {
          close.clone().map(
            move |_| TranslateAxis::new(t.clone(), axis.clone()),
            Some("to_translateaxis"),
          )
        },
        Some("close"),
      )
  }
}

impl TransformFunction {
  /// Transform function parser
  pub fn parse() -> TokenParser<TransformFunction> {
    TokenParser::one_of(vec![
      Matrix::parse().map(TransformFunction::Matrix, Some("matrix")),
      Matrix3d::parse().map(TransformFunction::Matrix3d, Some("matrix3d")),
      Perspective::parse().map(TransformFunction::Perspective, Some("perspective")),
      Rotate::parse().map(TransformFunction::Rotate, Some("rotate")),
      RotateXYZ::parse().map(TransformFunction::RotateXYZ, Some("rotatexyz")),
      Rotate3d::parse().map(TransformFunction::Rotate3d, Some("rotate3d")),
      Scale::parse().map(TransformFunction::Scale, Some("scale")),
      Scale3d::parse().map(TransformFunction::Scale3d, Some("scale3d")),
      ScaleAxis::parse().map(TransformFunction::ScaleAxis, Some("scaleaxis")),
      Skew::parse().map(TransformFunction::Skew, Some("skew")),
      SkewAxis::parse().map(TransformFunction::SkewAxis, Some("skewaxis")),
      Translate::parse().map(TransformFunction::Translate, Some("translate")),
      Translate3d::parse().map(TransformFunction::Translate3d, Some("translate3d")),
      TranslateAxis::parse().map(TransformFunction::TranslateAxis, Some("translateaxis")),
    ])
  }
}

fn format_number(n: f64) -> String {
  let rounded = (n * 1_000_000.0).round() / 1_000_000.0;
  if rounded.fract() == 0.0 {
    format!("{}", rounded as i64)
  } else {
    let s = format!("{:.6}", rounded);
    s.trim_end_matches('0').trim_end_matches('.').to_string()
  }
}

impl Display for TransformFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TransformFunction::Matrix(m) => write!(
        f,
        "matrix({}, {}, {}, {}, {}, {})",
        format_number(m.a),
        format_number(m.b),
        format_number(m.c),
        format_number(m.d),
        format_number(m.tx),
        format_number(m.ty)
      ),
      TransformFunction::Matrix3d(m) => {
        let args: Vec<String> = m.args.iter().map(|x| format_number(*x)).collect();
        write!(f, "matrix3d({})", args.join(", "))
      }
      TransformFunction::Perspective(p) => write!(f, "perspective({})", p.length),
      TransformFunction::Rotate(r) => write!(f, "rotate({})", r.angle),
      TransformFunction::RotateXYZ(r) => write!(
        f,
        "rotate{}({})",
        match r.axis {
          Axis::X => "X",
          Axis::Y => "Y",
          Axis::Z => "Z",
        },
        r.angle
      ),
      TransformFunction::Rotate3d(r) => match (r.x, r.y, r.z) {
        (x, y, z) if x == 1.0 && y == 0.0 && z == 0.0 => write!(f, "rotateX({})", r.angle),
        (x, y, z) if x == 0.0 && y == 1.0 && z == 0.0 => write!(f, "rotateY({})", r.angle),
        (x, y, z) if x == 0.0 && y == 0.0 && z == 1.0 => write!(f, "rotateZ({})", r.angle),
        _ => write!(
          f,
          "rotate3d({}, {}, {}, {})",
          format_number(r.x),
          format_number(r.y),
          format_number(r.z),
          r.angle
        ),
      },
      TransformFunction::Scale(s) => match &s.sy {
        Some(sy) => write!(f, "scale({}, {})", format_number(s.sx), format_number(*sy)),
        None => write!(f, "scale({})", format_number(s.sx)),
      },
      TransformFunction::Scale3d(s) => write!(
        f,
        "scale3d({}, {}, {})",
        format_number(s.sx),
        format_number(s.sy),
        format_number(s.sz)
      ),
      TransformFunction::ScaleAxis(s) => write!(
        f,
        "scale{}({})",
        match s.axis {
          Axis::X => "X",
          Axis::Y => "Y",
          Axis::Z => "Z",
        },
        format_number(s.s)
      ),
      TransformFunction::Skew(s) => match &s.ay {
        Some(ay) => write!(f, "skew({}, {})", s.ax, ay),
        None => write!(f, "skew({})", s.ax),
      },
      TransformFunction::SkewAxis(s) => write!(
        f,
        "skew{}({})",
        match s.axis {
          SkewAxis2D::X => "X",
          SkewAxis2D::Y => "Y",
        },
        s.a
      ),
      TransformFunction::Translate(t) => match &t.ty {
        Some(ty) => write!(f, "translate({}, {})", t.tx, ty),
        None => write!(f, "translate({})", t.tx),
      },
      TransformFunction::Translate3d(t) => write!(f, "translate3d({}, {}, {})", t.tx, t.ty, t.tz),
      TransformFunction::TranslateAxis(t) => write!(
        f,
        "translate{}({})",
        match t.axis {
          Axis::X => "X",
          Axis::Y => "Y",
          Axis::Z => "Z",
        },
        t.t
      ),
    }
  }
}
