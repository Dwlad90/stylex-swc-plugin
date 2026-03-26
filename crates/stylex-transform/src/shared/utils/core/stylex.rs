use crate::{
  shared::{
    enums::data_structures::fn_result::FnResult, utils::ast::convertors::create_string_expr,
  },
  transform::styleq::common::styleq,
};

use super::parse_nullable_style::ResolvedArg;

pub(crate) fn stylex(values: &[ResolvedArg]) -> Option<FnResult> {
  let result = styleq(values);

  Some(FnResult::Stylex(create_string_expr(
    result.class_name.as_str(),
  )))
}
