#[cfg(test)]
mod integer {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_an_integer() {
    let parser = Parser::<'static, i32>::integer();

    let integers = ["0", "1", "1234567890", "-1", "-1234567890"];

    for digit in integers.iter() {
      assert_eq!(
        parser.parse(digit.as_ref()).unwrap(),
        digit.parse::<i32>().unwrap()
      );
    }
  }

  #[test]
  fn fails_to_parse_a_different_string() {
    let parser = Parser::<'static, i32>::digit();

    let strings = ["foo", ".0", ".", "a", "A", "!", " "];

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
