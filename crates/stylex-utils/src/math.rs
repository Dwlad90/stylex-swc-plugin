/// Rounds a floating-point value to the specified number of decimal places.
///
/// For a single decimal place (`decimal_places == 1`), uses smart rounding
/// that preserves legitimate decimals (e.g. 0.25) while fixing
/// floating-point precision errors (e.g. 0.6000000000000001 → 0.6).
///
/// For other decimal places, always rounds to the specified precision.
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

/// Simple rounding to the given number of decimal places.
///
/// Unlike `round_to_decimal_places`, this always rounds without special
/// handling for single decimal places.
pub fn round_f64(value: f64, decimal_places: u32) -> f64 {
  let multiplier = 10f64.powi(decimal_places as i32);
  (value * multiplier).round() / multiplier
}
