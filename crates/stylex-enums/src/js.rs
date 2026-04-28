#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum ArrayJS {
  Map,
  Filter,
  Join,
}

impl TryFrom<&str> for ArrayJS {
  type Error = ();

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "map" => Ok(ArrayJS::Map),
      "filter" => Ok(ArrayJS::Filter),
      "join" => Ok(ArrayJS::Join),
      _ => Err(()),
    }
  }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum ObjectJS {
  Entries,
  Keys,
  Values,
  FromEntries,
}

impl TryFrom<&str> for ObjectJS {
  type Error = ();

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "entries" => Ok(ObjectJS::Entries),
      "keys" => Ok(ObjectJS::Keys),
      "values" => Ok(ObjectJS::Values),
      "fromEntries" => Ok(ObjectJS::FromEntries),
      _ => Err(()),
    }
  }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum MathJS {
  Pow,
  Round,
  Ceil,
  Floor,
  Max,
  Min,
  Abs,
}

impl TryFrom<&str> for MathJS {
  type Error = ();

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "pow" => Ok(MathJS::Pow),
      "round" => Ok(MathJS::Round),
      "ceil" => Ok(MathJS::Ceil),
      "floor" => Ok(MathJS::Floor),
      "max" => Ok(MathJS::Max),
      "min" => Ok(MathJS::Min),
      "abs" => Ok(MathJS::Abs),
      _ => Err(()),
    }
  }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum StringJS {
  Concat,
  CharCodeAt,
}

impl TryFrom<&str> for StringJS {
  type Error = ();

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "concat" => Ok(StringJS::Concat),
      "charCodeAt" => Ok(StringJS::CharCodeAt),
      _ => Err(()),
    }
  }
}
