// Tests for the data-level JavaScript value and its coercions.
// Source: crates/stylex-js/src/value.rs
//
// Each expected value is what `String(x)`, `Number(x)` or `Boolean(x)` answers
// in Node 24 for the same value.

use super::*;

fn array(elements: Vec<JsValue>) -> JsValue {
  JsValue::Array(elements)
}

fn object(properties: &[(&str, JsValue)]) -> JsValue {
  JsValue::object(
    properties
      .iter()
      .map(|(key, value)| (key.to_string(), value.clone())),
  )
}

#[test]
fn to_string_spells_every_kind_as_javascript_does() {
  let cases: Vec<(JsValue, &str)> = vec![
    (JsValue::Undefined, "undefined"),
    (JsValue::Hole, "undefined"),
    (JsValue::Null, "null"),
    (true.into(), "true"),
    (false.into(), "false"),
    ((-0.0).into(), "0"),
    (1e21.into(), "1e+21"),
    (f64::NAN.into(), "NaN"),
    (f64::NEG_INFINITY.into(), "-Infinity"),
    ("text".into(), "text"),
    (object(&[("toString", "x".into())]), "[object Object]"),
    (array(vec![]), ""),
    (array(vec![array(vec![])]), ""),
    (array(vec![array(vec![]), array(vec![])]), ","),
    (array(vec![JsValue::Hole, 1.0.into()]), ",1"),
    (
      array(vec![
        1.0.into(),
        array(vec![2.0.into(), 3.0.into()]),
        JsValue::Null,
        JsValue::Undefined,
      ]),
      "1,2,3,,",
    ),
    (array(vec![object(&[])]), "[object Object]"),
  ];

  for (value, expected) in cases {
    assert_eq!(value.to_js_string(), expected, "{value:?}");
  }
}

#[test]
fn to_number_reads_an_array_through_its_text_and_an_object_as_nan() {
  let cases: Vec<(JsValue, f64)> = vec![
    (JsValue::Null, 0.0),
    (true.into(), 1.0),
    (false.into(), 0.0),
    ("0x10".into(), 16.0),
    (" 12 ".into(), 12.0),
    ("\u{feff}1".into(), 1.0),
    ("".into(), 0.0),
    (array(vec![]), 0.0),
    (array(vec![5.0.into()]), 5.0),
    (array(vec!["5".into()]), 5.0),
    (array(vec![array(vec![])]), 0.0),
    (array(vec![1e21.into()]), 1e21),
    (2.5.into(), 2.5),
  ];

  for (value, expected) in cases {
    assert_eq!(value.to_js_number(), expected, "{value:?}");
  }

  for value in [
    JsValue::Undefined,
    JsValue::Hole,
    object(&[]),
    array(vec![1.0.into(), 2.0.into()]),
    array(vec![JsValue::Hole, 1.0.into()]),
    "\u{85}1".into(),
    "abc".into(),
  ] {
    assert!(value.to_js_number().is_nan(), "{value:?}");
  }
}

#[test]
fn to_boolean_is_false_for_the_six_falsy_values_only() {
  for value in [
    JsValue::Undefined,
    JsValue::Hole,
    JsValue::Null,
    false.into(),
    0.0.into(),
    (-0.0).into(),
    f64::NAN.into(),
    "".into(),
  ] {
    assert!(!value.to_js_boolean(), "{value:?}");
  }

  for value in [
    true.into(),
    1.0.into(),
    f64::INFINITY.into(),
    " ".into(),
    "0".into(),
    array(vec![]),
    object(&[]),
  ] {
    assert!(value.to_js_boolean(), "{value:?}");
  }
}

#[test]
fn join_writes_nothing_for_a_nullish_element() {
  assert_eq!(
    array(vec![
      "a".into(),
      JsValue::Null,
      JsValue::Undefined,
      JsValue::Hole,
      5.0.into()
    ])
    .to_js_string(),
    "a,,,,5"
  );
  assert_eq!(array(vec![]).to_js_string(), "");
  assert_eq!(JsValue::Null.to_js_join_element(), "");
  assert_eq!(JsValue::from(false).to_js_join_element(), "false");
}

