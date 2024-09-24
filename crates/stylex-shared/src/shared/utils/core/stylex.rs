use crate::{
  shared::{
    enums::data_structures::fn_result::FnResult, utils::ast::convertors::string_to_expression,
  },
  transform::styleq::common::styleq,
};

use super::parse_nullable_style::ResolvedArg;

pub(crate) fn stylex(values: &Vec<ResolvedArg>) -> Option<FnResult> {
  let result = styleq(values);

  Some(FnResult::Stylex(string_to_expression(
    result.class_name.as_str(),
  )))
}
