use stylex_css_parser::{base_types::SubString, css_types::length::Length};

#[test]
fn test_parses_css_length_types_correctly() {
  assert_eq!(
    Length::parse().parse_to_end("0"),
    Ok(Length::new(0.0, None))
  );
  assert_eq!(
    Length::parse().parse_to_end("10px"),
    Ok(Length::new(10.0, Some("px".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end("5rem"),
    Ok(Length::new(5.0, Some("rem".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end("2.5em"),
    Ok(Length::new(2.5, Some("em".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end("2in"),
    Ok(Length::new(2.0, Some("in".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end("15pt"),
    Ok(Length::new(15.0, Some("pt".to_string())))
  );
}

#[test]
fn test_parses_fractional_length_types_correctly() {
  assert_eq!(
    Length::parse().parse_to_end("0.5px"),
    Ok(Length::new(0.5, Some("px".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end(".5px"),
    Ok(Length::new(0.5, Some("px".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end(".5rem"),
    Ok(Length::new(0.5, Some("rem".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end("0.5em"),
    Ok(Length::new(0.5, Some("em".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end(".2in"),
    Ok(Length::new(0.2, Some("in".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end("1.5pt"),
    Ok(Length::new(1.5, Some("pt".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end("50dvh"),
    Ok(Length::new(50.0, Some("dvh".to_string())))
  );
  assert_eq!(
    Length::parse().parse_to_end("150vw"),
    Ok(Length::new(150.0, Some("vw".to_string())))
  );
}

#[test]
fn test_parses_css_length_types_with_remaining_input() {
  let mut input = SubString::new("0rem");
  let parser = Length::parse();
  let result = parser.run(&mut input).unwrap();
  assert_eq!(result, Length::new(0.0, Some("rem".to_string())));
  assert_eq!(input.to_string(), "");

  let mut input = SubString::new("10px foo");
  let parser = Length::parse();
  let result = parser.run(&mut input).unwrap();
  assert_eq!(result, Length::new(10.0, Some("px".to_string())));
  assert_eq!(input.to_string(), " foo");
}
