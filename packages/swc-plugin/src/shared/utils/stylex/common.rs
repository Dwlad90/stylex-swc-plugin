use crate::{
  shared::{enums::FnResult, utils::common::string_to_expression},
  transform::styleq::common::styleq,
};

use super::parse_nullable_style::ResolvedArg;

pub(crate) fn stylex(values: &Vec<ResolvedArg>) -> Option<FnResult> {
  let result = styleq(values);

  Some(FnResult::Stylex(
    string_to_expression(result.class_name.as_str()).unwrap(),
  ))
}

pub(crate) fn gen_condition_permutations(count: usize) -> Vec<Vec<bool>> {
  let mut result = Vec::new();
  for i in 0..2u32.pow(count as u32) {
    let mut combination = Vec::new();
    for j in 0..count {
      combination.push(i & (1 << j) != 0);
    }
    result.push(combination);
  }
  result
}
