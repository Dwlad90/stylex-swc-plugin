use swc_core::{
  common::comments::{Comment, CommentKind, Comments},
  ecma::{ast::Module, visit::FoldWith},
};

use crate::{
  shared::{
    enums::ModuleCycle, structures::meta_data::MetaData, utils::common::fill_top_level_expressions,
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
      fill_top_level_expressions(&module, &mut self.state);

      self.cycle = ModuleCycle::TransformEnter;
      module = module.fold_children_with(self);

      self.cycle = ModuleCycle::TransformExit;
      module = module.fold_children_with(self);

      dbg!(&self.state.options.runtime_injection);

      if self.state.options.runtime_injection.is_some() {
        self.cycle = ModuleCycle::InjectStyles;
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

      self.cycle = ModuleCycle::Cleaning;
      module.fold_children_with(self)
    } else {
      self.cycle = ModuleCycle::Skip;
      module
    }
  }
}
