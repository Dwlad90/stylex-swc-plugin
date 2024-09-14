use crate::shared::enums::data_structures::evaluate_result_value::EvaluateResultValue;

#[derive(Clone, Debug)]
pub(crate) struct SeenValue {
  pub(crate) value: Option<Box<EvaluateResultValue>>,
  pub(crate) resolved: bool,
}
