use crate::shared::{
  structures::functions::{FunctionConfig, FunctionType},
  utils::common::{
    get_key_str, get_key_values_from_object, get_string_val_from_lit,
    prop_or_spread_expression_creator, prop_or_spread_string_creator,
  },
};
use indexmap::IndexMap;
use phf::phf_map;
use std::fmt;
use std::rc::Rc;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{Expr, ObjectLit, PropOrSpread},
};

#[derive(Debug, PartialEq, Clone)]
pub enum ValueWithDefault {
  Number(f64),
  String(String),
  Map(IndexMap<String, ValueWithDefault>),
}

impl std::hash::Hash for ValueWithDefault {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      ValueWithDefault::Number(n) => n.to_bits().hash(state),
      ValueWithDefault::String(s) => s.hash(state),
      ValueWithDefault::Map(map) => {
        for (key, value) in map {
          key.hash(state);
          value.hash(state);
        }
      }
    }
  }
}

impl ValueWithDefault {
  pub(crate) fn as_map(&self) -> Option<&IndexMap<String, ValueWithDefault>> {
    match self {
      ValueWithDefault::Map(map) => Option::Some(map),
      _ => Option::None,
    }
  }

  fn _as_string(&self) -> Option<&String> {
    match self {
      ValueWithDefault::String(s) => Option::Some(s),
      _ => Option::None,
    }
  }
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum CSSSyntax {
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
  TransformList,
}

impl fmt::Display for CSSSyntax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      CSSSyntax::Angle => write!(f, "<angle>"),
      CSSSyntax::Color => write!(f, "<color>"),
      CSSSyntax::Image => write!(f, "<image>"),
      CSSSyntax::Integer => write!(f, "<integer>"),
      CSSSyntax::Length => write!(f, "<length>"),
      CSSSyntax::LengthPercentage => write!(f, "<lengthPercentage>"),
      CSSSyntax::Number => write!(f, "<number>"),
      CSSSyntax::Percentage => write!(f, "<percentage>"),
      CSSSyntax::Resolution => write!(f, "<resolution>"),
      CSSSyntax::Time => write!(f, "<time>"),
      CSSSyntax::TransformFunction => write!(f, "<transformFunction>"),
      CSSSyntax::TransformList => write!(f, "<transformList>"),
      CSSSyntax::Url => write!(f, "<url>"),
    }
  }
}

