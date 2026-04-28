use crate::shared::{
  enums::data_structures::fn_result::FnResult, utils::ast::convertors::create_string_expr,
};

use super::{parse_nullable_style::ResolvedArg, styleq::styleq};

pub(crate) fn stylex(values: &[ResolvedArg]) -> Option<FnResult> {
  let result = styleq(values);

  Some(FnResult::Stylex(create_string_expr(
    result.class_name.as_str(),
  )))
}
