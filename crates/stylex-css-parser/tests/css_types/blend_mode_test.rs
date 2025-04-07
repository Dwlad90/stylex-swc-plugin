#[cfg(test)]
mod test_css_type_blend_mode {
  use stylex_css_parser::css_types::blend_mode::BlendMode;

  #[test]
  fn parses_valid_blend_mode_values() {
    assert_eq!(
      BlendMode::parse().parse("normal").unwrap(),
      BlendMode::Normal
    );
    assert_eq!(
      BlendMode::parse().parse("multiply").unwrap(),
      BlendMode::Multiply
    );
    assert_eq!(
      BlendMode::parse().parse("screen").unwrap(),
      BlendMode::Screen
    );
    assert_eq!(
      BlendMode::parse().parse("overlay").unwrap(),
      BlendMode::Overlay
    );
    assert_eq!(
      BlendMode::parse().parse("darken").unwrap(),
      BlendMode::Darken
    );
    assert_eq!(
      BlendMode::parse().parse("lighten").unwrap(),
      BlendMode::Lighten
    );
    assert_eq!(
      BlendMode::parse().parse("color-dodge").unwrap(),
      BlendMode::ColorDodge
    );
    assert_eq!(
      BlendMode::parse().parse("color-burn").unwrap(),
      BlendMode::ColorBurn
    );
    assert_eq!(
      BlendMode::parse().parse("hard-light").unwrap(),
      BlendMode::HardLight
    );
    assert_eq!(
      BlendMode::parse().parse("soft-light").unwrap(),
      BlendMode::SoftLight
    );
    assert_eq!(
      BlendMode::parse().parse("difference").unwrap(),
      BlendMode::Difference
    );
    assert_eq!(
      BlendMode::parse().parse("exclusion").unwrap(),
      BlendMode::Exclusion
    );
    assert_eq!(BlendMode::parse().parse("hue").unwrap(), BlendMode::Hue);
    assert_eq!(
      BlendMode::parse().parse("saturation").unwrap(),
      BlendMode::Saturation
    );
    assert_eq!(BlendMode::parse().parse("color").unwrap(), BlendMode::Color);
    assert_eq!(
      BlendMode::parse().parse("luminosity").unwrap(),
      BlendMode::Luminosity
    );
  }

  // #[test]
  // fn rejects_invalid_blend_mode_values() {
  //   assert!(blend_mode.parse_to_end("invalid").is_err());
  //   assert!(blend_mode.parse_to_end("blend").is_err());
  //   assert!(blend_mode.parse_to_end("123").is_err());
  // }
}
