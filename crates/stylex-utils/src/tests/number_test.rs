use crate::number::{parse_js_float, to_js_string};

include!("number_parse_float_cases.rs");

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

/// Compared bit-for-bit so that `-0` cannot pass as `0`: the two are equal under
/// `==` but spell differently once a caller does arithmetic on them.
#[test]
fn should_match_js_parse_float_on_every_generated_case() {
  for (input, expected) in PARSE_FLOAT_CASES {
    let actual = parse_js_float(input);

    match (actual, expected) {
      (Some(actual), Some(expected)) => assert_eq!(
        actual.to_bits(),
        expected.to_bits(),
        "parseFloat({:?}) is {}, got {}",
        input,
        expected,
        actual
      ),
      (None, None) => {},
      _ => panic!(
        "parseFloat({:?}) is {:?}, got {:?}",
        input, expected, actual
      ),
    }
  }
}

/// The failure case has to be visible in the type, so a caller cannot mistake a
/// legitimate zero for "no number here" the way a NaN sentinel invites.
#[test]
fn should_report_failure_rather_than_a_sentinel_value() {
  assert_eq!(parse_js_float("auto"), None);
  assert_eq!(parse_js_float("0"), Some(0.0));
}
