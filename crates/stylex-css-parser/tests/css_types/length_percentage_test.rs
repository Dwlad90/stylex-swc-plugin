use stylex_css_parser::base_types::SubString;
use stylex_css_parser::css_types::length::{Length, Px, Rem};
use stylex_css_parser::css_types::common_types::Percentage;
use stylex_css_parser::css_types::length_percentage::length_percentage;

#[test]
fn test_parse_length_percentage_to_end() {
  assert_eq!(
    Into::<Length>::into(length_percentage().parse_to_end("0").unwrap()),
    Length::new(0.0, None)
  );

  assert_eq!(
    Into::<Px>::into(length_percentage().parse_to_end("10px").unwrap()),
    Px::new(10.0)
  );

  assert_eq!(
    Into::<Rem>::into(length_percentage().parse_to_end("5rem").unwrap()),
    Rem::new(5.0)
  );

  assert_eq!(
    Into::<Percentage>::into(length_percentage().parse_to_end("50%").unwrap()),
    Percentage::new(50.0)
  );

  assert_eq!(
    Into::<Percentage>::into(length_percentage().parse_to_end("10.5%").unwrap()),
    Percentage::new(10.5)
  );

  assert!(length_percentage().parse_to_end("50% 50%").is_err());
}

#[test]
fn test_parse_length_percentage_substring() {
  let mut val = SubString::new("0");
  assert_eq!(
    Into::<Length>::into(length_percentage().run(&mut val).unwrap()),
    Length::new(0.0, None)
  );
  assert_eq!(val.to_string(), "");

  let mut val = SubString::new("10px foo");
  assert_eq!(
    Into::<Px>::into(length_percentage().run(&mut val).unwrap()),
    Px::new(10.0)
  );
  assert_eq!(val.to_string(), " foo");

  let mut val = SubString::new("50% bar");
  assert_eq!(
    Into::<Percentage>::into(length_percentage().run(&mut val).unwrap()),
    Percentage::new(50.0)
  );
  assert_eq!(val.to_string(), " bar");
}
