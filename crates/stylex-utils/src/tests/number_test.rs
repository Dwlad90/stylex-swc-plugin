use crate::number::to_js_string;

/// Every expectation is the literal output of JS `String(value)`, which reaches
/// both generated code and the class-name hash.
#[test]
fn matches_js_string_number() {
  let cases: &[(f64, &str)] = &[
    (0.0, "0"),
    (-0.0, "0"),
    (1.0, "1"),
    (-1.0, "-1"),
    (42.5, "42.5"),
    (100.0, "100"),
    (0.1, "0.1"),
    (1234.5678, "1234.5678"),
    (1e6, "1000000"),
    (1.0 / 3.0, "0.3333333333333333"),
    // Boundary of the fixed-notation range: `n <= 21` stays expanded.
    (1e20, "100000000000000000000"),
    (123456789012345680000.0, "123456789012345680000"),
    (1e21, "1e+21"),
    (-1e21, "-1e+21"),
    (1.0000000000000001e21, "1.0000000000000001e+21"),
    // Boundary of the small-number range: `n > -6` stays expanded.
    (1e-6, "0.000001"),
    (1e-7, "1e-7"),
    (-1e-7, "-1e-7"),
    (2e-10, "2e-10"),
    (5e-324, "5e-324"),
    (f64::MAX, "1.7976931348623157e+308"),
  ];

  for (value, expected) in cases {
    assert_eq!(to_js_string(*value), *expected, "String({})", expected);
  }
}

#[test]
fn renders_non_finite_values() {
  assert_eq!(to_js_string(f64::NAN), "NaN");
  assert_eq!(to_js_string(f64::INFINITY), "Infinity");
  assert_eq!(to_js_string(f64::NEG_INFINITY), "-Infinity");
}
