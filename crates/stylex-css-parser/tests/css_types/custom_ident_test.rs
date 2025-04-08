use stylex_css_parser::base_types::SubString;
use stylex_css_parser::css_types::custom_ident::CustomIdentifier;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parses_css_custom_ident_types_strings_correctly() {
    assert_eq!(
      CustomIdentifier::parse().parse("foo").unwrap(),
      CustomIdentifier::new("foo".to_string())
    );
    assert_eq!(
      CustomIdentifier::parse().parse("nono79").unwrap(),
      CustomIdentifier::new("nono79".to_string())
    );
    assert_eq!(
      CustomIdentifier::parse().parse("ground-level").unwrap(),
      CustomIdentifier::new("ground-level".to_string())
    );
    assert_eq!(
      CustomIdentifier::parse().parse("-test").unwrap(),
      CustomIdentifier::new("-test".to_string())
    );
    assert_eq!(
      CustomIdentifier::parse().parse("_internal").unwrap(),
      CustomIdentifier::new("_internal".to_string())
    );
    assert_eq!(
      CustomIdentifier::parse().parse("\\22 toto").unwrap(),
      CustomIdentifier::new("\\22 toto".to_string())
    );
    assert_eq!(
      CustomIdentifier::parse().parse("bili\\.bob").unwrap(),
      CustomIdentifier::new("bili\\.bob".to_string())
    );
  }

  #[test]
  fn test_fails_to_parse_invalid_css_custom_ident_types_strings() {
    assert!(CustomIdentifier::parse().parse("34rem").is_err());
    assert!(CustomIdentifier::parse().parse("-12rad").is_err());

    let mut substring = SubString::new("bili.bob");
    let result = CustomIdentifier::parse().run(&mut substring);
    assert_eq!(result.unwrap(), CustomIdentifier::new("bili".to_string()));
    assert_eq!(substring.to_string(), ".bob");

    let mut substring = SubString::new("--toto");
    assert!(CustomIdentifier::parse().run(&mut substring).is_err());
    assert_eq!(substring.to_string(), "--toto");

    let mut substring = SubString::new("'bilibob'");
    assert!(CustomIdentifier::parse().run(&mut substring).is_err());
    assert_eq!(substring.to_string(), "'bilibob'");

    let mut substring = SubString::new("\"bilibob\"");
    assert!(CustomIdentifier::parse().run(&mut substring).is_err());
    assert_eq!(substring.to_string(), "\"bilibob\"");
  }
}
