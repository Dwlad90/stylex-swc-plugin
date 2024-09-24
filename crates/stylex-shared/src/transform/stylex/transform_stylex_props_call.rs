use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr},
};

use crate::{
  shared::utils::{
    core::{props::props, stylex_merge::stylex_merge},
    validators::is_props_call,
  },
  ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_props_call(&mut self, call: &mut CallExpr) -> Option<Expr> {
    let is_props_call = is_props_call(call, &self.state);

    if is_props_call {
      return stylex_merge(call, props, &mut self.state);
    }

    None
  }
}
