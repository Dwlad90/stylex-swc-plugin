#[cfg(test)]
mod test_number {

  use stylex_css_parser::css_types::number::number;

  #[test]
  fn should_parse_numbers() {
    assert_eq!(number().parse("0").unwrap(), 0.0);
    assert_eq!(number().parse("123").unwrap(), 123.0);
    assert_eq!(number().parse("-123").unwrap(), -123.0);
    assert_eq!(number().parse("123.123").unwrap(), 123.123);
    assert_eq!(number().parse("-123.123").unwrap(), -123.123);
    assert_eq!(number().parse("1two3").unwrap(), 1.0);
  }

  #[test]
  fn should_not_parse_invalid_values() {
    assert!(number().parse("invalid").is_err());
    assert!(number().parse("onetwothree").is_err());
    assert!(number().parse("minus1two3").is_err());
  }
}
