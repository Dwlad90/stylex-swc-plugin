use stylex_css_parser::css_types::time::{Time, TimeUnit};

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_time_parse_valid_css_time_types() {
    let result = Time::parse().parse_to_end("1s");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Time::new(1.0, TimeUnit::S));

    let result = Time::parse().parse_to_end("1000ms");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Time::new(1000.0, TimeUnit::Ms));

    let result = Time::parse().parse_to_end("0s");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Time::new(0.0, TimeUnit::S));

    let result = Time::parse().parse_to_end("0ms");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Time::new(0.0, TimeUnit::Ms));

    let result = Time::parse().parse_to_end("1.5s");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Time::new(1.5, TimeUnit::S));

    let result = Time::parse().parse_to_end("1.5ms");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Time::new(1.5, TimeUnit::Ms));
  }

  #[test]
  fn test_time_parse_invalid_css_time_types() {
    let result = Time::parse().parse_to_end("1 s");
    assert!(result.is_err());

    let result = Time::parse().parse_to_end("1ms ");
    assert!(result.is_err());

    let result = Time::parse().parse_to_end("1");
    assert!(result.is_err());

    let result = Time::parse().parse_to_end("s");
    assert!(result.is_err());

    let result = Time::parse().parse_to_end("ms");
    assert!(result.is_err());
  }
}
