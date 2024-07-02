#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum ArrayJS {
  Map,
  Filter,
  Join,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum ObjectJS {
  Entries,
  Keys,
  Values,
  FromEntries,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum MathJS {
  Pow,
  Round,
  Ceil,
  Floor,
  Max,
  Min
}


#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum StringJS {
  Concat,
  CharCodeAt
}
