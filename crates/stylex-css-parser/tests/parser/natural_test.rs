#[cfg(test)]
mod natural {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_a_natural_number() {
    let parser = Parser::natural();

    let natural_numbers = ["1", "1234567890"];

    for natural_number in natural_numbers.iter() {
      assert_eq!(
        parser.parse(natural_number.as_ref()).unwrap(),
        natural_number.parse::<u32>().unwrap()
      );
    }
  }

  #[test]
  fn fails_to_parse_a_different_string() {
    let parser = Parser::natural();

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
