#[cfg(test)]
mod float {

  use stylex_css_parser::parser::Parser;

  #[test]
  fn parse_a_float() {
    let parser = Parser::<'static, f32>::float();

    let floats = [
      // "0",
      "0.5",
      // ".5",
      // "1.5",
      // "1",
      // "1.0",
      // "12356.7890",
      // "-1",
      // "-1.0",
      // "-123456.7890",
    ];

    for float in floats.iter() {
      assert_eq!(
        parser.parse(float.as_ref()).unwrap(),
        float.parse::<f32>().unwrap()
      );
    }
  }

  #[test]
  fn fails_to_parse_a_different_string() {
    let parser = Parser::<'static, f32>::whole();

    let strings = [
      "0",
      "0.5",
      ".5",
      "1.5",
      "1",
      "12356.7890",
      "-1",
      "-123456.7890",
    ];

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
