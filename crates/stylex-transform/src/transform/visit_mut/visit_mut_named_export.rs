use swc_core::{
  common::comments::Comments,
  ecma::{ast::NamedExport, visit::VisitMutWith},
};

use crate::StyleXTransform;
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_named_export_impl(&mut self, named_export: &mut NamedExport) {
    if self.state.cycle == TransformationCycle::Skip {
      return;
    }

    if self.state.cycle == TransformationCycle::StateFilling {
      self.state.named_exports.insert(named_export.clone());
    }

    named_export.visit_mut_children_with(self);
  }
}
