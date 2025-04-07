use crate::parser::Parser;
use std::fmt;

use super::number::number;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LengthUnit {
  Cap,
  Ch,
  Em,
  Ex,
  Ic,
  Lh,
  Rem,
  Rlh,
  Vh,
  Svh,
  Lvh,
  Dvh,
  Vw,
  Svw,
  Lvw,
  Dvw,
  Vmin,
  Svmin,
  Lvmin,
  Dvmin,
  Vmax,
  Svmax,
  Lvmax,
  Dvmax,
  Cqw,
  Cqi,
  Cqh,
  Cqb,
  Cqmin,
  Cqmax,
  #[default]
  Px,
  Cm,
  Mm,
  In,
  Pt,
}

impl fmt::Display for LengthUnit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      Self::Cap => "cap",
      Self::Ch => "ch",
      Self::Em => "em",
      Self::Ex => "ex",
      Self::Ic => "ic",
      Self::Lh => "lh",
      Self::Rem => "rem",
      Self::Rlh => "rlh",
      Self::Vh => "vh",
      Self::Svh => "svh",
      Self::Lvh => "lvh",
      Self::Dvh => "dvh",
      Self::Vw => "vw",
      Self::Svw => "svw",
      Self::Lvw => "lvw",
      Self::Dvw => "dvw",
      Self::Vmin => "vmin",
      Self::Svmin => "svmin",
      Self::Lvmin => "lvmin",
      Self::Dvmin => "dvmin",
      Self::Vmax => "vmax",
      Self::Svmax => "svmax",
      Self::Lvmax => "lvmax",
      Self::Dvmax => "dvmax",
      Self::Cqw => "cqw",
      Self::Cqi => "cqi",
      Self::Cqh => "cqh",
      Self::Cqb => "cqb",
      Self::Cqmin => "cqmin",
      Self::Cqmax => "cqmax",
      Self::Px => "px",
      Self::Cm => "cm",
      Self::Mm => "mm",
      Self::In => "in",
      Self::Pt => "pt",
    };
    write!(f, "{}", s)
  }
}

impl std::str::FromStr for LengthUnit {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "cap" => Ok(Self::Cap),
      "ch" => Ok(Self::Ch),
      "em" => Ok(Self::Em),
      "ex" => Ok(Self::Ex),
      "ic" => Ok(Self::Ic),
      "lh" => Ok(Self::Lh),
      "rem" => Ok(Self::Rem),
      "rlh" => Ok(Self::Rlh),
      "vh" => Ok(Self::Vh),
      "svh" => Ok(Self::Svh),
      "lvh" => Ok(Self::Lvh),
      "dvh" => Ok(Self::Dvh),
      "vw" => Ok(Self::Vw),
      "svw" => Ok(Self::Svw),
      "lvw" => Ok(Self::Lvw),
      "dvw" => Ok(Self::Dvw),
      "vmin" => Ok(Self::Vmin),
      "svmin" => Ok(Self::Svmin),
      "lvmin" => Ok(Self::Lvmin),
      "dvmin" => Ok(Self::Dvmin),
      "vmax" => Ok(Self::Vmax),
      "svmax" => Ok(Self::Svmax),
      "lvmax" => Ok(Self::Lvmax),
      "dvmax" => Ok(Self::Dvmax),
      "cqw" => Ok(Self::Cqw),
      "cqi" => Ok(Self::Cqi),
      "cqh" => Ok(Self::Cqh),
      "cqb" => Ok(Self::Cqb),
      "cqmin" => Ok(Self::Cqmin),
      "cqmax" => Ok(Self::Cqmax),
      "px" => Ok(Self::Px),
      "cm" => Ok(Self::Cm),
      "mm" => Ok(Self::Mm),
      "in" => Ok(Self::In),
      "pt" => Ok(Self::Pt),
      _ => Err(()),
    }
  }
}

