use crate::parser::Parser;
use std::fmt;

pub trait EasingFunction: fmt::Debug + fmt::Display {
  fn clone_box(&self) -> Box<dyn EasingFunction>;
}

// Add a blanket implementation for all types that implement EasingFunction + Clone
impl<T: 'static + Clone + fmt::Debug + fmt::Display> EasingFunction for T {
  fn clone_box(&self) -> Box<dyn EasingFunction> {
    Box::new(self.clone())
  }
}

#[derive(Debug, Clone)]
pub struct LinearEasingFunction {
  points: Vec<f32>,
}

impl LinearEasingFunction {
  pub fn new(points: Vec<f32>) -> Self {
    Self { points }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, Vec<Vec<f32>>, String, ()>(
      Some(Parser::<'a, String>::string("linear(")),
      Some(
        Parser::one_or_more(Parser::<'a, f32>::float())
          .separated_by(
            Parser::<'a, String>::string(",")
              .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
          )
          .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, points, _, _) = values.expect("Expected values to be present");
      let flattened_points = points.unwrap().into_iter().flatten().collect();
      LinearEasingFunction::new(flattened_points)
    })
  }
}

impl fmt::Display for LinearEasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "linear({})",
      self
        .points
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    )
  }
}

#[derive(Debug, Clone)]
pub struct CubicBezierEasingFunction {
  points: [f32; 4],
}

impl CubicBezierEasingFunction {
  pub fn new(points: [f32; 4]) -> Self {
    Self { points }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<String, Vec<Vec<f32>>, String, ()>(
      Some(Parser::<'a, String>::string("cubic-bezier(")),
      Some(
        Parser::one_or_more(Parser::<'a, f32>::float())
          .separated_by(
            Parser::<'a, String>::string(",")
              .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
          )
          .surrounded_by(Parser::<'a, String>::whitespace().optional(), None)
          .where_fn(|points| points.len() == 4),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, points_opt, _, _) = values.expect("Expected values to be present");
      let points = points_opt
        .unwrap()
        .into_iter()
        .flatten()
        .collect::<Vec<f32>>();
      CubicBezierEasingFunction::new([points[0], points[1], points[2], points[3]])
    })
  }
}

impl fmt::Display for CubicBezierEasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "cubic-bezier({})",
      self
        .points
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    )
  }
}

#[derive(Debug, Clone)]
pub enum CubicBezierKeywordType {
  Ease,
  EaseIn,
  EaseOut,
  EaseInOut,
}

impl fmt::Display for CubicBezierKeywordType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CubicBezierKeywordType::Ease => write!(f, "ease"),
      CubicBezierKeywordType::EaseIn => write!(f, "ease-in"),
      CubicBezierKeywordType::EaseOut => write!(f, "ease-out"),
      CubicBezierKeywordType::EaseInOut => write!(f, "ease-in-out"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct CubicBezierKeyword {
  keyword: CubicBezierKeywordType,
}

impl CubicBezierKeyword {
  pub fn new(keyword: CubicBezierKeywordType) -> Self {
    Self { keyword }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::one_of(vec![
      Parser::<'a, String>::string("ease-in-out")
        .map(|_| CubicBezierKeyword::new(CubicBezierKeywordType::EaseInOut)),
      Parser::<'a, String>::string("ease-in")
        .map(|_| CubicBezierKeyword::new(CubicBezierKeywordType::EaseIn)),
      Parser::<'a, String>::string("ease-out")
        .map(|_| CubicBezierKeyword::new(CubicBezierKeywordType::EaseOut)),
      Parser::<'a, String>::string("ease")
        .map(|_| CubicBezierKeyword::new(CubicBezierKeywordType::Ease)),
    ])
  }
}

impl fmt::Display for CubicBezierKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.keyword)
  }
}

#[derive(Debug, Clone)]
pub enum StepPosition {
  Start,
  End,
}

