use swc_core::{
    common::comments::{Comment, CommentKind, Comments},
    ecma::{ast::Module, visit::FoldWith},
};

use crate::{
    shared::{
        enums::ModuleCycle, structures::meta_data::MetaData,
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

        if !self.state.stylex_import.is_empty() || !self.state.import_paths.is_empty() {
            fill_top_level_expressions(&module, &mut self.state);

            let cycles = [
                ModuleCycle::TransformEnter,
                ModuleCycle::TransformExit,
                ModuleCycle::InjectStyles,
                ModuleCycle::Cleaning,
            ];

            for cycle in &cycles {
                self.cycle = cycle.clone();
                module = module.fold_children_with(self);
            }

            if self.state.options.runtime_injection.is_none() {
                // Preparing stylex metadata for css extraction
                self.comments.add_leading_comments(
                    module.span.lo,
                    vec![Comment {
                        kind: CommentKind::Block,
                        text: format!(
                            "__stylex_metadata_start__{}__stylex_metadata_end__",
                            serde_json::to_string(
                                &self
                                    .css_output
                                    .iter()
                                    .map(|v| v.value().clone())
                                    .flatten()
                                    .collect::<Vec<MetaData>>()
                            )
                            .unwrap()
                        )
                        .into(),
                        span: module.span,
                    }],
                );
            }

            module
        } else {
            self.cycle = ModuleCycle::Skip;
            module
        }
    }
}
