use crate::shared::{
  enums::data_structures::{css_syntax::CSSSyntax, value_with_default::ValueWithDefault},
  structures::{
    base_css_type::BaseCSSType,
    functions::{FunctionConfig, FunctionType},
  },
  utils::ast::factories::prop_or_spread_string_factory,
};
use indexmap::IndexMap;
use phf::phf_map;
use std::rc::Rc;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{Expr, ObjectLit},
};

pub trait HasBase {
  fn new(value: ValueWithDefault) -> Self
  where
    Self: Sized;
}

struct Angle {
  base: BaseCSSType,
}

impl HasBase for Angle {
  fn new(value: ValueWithDefault) -> Self {
    Angle {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::Angle,
      },
    }
  }
}

struct Color {
  base: BaseCSSType,
}

impl HasBase for Color {
  fn new(value: ValueWithDefault) -> Self {
    Color {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::Color,
      },
    }
  }
}

struct Url {
  base: BaseCSSType,
}

impl HasBase for Url {
  fn new(value: ValueWithDefault) -> Self {
    Url {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::Url,
      },
    }
  }
}

struct Image {
  base: BaseCSSType,
}

impl HasBase for Image {
  fn new(value: ValueWithDefault) -> Self {
    Image {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::Image,
      },
    }
  }
}

struct Integer {
  base: BaseCSSType,
}

impl HasBase for Integer {
  fn new(value: ValueWithDefault) -> Self {
    Integer {
      base: BaseCSSType {
        value: convert_number_to_string_using(
          |a| ValueWithDefault::String(a.to_string()),
          "0".to_string(),
        )(value),
        syntax: CSSSyntax::Integer,
      },
    }
  }
}

struct LengthPercentage {
  base: BaseCSSType,
}

impl HasBase for LengthPercentage {
  fn new(value: ValueWithDefault) -> Self {
    LengthPercentage {
      base: BaseCSSType {
        value: convert_number_to_percentage(value),
        syntax: CSSSyntax::LengthPercentage,
      },
    }
  }
}

struct Length {
  base: BaseCSSType,
}

impl HasBase for Length {
  fn new(value: ValueWithDefault) -> Self {
    Length {
      base: BaseCSSType {
        value: convert_number_to_length(value),
        syntax: CSSSyntax::Length,
      },
    }
  }
}

pub struct Percentage {
  base: BaseCSSType,
}

impl HasBase for Percentage {
  fn new(value: ValueWithDefault) -> Self {
    Percentage {
      base: BaseCSSType {
        value: convert_number_to_percentage(value),
        syntax: CSSSyntax::Percentage,
      },
    }
  }
}

pub struct Num {
  base: BaseCSSType,
}

impl HasBase for Num {
  fn new(value: ValueWithDefault) -> Self {
    Num {
      base: BaseCSSType {
        value: convert_number_to_bare_string(value),
        syntax: CSSSyntax::Number,
      },
    }
  }
}

pub struct Resolution {
  base: BaseCSSType,
}

impl HasBase for Resolution {
  fn new(value: ValueWithDefault) -> Self {
    Resolution {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::Resolution,
      },
    }
  }
}

pub struct Time {
  base: BaseCSSType,
}

impl HasBase for Time {
  fn new(value: ValueWithDefault) -> Self {
    Time {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::Time,
      },
    }
  }
}

pub struct TransformFunction {
  base: BaseCSSType,
}

impl HasBase for TransformFunction {
  fn new(value: ValueWithDefault) -> Self {
    TransformFunction {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::TransformFunction,
      },
    }
  }
}

pub struct TransformList {
  base: BaseCSSType,
}

