use std::{collections::HashMap, rc::Rc};

// #[derive(Debug, PartialEq, Clone)]
// enum ValueWithDefault {
//     Number(f64),
//     String(String),
//     Map(HashMap<String, ValueWithDefault>),
// }
#[derive(Debug, PartialEq, Clone)]
enum ValueWithDefault {
  Number(f64),
  String(String),
  Map(HashMap<String, ValueWithDefault>),
}

#[derive(Debug, PartialEq, Clone)]
enum CSSSyntax {
  Asterisk,
  Length,
  Number,
  Percentage,
  LengthPercentage,
  Color,
  Image,
  Url,
  Integer,
  Angle,
  Time,
  Resolution,
  TransformFunction,
  CustomIdent,
  TransformList,
}

type CSSSyntaxType = CSSSyntax;

#[derive(Debug, PartialEq, Clone)]
struct BaseCSSType {
  value: ValueWithDefault,
  syntax: CSSSyntaxType,
}

impl BaseCSSType {
  fn new(value: ValueWithDefault) -> Self {
    BaseCSSType {
      value,
      syntax: CSSSyntaxType::Asterisk, // Default value
    }
  }
}

trait CSSType {
  fn value(&self) -> &ValueWithDefault;
  fn syntax(&self) -> &CSSSyntaxType;
}

fn is_css_type(value: &dyn CSSType) -> bool {
  println!("isCSSType {:?}", value.value());

  match value.syntax() {
    CSSSyntax::Asterisk => true,
    CSSSyntax::Length => true,
    CSSSyntax::Number => true,
    CSSSyntax::Percentage => true,
    CSSSyntax::LengthPercentage => true,
    CSSSyntax::Color => true,
    CSSSyntax::Image => true,
    CSSSyntax::Url => true,
    CSSSyntax::Integer => true,
    CSSSyntax::Angle => true,
    CSSSyntax::Time => true,
    CSSSyntax::Resolution => true,
    CSSSyntax::TransformFunction => true,
    CSSSyntax::CustomIdent => true,
    CSSSyntax::TransformList => true,
  }
}

struct Angle {
  value: ValueWithDefault,
  syntax: CSSSyntaxType,
}

impl Angle {
  fn new(value: ValueWithDefault) -> Self {
    Angle {
      value,
      syntax: CSSSyntaxType::Angle,
    }
  }
}

impl CSSType for Angle {
  fn value(&self) -> &ValueWithDefault {
    &self.value
  }

  fn syntax(&self) -> &CSSSyntaxType {
    &self.syntax
  }
}

fn angle(value: ValueWithDefault) -> Angle {
  Angle::new(value)
}

struct Color {
  value: ValueWithDefault,
  syntax: CSSSyntaxType,
}

impl Color {
  fn new(value: ValueWithDefault) -> Self {
    Color {
      value,
      syntax: CSSSyntaxType::Color,
    }
  }
}

impl CSSType for Color {
  fn value(&self) -> &ValueWithDefault {
    &self.value
  }

  fn syntax(&self) -> &CSSSyntaxType {
    &self.syntax
  }
}

fn color(value: ValueWithDefault) -> Color {
  Color::new(value)
}

struct Url {
  value: ValueWithDefault,
  syntax: CSSSyntaxType,
}

impl Url {
  fn new(value: ValueWithDefault) -> Self {
    Url {
      value,
      syntax: CSSSyntaxType::Url,
    }
  }
}

impl CSSType for Url {
  fn value(&self) -> &ValueWithDefault {
    &self.value
  }

  fn syntax(&self) -> &CSSSyntaxType {
    &self.syntax
  }
}

fn url(value: ValueWithDefault) -> Url {
  Url::new(value)
}

struct Image {
  value: ValueWithDefault,
  syntax: CSSSyntaxType,
}

impl Image {
  fn new(value: ValueWithDefault) -> Self {
    Image {
      value,
      syntax: CSSSyntaxType::Image,
    }
  }
}

impl CSSType for Image {
  fn value(&self) -> &ValueWithDefault {
    &self.value
  }

  fn syntax(&self) -> &CSSSyntaxType {
    &self.syntax
  }
}

fn image(value: ValueWithDefault) -> Image {
  Image::new(value)
}

struct Integer {
  value: ValueWithDefault,
  syntax: CSSSyntaxType,
}

impl Integer {
  fn new(value: ValueWithDefault) -> Self {
    Integer {
      value,
      syntax: CSSSyntaxType::Integer,
    }
  }
}

impl CSSType for Integer {
  fn value(&self) -> &ValueWithDefault {
    &self.value
  }

  fn syntax(&self) -> &CSSSyntaxType {
    &self.syntax
  }
}

fn integer(value: ValueWithDefault) -> Integer {
  Integer::new(convert_number_to_string_using(
    |a| ValueWithDefault::String(a.to_string()),
    "0".to_string(),
  )(value))
}