#[test]
fn same_value_zero_makes_one_key_of_nan_and_of_both_zeros() {
  let key = |value: JsValue| value.same_value_zero_key().map(|key| format!("{key:?}"));

  assert_eq!(key(f64::NAN.into()), key((-f64::NAN).into()));
  assert_eq!(key(0.0.into()), key((-0.0).into()));
  assert_eq!(key(JsValue::Hole), key(JsValue::Undefined));
  assert_ne!(key(1.0.into()), key("1".into()));
  assert_ne!(key(JsValue::Null), key(JsValue::Undefined));
  assert_ne!(key(true.into()), key(1.0.into()));
  assert_eq!(key("a".into()), Some("String(\"a\")".to_string()));
  assert_eq!(key(array(vec![])), None);
  assert_eq!(key(object(&[])), None);
}

#[test]
fn a_property_read_finds_an_object_property_and_nothing_else() {
  let mut value = object(&[("ltr", "a".into())]);

  assert_eq!(value.property("ltr"), &JsValue::from("a"));
  assert_eq!(value.property("rtl"), &JsValue::Undefined);
  assert_eq!(array(vec!["a".into()]).property("0"), &JsValue::Undefined);
  assert_eq!(value.take_property("ltr"), JsValue::from("a"));
  assert_eq!(value.property("ltr"), &JsValue::Undefined);
  assert_eq!(value.take_property("missing"), JsValue::Undefined);
  assert_eq!(JsValue::Null.take_property("ltr"), JsValue::Undefined);
}

#[test]
fn nullish_and_describe_name_each_kind() {
  let kinds: Vec<(JsValue, bool, &str)> = vec![
    (JsValue::Undefined, true, "undefined"),
    (JsValue::Hole, true, "undefined"),
    (JsValue::Null, true, "null"),
    (false.into(), false, "a boolean"),
    (0.0.into(), false, "a number"),
    ("".into(), false, "a string"),
    (array(vec![]), false, "an array"),
    (object(&[]), false, "an object"),
  ];

  for (value, nullish, description) in kinds {
    assert_eq!(value.is_nullish(), nullish, "{value:?}");
    assert_eq!(value.describe(), description, "{value:?}");
  }
}

#[test]
fn a_deeply_nested_array_spells_and_reads_without_trouble() {
  let mut value = JsValue::from("x");
  for _ in 0..200 {
    value = array(vec![value]);
  }

  assert_eq!(value.to_js_string(), "x");
  assert!(value.to_js_number().is_nan());
  assert!(value.to_js_boolean());
}

#[test]
fn a_wide_array_joins_every_element() {
  let wide = array((0..10_000).map(|index| f64::from(index).into()).collect());
  let text = wide.to_js_string();

  assert_eq!(text.split(',').count(), 10_000);
  assert!(text.starts_with("0,1,2,"));
  assert!(text.ends_with(",9999"));
}

#[test]
fn from_conversions_build_the_matching_kind() {
  assert_eq!(
    JsValue::from(String::from("a")),
    JsValue::String("a".into())
  );
  assert_eq!(JsValue::default(), JsValue::Undefined);
}

/// `String(x)` for a number, as Node 24 answers it: the shortest text that
/// reads back as the same number, with an exponent from 1e21 up and below
/// 1e-6.
#[test]
fn to_string_of_a_number_is_the_text_node_writes() {
  let cases: Vec<(f64, &str)> = vec![
    (0.0, "0"),
    (-0.0, "0"),
    (1.0, "1"),
    (-1.0, "-1"),
    (0.1, "0.1"),
    (0.5, "0.5"),
    (0.3333333333333333, "0.3333333333333333"),
    (-0.3333333333333333, "-0.3333333333333333"),
    (123456789012345680000.0, "123456789012345680000"),
    (1.0e21, "1e+21"),
    (2.5e21, "2.5e+21"),
    (1.0e100, "1e+100"),
    (1.0e-7, "1e-7"),
    (-1.0e-7, "-1e-7"),
    (1.5e-7, "1.5e-7"),
    (0.000001, "0.000001"),
    (1.23e-18, "1.23e-18"),
    (9007199254740992.0, "9007199254740992"),
    (9007199254740994.0, "9007199254740994"),
    (f64::MAX, "1.7976931348623157e+308"),
    (5.0e-324, "5e-324"),
    (-5.0e-324, "-5e-324"),
    (f64::INFINITY, "Infinity"),
    (f64::NEG_INFINITY, "-Infinity"),
    (f64::NAN, "NaN"),
    (100.0, "100"),
    (4.35, "4.35"),
  ];

  for (number, text) in cases {
    assert_eq!(JsValue::from(number).to_js_string(), text, "{number:?}");
  }
}

