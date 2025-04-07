use super::alpha_value::AlphaValue;
use super::angle::Angle;
use super::common_types::Percentage;
use crate::parser::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
  Named(NamedColor),
  Hash(HashColor),
  Rgb(Rgb),
  Rgba(Rgba),
  Hsl(Hsl),
  Hsla(Hsla),
}

impl Color {
  pub fn parse<'a>() -> Parser<'a, Color> {
    let a = Parser::one_of(vec![
      NamedColor::parse().map(|c| {
        c.map(Color::Named)
          .unwrap_or_else(|| panic!("Failed to parse NamedColor"))
      }),
      HashColor::parse().map(|c| {
        c.map(Color::Hash)
          .unwrap_or_else(|| panic!("Failed to parse HashColor"))
      }),
      Rgb::parse().map(|c| {
        c.map(Color::Rgb)
          .unwrap_or_else(|| panic!("Failed to parse Rgb"))
      }),
      Rgba::parse().map(|c| {
        c.map(Color::Rgba)
          .unwrap_or_else(|| panic!("Failed to parse Rgba"))
      }),
      // Hsl::parse().map(|c| {
      //   c.map(Color::Hsl)
      //     .unwrap_or_else(|| panic!("Failed to parse Hsl"))
      // }),
      // Hsla::parse().map(|c| {
      //   c.map(Color::Hsla)
      //     .unwrap_or_else(|| panic!("Failed to parse Hsla"))
      // }),
    ]);

    a
  }
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Color::Named(c) => write!(f, "{}", c),
      Color::Hash(c) => write!(f, "{}", c),
      Color::Rgb(c) => write!(f, "{}", c),
      Color::Rgba(c) => write!(f, "{}", c),
      Color::Hsl(c) => write!(f, "{}", c),
      Color::Hsla(c) => write!(f, "{}", c),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedColor {
  pub value: String,
}

impl NamedColor {
  pub fn new(value: String) -> Self {
    NamedColor { value }
  }

