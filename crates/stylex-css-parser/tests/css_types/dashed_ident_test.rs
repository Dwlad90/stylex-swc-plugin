use stylex_css_parser::css_types::dashed_ident::DashedIdentifier;

#[test]
fn test_parses_css_dashed_ident_types_strings_correctly() {
  assert_eq!(
    DashedIdentifier::parse().parse("--primary-color").unwrap(),
    DashedIdentifier::new("--primary-color".to_string())
  );
  assert_eq!(
    DashedIdentifier::parse()
      .parse("--secondary-color")
      .unwrap(),
    DashedIdentifier::new("--secondary-color".to_string())
  );
  assert_eq!(
    DashedIdentifier::parse()
      .parse("--_tertiary-color")
      .unwrap(),
    DashedIdentifier::new("--_tertiary-color".to_string())
  );
  assert_eq!(
    DashedIdentifier::parse()
      .parse("--_tertiary-color-")
      .unwrap(),
    DashedIdentifier::new("--_tertiary-color-".to_string())
  );
  assert_eq!(
    DashedIdentifier::parse().parse("--_1").unwrap(),
    DashedIdentifier::new("--_1".to_string())
  );
  assert_eq!(
    DashedIdentifier::parse().parse("--_1\\.1").unwrap(),
    DashedIdentifier::new("--_1\\.1".to_string())
  );
}

#[test]
fn test_fails_to_parse_invalid_css_dashed_ident_types_strings() {
  assert!(DashedIdentifier::parse().parse("-_1").is_err());
  assert!(DashedIdentifier::parse().parse("--").is_err());
  assert!(DashedIdentifier::parse().parse("1").is_err());
  assert!(DashedIdentifier::parse().parse("1-").is_err());
  assert!(DashedIdentifier::parse().parse("1-2").is_err());
  assert!(DashedIdentifier::parse().parse("1-2-").is_err());
}
