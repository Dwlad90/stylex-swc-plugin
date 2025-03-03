use swc_core::{
  common::comments::Comments,
  ecma::{ast::Module, visit::FoldWith},
};

use crate::{
  shared::{enums::core::TransformationCycle, utils::common::fill_top_level_expressions},
  StyleXTransform,
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_module_impl(&mut self, module: Module) -> Module {
    if cfg!(debug_assertions) {
      self.state.set_debug_assertions_module(&module);
    }

    let mut module = module.fold_children_with(self);

    if !self.state.import_paths.is_empty() {
      self.state.cycle = TransformationCycle::StateFilling;
      module = module.fold_children_with(self);

      fill_top_level_expressions(&module, &mut self.state);

      self.state.cycle = TransformationCycle::TransformEnter;
      module = module.fold_children_with(self);

      self.state.cycle = TransformationCycle::TransformExit;
      module = module.fold_children_with(self);

      if self.state.options.runtime_injection.is_some() {
        self.state.cycle = TransformationCycle::InjectStyles;
        module = module.fold_children_with(self);
      }

      self.state.cycle = TransformationCycle::PreCleaning;
      module = module.fold_children_with(self);

      self.state.cycle = TransformationCycle::Cleaning;

      // NOTE: Reversing the module body to clean the module items in the correct order,
      // so removing unused variable declarations will more efficient
      // After cleaning the module items, the module body will be reversed back to its original order
      module.body.reverse();

      module = module.fold_children_with(self);

      module.body.reverse();

      if !cfg!(debug_assertions) && self.state.debug_assertions_module.is_some() {
        panic!("Debug assertions module is not empty in release mode");
      }

      module
    } else {
      self.state.cycle = TransformationCycle::Skip;
      module
    }
  }
}
