use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{ImportDecl, ImportSpecifier},
        visit::FoldWith,
    },
};

use crate::{
    shared::{constants, enums::ModuleCycle},
    ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn fold_import_decl_impl(&mut self, import_decl: ImportDecl) -> ImportDecl {
        if self.cycle == ModuleCycle::Skip {
            return import_decl;
        }

        if self.cycle == ModuleCycle::Initializing {
            if import_decl.type_only {
                return import_decl;
            }

            let src = &import_decl.src;
            let declaration = &src.value;

            if declaration.eq(self.package_name.as_str()) {
                for specifier in &import_decl.specifiers {
                    match &specifier {
                        ImportSpecifier::Default(import_specifier) => {
                            self.declaration = Some(import_specifier.local.to_id());
                        }
                        ImportSpecifier::Namespace(import_specifier) => {
                            self.declaration = Some(import_specifier.local.to_id());
                        }
                        ImportSpecifier::Named(import_specifier) => {
                            match import_specifier.local.sym.as_str() {
                                "create" => {
                                    self.declaration = Some(import_specifier.local.to_id());
                                }
                                _ => {
                                    panic!("{}", constants::common::MUST_BE_DEFAULT_IMPORT)
                                }
                            };
                        }
                    };
                }
            }

            if self.declaration.is_none() {
                import_decl
            } else {
                import_decl.fold_children_with(self)
            }
        } else {
            import_decl
        }
    }
}
