#[cfg(test)]
mod test_css_type_blend_mode {

  use stylex_css_parser::css_types::blend_mode::blend_mode;

  #[test]
  fn parses_valid_blend_mode_values() {
    assert_eq!(blend_mode().parse("normal").unwrap(), "normal");
    assert_eq!(blend_mode().parse("multiply").unwrap(), "multiply");
    assert_eq!(blend_mode().parse("screen").unwrap(), "screen");
    assert_eq!(blend_mode().parse("overlay").unwrap(), "overlay");
    assert_eq!(blend_mode().parse("darken").unwrap(), "darken");
    assert_eq!(blend_mode().parse("lighten").unwrap(), "lighten");
    assert_eq!(blend_mode().parse("color-dodge").unwrap(), "color-dodge");
    assert_eq!(blend_mode().parse("color-burn").unwrap(), "color-burn");
    assert_eq!(blend_mode().parse("hard-light").unwrap(), "hard-light");
    assert_eq!(blend_mode().parse("soft-light").unwrap(), "soft-light");
    assert_eq!(blend_mode().parse("difference").unwrap(), "difference");
    assert_eq!(blend_mode().parse("exclusion").unwrap(), "exclusion");
    assert_eq!(blend_mode().parse("hue").unwrap(), "hue");
    assert_eq!(blend_mode().parse("saturation").unwrap(), "saturation");
    assert_eq!(blend_mode().parse("color").unwrap(), "color");
    assert_eq!(blend_mode().parse("luminosity").unwrap(), "luminosity");
  }

  // #[test]
  // fn rejects_invalid_blend_mode_values() {
  //   assert!(blend_mode.parse_to_end("invalid").is_err());
  //   assert!(blend_mode.parse_to_end("blend").is_err());
  //   assert!(blend_mode.parse_to_end("123").is_err());
  // }
}
