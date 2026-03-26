use crate::shared::enums::data_structures::evaluate_result_value::EvaluateResultValue;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SeenValue {
  pub(crate) value: Option<EvaluateResultValue>,
  pub(crate) resolved: bool,
}