impl fmt::Display for StepPosition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      StepPosition::Start => write!(f, "start"),
      StepPosition::End => write!(f, "end"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct StepsEasingFunction {
  steps: u32,
  start: StepPosition,
}

impl StepsEasingFunction {
  pub fn new(steps: u32, start: StepPosition) -> Self {
    Self { steps, start }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::<'a, String>::sequence::<
      String,
      Vec<(Option<u32>, Option<StepPosition>, Option<()>, Option<()>)>,
      String,
      (),
    >(
      Some(Parser::<'a, String>::string("steps(")),
      Some(
        Parser::<'a, u32>::sequence::<u32, StepPosition, (), ()>(
          Some(Parser::<'a, u32>::natural()),
          Some(Parser::one_of(vec![
            Parser::<'a, String>::string("start").map(|_| StepPosition::Start),
            Parser::<'a, String>::string("end").map(|_| StepPosition::End),
          ])),
          None,
          None,
        )
        .separated_by(
          Parser::<'a, String>::string(",")
            .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
        )
        .to_parser()
        .map(|p| {
          let p = p.unwrap();

          vec![(p.0, p.1, p.2, p.3)]
        })
        .surrounded_by(Parser::<'a, String>::whitespace().optional(), None),
      ),
      Some(Parser::<'a, String>::string(")")),
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, params_opt, _, _) = values.expect("Expected values to be present");
      let params = params_opt.unwrap();
      // Take the first element from the vector
      let (steps_opt, start_opt, _, _) = params[0].clone();
      StepsEasingFunction::new(steps_opt.unwrap(), start_opt.unwrap())
    })
  }
}

impl fmt::Display for StepsEasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "steps({}, {})", self.steps, self.start)
  }
}

#[derive(Debug, Clone)]
pub enum StepsKeywordType {
  StepStart,
  StepEnd,
}

impl fmt::Display for StepsKeywordType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      StepsKeywordType::StepStart => write!(f, "step-start"),
      StepsKeywordType::StepEnd => write!(f, "step-end"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct StepsKeyword {
  keyword: StepsKeywordType,
}

impl StepsKeyword {
  pub fn new(keyword: StepsKeywordType) -> Self {
    Self { keyword }
  }

  pub fn parse<'a>() -> Parser<'a, Self> {
    Parser::one_of(vec![
      Parser::<'a, String>::string("step-start")
        .map(|_| StepsKeyword::new(StepsKeywordType::StepStart)),
      Parser::<'a, String>::string("step-end")
        .map(|_| StepsKeyword::new(StepsKeywordType::StepEnd)),
    ])
  }
}

impl fmt::Display for StepsKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.keyword)
  }
}
// Define a wrapper for Box<dyn EasingFunction> that implements Clone
#[derive(Debug)]
pub struct BoxedEasingFunction(Box<dyn EasingFunction>);

impl Clone for BoxedEasingFunction {
  fn clone(&self) -> Self {
    Self(self.0.clone_box())
  }
}

impl fmt::Display for BoxedEasingFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl BoxedEasingFunction {
  pub fn new(function: Box<dyn EasingFunction>) -> Self {
    Self(function)
  }
}

pub fn parse_easing_function<'a>() -> Parser<'a, BoxedEasingFunction> {
  Parser::one_of(vec![
    LinearEasingFunction::parse().map(|f| BoxedEasingFunction::new(Box::new(f.unwrap()))),
    CubicBezierEasingFunction::parse().map(|f| BoxedEasingFunction::new(Box::new(f.unwrap()))),
    CubicBezierKeyword::parse().map(|f| BoxedEasingFunction::new(Box::new(f.unwrap()))),
    StepsEasingFunction::parse().map(|f| BoxedEasingFunction::new(Box::new(f.unwrap()))),
    StepsKeyword::parse().map(|f| BoxedEasingFunction::new(Box::new(f.unwrap()))),
  ])
}
