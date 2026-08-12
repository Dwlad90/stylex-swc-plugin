/// Renders an `f64` exactly as JS `String(Number)` does.
///
/// Rust's `f64` `Display` is not a substitute: it never switches to
/// exponential form, so `1e21` would render as `"1000000000000000000000"` where
/// JS renders `"1e+21"`. Since this rendering reaches both generated code and
/// the class-name hash, the spelling itself is observable and has to match, not
/// merely round-trip to the same `f64`.
pub fn to_js_string(value: f64) -> String {
  if value.is_nan() {
    return "NaN".to_string();
  }
  if value.is_infinite() {
    return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
  }
  if value == 0.0 {
    // Covers `-0`, which JS also renders as `"0"`.
    return "0".to_string();
  }

  let mut result = String::with_capacity(24);

  if value < 0.0 {
    result.push('-');
  }

  // `s` and `n` are ECMA-262's `Number::toString` variables: `s` is the shortest
  // digit string that round-trips, and the value is `s × 10^(n - k)` where `k`
  // is the digit count. Rust's `LowerExp` emits both, as `d.ddde±x`.
  let (s, n) = shortest_digits_and_exponent(value.abs());
  let k = s.len() as i32;

  if k <= n && n <= 21 {
    result.push_str(&s);
    for _ in 0..(n - k) {
      result.push('0');
    }
  } else if 0 < n && n <= 21 {
    let (integral, fractional) = s.split_at(n as usize);
    result.push_str(integral);
    result.push('.');
    result.push_str(fractional);
  } else if -6 < n && n <= 0 {
    result.push_str("0.");
    for _ in 0..(-n) {
      result.push('0');
    }
    result.push_str(&s);
  } else {
    let (first, rest) = s.split_at(1);
    result.push_str(first);
    if !rest.is_empty() {
      result.push('.');
      result.push_str(rest);
    }
    result.push('e');
    result.push(if n >= 1 { '+' } else { '-' });
    result.push_str(&(n - 1).abs().to_string());
  }

  result
}

/// Decomposes a finite, strictly positive `f64` into ECMA-262's `s` (the
/// shortest round-tripping digit string) and `n` (the decimal exponent, such
/// that the value is `s × 10^(n - s.len())`).
///
/// Rust's `LowerExp` already picks the same shortest digits as JS, so this only
/// has to re-read them out of `d.ddde±x`. The scan is hand-rolled rather than
/// `split_once` + `parse` so that every branch is reachable for some input and
/// no unreachable error path is left behind.
fn shortest_digits_and_exponent(value: f64) -> (String, i32) {
  let formatted = format!("{:e}", value);

  let mut digits = String::with_capacity(17);
  let mut exponent = 0i32;
  let mut exponent_is_negative = false;
  let mut in_exponent = false;

  for ch in formatted.chars() {
    match ch {
      'e' => in_exponent = true,
      '.' => {},
      '-' => exponent_is_negative = true,
      _ if in_exponent => exponent = exponent * 10 + i32::from(ch as u8 - b'0'),
      _ => digits.push(ch),
    }
  }

  if exponent_is_negative {
    exponent = -exponent;
  }

  // ECMA's `n` is one past the exponent of the leading digit.
  (digits, exponent + 1)
}

#[cfg(test)]
#[path = "tests/number_test.rs"]
mod tests;
