use crate::types::type_of;

mod type_of_tests {
  use super::*;

  #[test]
  fn returns_type_name_for_i32() {
    let result = type_of(42_i32);
    assert_eq!(result, "i32");
  }

  #[test]
  fn returns_type_name_for_string() {
    let result = type_of(String::from("hello"));
    assert!(result.contains("String"));
  }

  #[test]
  fn returns_type_name_for_bool() {
    let result = type_of(true);
    assert_eq!(result, "bool");
  }
}
