use super::*;

#[test]
fn validate_import_path_accepts_valid_npm_names() {
  assert!(validate_import_path("@scope/pkg-name").is_ok());
  assert!(validate_import_path("stylex").is_ok());
}

#[test]
fn validate_import_path_rejects_too_long_values() {
  let long_path = "a".repeat(MAX_IMPORT_PATH_LENGTH + 1);
  let error = validate_import_path(&long_path).unwrap_err();
  assert!(error.contains("too long"));
}

#[test]
fn validate_import_path_rejects_invalid_pattern() {
  let error = validate_import_path("Invalid Package Name!").unwrap_err();
  assert!(error.contains("required pattern"));
}

#[test]
fn validate_import_path_accepts_scoped_packages() {
  assert!(validate_import_path("@stylexjs/stylex").is_ok());
  assert!(validate_import_path("@scope/sub-pkg").is_ok());
}

#[test]
fn validate_import_path_accepts_short_names() {
  assert!(validate_import_path("a").is_ok());
  assert!(validate_import_path("ab").is_ok());
}

#[test]
fn validate_import_path_at_max_length_boundary() {
  let exactly_max = "a".repeat(MAX_IMPORT_PATH_LENGTH);
  assert!(validate_import_path(&exactly_max).is_ok());

  let over_max = "a".repeat(MAX_IMPORT_PATH_LENGTH + 1);
  assert!(validate_import_path(&over_max).is_err());
}

#[test]
fn validate_import_path_rejects_empty_string() {
  assert!(validate_import_path("").is_err());
}

#[test]
fn validate_import_path_rejects_dot_path() {
  assert!(validate_import_path("..").is_err());
}

#[test]
fn validate_import_path_rejects_uppercase_start() {
  assert!(validate_import_path("UpperCase").is_err());
}
