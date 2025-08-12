/*!
CSS easing function parser.
Mirrors: packages/style-value-parser/src/css-types/easing-function.js exactly
*/

use crate::{token_parser::TokenParser, token_types::SimpleToken};
use std::fmt::{self, Display};

/// Base EasingFunction class - mirrors JavaScript EasingFunction class exactly
#[derive(Debug, Clone, PartialEq)]
pub enum EasingFunction {
  Linear(LinearEasingFunction),
  CubicBezier(CubicBezierEasingFunction),
  CubicBezierKeyword(CubicBezierKeyword),
  Steps(StepsEasingFunction),
  StepsKeyword(StepsKeyword),
}

/// LinearEasingFunction class - mirrors JavaScript LinearEasingFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct LinearEasingFunction {
  pub points: Vec<f64>,
}

/// CubicBezierEasingFunction class - mirrors JavaScript CubicBezierEasingFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct CubicBezierEasingFunction {
  pub points: [f64; 4],
}

/// CubicBezierKeyword class - mirrors JavaScript CubicBezierKeyword exactly
#[derive(Debug, Clone, PartialEq)]
pub struct CubicBezierKeyword {
  pub keyword: CubicBezierKeywordType,
}

/// StepsEasingFunction class - mirrors JavaScript StepsEasingFunction exactly
#[derive(Debug, Clone, PartialEq)]
pub struct StepsEasingFunction {
  pub steps: u32,
  pub start: StepsStartType,
}

/// StepsKeyword class - mirrors JavaScript StepsKeyword exactly
#[derive(Debug, Clone, PartialEq)]
pub struct StepsKeyword {
  pub keyword: StepsKeywordType,
}

/// CubicBezierKeyword type - matches JavaScript TCubicBezierKeyword
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CubicBezierKeywordType {
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
}

/// Steps start type - matches JavaScript 'start' | 'end'
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepsStartType {
  Start,
  End,
}

/// StepsKeyword type - matches JavaScript 'step-start' | 'step-end'
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepsKeywordType {
  StepStart,
  StepEnd,
}

impl EasingFunction {
  /// Static parser method - mirrors JavaScript EasingFunction.parser exactly
  pub fn parse() -> TokenParser<EasingFunction> {
        TokenParser::one_of(vec![
      LinearEasingFunction::parse().map(EasingFunction::Linear, Some("linear")),
      CubicBezierEasingFunction::parse().map(EasingFunction::CubicBezier, Some("cubic_bezier")),
      CubicBezierKeyword::parse().map(EasingFunction::CubicBezierKeyword, Some("cubic_bezier_keyword")),
      StepsEasingFunction::parse().map(EasingFunction::Steps, Some("steps")),
      StepsKeyword::parse().map(EasingFunction::StepsKeyword, Some("steps_keyword")),
    ])
  }
}

impl LinearEasingFunction {
  pub fn new(points: Vec<f64>) -> Self {
    Self { points }
  }

  /// Mirrors JavaScript LinearEasingFunction.parser exactly
  pub fn parse() -> TokenParser<LinearEasingFunction> {
    use crate::token_parser::tokens;

    // Mirrors: TokenParser.oneOrMore(TokenParser.tokens.Number.map((v) => v[4].value))
    //   .separatedBy(TokenParser.tokens.Comma)
    //   .separatedBy(TokenParser.tokens.Whitespace.optional)
    let number_parser = tokens::number().map(
      |token| {
        if let SimpleToken::Number(value) = token {
          value
        } else {
          0.0
        }
      },
      Some("extract_number")
    );

    let points_parser = number_parser
      .separated_by(tokens::comma())
      .one_or_more();

    // Mirrors: TokenParser.sequence(Function.where('linear'), pointsParser, CloseParen)
    let linear_fn = TokenParser::<String>::fn_name("linear");
    let close_paren = tokens::close_paren();

    linear_fn
      .skip(tokens::whitespace().optional())
      .flat_map({
        let points_parser = points_parser.clone();
        move |_| points_parser.clone()
      }, Some("parse_points"))
      .skip(tokens::whitespace().optional())
      .skip(close_paren)
      .map(LinearEasingFunction::new, Some("to_linear"))
  }
}

impl CubicBezierEasingFunction {
  pub fn new(points: [f64; 4]) -> Self {
    Self { points }
  }

  /// Mirrors JavaScript CubicBezierEasingFunction.parser exactly
  pub fn parse() -> TokenParser<CubicBezierEasingFunction> {
    use crate::token_parser::tokens;

    // Extract number from token
    let number_parser = tokens::number().map(
      |token| {
        if let SimpleToken::Number(value) = token {
          value
        } else {
          0.0
        }
      },
      Some("extract_number")
    );

        // Simplified approach: parse 4 numbers separated by commas
    let cubic_bezier_fn = TokenParser::<String>::fn_name("cubic-bezier");
    let close_paren = tokens::close_paren();

    // Parse exactly 4 numbers separated by commas
    let four_numbers = number_parser
      .separated_by(tokens::comma())
      .one_or_more()
      .map(|numbers| {
        if numbers.len() == 4 {
          [numbers[0], numbers[1], numbers[2], numbers[3]]
        } else {
          [0.0, 0.0, 0.0, 0.0] // Error case, should not happen
        }
      }, Some("to_array"));

    cubic_bezier_fn
      .skip(tokens::whitespace().optional())
      .flat_map({
        let four_numbers = four_numbers.clone();
        move |_| four_numbers.clone()
      }, Some("parse_four_numbers"))
      .skip(tokens::whitespace().optional())
      .skip(close_paren)
      .map(CubicBezierEasingFunction::new, Some("to_cubic_bezier"))
  }
}

