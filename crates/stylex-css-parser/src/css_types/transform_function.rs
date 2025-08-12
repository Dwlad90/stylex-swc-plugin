/*!
CSS transform function parser.
Mirrors: packages/style-value-parser/src/css-types/transform-function.js
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
    let fn_name = TokenParser::<String>::fn_name("matrix");
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    // Parse 6 numbers separated by optional whitespace and commas
    fn_name
      .flat_map(move |_| number_parser(), Some("a"))
      .flat_map(
        move |a| {
          let a_clone = a;
          number_parser().map(move |b| (a_clone, b), Some("b"))
        },
        Some("with_b"),
      )
      .flat_map(
        move |(a, b)| {
          let a_clone = a;
          let b_clone = b;
          number_parser().map(move |c| (a_clone, b_clone, c), Some("c"))
        },
        Some("with_c"),
      )
      .flat_map(
        move |(a, b, c)| {
          let a_clone = a;
          let b_clone = b;
          let c_clone = c;
          number_parser().map(move |d| (a_clone, b_clone, c_clone, d), Some("d"))
        },
        Some("with_d"),
      )
      .flat_map(
        move |(a, b, c, d)| {
          let a_clone = a;
          let b_clone = b;
          let c_clone = c;
          let d_clone = d;
          number_parser().map(
            move |tx| (a_clone, b_clone, c_clone, d_clone, tx),
            Some("tx"),
          )
        },
        Some("with_tx"),
      )
      .flat_map(
        move |(a, b, c, d, tx)| {
          let a_clone = a;
          let b_clone = b;
          let c_clone = c;
          let d_clone = d;
          let tx_clone = tx;
          number_parser().map(
            move |ty| (a_clone, b_clone, c_clone, d_clone, tx_clone, ty),
            Some("ty"),
          )
        },
        Some("with_ty"),
      )
      .flat_map(
        move |(a, b, c, d, tx, ty)| {
          close.map(move |_| Matrix::new(a, b, c, d, tx, ty), Some("to_matrix"))
        },
        Some("close"),
      )
  }
}

impl Matrix3d {
  pub fn new(args: [f64; 16]) -> Self {
    Self { args }
  }

  pub fn parse() -> TokenParser<Matrix3d> {
    let fn_name = TokenParser::<String>::fn_name("matrix3d");
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    // Parse 16 numbers for matrix3d

    fn_name
      .flat_map(|_| {
        // Parse all 16 numbers in sequence
        let first = number_parser();
        first.flat_map(|n1| {
          number_parser().flat_map(move |n2| {
            number_parser().flat_map(move |n3| {
              number_parser().flat_map(move |n4| {
                number_parser().flat_map(move |n5| {
                  number_parser().flat_map(move |n6| {
                    number_parser().flat_map(move |n7| {
                      number_parser().flat_map(move |n8| {
                        number_parser().flat_map(move |n9| {
                          number_parser().flat_map(move |n10| {
                            number_parser().flat_map(move |n11| {
                              number_parser().flat_map(move |n12| {
                                number_parser().flat_map(move |n13| {
                                  number_parser().flat_map(move |n14| {
                                    number_parser().flat_map(move |n15| {
                                      number_parser().map(move |n16| {
                                        [n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11, n12, n13, n14, n15, n16]
                                      }, Some("collect_16"))
                                    }, Some("n16"))
                                  }, Some("n15"))
                                }, Some("n14"))
                              }, Some("n13"))
                            }, Some("n12"))
                          }, Some("n11"))
                        }, Some("n10"))
                      }, Some("n9"))
                    }, Some("n8"))
                  }, Some("n7"))
                }, Some("n6"))
              }, Some("n5"))
            }, Some("n4"))
          }, Some("n3"))
        }, Some("n2"))
      }, Some("all_numbers"))
      .flat_map(move |args| {
        close.map(move |_| Matrix3d::new(args), Some("to_matrix3d"))
      }, Some("close"))
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

  pub fn parse() -> TokenParser<Rotate3d> {
    let fn_name = TokenParser::<String>::fn_name("rotate3d");
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    fn_name
      .flat_map(|_| number_parser(), Some("x"))
      .flat_map(|x| {
        let x_clone = x;
        number_parser().map(move |y| (x_clone, y), Some("y"))
      }, Some("with_y"))
      .flat_map(|(x, y)| {
        let x_clone = x;
        let y_clone = y;
        number_parser().map(move |z| (x_clone, y_clone, z), Some("z"))
      }, Some("with_z"))
      .flat_map(|(x, y, z)| {
        let x_clone = x;
        let y_clone = y;
        let z_clone = z;
        Angle::parser().map(move |angle| (x_clone, y_clone, z_clone, angle), Some("angle"))
      }, Some("with_angle"))
      .flat_map(move |(x, y, z, angle)| {
        close.map(move |_| Rotate3d::new(x, y, z, angle.clone()), Some("to_rotate3d"))
      }, Some("close"))
  }
}

impl Scale {
  pub fn new(sx: f64, sy: Option<f64>) -> Self {
    Self { sx, sy }
  }

  pub fn parse() -> TokenParser<Scale> {
    let fn_name = TokenParser::<String>::fn_name("scale");
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    let num_or_pct = number_or_percentage_parser();
    let second_val = num_or_pct.clone().optional();

    fn_name
      .flat_map(move |_| num_or_pct.clone(), Some("sx"))
      .flat_map(
        move |sx| {
          let sx_clone = sx.clone();
          second_val
            .clone()
            .map(move |sy_opt| (sx_clone.clone(), sy_opt), Some("sy"))
        },
        Some("with_sy"),
      )
      .flat_map(
        move |(sx, sy_opt)| {
          let sx_f64 = number_or_percentage_to_f64(sx);
          let sy_f64 = sy_opt.map(number_or_percentage_to_f64);
          close
            .clone()
            .map(move |_| Scale::new(sx_f64, sy_f64), Some("to_scale"))
        },
        Some("close"),
      )
  }
}

impl Scale3d {
  pub fn new(sx: f64, sy: f64, sz: f64) -> Self {
    Self { sx, sy, sz }
  }

  pub fn parse() -> TokenParser<Scale3d> {
    let fn_name = TokenParser::<String>::fn_name("scale3d");
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    fn_name
      .flat_map(|_| number_or_percentage_parser(), Some("sx"))
      .flat_map(|sx| {
        let sx_clone = sx.clone();
        number_or_percentage_parser().map(move |sy| (sx_clone.clone(), sy), Some("sy"))
      }, Some("with_sy"))
      .flat_map(|(sx, sy)| {
        let sx_clone = sx.clone();
        let sy_clone = sy.clone();
        number_or_percentage_parser().map(move |sz| (sx_clone.clone(), sy_clone.clone(), sz), Some("sz"))
      }, Some("with_sz"))
      .flat_map(move |(sx, sy, sz)| {
        let sx_f64 = number_or_percentage_to_f64(sx);
        let sy_f64 = number_or_percentage_to_f64(sy);
        let sz_f64 = number_or_percentage_to_f64(sz);
        close.map(move |_| Scale3d::new(sx_f64, sy_f64, sz_f64), Some("to_scale3d"))
      }, Some("close"))
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
    let fn_name = TokenParser::<String>::fn_name("translate");
    let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

    let second_val = length_percentage_parser().optional();

    fn_name
      .flat_map(move |_| length_percentage_parser(), Some("tx"))
      .flat_map(
        move |tx| {
          let tx_clone = tx.clone();
          second_val
            .clone()
            .map(move |ty_opt| (tx_clone.clone(), ty_opt), Some("ty"))
        },
        Some("with_ty"),
      )
      .flat_map(
        move |(tx, ty_opt)| {
          close.clone().map(
            move |_| Translate::new(tx.clone(), ty_opt.clone()),
            Some("to_translate"),
          )
        },
        Some("close"),
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
      .flat_map(|tx| {
        let tx_clone = tx.clone();
        length_percentage_parser().map(move |ty| (tx_clone.clone(), ty), Some("ty"))
      }, Some("with_ty"))
      .flat_map(|(tx, ty)| {
        let tx_clone = tx.clone();
        let ty_clone = ty.clone();
        Length::parser().map(move |tz| (tx_clone.clone(), ty_clone.clone(), tz), Some("tz"))
      }, Some("with_tz"))
      .flat_map(move |(tx, ty, tz)| {
        close.map(move |_| Translate3d::new(tx.clone(), ty.clone(), tz.clone()), Some("to_translate3d"))
      }, Some("close"))
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
  /// Parser for all transform functions
  /// Mirrors: TransformFunction.parser in transform-function.js exactly
  pub fn parse() -> TokenParser<TransformFunction> {
    TokenParser::one_of(vec![
      Matrix::parse().map(TransformFunction::Matrix, Some("matrix")),
      Matrix3d::parse().map(TransformFunction::Matrix3d, Some("matrix3d")),
      Perspective::parse().map(TransformFunction::Perspective, Some("perspective")),
      Rotate::parse().map(TransformFunction::Rotate, Some("rotate")),
      RotateXYZ::parse().map(TransformFunction::RotateXYZ, Some("rotate_xyz")),
      Rotate3d::parse().map(TransformFunction::Rotate3d, Some("rotate3d")),
      Scale::parse().map(TransformFunction::Scale, Some("scale")),
      Scale3d::parse().map(TransformFunction::Scale3d, Some("scale3d")),
      ScaleAxis::parse().map(TransformFunction::ScaleAxis, Some("scale_axis")),
      Skew::parse().map(TransformFunction::Skew, Some("skew")),
      SkewAxis::parse().map(TransformFunction::SkewAxis, Some("skew_axis")),
      Translate3d::parse().map(TransformFunction::Translate3d, Some("translate3d")),
      Translate::parse().map(TransformFunction::Translate, Some("translate")),
      TranslateAxis::parse().map(TransformFunction::TranslateAxis, Some("translate_axis")),
    ])
  }
}

impl Display for TransformFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TransformFunction::Matrix(m) => write!(
        f,
        "matrix({}, {}, {}, {}, {}, {})",
        m.a, m.b, m.c, m.d, m.tx, m.ty
      ),
      TransformFunction::Matrix3d(m) => {
        let args: Vec<String> = m.args.iter().map(|x| x.to_string()).collect();
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
      TransformFunction::Rotate3d(r) => {
        // Mirror JavaScript optimization logic exactly
        match (r.x, r.y, r.z) {
          (x, y, z) if x == 1.0 && y == 0.0 && z == 0.0 => write!(f, "rotateX({})", r.angle),
          (x, y, z) if x == 0.0 && y == 1.0 && z == 0.0 => write!(f, "rotateY({})", r.angle),
          (x, y, z) if x == 0.0 && y == 0.0 && z == 1.0 => write!(f, "rotateZ({})", r.angle),
          _ => write!(f, "rotate3d({}, {}, {}, {})", r.x, r.y, r.z, r.angle),
        }
      }
      TransformFunction::Scale(s) => match &s.sy {
        Some(sy) => write!(f, "scale({}, {})", s.sx, sy),
        None => write!(f, "scale({})", s.sx),
      },
      TransformFunction::Scale3d(s) => write!(f, "scale3d({}, {}, {})", s.sx, s.sy, s.sz),
      TransformFunction::ScaleAxis(s) => write!(
        f,
        "scale{}({})",
        match s.axis {
          Axis::X => "X",
          Axis::Y => "Y",
          Axis::Z => "Z",
        },
        s.s
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
