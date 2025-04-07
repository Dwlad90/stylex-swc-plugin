use crate::parser::Parser;
use anyhow::bail;
use std::fmt;
use std::{convert::TryFrom, fmt::Display};

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

impl LengthUnit {
  fn as_str(&self) -> &'static str {
    match self {
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
    }
  }
}

impl fmt::Display for LengthUnit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

impl std::str::FromStr for LengthUnit {
  type Err = anyhow::Error;

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
      _ => bail!("Invalid length unit: {}", s),
    }
  }
}

impl From<LengthUnit> for String {
  fn from(unit: LengthUnit) -> Self {
    unit.as_str().to_string()
  }
}

impl TryFrom<String> for LengthUnit {
  type Error = anyhow::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    value.parse()
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

impl fmt::Display for LengthBasedOnFont {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Length> for String {
  fn from(length: Length) -> Self {
    length.to_string()
  }
}

impl From<String> for Length {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Length::parse()
      .run(&mut input)
      .expect("Failed to parse length")
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

impl fmt::Display for LengthBasedOnViewport {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
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

impl Display for Cap {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Cap> for String {
  fn from(val: Cap) -> Self {
    val.to_string()
  }
}

impl From<Length> for Cap {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl From<String> for Cap {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Cap::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
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

impl Display for Ch {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Ch> for String {
  fn from(val: Ch) -> Self {
    val.to_string()
  }
}

impl From<Length> for Ch {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl From<String> for Ch {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Ch::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
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

impl Display for Em {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Em> for String {
  fn from(val: Em) -> Self {
    val.to_string()
  }
}

impl From<Length> for Em {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl From<String> for Em {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Em::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
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

impl Display for Ex {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Ex> for String {
  fn from(val: Ex) -> Self {
    val.to_string()
  }
}

impl From<String> for Ex {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Ex::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
  }
}

impl From<Length> for Ex {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
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

impl Display for Ic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Ic> for String {
  fn from(val: Ic) -> Self {
    val.to_string()
  }
}

impl From<Length> for Ic {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl From<String> for Ic {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Ic::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
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

impl Display for Lh {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Lh> for String {
  fn from(val: Lh) -> Self {
    val.to_string()
  }
}

impl From<Length> for Lh {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl From<String> for Lh {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Lh::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
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

impl Display for Rem {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Rem> for String {
  fn from(val: Rem) -> Self {
    val.to_string()
  }
}

impl From<Length> for Rem {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl From<String> for Rem {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Rem::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
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

impl Display for Rlh {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Rlh> for String {
  fn from(val: Rlh) -> Self {
    val.to_string()
  }
}

impl From<String> for Rlh {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Rlh::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
  }
}

impl From<Length> for Rlh {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
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

impl Display for Vh {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Vh> for String {
  fn from(val: Vh) -> Self {
    val.to_string()
  }
}

impl From<Length> for Vh {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl From<String> for Vh {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Vh::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
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

impl Display for Svh {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Svh> for String {
  fn from(val: Svh) -> Self {
    val.to_string()
  }
}

impl From<String> for Svh {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Svh::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
  }
}

impl From<Length> for Svh {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
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

impl Display for Lvh {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Lvh> for String {
  fn from(val: Lvh) -> Self {
    val.to_string()
  }
}

impl From<String> for Lvh {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Lvh::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
  }
}

impl From<Length> for Lvh {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
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

impl Display for Dvh {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Dvh> for String {
  fn from(val: Dvh) -> Self {
    val.to_string()
  }
}

impl From<String> for Dvh {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Dvh::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
  }
}

impl From<Length> for Dvh {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
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

impl Display for Vw {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Vw> for String {
  fn from(val: Vw) -> Self {
    val.to_string()
  }
}

impl From<Length> for Vw {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
  }
}

impl From<String> for Vw {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Vw::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
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

impl Display for Svw {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Svw> for String {
  fn from(val: Svw) -> Self {
    val.to_string()
  }
}

impl From<String> for Svw {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Svw::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
  }
}

impl From<Length> for Svw {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
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

impl Display for Lvw {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Lvw> for String {
  fn from(val: Lvw) -> Self {
    val.to_string()
  }
}

impl From<String> for Lvw {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Lvw::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
  }
}

impl From<Length> for Lvw {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
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

impl Display for Vmin {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Vmin> for String {
  fn from(val: Vmin) -> Self {
    val.to_string()
  }
}

impl From<String> for Vmin {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Vmin::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
  }
}

impl From<Length> for Vmin {
  fn from(vmin: Length) -> Self {
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

impl Display for Vmax {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Vmax> for String {
  fn from(val: Vmax) -> Self {
    val.to_string()
  }
}

impl From<String> for Vmax {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Vmax::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
  }
}

impl From<Length> for Vmax {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
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

impl Display for Px {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

impl From<Px> for String {
  fn from(val: Px) -> Self {
    val.to_string()
  }
}

impl From<String> for Px {
  fn from(s: String) -> Self {
    let mut input = crate::base_types::SubString::new(&s);
    Px::parse()
      .run(&mut input)
      .expect("Failed to parse length")
      .into()
  }
}

impl From<Length> for Px {
  fn from(length: Length) -> Self {
    Self {
      value: length.value,
      unit: length.unit,
    }
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