  pub fn parse<'a>() -> Parser<'a, NamedColor> {
    let color_parsers = vec![
      Parser::<String>::string("aliceblue"),
      Parser::<String>::string("antiquewhite"),
      Parser::<String>::string("aqua").map(|_| "cyan".to_string()),
      Parser::<String>::string("aquamarine"),
      Parser::<String>::string("azure"),
      Parser::<String>::string("beige"),
      Parser::<String>::string("bisque"),
      Parser::<String>::string("black"),
      Parser::<String>::string("blanchedalmond"),
      Parser::<String>::string("blue"),
      Parser::<String>::string("blueviolet"),
      Parser::<String>::string("brown"),
      Parser::<String>::string("burlywood"),
      Parser::<String>::string("cadetblue"),
      Parser::<String>::string("chartreuse"),
      Parser::<String>::string("chocolate"),
      Parser::<String>::string("coral"),
      Parser::<String>::string("cornflowerblue"),
      Parser::<String>::string("cornsilk"),
      Parser::<String>::string("crimson"),
      Parser::<String>::string("cyan"),
      Parser::<String>::string("darkblue"),
      Parser::<String>::string("darkcyan"),
      Parser::<String>::string("darkgoldenrod"),
      Parser::<String>::string("darkgray"),
      Parser::<String>::string("darkgreen"),
      Parser::<String>::string("darkgrey").map(|_| "darkgray".to_string()),
      Parser::<String>::string("darkkhaki"),
      Parser::<String>::string("darkmagenta"),
      Parser::<String>::string("darkolivegreen"),
      Parser::<String>::string("darkorange"),
      Parser::<String>::string("darkorchid"),
      Parser::<String>::string("darkred"),
      Parser::<String>::string("darksalmon"),
      Parser::<String>::string("darkseagreen"),
      Parser::<String>::string("darkslateblue"),
      Parser::<String>::string("darkslategray"),
      Parser::<String>::string("darkslategrey").map(|_| "darkslategray".to_string()),
      Parser::<String>::string("darkturquoise"),
      Parser::<String>::string("darkviolet"),
      Parser::<String>::string("deeppink"),
      Parser::<String>::string("deepskyblue"),
      Parser::<String>::string("dimgray"),
      Parser::<String>::string("dimgrey").map(|_| "dimgray".to_string()),
      Parser::<String>::string("dodgerblue"),
      Parser::<String>::string("firebrick"),
      Parser::<String>::string("floralwhite"),
      Parser::<String>::string("forestgreen"),
      Parser::<String>::string("fuchsia").map(|_| "magenta".to_string()),
      Parser::<String>::string("gainsboro"),
      Parser::<String>::string("ghostwhite"),
      Parser::<String>::string("gold"),
      Parser::<String>::string("goldenrod"),
      Parser::<String>::string("gray"),
      Parser::<String>::string("green"),
      Parser::<String>::string("greenyellow"),
      Parser::<String>::string("grey").map(|_| "gray".to_string()),
      Parser::<String>::string("honeydew"),
      Parser::<String>::string("hotpink"),
      Parser::<String>::string("indianred"),
      Parser::<String>::string("indigo"),
      Parser::<String>::string("ivory"),
      Parser::<String>::string("khaki"),
      Parser::<String>::string("lavender"),
      Parser::<String>::string("lavenderblush"),
      Parser::<String>::string("lawngreen"),
      Parser::<String>::string("lemonchiffon"),
      Parser::<String>::string("lightblue"),
      Parser::<String>::string("lightcoral"),
      Parser::<String>::string("lightcyan"),
      Parser::<String>::string("lightgoldenrodyellow"),
      Parser::<String>::string("lightgray"),
      Parser::<String>::string("lightgreen"),
      Parser::<String>::string("lightgrey").map(|_| "lightgray".to_string()),
      Parser::<String>::string("lightpink"),
      Parser::<String>::string("lightsalmon"),
      Parser::<String>::string("lightseagreen"),
      Parser::<String>::string("lightskyblue"),
      Parser::<String>::string("lightslategray"),
      Parser::<String>::string("lightslategrey").map(|_| "lightslategray".to_string()),
      Parser::<String>::string("lightsteelblue"),
      Parser::<String>::string("lightyellow"),
      Parser::<String>::string("lime"),
      Parser::<String>::string("limegreen"),
      Parser::<String>::string("linen"),
      Parser::<String>::string("magenta"),
      Parser::<String>::string("maroon"),
      Parser::<String>::string("mediumaquamarine"),
      Parser::<String>::string("mediumblue"),
      Parser::<String>::string("mediumorchid"),
      Parser::<String>::string("mediumpurple"),
      Parser::<String>::string("mediumseagreen"),
      Parser::<String>::string("mediumslateblue"),
      Parser::<String>::string("mediumspringgreen"),
      Parser::<String>::string("mediumturquoise"),
      Parser::<String>::string("mediumvioletred"),
      Parser::<String>::string("midnightblue"),
      Parser::<String>::string("mintcream"),
      Parser::<String>::string("mistyrose"),
      Parser::<String>::string("moccasin"),
      Parser::<String>::string("navajowhite"),
      Parser::<String>::string("navy"),
      Parser::<String>::string("oldlace"),
      Parser::<String>::string("olive"),
      Parser::<String>::string("olivedrab"),
      Parser::<String>::string("orange"),
      Parser::<String>::string("orangered"),
      Parser::<String>::string("orchid"),
      Parser::<String>::string("palegoldenrod"),
      Parser::<String>::string("palegreen"),
      Parser::<String>::string("paleturquoise"),
      Parser::<String>::string("palevioletred"),
      Parser::<String>::string("papayawhip"),
      Parser::<String>::string("peachpuff"),
      Parser::<String>::string("peru"),
      Parser::<String>::string("pink"),
      Parser::<String>::string("plum"),
      Parser::<String>::string("powderblue"),
      Parser::<String>::string("purple"),
      Parser::<String>::string("rebeccapurple"),
      Parser::<String>::string("red"),
      Parser::<String>::string("rosybrown"),
      Parser::<String>::string("royalblue"),
      Parser::<String>::string("saddlebrown"),
      Parser::<String>::string("salmon"),
      Parser::<String>::string("sandybrown"),
      Parser::<String>::string("seagreen"),
      Parser::<String>::string("seashell"),
      Parser::<String>::string("sienna"),
      Parser::<String>::string("silver"),
      Parser::<String>::string("skyblue"),
      Parser::<String>::string("slateblue"),
      Parser::<String>::string("slategray"),
      Parser::<String>::string("slategrey").map(|_| "slategrey".to_string()),
      Parser::<String>::string("snow"),
      Parser::<String>::string("springgreen"),
      Parser::<String>::string("steelblue"),
      Parser::<String>::string("tan"),
      Parser::<String>::string("teal"),
      Parser::<String>::string("thistle"),
      Parser::<String>::string("tomato"),
      Parser::<String>::string("transparent"),
      Parser::<String>::string("turquoise"),
      Parser::<String>::string("violet"),
      Parser::<String>::string("wheat"),
      Parser::<String>::string("white"),
      Parser::<String>::string("whitesmoke"),
      Parser::<String>::string("yellow"),
      Parser::<String>::string("yellowgreen"),
    ];

    Parser::one_of(color_parsers)
      .map(|value| NamedColor::new(value.expect("Failed to parse named color")))
  }
}

impl Display for NamedColor {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.value)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashColor {
  pub value: String,
}

impl HashColor {
  pub fn new(value: String) -> Self {
    HashColor { value }
  }

