//! Tests for the debug class names a dev build adds to each style namespace.

use super::{dev_class_name, dev_class_name_prefix, file_basename};

/// Shorthand for the name built from a namespace, a variable name and a file.
fn name(namespace: &str, var_name: Option<&str>, filename: &str) -> String {
  let prefix = dev_class_name_prefix(&var_name.map(str::to_owned), filename);

  dev_class_name(&prefix, namespace)
}

#[test]
fn names_a_namespace_after_its_file_and_variable() {
  assert_eq!(
    name("root", Some("styles"), "MyComponent"),
    "MyComponent__styles.root"
  );
}

#[test]
fn leaves_out_the_variable_when_there_is_none() {
  assert_eq!(name("root", None, "MyComponent"), "MyComponent__root");
}

/// A class name holds one class. A space in the name would make it two, and the
/// first of the two is the name of another namespace of the same variable.
#[test]
fn removes_a_character_that_a_class_name_cannot_hold() {
  assert_eq!(name("p+1", Some("c"), "input"), "input__c.p1");
  assert_eq!(name("a b+c!d", Some("c"), "input"), "input__c.abcd");
}

#[test]
fn keeps_the_characters_a_class_name_can_hold() {
  assert_eq!(name("a-b_c.d", None, "input"), "input__a-b_c.d");
}

/// Two namespaces that differ only in a character no class name can hold get
/// the same debug name. The name is a label for a person to read, so a
/// duplicate is acceptable; a space, which silently adds a second class, is
/// not.
#[test]
fn two_namespaces_can_share_a_name() {
  assert_eq!(
    name("p+1", Some("c"), "input"),
    name("p1", Some("c"), "input")
  );
}

#[test]
fn keeps_only_the_first_part_of_a_file_name_with_several_dots() {
  assert_eq!(name("root", Some("s"), "input.stylex"), "input__s.root");
  assert_eq!(name("root", Some("s"), "a.b.c"), "a__s.root");
}

#[test]
fn reads_the_file_name_out_of_a_path() {
  assert_eq!(
    name("root", Some("s"), "src/components/Card.tsx"),
    "Card__s.root"
  );
}

/// The compiler does not always know the file. The name still has to say which
/// part is the file, or every file gives the same name.
#[test]
fn names_an_unknown_file() {
  assert_eq!(name("base", Some("styles"), ""), "UnknownFile__styles.base");
  assert_eq!(name("base", None, "."), "UnknownFile__base");
  assert_eq!(name("base", None, "/"), "UnknownFile__base");
}

/// Every character of the namespace can be one a class name cannot hold. The
/// name then says the file and the variable and nothing more.
#[test]
fn accepts_a_namespace_that_is_left_empty() {
  assert_eq!(name("+++", Some("c"), "input"), "input__c.");
}

/// A class name holds no letter outside the ASCII range, so an accented letter
/// goes and the plain letters beside it stay.
#[test]
fn removes_the_accented_letters_of_a_namespace() {
  assert_eq!(name("ünïcödé", None, "input"), "input__ncd");
}

#[test]
fn accepts_a_very_long_namespace() {
  let namespace = "a".repeat(100_000);

  assert_eq!(
    name(&namespace, Some("c"), "input").len(),
    "input__c.".len() + 100_000
  );
}

/// Every character is one to remove, so the work is a full rewrite of a large
/// string rather than a copy of it.
#[test]
fn accepts_a_very_long_namespace_that_is_left_empty() {
  let namespace = "+".repeat(100_000);

  assert_eq!(name(&namespace, Some("c"), "input"), "input__c.");
}

#[test]
fn reads_a_base_name_that_needs_no_change() {
  assert_eq!(file_basename("MyComponent"), "MyComponent");
}
