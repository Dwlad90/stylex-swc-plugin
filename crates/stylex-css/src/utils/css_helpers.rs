use stylex_regex::regex::DASHIFY_REGEX;

/// Converts a camelCase string to kebab-case by inserting hyphens before
/// uppercase letters and lowercasing the result.
pub fn dashify(s: &str) -> String {
  DASHIFY_REGEX.replace_all(s, "-$1").to_lowercase()
}

/// Rounds a floating-point value to the specified number of decimal places.
///
/// For a single decimal place (`decimal_places == 1`), uses smart rounding
/// that preserves legitimate decimals (e.g. 0.25) while fixing
/// floating-point precision errors.
///
/// # Examples
/// ```ignore
/// round_to_decimal_places(0.6000000000000001, 1) // → 0.6
/// round_to_decimal_places(0.25, 1)               // → 0.25 (preserved)
///
/// round_to_decimal_places(33.333333333333336, 4) // → 33.3333
/// round_to_decimal_places(10.0, 4)               // → 10.0
/// ```
pub fn round_to_decimal_places(value: f64, decimal_places: u32) -> f64 {
  let multiplier = 10_f64.powi(decimal_places as i32);
  let rounded = (value * multiplier).round() / multiplier;

  // For single decimal place (priorities), use smart rounding that preserves
  // legitimate decimals like 0.25 while fixing precision errors
  if decimal_places == 1 {
    let diff = (value - rounded).abs();
    // If difference is within floating-point error tolerance, use rounded value
    // Otherwise, keep the original to preserve values like 0.25
    if diff < 1e-10 { rounded } else { value }
  } else {
    // For other decimal places, always round
    rounded
  }
}
