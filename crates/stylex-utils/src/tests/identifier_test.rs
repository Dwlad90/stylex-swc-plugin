use crate::identifier::gen_file_based_identifier;

mod gen_file_based_identifier_tests {
  use super::*;

  #[test]
  fn generates_identifier_without_key() {
    let result = gen_file_based_identifier("file.js", "styles", None);
    assert_eq!(result, "file.js//styles");
  }

  #[test]
  fn generates_identifier_with_key() {
    let result = gen_file_based_identifier("file.js", "styles", Some("color"));
    assert_eq!(result, "file.js//styles.color");
  }

  #[test]
  fn handles_empty_file_name() {
    let result = gen_file_based_identifier("", "export", None);
    assert_eq!(result, "//export");
  }

  #[test]
  fn handles_empty_export_name() {
    let result = gen_file_based_identifier("file.js", "", None);
    assert_eq!(result, "file.js//");
  }
}
