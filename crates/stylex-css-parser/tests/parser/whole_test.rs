#[cfg(test)]
mod whole {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_a_natural_number() {
    let parser = Parser::<'static, String>::whole();

    let wholes = ["0", "1", "1234567890"];

    for whole in wholes.iter() {
      assert_eq!(
        parser.parse(whole.as_ref()).unwrap(),
        whole.parse::<i32>().unwrap()
      );
    }
  }

  #[test]
  fn fails_to_parse_a_different_string() {
    let parser = Parser::<'static, String>::whole();

    let strings = ["foo", ".0", ".", "-1", "-1234567890"];

    for strng in strings.iter() {
      parser.parse(strng.as_ref()).unwrap_or_else(|pe| {
        assert_eq!(
          pe.message,
          format!("Expected digit, got {}", strng.chars().next().unwrap())
        );
        0
      });
    }
  }
}