  pub fn r(&self) -> u8 {
    u8::from_str_radix(&self.value[0..2], 16).unwrap_or(0)
  }

  pub fn g(&self) -> u8 {
    u8::from_str_radix(&self.value[2..4], 16).unwrap_or(0)
  }

  pub fn b(&self) -> u8 {
    u8::from_str_radix(&self.value[4..6], 16).unwrap_or(0)
  }

  pub fn a(&self) -> f32 {
    if self.value.len() == 8 {
      u8::from_str_radix(&self.value[6..8], 16).unwrap_or(0) as f32 / 255.0
    } else {
      1.0
    }
  }

  pub fn parse<'a>() -> Parser<'a, HashColor> {
    lazy_static! {
      static ref HEX_3_REGEX: Regex = Regex::new(r"^[0-9a-fA-F]{3}$").unwrap();
      static ref HEX_6_REGEX: Regex = Regex::new(r"^[0-9a-fA-F]{6}$").unwrap();
      static ref HEX_8_REGEX: Regex = Regex::new(r"^[0-9a-fA-F]{8}$").unwrap();
    }

    let hash_symbol = Parser::<String>::string("#");
    let hash_value = Parser::one_of(vec![
      Parser::<String>::regex(&HEX_3_REGEX),
      Parser::<String>::regex(&HEX_6_REGEX),
      Parser::<String>::regex(&HEX_8_REGEX),
    ]);

    Parser::<'a, String>::sequence::<String, String, String, String>(
      Some(hash_symbol),
      Some(hash_value),
      None,
      None,
    )
    .to_parser()
    .map(|values| {
      let (_, hex_value, _, _) = values.unwrap();
      let hex_value = hex_value.unwrap();

      if hex_value.len() == 3 {
        let expanded = hex_value
          .chars()
          .map(|c| format!("{}{}", c, c))
          .collect::<Vec<String>>()
          .join("");

        HashColor::new(expanded)
      } else {
        HashColor::new(hex_value)
      }
    })
  }
}

impl Display for HashColor {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "#{}", self.value)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rgb {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl Rgb {
  pub fn new(r: f32, g: f32, b: f32) -> Self {
    Rgb { r, g, b }
  }

  pub fn parse<'a>() -> Parser<'a, Rgb> {
    let rgb_num = Parser::one_of(vec![
      Parser::<f32>::float().where_fn(|v| *v >= 0.0 && *v <= 255.0),
      Parser::<String>::string("none").map(|_| 0.0),
    ]);

    let percentage = Parser::one_of(vec![
      Percentage::parse().map(|p| (p.expect("Failed to parse percentage").value * 255.0) / 100.0),
      Parser::<String>::string("none").map(|_| 0.0),
    ]);

    let rgb_nums3 = Parser::<'a, f32>::sequence::<f32, f32, f32, String>(
      Some(rgb_num.clone()),
      Some(rgb_num.clone()),
      Some(rgb_num.clone()),
      None,
    );

    let percentages3 = Parser::<'a, f32>::sequence::<f32, f32, f32, String>(
      Some(percentage.clone()),
      Some(percentage.clone()),
      Some(percentage.clone()),
      None,
    );

    let comma_num = rgb_nums3
      .clone()
      .separated_by(
        Parser::<String>::string(",")
          .surrounded_by(Parser::<String>::whitespace().optional(), None),
      )
      .to_parser();

    let space_num = rgb_nums3
      .separated_by(Parser::<String>::whitespace())
      .to_parser();

    let comma_percentage = percentages3
      .clone()
      .separated_by(
        Parser::<String>::string(",")
          .surrounded_by(Parser::<String>::whitespace().optional(), None),
      )
      .to_parser();

    let space_percentage = percentages3
      .separated_by(Parser::<String>::whitespace())
      .to_parser();

    Parser::one_of(vec![
      comma_num,
      space_num,
      comma_percentage,
      space_percentage,
    ])
    .surrounded_by(Parser::<String>::whitespace().optional(), None)
    .surrounded_by(
      Parser::<String>::string("rgb("),
      Some(Parser::<String>::string(")")),
    )
    .map(|values| {
      let (r, g, b, _) = values.unwrap();
      Rgb::new(
        *r.as_ref().unwrap(),
        *g.as_ref().unwrap(),
        *b.as_ref().unwrap(),
      )
    })
  }
}

