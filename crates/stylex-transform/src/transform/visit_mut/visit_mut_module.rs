use swc_core::{
  common::comments::Comments,
  ecma::{ast::Module, visit::VisitMutWith},
};

use crate::{StyleXTransform, shared::utils::common::fill_top_level_expressions};
use stylex_enums::core::TransformationCycle;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_module_impl(&mut self, module: &mut Module) {
    if cfg!(debug_assertions) || !self.state.options.use_real_file_for_source {
      self.state.set_seen_module_source_code(module, None);
    }

    module.visit_mut_children_with(self);

    if self.state.has_import_paths() {
      self.state.cycle = TransformationCycle::StateFilling;
      module.visit_mut_children_with(self);

      fill_top_level_expressions(module, &mut self.state);

      self.state.cycle = TransformationCycle::TransformEnter;
      module.visit_mut_children_with(self);

      self.state.cycle = TransformationCycle::TransformExit;
      self.state.evaluate_preserve_bindings = true;
      module.visit_mut_children_with(self);
      self.state.evaluate_preserve_bindings = false;

      if self.state.options.runtime_injection.is_some() {
        self.state.cycle = TransformationCycle::InjectStyles;
        module.visit_mut_children_with(self);
      }

      self.state.cycle = TransformationCycle::PreCleaning;
      module.visit_mut_children_with(self);

      self.state.cycle = TransformationCycle::Cleaning;

      // NOTE: Reversing the module body to clean the module items in the correct
      // order, so removing unused variable declarations will more efficient
      // After cleaning the module items, the module body will be reversed back to its
      // original order
      module.body.reverse();

      module.visit_mut_children_with(self);

      module.body.reverse();
    } else {
      self.state.cycle = TransformationCycle::Skip;
    }
  }
}
