use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::NamedExport,
    visit::FoldWith,
  },
};

use crate::{
  StyleXTransform,
  shared::enums::core::TransformationCycle,
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_named_export_impl(&mut self, named_export: NamedExport) -> NamedExport {
    if self.state.cycle == TransformationCycle::Skip {
      return named_export;
    }

    if self.state.cycle == TransformationCycle::StateFilling {
      self.state.named_exports.insert(named_export.clone());
    }

    named_export.fold_children_with(self)
  }
}