impl From<String> for LengthUnit {
  fn from(unit_str: String) -> Self {
    match unit_str.as_str() {
      "cap" => Self::Cap,
      "ch" => Self::Ch,
      "em" => Self::Em,
      "ex" => Self::Ex,
      "ic" => Self::Ic,
      "lh" => Self::Lh,
      "rem" => Self::Rem,
      "rlh" => Self::Rlh,
      "vh" => Self::Vh,
      "svh" => Self::Svh,
      "lvh" => Self::Lvh,
      "dvh" => Self::Dvh,
      "vw" => Self::Vw,
      "svw" => Self::Svw,
      "lvw" => Self::Lvw,
      "dvw" => Self::Dvw,
      "vmin" => Self::Vmin,
      "svmin" => Self::Svmin,
      "lvmin" => Self::Lvmin,
      "dvmin" => Self::Dvmin,
      "vmax" => Self::Vmax,
      "svmax" => Self::Svmax,
      "lvmax" => Self::Lvmax,
      "dvmax" => Self::Dvmax,
      "cqw" => Self::Cqw,
      "cqi" => Self::Cqi,
      "cqh" => Self::Cqh,
      "cqb" => Self::Cqb,
      "cqmin" => Self::Cqmin,
      "cqmax" => Self::Cqmax,
      "px" => Self::Px,
      "cm" => Self::Cm,
      "mm" => Self::Mm,
      "in" => Self::In,
      "pt" => Self::Pt,
      _ => panic!("Invalid length unit"),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Length {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Length {
  pub fn new(value: f32, unit: Option<String>) -> Self {
    Self {
      value,
      unit: match unit {
        Some(unit_str) => unit_str.parse().unwrap_or_default(),
        None => LengthUnit::default(),
      },
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    Parser::one_of(vec![
      LengthBasedOnFont::parse(),
      LengthBasedOnViewport::parse(),
      LengthBasedOnContainer::parse(),
      LengthBasedOnAbsoluteUnits::parse(),
      // If nothing else, check for a plain `0`
      Parser::<'a, String>::string("0").map(|_| Length::new(0.0, None)),
    ])
  }
}

impl fmt::Display for Length {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LengthBasedOnFont {
  pub value: f32,
  pub unit: LengthUnit,
}

impl From<LengthBasedOnFont> for Length {
  fn from(length: LengthBasedOnFont) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl LengthBasedOnFont {
  pub fn new(value: f32, unit: LengthUnit) -> Self {
    Self { value, unit }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    Parser::one_of(vec![
      Ch::parse(),
      Em::parse(),
      Ex::parse(),
      Ic::parse(),
      Lh::parse(),
      Rem::parse(),
      Rlh::parse(),
    ])
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LengthBasedOnViewport {
  pub value: f32,
  pub unit: LengthUnit,
}

impl From<LengthBasedOnViewport> for Length {
  fn from(length: LengthBasedOnViewport) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl LengthBasedOnViewport {
  pub fn new(value: f32, unit: LengthUnit) -> Self {
    Self { value, unit }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    Parser::one_of(vec![
      Vh::parse(),
      Svh::parse(),
      Lvh::parse(),
      Dvh::parse(),
      Vw::parse(),
      Svw::parse(),
      Lvw::parse(),
      Dvw::parse(),
      Vmin::parse(),
      Svmin::parse(),
      Lvmin::parse(),
      Dvmin::parse(),
      Vmax::parse(),
      Svmax::parse(),
      Lvmax::parse(),
      Dvmax::parse(),
    ])
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LengthBasedOnContainer {
  pub value: f32,
  pub unit: LengthUnit,
}

impl From<LengthBasedOnContainer> for Length {
  fn from(length: LengthBasedOnContainer) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl LengthBasedOnContainer {
  pub fn new(value: f32, unit: LengthUnit) -> Self {
    Self { value, unit }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    Parser::one_of(vec![
      Cqw::parse(),
      Cqi::parse(),
      Cqh::parse(),
      Cqb::parse(),
      Cqmin::parse(),
      Cqmax::parse(),
    ])
  }
}

impl fmt::Display for LengthBasedOnContainer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LengthBasedOnAbsoluteUnits {
  pub value: f32,
  pub unit: LengthUnit,
}

impl From<LengthBasedOnAbsoluteUnits> for Length {
  fn from(length: LengthBasedOnAbsoluteUnits) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl LengthBasedOnAbsoluteUnits {
  pub fn new(value: f32, unit: LengthUnit) -> Self {
    Self { value, unit }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    Parser::one_of(vec![
      Px::parse(),
      Cm::parse(),
      Mm::parse(),
      In::parse(),
      Pt::parse(),
    ])
  }
}

impl fmt::Display for LengthBasedOnAbsoluteUnits {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

fn unit<'a>(unit_str: &'static str) -> Parser<'a, Length> {
  number()
    .skip(Parser::<'a, String>::string(unit_str))
    .map(move |v| Length::new(v.unwrap(), Some(unit_str.to_string())))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cap {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Cap {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Cap,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("cap")
  }
}

impl From<Cap> for Length {
  fn from(cap: Cap) -> Self {
    Self {
      value: cap.value,
      unit: cap.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ch {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Ch {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Ch,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("ch")
  }
}

impl From<Ch> for Length {
  fn from(ch: Ch) -> Self {
    Self {
      value: ch.value,
      unit: ch.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Em {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Em {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Em,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("em")
  }
}

impl From<Em> for Length {
  fn from(em: Em) -> Self {
    Self {
      value: em.value,
      unit: em.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ex {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Ex {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Ex,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("ex")
  }
}

impl From<Ex> for Length {
  fn from(ex: Ex) -> Self {
    Self {
      value: ex.value,
      unit: ex.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ic {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Ic {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Ic,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("ic")
  }
}

impl From<Ic> for Length {
  fn from(ic: Ic) -> Self {
    Self {
      value: ic.value,
      unit: ic.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lh {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Lh {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Lh,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("lh")
  }
}

impl From<Lh> for Length {
  fn from(lh: Lh) -> Self {
    Self {
      value: lh.value,
      unit: lh.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rem {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Rem {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Rem,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("rem")
  }
}

impl From<Rem> for Length {
  fn from(rem: Rem) -> Self {
    Self {
      value: rem.value,
      unit: rem.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rlh {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Rlh {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Rlh,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("rlh")
  }
}

impl From<Rlh> for Length {
  fn from(rlh: Rlh) -> Self {
    Self {
      value: rlh.value,
      unit: rlh.unit,
    }
  }
}

// Viewport-based units
#[derive(Debug, Clone, PartialEq)]
pub struct Vh {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Vh {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Vh,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("vh")
  }
}

impl From<Vh> for Length {
  fn from(vh: Vh) -> Self {
    Self {
      value: vh.value,
      unit: vh.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Svh {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Svh {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Svh,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("svh")
  }
}

impl From<Svh> for Length {
  fn from(svh: Svh) -> Self {
    Self {
      value: svh.value,
      unit: svh.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lvh {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Lvh {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Lvh,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("lvh")
  }
}

impl From<Lvh> for Length {
  fn from(lvh: Lvh) -> Self {
    Self {
      value: lvh.value,
      unit: lvh.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dvh {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Dvh {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Dvh,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("dvh")
  }
}

impl From<Dvh> for Length {
  fn from(dvh: Dvh) -> Self {
    Self {
      value: dvh.value,
      unit: dvh.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vw {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Vw {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Vw,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("vw")
  }
}

impl From<Vw> for Length {
  fn from(vw: Vw) -> Self {
    Self {
      value: vw.value,
      unit: vw.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Svw {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Svw {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Svw,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("svw")
  }
}

impl From<Svw> for Length {
  fn from(svw: Svw) -> Self {
    Self {
      value: svw.value,
      unit: svw.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lvw {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Lvw {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Lvw,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("lvw")
  }
}

impl From<Lvw> for Length {
  fn from(lvw: Lvw) -> Self {
    Self {
      value: lvw.value,
      unit: lvw.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dvw {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Dvw {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Dvw,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("dvw")
  }
}

impl From<Dvw> for Length {
  fn from(dvw: Dvw) -> Self {
    Self {
      value: dvw.value,
      unit: dvw.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vmin {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Vmin {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Vmin,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("vmin")
  }
}

impl From<Vmin> for Length {
  fn from(vmin: Vmin) -> Self {
    Self {
      value: vmin.value,
      unit: vmin.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Svmin {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Svmin {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Svmin,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("svmin")
  }
}

impl From<Svmin> for Length {
  fn from(svmin: Svmin) -> Self {
    Self {
      value: svmin.value,
      unit: svmin.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lvmin {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Lvmin {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Lvmin,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("lvmin")
  }
}

impl From<Lvmin> for Length {
  fn from(lvmin: Lvmin) -> Self {
    Self {
      value: lvmin.value,
      unit: lvmin.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dvmin {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Dvmin {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Dvmin,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("dvmin")
  }
}

impl From<Dvmin> for Length {
  fn from(dvmin: Dvmin) -> Self {
    Self {
      value: dvmin.value,
      unit: dvmin.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vmax {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Vmax {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Vmax,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("vmax")
  }
}

impl From<Vmax> for Length {
  fn from(vmax: Vmax) -> Self {
    Self {
      value: vmax.value,
      unit: vmax.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Svmax {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Svmax {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Svmax,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("svmax")
  }
}

impl From<Svmax> for Length {
  fn from(svmax: Svmax) -> Self {
    Self {
      value: svmax.value,
      unit: svmax.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lvmax {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Lvmax {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Lvmax,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("lvmax")
  }
}

impl From<Lvmax> for Length {
  fn from(lvmax: Lvmax) -> Self {
    Self {
      value: lvmax.value,
      unit: lvmax.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dvmax {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Dvmax {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Dvmax,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("dvmax")
  }
}

impl From<Dvmax> for Length {
  fn from(dvmax: Dvmax) -> Self {
    Self {
      value: dvmax.value,
      unit: dvmax.unit,
    }
  }
}

// Container query units
#[derive(Debug, Clone, PartialEq)]
pub struct Cqw {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Cqw {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Cqw,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("cqw")
  }
}

impl From<Cqw> for Length {
  fn from(cqw: Cqw) -> Self {
    Self {
      value: cqw.value,
      unit: cqw.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cqi {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Cqi {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Cqi,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("cqi")
  }
}

impl From<Cqi> for Length {
  fn from(cqi: Cqi) -> Self {
    Self {
      value: cqi.value,
      unit: cqi.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cqh {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Cqh {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Cqh,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("cqh")
  }
}

impl From<Cqh> for Length {
  fn from(cqh: Cqh) -> Self {
    Self {
      value: cqh.value,
      unit: cqh.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cqb {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Cqb {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Cqb,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("cqb")
  }
}

impl From<Cqb> for Length {
  fn from(cqb: Cqb) -> Self {
    Self {
      value: cqb.value,
      unit: cqb.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cqmin {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Cqmin {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Cqmin,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("cqmin")
  }
}

impl From<Cqmin> for Length {
  fn from(cqmin: Cqmin) -> Self {
    Self {
      value: cqmin.value,
      unit: cqmin.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cqmax {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Cqmax {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Cqmax,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("cqmax")
  }
}

impl From<Cqmax> for Length {
  fn from(cqmax: Cqmax) -> Self {
    Self {
      value: cqmax.value,
      unit: cqmax.unit,
    }
  }
}

// Absolute length units
#[derive(Debug, Clone, PartialEq)]
pub struct Px {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Px {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Px,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("px")
  }
}

impl From<Px> for Length {
  fn from(px: Px) -> Self {
    Self {
      value: px.value,
      unit: px.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cm {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Cm {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Cm,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("cm")
  }
}

impl From<Cm> for Length {
  fn from(cm: Cm) -> Self {
    Self {
      value: cm.value,
      unit: cm.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mm {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Mm {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Mm,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("mm")
  }
}

impl From<Mm> for Length {
  fn from(mm: Mm) -> Self {
    Self {
      value: mm.value,
      unit: mm.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct In {
  pub value: f32,
  pub unit: LengthUnit,
}

impl In {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::In,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("in")
  }
}

impl From<In> for Length {
  fn from(inches: In) -> Self {
    Self {
      value: inches.value,
      unit: inches.unit,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pt {
  pub value: f32,
  pub unit: LengthUnit,
}

impl Pt {
  pub fn new(value: f32) -> Self {
    Self {
      value,
      unit: LengthUnit::Pt,
    }
  }

  pub fn parse<'a>() -> Parser<'a, Length> {
    unit("pt")
  }
}

impl From<Pt> for Length {
  fn from(pt: Pt) -> Self {
    Self {
      value: pt.value,
      unit: pt.unit,
    }
  }
}
