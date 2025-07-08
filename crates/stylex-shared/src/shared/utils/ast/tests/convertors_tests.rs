#[cfg(test)]
mod tests {
  use crate::shared::utils::ast::convertors::string_to_prop_name;

  #[test]
  fn string_to_prop_name_with_quotes() {
    let keys_with_quotes = vec!["2ip", "123", "1b3", "1bc", "2xl", "x*x", "x-x", "x,x"];

    for key in keys_with_quotes {
      assert!(
        string_to_prop_name(key).unwrap().is_str(),
        "Key '{}' should be wrapped in quotes",
        key
      );
    }
  }

  #[test]
  fn string_to_prop_name_without_quotes() {
    let keys_without_quotes = vec![
      "_abc_",
      "_ABC_",
      "$123AB",
      "$abc_",
      "$abc$",
      "$ABC$",
      "$ABC123",
      "abc_",
      "abc",
      "ABC",
      "abc$",
      "break",
      "case",
      "catch",
      "class",
      "const",
      "continue",
      "debugger",
      "default",
      "delete",
      "do",
      "else",
      "export",
      "extends",
      "false",
      "finally",
      "for",
      "function",
      "if",
      "import",
      "in",
      "instanceof",
      "new",
      "null",
      "return",
      "super",
      "switch",
      "this",
      "throw",
      "true",
      "try",
      "typeof",
      "var",
      "void",
      "while",
      "with",
      "x_x",
      "x$x",
      "xl",
    ];

    for key in keys_without_quotes {
      assert!(
        string_to_prop_name(key).unwrap().is_ident(),
        "Key '{}' should not be wrapped in quotes",
        key
      );
    }
  }
}
