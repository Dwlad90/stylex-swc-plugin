use crate::values::parser::split_value_parts;
use stylex_structures::raw_value::TRawValue;

pub fn split_value_required(
  strng: Option<&TRawValue>,
) -> (TRawValue, TRawValue, TRawValue, TRawValue) {
  let values = split_value(strng);

  let top = values.0;
  let right = values.1.unwrap_or(top.clone());
  let bottom = values.2.unwrap_or(top.clone());
  let left = values.3.unwrap_or(right.clone());

  (top, right, bottom, left)
}

pub fn split_value(
  value: Option<&TRawValue>,
) -> (
  TRawValue,
  Option<TRawValue>,
  Option<TRawValue>,
  Option<TRawValue>,
) {
  // A number is returned untouched, so a shorthand hands the authored number
  // to each expanded property and each appends its own unit suffix. Parsing it
  // as CSS text would settle the unit here instead, and every expansion would
  // inherit the shorthand's.
  if let Some(TRawValue::Number(number)) = value {
    return (TRawValue::Number(*number), None, None, None);
  }

  let value = value.map(TRawValue::as_css_text).unwrap_or_default();
  let nodes = split_value_parts(value.as_ref());

  let top = nodes
    .first()
    .cloned()
    .map(TRawValue::String)
    .unwrap_or_default();
  let right = nodes.get(1).cloned().map(TRawValue::String);
  let bottom = nodes.get(2).cloned().map(TRawValue::String);
  let left = nodes.get(3).cloned().map(TRawValue::String);

  (top, right, bottom, left)
}
