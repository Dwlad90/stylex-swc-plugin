use swc_core::{
    common::comments::{Comment, CommentKind, Comments},
    ecma::{ast::Module, visit::FoldWith},
};

use crate::{
    shared::{enums::ModuleCycle, utils::common::fill_top_level_expressions},
    ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_module_impl(&mut self, module: Module) -> Module {
        let module = module.clone().fold_children_with(self);

        if !self.state.stylex_import.is_empty() || !self.state.import_paths.is_empty() {
            fill_top_level_expressions(&module, &mut self.state);

            self.cycle = ModuleCycle::TransformEnter;
            let module = module.clone().fold_children_with(self);

            self.cycle = ModuleCycle::TransformExit;
            let module = module.clone().fold_children_with(self);

            let module = if self.state.options.runtime_injection.is_some() {
                self.cycle = ModuleCycle::InjectStyles;

                let module = module.clone().fold_children_with(self);

                module
            } else {
                self.cycle = ModuleCycle::InjectClassName;

                let module = module.clone().fold_children_with(self);

                self.comments.add_leading_comments(
                    module.span.lo,
                    vec![Comment {
                        kind: CommentKind::Block,
                        text: format!(
                            "__stylex_metadata_start__{}__stylex_metadata_end__",
                            serde_json::to_string(&self.css_output).unwrap()
                        )
                        .into(),
                        span: module.span,
                    }],
                );

                module
            };

            self.cycle = ModuleCycle::Cleaning;

            module.fold_children_with(self)
        } else {
            self.cycle = ModuleCycle::Skip;
            module
        }
    }
}
