use stylex_css_parser::css_types::frequency::{Frequency, FrequencyUnit};

#[cfg(test)]
mod tests {
  use stylex_css_parser::base_types::SubString;

  use super::*;

  #[test]
  fn test_parses_css_frequency_type_strings_correctly() {
    assert_eq!(
      Frequency::parse().parse("1Hz").unwrap(),
      Frequency::new(1.0, FrequencyUnit::Hz)
    );
    assert_eq!(
      Frequency::parse().parse("2.5Hz").unwrap(),
      Frequency::new(2.5, FrequencyUnit::Hz)
    );
    assert_eq!(
      Frequency::parse().parse("10KHz").unwrap(),
      Frequency::new(10.0, FrequencyUnit::KHz)
    );
    assert_eq!(
      Frequency::parse().parse("0.01KHz").unwrap(),
      Frequency::new(0.01, FrequencyUnit::KHz)
    );
  }

  #[test]
  fn test_parses_css_frequency_type_substring_correctly() {
    let mut input = SubString::new("1Hz");
    let result = Frequency::parse().run(&mut input).unwrap();
    assert_eq!(result, Frequency::new(1.0, FrequencyUnit::Hz));
    assert_eq!(input.to_string(), "");

    let mut input = SubString::new("2.5Hz foo");
    let result = Frequency::parse().run(&mut input).unwrap();
    assert_eq!(result, Frequency::new(2.5, FrequencyUnit::Hz));
    assert_eq!(input.to_string(), " foo");
  }
}
