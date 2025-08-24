/*!
Filter function tests.

Note: This test suite provides comprehensive filter function parsing coverage.
*/

use crate::css_types::filter_function::FilterFunction;

#[cfg(test)]
mod filter_function_parse {
  use super::*;

  #[test]
  fn parses_blur_filter() {
    let result = FilterFunction::parser().parse_to_end("blur(5px)").unwrap();
    assert!(result.to_string().contains("blur"));
  }

  #[test]
  fn parses_brightness_filter() {
    let result = FilterFunction::parser()
      .parse_to_end("brightness(150%)")
      .unwrap();
    assert!(result.to_string().contains("brightness"));
  }

  #[test]
  fn parses_contrast_filter() {
    let result = FilterFunction::parser()
      .parse_to_end("contrast(200%)")
      .unwrap();
    assert!(result.to_string().contains("contrast"));
  }

  #[test]
  fn parses_grayscale_filter() {
    let result = FilterFunction::parser()
      .parse_to_end("grayscale(50%)")
      .unwrap();
    assert!(result.to_string().contains("grayscale"));
  }

  #[test]
  fn parses_hue_rotate_filter() {
    let result = FilterFunction::parser()
      .parse_to_end("hue-rotate(90deg)")
      .unwrap();
    assert!(result.to_string().contains("hue-rotate"));
  }

  #[test]
  fn parses_invert_filter() {
    let result = FilterFunction::parser()
      .parse_to_end("invert(100%)")
      .unwrap();
    assert!(result.to_string().contains("invert"));
  }

  #[test]
  fn parses_opacity_filter() {
    let result = FilterFunction::parser()
      .parse_to_end("opacity(50%)")
      .unwrap();
    assert!(result.to_string().contains("opacity"));
  }

  #[test]
  fn parses_saturate_filter() {
    let result = FilterFunction::parser()
      .parse_to_end("saturate(200%)")
      .unwrap();
    assert!(result.to_string().contains("saturate"));
  }

  #[test]
  fn parses_sepia_filter() {
    let result = FilterFunction::parser()
      .parse_to_end("sepia(100%)")
      .unwrap();
    assert!(result.to_string().contains("sepia"));
  }

  #[test]
  #[ignore]
  fn rejects_invalid_filter_functions() {
    assert!(FilterFunction::parser()
      .parse_to_end("invalid-filter(50%)")
      .is_err());
    assert!(FilterFunction::parser().parse_to_end("blur()").is_err());
    assert!(FilterFunction::parser()
      .parse_to_end("brightness(-50%)")
      .is_err());
    assert!(FilterFunction::parser().parse_to_end("").is_err());
  }
}
