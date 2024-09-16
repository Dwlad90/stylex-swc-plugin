use swc_core::{
  common::comments::{Comment, CommentKind, Comments},
  ecma::{ast::Module, visit::FoldWith},
};

use crate::{
  shared::{
    enums::core::TransformationCycle, structures::meta_data::MetaData,
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
      self.state.cycle = TransformationCycle::StateFilling;
      module = module.fold_children_with(self);

      fill_top_level_expressions(&module, &mut self.state);

      self.state.cycle = TransformationCycle::TransformEnter;
      module = module.fold_children_with(self);

      self.state.cycle = TransformationCycle::TransformExit;
      module = module.fold_children_with(self);

      if !&self.state.metadata.is_empty() {
        if self.comments.has_leading(module.span.lo) {
          self.comments.take_leading(module.span.lo);
        }

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

      module
    } else {
      self.state.cycle = TransformationCycle::Skip;
      module
    }
  }
}
