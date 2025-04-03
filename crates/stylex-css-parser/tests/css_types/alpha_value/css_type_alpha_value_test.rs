#[cfg(test)]
mod number_fractions {
  use stylex_css_parser::css_types::alpha_value::AlphaValue;

  #[test]
  fn decimal_point_five() {
    assert_eq!(
      AlphaValue::parse().parse("0.5").unwrap(),
      AlphaValue::new(0.5)
    );
  }

  #[test]
  fn point_five() {
    assert_eq!(
      AlphaValue::parse().parse(".5").unwrap(),
      AlphaValue::new(0.5)
    );
  }

  #[test]
  fn decimal_point_twenty_five() {
    assert_eq!(
      AlphaValue::parse().parse("0.25").unwrap(),
      AlphaValue::new(0.25)
    );
  }

  #[test]
  fn point_twenty_five() {
    assert_eq!(
      AlphaValue::parse().parse(".25").unwrap(),
      AlphaValue::new(0.25)
    );
  }

  #[test]
  fn decimal_point_seventy_five() {
    assert_eq!(
      AlphaValue::parse().parse("0.75").unwrap(),
      AlphaValue::new(0.75)
    );
  }

  #[test]
  fn point_seventy_five() {
    assert_eq!(
      AlphaValue::parse().parse(".75").unwrap(),
      AlphaValue::new(0.75)
    );
  }

  #[test]
  fn one() {
    assert_eq!(
      AlphaValue::parse().parse("1").unwrap(),
      AlphaValue::new(1.0)
    );
  }

  #[test]
  fn parses_decimal_alpha_values() {
    // Test multiple values in one test for efficiency
    assert_eq!(
      AlphaValue::parse().parse("0").unwrap(),
      AlphaValue::new(0.0)
    );
    assert_eq!(
      AlphaValue::parse().parse("0.25").unwrap(),
      AlphaValue::new(0.25)
    );
    assert_eq!(
      AlphaValue::parse().parse("0.5").unwrap(),
      AlphaValue::new(0.5)
    );
    assert_eq!(
      AlphaValue::parse().parse("1").unwrap(),
      AlphaValue::new(1.0)
    );
  }
}

#[cfg(test)]
mod percentage {
  use stylex_css_parser::css_types::alpha_value::AlphaValue;

  #[test]
  fn fifty_percent() {
    assert_eq!(
      AlphaValue::parse().parse("50%").unwrap(),
      AlphaValue::new(0.5)
    );
  }

  #[test]
  fn twenty_five_percent() {
    assert_eq!(
      AlphaValue::parse().parse("25%").unwrap(),
      AlphaValue::new(0.25)
    );
  }

  #[test]
  fn seventy_five_percent() {
    assert_eq!(
      AlphaValue::parse().parse("75%").unwrap(),
      AlphaValue::new(0.75)
    );
  }

  #[test]
  fn zero_percent() {
    assert_eq!(
      AlphaValue::parse().parse("0%").unwrap(),
      AlphaValue::new(0.0)
    );
  }

  #[test]
  fn hundred_percent() {
    assert_eq!(
      AlphaValue::parse().parse("100%").unwrap(),
      AlphaValue::new(1.0)
    );
  }

  #[test]
  fn parses_percentage_alpha_values() {
    assert_eq!(
      AlphaValue::parse().parse("0%").unwrap(),
      AlphaValue::new(0.0)
    );
    assert_eq!(
      AlphaValue::parse().parse("25%").unwrap(),
      AlphaValue::new(0.25)
    );
    assert_eq!(
      AlphaValue::parse().parse("50%").unwrap(),
      AlphaValue::new(0.5)
    );
    assert_eq!(
      AlphaValue::parse().parse("100%").unwrap(),
      AlphaValue::new(1.0)
    );

    assert_eq!(
      AlphaValue::parse().parse("-50%").unwrap(),
      AlphaValue::new(-0.5)
    );

    assert_eq!(
      AlphaValue::parse().parse("150%").unwrap(),
      AlphaValue::new(1.5)
    );
  }
}

#[cfg(test)]
mod invalid_values {
  use stylex_css_parser::css_types::alpha_value::AlphaValue;

  #[test]
  fn rejects_negative_number() {
    assert!(AlphaValue::parse().parse("-0.5").is_err());
  }

  #[test]
  fn rejects_greater_than_one() {
    assert!(AlphaValue::parse().parse("1.5").is_err());
  }
}
