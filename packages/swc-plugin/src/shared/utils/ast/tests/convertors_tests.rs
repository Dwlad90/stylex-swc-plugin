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
      "abc", "abc$", "abc_", "_abc_", "$abc_", "$abc$", "ABC", "_ABC_", "$ABC$", "$ABC123",
      "$123AB", "xl", "if", "else", "for", "while", "do", "switch", "case", "default", "break",
      "x_x", "x$x",
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
