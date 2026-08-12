/// Rounds like JS `Math.round`: to the nearest integer, and on an exact tie
/// towards positive infinity.
///
/// `f64::round` breaks ties away from zero instead, so the two disagree on
/// every negative half — `Math.round(-0.5)` is `0`, `(-0.5f64).round()` is
/// `-1`. Correcting the tie afterwards keeps `f64::round`'s exactness for
/// everything else, which a naive `(value + 0.5).floor()` would lose:
/// `0.49999999999999994 + 0.5` rounds up to `1.0` before the floor ever runs.
///
/// A tie that lands on zero yields `+0` where JS yields `-0`; the two compare
/// equal and render identically as `"0"`.
pub fn js_math_round(value: f64) -> f64 {
  let rounded = value.round();

  // Only an exact negative tie lands on the wrong side of `value`.
  if value - rounded == 0.5 {
    rounded + 1.0
  } else {
    rounded
  }
}

/// Rounds a floating-point value to the specified number of decimal places.
///
/// For a single decimal place (`decimal_places == 1`), uses smart rounding
/// that preserves legitimate decimals (e.g. 0.25) while fixing
/// floating-point precision errors (e.g. 0.6000000000000001 → 0.6).
///
/// For other decimal places, always rounds to the specified precision.
pub fn round_to_decimal_places(value: f64, decimal_places: u32) -> f64 {
  let multiplier = 10_f64.powi(decimal_places as i32);
  let rounded = js_math_round(value * multiplier) / multiplier;

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

#[cfg(test)]
#[path = "tests/math_test.rs"]
mod tests;
