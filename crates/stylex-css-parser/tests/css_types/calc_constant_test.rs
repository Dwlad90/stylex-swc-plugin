#[cfg(test)]
mod test_calc_constant {
  use stylex_css_parser::css_types::calc_constant::CalcConstant;

  #[test]
  fn should_parse_valid_calc_constants() {
    assert_eq!(CalcConstant::parse().parse("pi").unwrap(), CalcConstant::Pi);
    assert_eq!(CalcConstant::parse().parse("e").unwrap(), CalcConstant::E);
    assert_eq!(
      CalcConstant::parse().parse("infinity").unwrap(),
      CalcConstant::Infinity
    );
    assert_eq!(
      CalcConstant::parse().parse("-infinity").unwrap(),
      CalcConstant::NegativeInfinity
    );
    assert_eq!(
      CalcConstant::parse().parse("NaN").unwrap(),
      CalcConstant::NaN
    );
  }

  #[test]
  fn should_not_parse_invalid_calc_constants() {
    assert!(CalcConstant::parse().parse("invalid").is_err());
    assert!(CalcConstant::parse().parse("123").is_err());
  }
}