impl HasBase for TransformList {
  fn new(value: ValueWithDefault) -> Self {
    TransformList {
      base: BaseCSSType {
        value,
        syntax: CSSSyntax::TransformList,
      },
    }
  }
}
fn convert_number_to_string_using(
  transform_number: fn(f64) -> ValueWithDefault,
  default_str: String,
) -> Rc<dyn Fn(ValueWithDefault) -> ValueWithDefault + 'static> {
  Rc::new(move |value: ValueWithDefault| -> ValueWithDefault {
    match value {
      ValueWithDefault::Number(n) => transform_number(n),
      ValueWithDefault::String(s) => s
        .parse()
        .map_or(ValueWithDefault::String(s), transform_number),
      ValueWithDefault::Map(o) => {
        let mut result = IndexMap::new();
        for (key, val) in o {
          result.insert(
            key,
            convert_number_to_string_using(transform_number, default_str.clone())(val),
          );
        }

        ValueWithDefault::Map(result)
      }
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
    |value: f64| {
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

impl From<Angle> for BaseCSSType {
  fn from(instance: Angle) -> Self {
    instance.base
  }
}

impl From<Color> for BaseCSSType {
  fn from(instance: Color) -> Self {
    instance.base
  }
}

impl From<Image> for BaseCSSType {
  fn from(instance: Image) -> Self {
    instance.base
  }
}

impl From<Integer> for BaseCSSType {
  fn from(instance: Integer) -> Self {
    instance.base
  }
}

impl From<Length> for BaseCSSType {
  fn from(instance: Length) -> Self {
    instance.base
  }
}

impl From<LengthPercentage> for BaseCSSType {
  fn from(instance: LengthPercentage) -> Self {
    instance.base
  }
}

impl From<Num> for BaseCSSType {
  fn from(instance: Num) -> Self {
    instance.base
  }
}

impl From<Percentage> for BaseCSSType {
  fn from(instance: Percentage) -> Self {
    instance.base
  }
}

impl From<Resolution> for BaseCSSType {
  fn from(instance: Resolution) -> Self {
    instance.base
  }
}

impl From<Time> for BaseCSSType {
  fn from(instance: Time) -> Self {
    instance.base
  }
}

impl From<TransformFunction> for BaseCSSType {
  fn from(instance: TransformFunction) -> Self {
    instance.base
  }
}

impl From<TransformList> for BaseCSSType {
  fn from(instance: TransformList) -> Self {
    instance.base
  }
}

impl From<Url> for BaseCSSType {
  fn from(instance: Url) -> Self {
    instance.base
  }
}

impl From<BaseCSSType> for Expr {
  fn from(instance: BaseCSSType) -> Self {
    let syntax_prop =
      prop_or_spread_string_factory("syntax", format!("{}", instance.syntax).as_str());

    let mut props = vec![syntax_prop];

    props.extend(BaseCSSType::value_to_props(instance.value, Option::None));

    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props,
    })
  }
}
fn angle(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = Angle::new(value).into();

  base_css_type.into()
}

fn color(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = Color::new(value).into();

  base_css_type.into()
}

fn image(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = Image::new(value).into();

  base_css_type.into()
}

fn integer(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = Integer::new(value).into();

  base_css_type.into()
}

fn length(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = Length::new(value).into();

  base_css_type.into()
}

fn length_percentage(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = LengthPercentage::new(value).into();

  base_css_type.into()
}

fn num(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = Num::new(value).into();

  base_css_type.into()
}

fn resolution(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = Resolution::new(value).into();

  base_css_type.into()
}

fn percentage(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = Percentage::new(value).into();

  base_css_type.into()
}

fn time(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = Time::new(value).into();

  base_css_type.into()
}

fn transform_function(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = TransformFunction::new(value).into();

  base_css_type.into()
}

fn transform_list(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = TransformList::new(value).into();

  base_css_type.into()
}

fn url(value: ValueWithDefault) -> Expr {
  let base_css_type: BaseCSSType = Url::new(value).into();

  base_css_type.into()
}

pub(crate) static FN_MAP: phf::Map<&'static str, fn(value: ValueWithDefault) -> Expr> = phf_map! {
  "angle" => angle,
  "color" => color,
  "image" => image,
  "integer" => integer,
  "length" => length,
  "lengthPercentage" => length_percentage,
  "number" => num,
  "percentage" => percentage,
  "resolution" => resolution,
  "time" => time,
  "transformFunction" => transform_function,
  "transformList" => transform_list,
  "url" => url,

};

pub(crate) fn get_types_fn() -> FunctionConfig {
  FunctionConfig {
    fn_ptr: FunctionType::StylexFnsFactory(
      |prop_name| -> Rc<dyn Fn(ValueWithDefault) -> Expr + 'static> {
        Rc::new(
          *FN_MAP
            .get(prop_name.as_str())
            .unwrap_or_else(|| panic!(r#"Function "{}" not found"#, prop_name)),
        )
      },
    ),
    takes_path: false,
  }
}
