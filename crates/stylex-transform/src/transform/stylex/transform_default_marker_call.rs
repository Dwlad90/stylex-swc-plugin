use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::{
  StyleXTransform,
  shared::{
    transformers::stylex_default_marker::stylex_default_marker,
    utils::{
      core::js_to_ast::convert_object_to_ast,
      validators::{is_default_marker_call, validate_stylex_default_marker_indent},
    },
  },
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_default_marker_call(&mut self, call: &CallExpr) -> Option<Expr> {
    let is_default_marker_call = is_default_marker_call(call, &self.state);

    if is_default_marker_call {
      validate_stylex_default_marker_indent(call, &mut self.state);

      let marker_result = stylex_default_marker(&self.state.options);

      let marker_ast = convert_object_to_ast(&marker_result);

      Some(marker_ast)
    } else {
      None
    }
  }
}
