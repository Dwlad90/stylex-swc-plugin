use convert_case::{Case, Casing};

pub fn to_kebab_case(string: &str) -> String {
  string.to_case(Case::Kebab)
}

pub fn to_camel_case(string: &str) -> String {
  string.to_case(Case::Camel)
}

pub fn to_pascal_case(string: &str) -> String {
  string.to_case(Case::Pascal)
}

pub fn to_snake_case(string: &str) -> String {
  string.to_case(Case::Snake)
}

pub fn to_title_case(string: &str) -> String {
  string.to_case(Case::Title)
}
