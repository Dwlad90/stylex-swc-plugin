use crate::{
  shared::{enums::FnResult, utils::common::string_to_expression},
  transform::styleq::styleq::styleq,
};

use super::parse_nullable_style::ResolvedArg;

pub(crate) fn stylex(values: &Vec<ResolvedArg>) -> Option<FnResult> {
  let result = styleq(values);

  let class_name = result.class_name.to_string();

  // let props: Vec<PropOrSpread> = vec![];

  // panic!("Implement stylex");

  // if let Some(value) =  {
  // vec![];
  // };
  //  else {
  //     vec![]
  // };

  dbg!(&values, &class_name);

  Some(FnResult::Stylex(string_to_expression(class_name).unwrap()))

  // Some(FnResult::Stylex(
  //     object_expression_factory(vec![prop_or_spread_expression_creator(
  //         String::from("className"),
  //         string_to_expression(class_name).unwrap(),
  //     )])
  //     .unwrap(),
  // ))
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
