#[cfg(test)]
mod digit {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_a_digit() {
    let parser = Parser::digit();

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for digit in digits.iter() {
      assert_eq!(parser.parse(&digit.to_string()).unwrap(), digit.to_string());
    }
  }

  #[test]
  fn fails_to_parse_a_different_string() {
    let parser = Parser::digit();

    let strings = ["foo", "a", "A", "!", " "];

    for strng in strings.iter() {
      parser.parse(strng.as_ref()).unwrap_or_else(|pe| {
        assert_eq!(
          pe.message,
          format!("Expected digit, got {}", strng.chars().next().unwrap())
        );
        String::new()
      });
    }
  }
}
