#[cfg(test)]
mod tests {
  use stylex_css_parser::css_types::{dimension::Dimension, length::Length};

  #[test]
  fn test_px_dimension() {
    let px = Dimension::parse().parse("42px").unwrap();
    let px2 = Dimension::parse().parse("42.0px").unwrap();
    assert_eq!(
      px,
      Dimension::Length(Length::new(42.0, Some("px".to_string())))
    );

    assert_eq!(px, px2, "42px should be equal to 42.0px");
  }

  #[test]
  fn test_em_dimension() {
    let em = Dimension::parse().parse("15em").unwrap();
    let em2 = Dimension::parse().parse("15.0em").unwrap();
    assert_eq!(
      em,
      Dimension::Length(Length::new(15.0, Some("em".to_string())))
    );

    assert_eq!(em, em2, "15em should be equal to 15.0em");
  }

  #[test]
  fn test_rem_dimension() {
    let rem = Dimension::parse().parse("2.0rem").unwrap();
    let rem2 = Dimension::parse().parse("2rem").unwrap();
    assert_eq!(
      rem,
      Dimension::Length(Length::new(2.0, Some("rem".to_string())))
    );

    assert_eq!(rem, rem2, "2rem should be equal to 2.0rem");
  }

  #[test]
  fn test_vh_dimension() {
    let vh = Dimension::parse().parse("100vh").unwrap();
    let vh2 = Dimension::parse().parse("100.0vh").unwrap();
    assert_eq!(
      vh,
      Dimension::Length(Length::new(100.0, Some("vh".to_string())))
    );
    assert_eq!(vh, vh2, "100vh should be equal to 100.0vh");
  }

  #[test]
  fn test_dimension_equality() {
    let dim1 = Dimension::parse().parse("42.0px").unwrap();
    let dim2 = Dimension::parse().parse("42.0px").unwrap();
    let dim3 = Dimension::parse().parse("42.0em").unwrap();
    let dim4 = Dimension::parse().parse("50.0px").unwrap();

    assert_eq!(dim1, dim2);
    assert_ne!(dim1, dim3);
    assert_ne!(dim1, dim4);
  }

  #[test]
  fn test_dimension_display() {
    let px = Dimension::parse().parse("42px").unwrap();

    assert_eq!(format!("{}", px), "42px");
  }

  #[test]
  fn test_invalid_dimension_no_unit() {
    let result = Dimension::parse().parse("42");
    assert!(result.is_err(), "Expected error for dimension without unit");
  }

  #[test]
  fn test_invalid_dimension_empty() {
    let result = Dimension::parse().parse("");
    assert!(result.is_err(), "Expected error for empty dimension string");
  }

  #[test]
  fn test_invalid_dimension_invalid_number() {
    let result = Dimension::parse().parse("not-a-numberpx");
    assert!(
      result.is_err(),
      "Expected error for invalid number in dimension"
    );
  }

  #[test]
  fn test_invalid_dimension_whitespace() {
    let result = Dimension::parse().parse("42 px");
    assert!(
      result.is_err(),
      "Expected error for whitespace between number and unit"
    );
  }
}
