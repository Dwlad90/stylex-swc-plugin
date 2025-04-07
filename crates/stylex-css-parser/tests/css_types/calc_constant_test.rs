#[cfg(test)]
mod test_calc_constant {

  use stylex_css_parser::css_types::calc_constant::calc_constant;

  #[test]
  fn should_parse_valid_calc_constants() {
    assert_eq!(calc_constant().parse("pi").unwrap(), "pi");
    assert_eq!(calc_constant().parse("e").unwrap(), "e");
    assert_eq!(calc_constant().parse("infinity").unwrap(), "infinity");
    assert_eq!(calc_constant().parse("-infinity").unwrap(), "-infinity");
    assert_eq!(calc_constant().parse("NaN").unwrap(), "NaN");
  }

  #[test]
  fn should_not_parse_invalid_calc_constants() {
    assert!(calc_constant().parse("invalid").is_err());
    assert!(calc_constant().parse("123").is_err());
  }
}
