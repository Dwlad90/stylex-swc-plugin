use stylex_css_parser::base_types::SubString;
use stylex_css_parser::css_types::angle::{Angle, Deg, Grad, Rad, Turn};
use stylex_css_parser::css_types::angle_percentage::angle_percentage;
use stylex_css_parser::css_types::common_types::Percentage;

#[test]
fn test_parse_angle_percentage_to_end() {
  assert_eq!(
    Into::<Angle>::into(angle_percentage().parse_to_end("0").unwrap()),
    Angle::new(0.0, None)
  );

  assert_eq!(
    Into::<Deg>::into(angle_percentage().parse_to_end("0deg").unwrap()),
    Deg::new(0.0)
  );

  assert_eq!(
    Into::<Percentage>::into(angle_percentage().parse_to_end("50%").unwrap()),
    Percentage::new(50.0)
  );

  assert_eq!(
    Into::<Deg>::into(angle_percentage().parse_to_end("45deg").unwrap()),
    Deg::new(45.0)
  );

  assert_eq!(
    Into::<Deg>::into(angle_percentage().parse_to_end("90deg").unwrap()),
    Deg::new(90.0)
  );

  assert_eq!(
    Into::<Deg>::into(angle_percentage().parse_to_end("180deg").unwrap()),
    Deg::new(180.0)
  );

  assert_eq!(
    Into::<Deg>::into(angle_percentage().parse_to_end("270deg").unwrap()),
    Deg::new(270.0)
  );

  assert_eq!(
    Into::<Deg>::into(angle_percentage().parse_to_end("-90deg").unwrap()),
    Deg::new(-90.0)
  );

  assert_eq!(
    Into::<Turn>::into(angle_percentage().parse_to_end("0.5turn").unwrap()),
    Turn::new(0.5)
  );

  assert_eq!(
    Into::<Rad>::into(angle_percentage().parse_to_end("2rad").unwrap()),
    Rad::new(2.0)
  );

  assert_eq!(
    Into::<Grad>::into(angle_percentage().parse_to_end("100grad").unwrap()),
    Grad::new(100.0)
  );

  assert_eq!(
    Into::<Deg>::into(angle_percentage().parse_to_end("1.5deg").unwrap()),
    Deg::new(1.5)
  );

  assert!(angle_percentage().parse_to_end("0.75").is_err());
  assert!(angle_percentage().parse_to_end("50% 50%").is_err());
}

#[test]
fn test_parse_angle_percentage_substring() {
  let mut val = SubString::new("0deg");
  assert_eq!(
    Into::<Deg>::into(angle_percentage().run(&mut val).unwrap()),
    Deg::new(0.0)
  );
  assert_eq!(val.to_string(), "");

  let mut val = SubString::new("45deg foo");
  assert_eq!(
    Into::<Deg>::into(angle_percentage().run(&mut val).unwrap()),
    Deg::new(45.0)
  );
  assert_eq!(val.to_string(), " foo");

  let mut val = SubString::new("50% bar");
  assert_eq!(
    Into::<Percentage>::into(angle_percentage().run(&mut val).unwrap()),
    Percentage::new(50.0)
  );
  assert_eq!(val.to_string(), " bar");
}
