#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub(crate) enum ArrayJS {
  Map,
  Filter,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub(crate) enum ObjectJS {
  Entries,
  Keys,
  Values,
  FromEntries,
}
