use swc_core::{
  common::comments::{Comment, CommentKind, Comments},
  ecma::{ast::Module, visit::FoldWith},
};

use crate::{
  shared::{
    enums::core::ModuleCycle, structures::meta_data::MetaData,
    utils::common::fill_top_level_expressions,
  },
  ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_module_impl(&mut self, module: Module) -> Module {
    let mut module = module.fold_children_with(self);

    if !self.state.import_paths.is_empty() {
      self.state.cycle = ModuleCycle::StateFilling;
      module = module.fold_children_with(self);

      fill_top_level_expressions(&module, &mut self.state);

      self.state.cycle = ModuleCycle::TransformEnter;
      module = module.fold_children_with(self);

      self.state.cycle = ModuleCycle::TransformExit;
      module = module.fold_children_with(self);

      if self.state.options.runtime_injection.is_some() {
        self.state.cycle = ModuleCycle::InjectStyles;
        module = module.fold_children_with(self);
      } else {
        // Preparing stylex metadata for css extraction
        self.comments.add_leading(
          module.span.lo,
          Comment {
            kind: CommentKind::Line,
            text: format!(
              "__stylex_metadata_start__{}__stylex_metadata_end__",
              serde_json::to_string(
                &self
                  .state
                  .metadata
                  .iter()
                  .flat_map(|v| v.1.clone())
                  .collect::<Vec<MetaData>>()
              )
              .unwrap()
            )
            .into(),
            span: module.span,
          },
        );
      }

      self.state.cycle = ModuleCycle::PreCleaning;
      module = module.fold_children_with(self);

      self.state.cycle = ModuleCycle::Cleaning;

      // NOTE: Reversing the module body to clean the module items in the correct order,
      // so removing unused variable declarations will more efficient
      // After cleaning the module items, the module body will be reversed back to its original order
      module.body.reverse();

      module = module.fold_children_with(self);

      module.body.reverse();

      module
    } else {
      self.state.cycle = ModuleCycle::Skip;
      module
    }
  }
}
