use swc_core::{
    common::comments::{Comment, CommentKind, Comments},
    ecma::{ast::Module, visit::FoldWith},
};

use crate::{
    shared::{enums::ModuleCycle, utils::validators::validate_style_x_create},
    ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_module_impl(&mut self, module: Module) -> Module {
        let module = module.clone().fold_children_with(self);

        if let Some(declaration) = self.declaration.clone() {
            validate_style_x_create(&module, &declaration);

            self.cycle = ModuleCycle::Processing;
            let module = module.clone().fold_children_with(self);

            let module = if self.config.runtime_injection == true {
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