impl From<String> for CSSSyntax {
  fn from(value: String) -> Self {
    match value.as_str() {
      "<angle>" => CSSSyntax::Angle,
      "<color>" => CSSSyntax::Color,
      "<image>" => CSSSyntax::Image,
      "<integer>" => CSSSyntax::Integer,
      "<length>" => CSSSyntax::Length,
      "<lengthPercentage>" => CSSSyntax::LengthPercentage,
      "<number>" => CSSSyntax::Number,
      "<percentage>" => CSSSyntax::Percentage,
      "<resolution>" => CSSSyntax::Resolution,
      "<time>" => CSSSyntax::Time,
      "<transformFunction>" => CSSSyntax::TransformFunction,
      "<transformList>" => CSSSyntax::TransformList,
      "<url>" => CSSSyntax::Url,
      _ => panic!(r#"CSSSyntax "{}" not found"#, value),
    }
  }
}

type CSSSyntaxType = CSSSyntax;

#[derive(Debug, PartialEq, Clone, Hash)]
pub(crate) struct BaseCSSType {
  pub(crate) value: ValueWithDefault,
  pub(crate) syntax: CSSSyntaxType,
}

impl BaseCSSType {
  fn value_to_props(value: ValueWithDefault, top_key: Option<String>) -> Vec<PropOrSpread> {
    let value = match value {
      ValueWithDefault::Number(n) => {
        let value_prop = prop_or_spread_string_creator(
          top_key.unwrap_or(String::from("value")).as_str(),
          n.to_string().as_str(),
        );
        let props = vec![value_prop];
        props
      }
      ValueWithDefault::String(s) => {
        let value_prop = prop_or_spread_string_creator(
          top_key.unwrap_or(String::from("value")).as_str(),
          s.as_str(),
        );
        let props = vec![value_prop];

        props
      }
      ValueWithDefault::Map(map) => {
        let mut local_props = vec![];

        for (key, val) in map {
          let props_to_extend = BaseCSSType::value_to_props(val, Some(key.clone()));

          local_props.extend(props_to_extend);
        }

        let object_expr = Expr::Object(ObjectLit {
          span: DUMMY_SP,
          props: local_props,
        });
        let prop = prop_or_spread_expression_creator(
          top_key.unwrap_or("value".to_string()).as_str(),
          Box::new(object_expr),
        );

        vec![prop]
      }
    };

    value
  }
}

impl From<ObjectLit> for BaseCSSType {
  fn from(obj: ObjectLit) -> BaseCSSType {
    let key_values = get_key_values_from_object(&obj);
    let mut syntax: Option<CSSSyntax> = Option::None;

    let mut values: IndexMap<String, ValueWithDefault> = IndexMap::new();

    for key_value in key_values {
      let key = get_key_str(&key_value);

      match key.as_str() {
        "syntax" => {
          syntax = key_value
            .value
            .as_lit()
            .and_then(get_string_val_from_lit)
            .map(|str| str.into())
        }
        "value" => {
          let obj_value = match key_value.value.as_ref() {
            Expr::Object(obj) => obj.clone(),
            Expr::Lit(obj) => {
              let value = get_string_val_from_lit(obj).expect("Value must be a string");

              let prop = prop_or_spread_string_creator("default", value.as_str());

              ObjectLit {
                span: DUMMY_SP,
                props: vec![prop],
              }
            }
            _ => panic!("Value must be an object or string"),
          };

          // key_value
          //   .value
          //   .as_object()
          //   .expect("Value must be an object");

          for key_value in get_key_values_from_object(&obj_value) {
            let key = get_key_str(&key_value);

            match key_value.value.as_ref() {
              Expr::Object(obj) => {
                let mut obj_map = IndexMap::new();

                let key_values = get_key_values_from_object(obj);

                for key_value in key_values {
                  let key = get_key_str(&key_value);

                  match key_value.value.as_ref() {
                    Expr::Lit(lit) => {
                      let value = get_string_val_from_lit(lit).expect("Value must be a string");

                      obj_map.insert(key, ValueWithDefault::String(value));
                    }
                    _ => panic!("Value must be a string"),
                  }
                }

                let value = ValueWithDefault::Map(obj_map);

                values.insert(key, value);
              }
              Expr::Lit(lit) => {
                let value = get_string_val_from_lit(lit).expect("Value must be a string");

                values.insert(key, ValueWithDefault::String(value));
              }
              _ => panic!("Value must be a string or object"),
            }
          }
        }
        _ => {
          panic!(r#"Key "{}" not support by BaseCSSType"#, key)
        }
      }
    }

    assert!(!values.is_empty(), "Invalid value in stylex.defineVars");

    assert!(
      values.contains_key("default"),
      "Default value is not defined for variable."
    );

    BaseCSSType {
      value: ValueWithDefault::Map(values),
      syntax: syntax.expect("Syntax is required"),
    }
  }
}

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
        syntax: CSSSyntaxType::Color,
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
        syntax: CSSSyntaxType::Url,
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
        syntax: CSSSyntaxType::Image,
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
        syntax: CSSSyntaxType::Integer,
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
        syntax: CSSSyntaxType::LengthPercentage,
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
        syntax: CSSSyntaxType::Length,
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
      prop_or_spread_string_creator("syntax", format!("{}", instance.syntax).as_str());

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
        // let orig_args: Vec<Pat> = params.clone();

        // let arrow_closure_fabric = |orig_args: Vec<Pat>| move |expr: Expr| expr.clone();

        // let functions = state.functions.identifiers.clone();

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
