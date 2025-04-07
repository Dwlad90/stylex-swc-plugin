use stylex_css_parser::css_types::resolution::{Resolution, ResolutionUnit};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parses_dpi_values() {
    let result = Resolution::parse().parse("300dpi").unwrap();
    let expected = Resolution::new(300.0, ResolutionUnit::Dpi);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_parses_dpcm_values() {
    let result = Resolution::parse().parse("118.11dpcm").unwrap();
    let expected = Resolution::new(118.11, ResolutionUnit::Dpcm);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_parses_dppx_values() {
    let result = Resolution::parse().parse("96dppx").unwrap();
    let expected = Resolution::new(96.0, ResolutionUnit::Dppx);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_rejects_invalid_resolution_values_invalid() {
    assert!(Resolution::parse().parse_to_end("invalid").is_err());
    assert!(Resolution::parse().parse_to_end("10abc").is_err());
    assert!(Resolution::parse().parse_to_end("10").is_err());
  }
}