/// `Number(s)` for a string, as Node 24 answers it: white space around the
/// literal is ignored (U+FEFF and U+2028 are white space, U+0085 is not), and
/// a radix prefix takes no sign.
#[test]
fn to_number_of_a_string_is_the_number_node_reads() {
  let cases: Vec<(&str, f64)> = vec![
    ("", 0.0),
    ("  ", 0.0),
    ("12", 12.0),
    ("  12  ", 12.0),
    ("\u{a0}12\u{2028}", 12.0),
    ("\u{feff}7", 7.0),
    ("\t\n 3 \r", 3.0),
    ("00012", 12.0),
    ("0x1F", 31.0),
    ("0X1f", 31.0),
    ("0b101", 5.0),
    ("0o17", 15.0),
    ("-0x1F", f64::NAN),
    ("0b2", f64::NAN),
    ("1e3", 1000.0),
    ("1E3", 1000.0),
    ("1e1000", f64::INFINITY),
    ("0.0000001", 1.0e-7),
    (".5", 0.5),
    ("5.", 5.0),
    ("+5", 5.0),
    ("-5", -5.0),
    ("-0", -0.0),
    ("Infinity", f64::INFINITY),
    ("-Infinity", f64::NEG_INFINITY),
    ("+Infinity", f64::INFINITY),
    ("infinity", f64::NAN),
    ("1_000", f64::NAN),
    ("12px", f64::NAN),
    ("abc", f64::NAN),
    ("\u{85}1", f64::NAN),
  ];

  for (text, number) in cases {
    let read = JsValue::from(text).to_js_number();
    assert!(
      read.to_bits() == number.to_bits() || (read.is_nan() && number.is_nan()),
      "{text:?}: {read} is not {number}"
    );
  }
}

/// `SameValueZero` over numbers of every shape: two numbers are one key
/// exactly when they are equal or both `NaN`, whatever their bits.
#[test]
fn same_value_zero_holds_over_random_bit_patterns() {
  // A 64-bit linear congruential generator, so the test is the same each run.
  let mut state: u64 = 0x2545_F491_4F6C_DD1D;
  let mut next = || {
    state = state
      .wrapping_mul(6_364_136_223_846_793_005)
      .wrapping_add(1_442_695_040_888_963_407);
    f64::from_bits(state)
  };

  for _ in 0..10_000 {
    let a = next();
    // Half the pairs share a value, so equal keys are tested as often.
    let b = if a.to_bits() % 2 == 0 { a } else { next() };
    let same = a == b || (a.is_nan() && b.is_nan());

    assert_eq!(
      JsValue::from(a).same_value_zero_key() == JsValue::from(b).same_value_zero_key(),
      same,
      "{a:?} and {b:?}"
    );
  }

  // Every NaN is one key, and so are both zeros.
  let quiet = f64::from_bits(0x7ff8_0000_0000_0001);
  let negative = f64::from_bits(0xfff8_0000_0000_0000);
  assert_eq!(
    JsValue::from(quiet).same_value_zero_key(),
    JsValue::from(negative).same_value_zero_key()
  );
  assert_eq!(
    JsValue::from(0.0).same_value_zero_key(),
    JsValue::from(-0.0).same_value_zero_key()
  );
}

/// A string that holds a real U+FFFD spells as itself: the replacement
/// character is a character like any other.
#[test]
fn to_string_keeps_a_replacement_character() {
  assert_eq!(JsValue::from("a\u{fffd}b").to_js_string(), "a\u{fffd}b");
  assert_eq!(
    array(vec!["\u{fffd}".into(), JsValue::Null]).to_js_string(),
    "\u{fffd},"
  );
}
