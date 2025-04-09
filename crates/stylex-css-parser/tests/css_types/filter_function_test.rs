use stylex_css_parser::css_types::{
  angle::Angle,
  filter_function::{
    parse_filter_function, BlurFilterFunction, BrightnessFilterFunction, ContrastFilterFunction,
    GrayscaleFilterFunction, HueRotateFilterFunction, InvertFilterFunction, OpacityFilterFunction,
    SaturateFilterFunction, SepiaFilterFunction,
  },
  length::Length,
};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parses_blur_filter() {
    let input = "blur(5px)";
    let parsed = parse_filter_function().parse(input).unwrap();
    let expected = BlurFilterFunction::new(Length::new(5.0, Some("px".to_string())));

    // Since we can't directly compare BoxedFilterFunction, we compare their string representations
    assert_eq!(parsed.to_string(), expected.to_string());
  }

  #[test]
  fn test_parses_brightness_filter() {
    let input = "brightness(150%)";
    let parsed = parse_filter_function().parse(input).unwrap();
    let expected = BrightnessFilterFunction::new(1.5);

    assert_eq!(parsed.to_string(), expected.to_string());
  }

  #[test]
  fn test_parses_contrast_filter() {
    let input = "contrast(200%)";
    let parsed = parse_filter_function().parse(input).unwrap();
    let expected = ContrastFilterFunction::new(2.0);

    assert_eq!(parsed.to_string(), expected.to_string());
  }

  #[test]
  fn test_parses_grayscale_filter() {
    let input = "grayscale(50%)";
    let parsed = parse_filter_function().parse(input).unwrap();
    let expected = GrayscaleFilterFunction::new(0.5);

    assert_eq!(parsed.to_string(), expected.to_string());
  }

  #[test]
  fn test_parses_hue_rotate_filter() {
    let input = "hue-rotate(90deg)";
    let parsed = parse_filter_function().parse(input).unwrap();
    let expected = HueRotateFilterFunction::new(Angle::new(90.0, Some("deg".to_string())));

    assert_eq!(parsed.to_string(), expected.to_string());
  }

  #[test]
  fn test_parses_invert_filter() {
    let input = "invert(100%)";
    let parsed = parse_filter_function().parse(input).unwrap();
    let expected = InvertFilterFunction::new(1.0);

    assert_eq!(parsed.to_string(), expected.to_string());
  }

  #[test]
  fn test_parses_opacity_filter() {
    let input = "opacity(75%)";
    let parsed = parse_filter_function().parse(input).unwrap();
    let expected = OpacityFilterFunction::new(0.75);

    assert_eq!(parsed.to_string(), expected.to_string());
  }

  #[test]
  fn test_parses_saturate_filter() {
    let input = "saturate(120%)";
    let parsed = parse_filter_function().parse(input).unwrap();
    let expected = SaturateFilterFunction::new(1.2);

    assert_eq!(parsed.to_string(), expected.to_string());
  }

  #[test]
  fn test_parses_sepia_filter() {
    let input = "sepia(30%)";
    let parsed = parse_filter_function().parse(input).unwrap();
    let expected = SepiaFilterFunction::new(0.3);

    assert_eq!(parsed.to_string(), expected.to_string());
  }

  #[test]
  fn rejects_invalid_filter_functions() {
    assert!(parse_filter_function().parse_to_end("blur()").is_err());
    assert!(parse_filter_function().parse_to_end("contrast()").is_err());
    assert!(parse_filter_function()
      .parse_to_end("brightness()")
      .is_err());
    assert!(parse_filter_function().parse_to_end("grayscale()").is_err());
    assert!(parse_filter_function()
      .parse_to_end("hue-rotate()")
      .is_err());
    assert!(parse_filter_function().parse_to_end("invert()").is_err());
    assert!(parse_filter_function().parse_to_end("opacity()").is_err());
    assert!(parse_filter_function().parse_to_end("saturate()").is_err());
    assert!(parse_filter_function().parse_to_end("sepia()").is_err());
    assert!(parse_filter_function()
      .parse_to_end("drop-shadow()")
      .is_err());
    assert!(parse_filter_function().parse_to_end("invalid()").is_err());
    assert!(parse_filter_function()
      .parse_to_end("brightness(abc)")
      .is_err());
  }
}