impl Display for Rgb {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "rgb({},{},{})", self.r, self.g, self.b)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rgba {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Rgba {
  pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    Rgba { r, g, b, a }
  }

  pub fn parse<'a>() -> Parser<'a, Rgba> {
    let rgb_num = Parser::one_of(vec![
      Parser::<f32>::float().where_fn(|v| *v >= 0.0 && *v <= 255.0),
      Parser::<String>::string("none").map(|_| 0.0),
    ]);

    let percentage = Parser::one_of(vec![
      Percentage::parse().map(|p| (p.expect("Failed to parse percentage").value * 255.0) / 100.0),
      Parser::<String>::string("none").map(|_| 0.0),
    ]);

    let rgb_nums3 = Parser::<'a, f32>::sequence::<f32, f32, f32, String>(
      Some(rgb_num.clone()),
      Some(rgb_num.clone()),
      Some(rgb_num.clone()),
      None,
    );

    let percentages3 = Parser::<'a, f32>::sequence::<f32, f32, f32, String>(
      Some(percentage.clone()),
      Some(percentage.clone()),
      Some(percentage.clone()),
      None,
    );

    let comma_num = rgb_nums3
      .clone()
      .separated_by(
        Parser::<String>::string(",")
          .surrounded_by(Parser::<String>::whitespace().optional(), None),
      )
      .to_parser();

    let comma_percentage = percentages3
      .clone()
      .separated_by(
        Parser::<String>::string(",")
          .surrounded_by(Parser::<String>::whitespace().optional(), None),
      )
      .to_parser();

    let space_num = rgb_nums3
      .separated_by(Parser::<String>::whitespace())
      .to_parser();

    let space_percentage = percentages3
      .separated_by(Parser::<String>::whitespace())
      .to_parser();

    let rgb_parser = Parser::one_of(vec![comma_num, comma_percentage]);

    let comma_fn = Parser::<'a, f32>::sequence::<
      (Option<f32>, Option<f32>, Option<f32>, Option<String>),
      f32,
      String,
      String,
    >(
      Some(rgb_parser),
      Some(AlphaValue::parse().map(|a| a.expect("Failed to parse alpha value").value)),
      None,
      None,
    )
    .separated_by(
      Parser::<String>::string(",").surrounded_by(Parser::<String>::whitespace().optional(), None),
    )
    .to_parser()
    .surrounded_by(Parser::<String>::whitespace().optional(), None)
    .surrounded_by(
      Parser::one_of(vec![
        Parser::<String>::string("rgba("),
        Parser::<String>::string("rgb("),
      ]),
      Some(Parser::<String>::string(")")),
    )
    .map(|values| {
      let (rgb_values, alpha, _, _) = values.unwrap();
      let rgb_values = rgb_values.unwrap();
      let alpha = alpha.unwrap();

      let (r, g, b, _) = rgb_values;

      Rgba::new(
        *r.as_ref().unwrap(),
        *g.as_ref().unwrap(),
        *b.as_ref().unwrap(),
        alpha,
      )
    });

    let space_fn = Parser::<'a, f32>::sequence::<
      (Option<f32>, Option<f32>, Option<f32>, Option<String>),
      f32,
      f32,
      String,
    >(
      Some(Parser::one_of(vec![space_num, space_percentage])),
      Some(AlphaValue::parse().map(|a| a.expect("Failed to parse alpha value").value)),
      None,
      None,
    )
    .separated_by(Parser::<String>::string("/").surrounded_by(Parser::<String>::whitespace(), None))
    .to_parser()
    .surrounded_by(Parser::<String>::whitespace().optional(), None)
    .surrounded_by(
      Parser::one_of(vec![
        Parser::<String>::string("rgba("),
        Parser::<String>::string("rgb("),
      ]),
      Some(Parser::<String>::string(")")),
    )
    .map(|values| {
      let (rgb_values, alpha, _, _) = values.unwrap();
      let rgb_values = rgb_values.unwrap();
      let alpha = alpha.unwrap();

      let (r, g, b, _) = rgb_values;
      Rgba::new(
        *r.as_ref().unwrap(),
        *g.as_ref().unwrap(),
        *b.as_ref().unwrap(),
        alpha,
      )
    });

    Parser::one_of(vec![comma_fn, space_fn])
  }
}

impl Display for Rgba {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "rgba({},{},{},{})", self.r, self.g, self.b, self.a)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hsl {
  pub h: Angle,
  pub s: Percentage,
  pub l: Percentage,
}

