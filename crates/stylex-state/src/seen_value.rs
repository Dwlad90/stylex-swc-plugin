use crate::evaluate_result_value::EvaluateResultValue;

#[derive(Clone, Debug, PartialEq)]
pub struct SeenValue {
  pub value: Option<EvaluateResultValue>,
  pub resolved: bool,
}
