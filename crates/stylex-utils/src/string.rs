use convert_case::{Case, Casing};

pub fn to_kebab_case(string: impl AsRef<str>) -> String {
  let string = string.as_ref();
  string.to_case(Case::Kebab)
}

pub fn to_camel_case(string: impl AsRef<str>) -> String {
  let string = string.as_ref();
  string.to_case(Case::Camel)
}

pub fn to_pascal_case(string: impl AsRef<str>) -> String {
  let string = string.as_ref();
  string.to_case(Case::Pascal)
}

pub fn to_snake_case(string: impl AsRef<str>) -> String {
  let string = string.as_ref();
  string.to_case(Case::Snake)
}

pub fn to_title_case(string: impl AsRef<str>) -> String {
  let string = string.as_ref();
  string.to_case(Case::Title)
}