struct LengthPercentage {
  value: ValueWithDefault,
  syntax: CSSSyntaxType,
}

impl LengthPercentage {
  fn new(value: ValueWithDefault) -> Self {
    LengthPercentage {
      value,
      syntax: CSSSyntaxType::LengthPercentage,
    }
  }
}

impl CSSType for LengthPercentage {
  fn value(&self) -> &ValueWithDefault {
    &self.value
  }

  fn syntax(&self) -> &CSSSyntaxType {
    &self.syntax
  }
}

fn length_percentage(value: ValueWithDefault) -> LengthPercentage {
  LengthPercentage::new(convert_number_to_percentage(value))
}

struct Length {
  value: ValueWithDefault,
  syntax: CSSSyntaxType,
}

impl Length {
  fn new(value: ValueWithDefault) -> Self {
    Length {
      value,
      syntax: CSSSyntaxType::Length,
    }
  }
}

impl CSSType for Length {
  fn value(&self) -> &ValueWithDefault {
    &self.value
  }

  fn syntax(&self) -> &CSSSyntaxType {
    &self.syntax
  }
}

fn length(value: ValueWithDefault) -> Length {
  Length::new(convert_number_to_length(value))
}

pub struct Percentage {
  base: BaseCSSType,
}

impl Percentage {
  pub fn new(value: ValueWithDefault) -> Self {
    Percentage {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::Percentage,
      },
    }
  }

  // Assuming convert_number_to_percentage is a function that takes a ValueWithDefault and returns a String
  pub fn create(value: ValueWithDefault) -> Self {
    Percentage::new(convert_number_to_percentage(value))
  }
}

pub struct Num {
  base: BaseCSSType,
}

impl Num {
  pub fn new(value: ValueWithDefault) -> Self {
    Num {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::Number,
      },
    }
  }

  // Assuming convert_number_to_bare_string is a function that takes a ValueWithDefault and returns a String
  pub fn create(value: ValueWithDefault) -> Self {
    Num::new(convert_number_to_bare_string(value))
  }
}

pub struct Resolution {
  base: BaseCSSType,
}

impl Resolution {
  pub fn new(value: ValueWithDefault) -> Self {
    Resolution {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::Resolution,
      },
    }
  }

  pub fn create(value: ValueWithDefault) -> Self {
    Resolution::new(value)
  }
}

pub struct Time {
  base: BaseCSSType,
}

impl Time {
  pub fn new(value: ValueWithDefault) -> Self {
    Time {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::Time,
      },
    }
  }

  pub fn create(value: ValueWithDefault) -> Self {
    Time::new(value)
  }
}

pub struct TransformFunction {
  base: BaseCSSType,
}

impl TransformFunction {
  pub fn new(value: ValueWithDefault) -> Self {
    TransformFunction {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::TransformFunction,
      },
    }
  }

  pub fn create(value: ValueWithDefault) -> Self {
    TransformFunction::new(value)
  }
}

pub struct TransformList {
  base: BaseCSSType,
}

impl TransformList {
  pub fn new(value: ValueWithDefault) -> Self {
    TransformList {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::TransformList,
      },
    }
  }

  pub fn create(value: ValueWithDefault) -> Self {
    TransformList::new(value)
  }
}
fn convert_number_to_string_using(
  transform_number: fn(f64) -> ValueWithDefault,
  default_str: String,
) -> Rc<dyn Fn(ValueWithDefault) -> ValueWithDefault + 'static> {
  Rc::new(move |value: ValueWithDefault| -> ValueWithDefault {
    match value {
      ValueWithDefault::Number(n) => transform_number(n),
      ValueWithDefault::String(s) => ValueWithDefault::String(s),
      ValueWithDefault::Map(o) => {
        let mut result = HashMap::new();
        for (key, val) in o {
          result.insert(
            key,
            convert_number_to_string_using(transform_number, default_str.clone())(val),
          );
        }
        ValueWithDefault::Map(result)
      }
      _ => ValueWithDefault::String(default_str.clone()),
    }
  })
}

fn convert_number_to_bare_string(value: ValueWithDefault) -> ValueWithDefault {
  convert_number_to_string_using(
    |value| ValueWithDefault::String(value.to_string()),
    "0".to_string(),
  )(value)
}

fn convert_number_to_length(value: ValueWithDefault) -> ValueWithDefault {
  convert_number_to_string_using(
    |value| {
      if value == 0.0 {
        ValueWithDefault::String("0".to_string())
      } else {
        ValueWithDefault::String(format!("{}px", value))
      }
    },
    "0px".to_string(),
  )(value)
}

fn convert_number_to_percentage(value: ValueWithDefault) -> ValueWithDefault {
  convert_number_to_string_using(
    |value| {
      if value == 0.0 {
        ValueWithDefault::String("0".to_string())
      } else {
        ValueWithDefault::String(format!("{}%", value * 100.0))
      }
    },
    "0".to_string(),
  )(value)
}