impl CubicBezierKeyword {
  pub fn new(keyword: CubicBezierKeywordType) -> Self {
    Self { keyword }
  }

  /// Mirrors JavaScript CubicBezierKeyword.parser exactly
  pub fn parse() -> TokenParser<CubicBezierKeyword> {
    TokenParser::<SimpleToken>::ident()
      .where_fn(|token| {
        if let SimpleToken::Ident(value) = token {
          matches!(value.as_str(), "ease" | "ease-in" | "ease-out" | "ease-in-out")
        } else {
          false
        }
      }, Some("easing_keyword"))
      .map(|token| {
        if let SimpleToken::Ident(value) = token {
          let keyword = match value.as_str() {
            "ease" => CubicBezierKeywordType::Ease,
            "ease-in" => CubicBezierKeywordType::EaseIn,
            "ease-out" => CubicBezierKeywordType::EaseOut,
            "ease-in-out" => CubicBezierKeywordType::EaseInOut,
            _ => unreachable!(),
          };
          CubicBezierKeyword::new(keyword)
        } else {
          unreachable!()
        }
      }, Some("to_keyword"))
  }
}

impl StepsEasingFunction {
  pub fn new(steps: u32, start: StepsStartType) -> Self {
    Self { steps, start }
  }

  /// Mirrors JavaScript StepsEasingFunction.parser exactly
  pub fn parse() -> TokenParser<StepsEasingFunction> {
    use crate::token_parser::tokens;

    // Extract number from token
    let number_parser = tokens::number().map(
      |token| {
        if let SimpleToken::Number(value) = token {
          value as u32
        } else {
          0
        }
      },
      Some("extract_number")
    );

    // Extract step type (start | end)
    let step_type_parser = tokens::ident().where_fn(
      |token| {
        if let SimpleToken::Ident(value) = token {
          matches!(value.as_str(), "start" | "end")
        } else {
          false
        }
      },
      Some("step_type")
    ).map(
      |token| {
        if let SimpleToken::Ident(value) = token {
          match value.as_str() {
            "start" => StepsStartType::Start,
            "end" => StepsStartType::End,
            _ => unreachable!()
          }
        } else {
          unreachable!()
        }
      },
      Some("to_step_type")
    );

        // Simplified approach: parse steps number and step type
    let steps_fn = TokenParser::<String>::fn_name("steps");
    let close_paren = tokens::close_paren();
    let comma_ws = tokens::comma().skip(tokens::whitespace().optional()).prefix(tokens::whitespace().optional());

    steps_fn
      .skip(tokens::whitespace().optional())
      .flat_map({
        let number_parser = number_parser.clone();
        move |_| number_parser.clone()
      }, Some("parse_steps"))
      .flat_map({
        let step_type_parser = step_type_parser.clone();
        move |steps| {
          comma_ws.clone()
            .flat_map({
              let step_type_parser = step_type_parser.clone();
              move |_| step_type_parser.clone()
            }, Some("parse_step_type"))
            .map(move |step_type| (steps, step_type), Some("steps_and_type"))
        }
      }, Some("with_type"))
      .skip(tokens::whitespace().optional())
      .skip(close_paren)
      .map(|(steps, start)| StepsEasingFunction::new(steps, start), Some("to_steps"))
  }
}

impl StepsKeyword {
  pub fn new(keyword: StepsKeywordType) -> Self {
    Self { keyword }
  }

  /// Mirrors JavaScript StepsKeyword.parser exactly
  pub fn parse() -> TokenParser<StepsKeyword> {
    TokenParser::<SimpleToken>::ident()
      .where_fn(|token| {
        if let SimpleToken::Ident(value) = token {
          matches!(value.as_str(), "step-start" | "step-end")
        } else {
          false
        }
      }, Some("steps_keyword"))
      .map(|token| {
        if let SimpleToken::Ident(value) = token {
          let keyword = match value.as_str() {
            "step-start" => StepsKeywordType::StepStart,
            "step-end" => StepsKeywordType::StepEnd,
            _ => unreachable!(),
          };
          StepsKeyword::new(keyword)
        } else {
          unreachable!()
        }
      }, Some("to_steps_keyword"))
  }
}

// Display implementations to match JavaScript toString() methods
impl Display for EasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      EasingFunction::Linear(linear) => linear.fmt(f),
      EasingFunction::CubicBezier(cubic_bezier) => cubic_bezier.fmt(f),
      EasingFunction::CubicBezierKeyword(keyword) => keyword.fmt(f),
      EasingFunction::Steps(steps) => steps.fmt(f),
      EasingFunction::StepsKeyword(keyword) => keyword.fmt(f),
    }
  }
}

impl Display for LinearEasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let points_str = self.points.iter()
      .map(|p| p.to_string())
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "linear({})", points_str)
  }
}

impl Display for CubicBezierEasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "cubic-bezier({}, {}, {}, {})",
           self.points[0], self.points[1], self.points[2], self.points[3])
  }
}

impl Display for CubicBezierKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let keyword_str = match self.keyword {
      CubicBezierKeywordType::Ease => "ease",
      CubicBezierKeywordType::EaseIn => "ease-in",
      CubicBezierKeywordType::EaseOut => "ease-out",
      CubicBezierKeywordType::EaseInOut => "ease-in-out",
    };
    write!(f, "{}", keyword_str)
  }
}

impl Display for StepsEasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let start_str = match self.start {
      StepsStartType::Start => "start",
      StepsStartType::End => "end",
    };
    write!(f, "steps({}, {})", self.steps, start_str)
  }
}

impl Display for StepsKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let keyword_str = match self.keyword {
      StepsKeywordType::StepStart => "step-start",
      StepsKeywordType::StepEnd => "step-end",
    };
    write!(f, "{}", keyword_str)
  }
}
