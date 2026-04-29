use super::{ArrayJS, MathJS, ObjectJS, StringJS};

#[test]
fn array_js_try_from_maps_known_methods() {
  assert_eq!(ArrayJS::try_from("map"), Ok(ArrayJS::Map));
  assert_eq!(ArrayJS::try_from("filter"), Ok(ArrayJS::Filter));
  assert_eq!(ArrayJS::try_from("join"), Ok(ArrayJS::Join));
  assert_eq!(ArrayJS::try_from("reduce"), Err(()));
}

#[test]
fn object_js_try_from_maps_known_methods() {
  assert_eq!(ObjectJS::try_from("entries"), Ok(ObjectJS::Entries));
  assert_eq!(ObjectJS::try_from("keys"), Ok(ObjectJS::Keys));
  assert_eq!(ObjectJS::try_from("values"), Ok(ObjectJS::Values));
  assert_eq!(ObjectJS::try_from("fromEntries"), Ok(ObjectJS::FromEntries));
  assert_eq!(ObjectJS::try_from("assign"), Err(()));
}

#[test]
fn math_js_try_from_maps_known_methods() {
  assert_eq!(MathJS::try_from("pow"), Ok(MathJS::Pow));
  assert_eq!(MathJS::try_from("round"), Ok(MathJS::Round));
  assert_eq!(MathJS::try_from("ceil"), Ok(MathJS::Ceil));
  assert_eq!(MathJS::try_from("floor"), Ok(MathJS::Floor));
  assert_eq!(MathJS::try_from("max"), Ok(MathJS::Max));
  assert_eq!(MathJS::try_from("min"), Ok(MathJS::Min));
  assert_eq!(MathJS::try_from("abs"), Ok(MathJS::Abs));
  assert_eq!(MathJS::try_from("random"), Err(()));
}

#[test]
fn string_js_try_from_maps_known_methods() {
  assert_eq!(StringJS::try_from("concat"), Ok(StringJS::Concat));
  assert_eq!(StringJS::try_from("charCodeAt"), Ok(StringJS::CharCodeAt));
  assert_eq!(StringJS::try_from("slice"), Err(()));
}