impl Hsl {
  pub fn new(h: Angle, s: Percentage, l: Percentage) -> Self {
    Hsl { h, s, l }
  }

  pub fn parse<'a>() -> Parser<'a, Hsl> {
    let hue = Parser::one_of(vec![
      Angle::parse(),
      Parser::<f32>::float().map(|v| Angle {
        value: v.expect("Failed to parse angle"),
        unit: "deg".to_string(),
      }),
    ]);

    let saturation = Percentage::parse();
    let lightness = Percentage::parse();

    let hsl_tuple = Parser::<'a, Angle>::sequence::<Angle, Percentage, Percentage, String>(
      Some(hue.clone()),
      Some(saturation.clone()),
      Some(lightness.clone()),
      None,
    );

    let hsl_comma_tuple = hsl_tuple
      .clone()
      .separated_by(
        Parser::<String>::string(",")
          .surrounded_by(Parser::<String>::whitespace().optional(), None),
      )
      .to_parser();

    let hsl_space_tuple = hsl_tuple
      .separated_by(Parser::<String>::whitespace())
      .to_parser();

    Parser::one_of(vec![hsl_comma_tuple, hsl_space_tuple])
      .surrounded_by(Parser::<String>::whitespace().optional(), None)
      .surrounded_by(
        Parser::<String>::string("hsl("),
        Some(Parser::<String>::string(")")),
      )
      .map(|tuple| {
        let (h, s, l, _) = tuple.unwrap();
        Hsl::new(h.unwrap(), s.unwrap(), l.unwrap())
      })
  }
}

impl Display for Hsl {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "hsl({},{},{})", self.h, self.s, self.l)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hsla {
  pub h: Angle,
  pub s: Percentage,
  pub l: Percentage,
  pub a: f32,
}

impl Hsla {
  pub fn new(h: Angle, s: Percentage, l: Percentage, a: f32) -> Self {
    Hsla { h, s, l, a }
  }

  pub fn parse<'a>() -> Parser<'a, Hsla> {
    let hue = Parser::one_of(vec![
      Angle::parse(),
      Parser::<f32>::float().map(|v| Angle {
        value: v.expect("Failed to parse angle"),
        unit: "deg".to_string(),
      }),
    ]);

    let saturation = Percentage::parse();
    let lightness = Percentage::parse();

    let fn_name = Parser::one_of(vec![
      Parser::<String>::string("hsla"),
      Parser::<String>::string("hsl"),
    ]);

    let hsl_tuple = Parser::<'a, Angle>::sequence::<Angle, Percentage, Percentage, String>(
      Some(hue.clone()),
      Some(saturation.clone()),
      Some(lightness.clone()),
      None,
    );

    let hsl_comma_tuple = hsl_tuple
      .clone()
      .separated_by(
        Parser::<String>::string(",")
          .surrounded_by(Parser::<String>::whitespace().optional(), None),
      )
      .to_parser();

    let comma_fn_args = Parser::<'a, Angle>::sequence::<
      (
        Option<Angle>,
        Option<Percentage>,
        Option<Percentage>,
        Option<String>,
      ),
      f32,
      String,
      String,
    >(
      Some(hsl_comma_tuple.clone()),
      Some(AlphaValue::parse().map(|a| a.expect("Failed to parse alpha").value)),
      None,
      None,
    )
    .separated_by(
      Parser::<String>::string(",").surrounded_by(Parser::<String>::whitespace().optional(), None),
    )
    .to_parser();

    let space_fn_args = Parser::<'a, Angle>::sequence::<
      (
        Option<Angle>,
        Option<Percentage>,
        Option<Percentage>,
        Option<String>,
      ),
      f32,
      String,
      String,
    >(
      Some(hsl_comma_tuple),
      Some(AlphaValue::parse().map(|a| a.expect("Failed to parse alpha").value)),
      None,
      None,
    )
    .separated_by(Parser::<String>::string("/").surrounded_by(Parser::<String>::whitespace(), None))
    .to_parser();

    Parser::one_of(vec![comma_fn_args, space_fn_args])
      .surrounded_by(Parser::<String>::whitespace().optional(), None)
      .surrounded_by(fn_name, Some(Parser::<String>::string(")")))
      .map(|values| {
        let (hsl_values, alpha, _, _) = values.unwrap();
        let hsl_values = hsl_values.unwrap();
        let alpha = alpha.unwrap();

        let (h, s, l, _) = hsl_values;

        Hsla::new(h.unwrap(), s.unwrap(), l.unwrap(), alpha)
      })
  }
}

impl Display for Hsla {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "hsla({},{},{},{})", self.h, self.s, self.l, self.a)
  }
}
