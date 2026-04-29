use super::evaluate_result_value::EvaluateResultValue;

#[test]
fn serializes_null_as_json_null() {
  let json = match serde_json::to_string(&EvaluateResultValue::Null) {
    Ok(json) => json,
    Err(error) => panic!("failed to serialize null evaluate result: {error}"),
  };

  assert_eq!(json, "null");
}

#[test]
fn deserializes_json_null_as_null() {
  let value = match serde_json::from_str::<EvaluateResultValue>("null") {
    Ok(value) => value,
    Err(error) => panic!("failed to deserialize null evaluate result: {error}"),
  };

  assert_eq!(value, EvaluateResultValue::Null);
}
