#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum ArrayJS {
  Map,
  Filter,
  Join,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum ObjectJS {
  Entries,
  Keys,
  Values,
  FromEntries,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum MathJS {
  Pow,
  Round,
  Ceil,
  Floor,
  Max,
  Min
}
