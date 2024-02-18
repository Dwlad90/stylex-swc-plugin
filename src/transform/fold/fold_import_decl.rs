use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{ImportDecl, ImportSpecifier, ModuleExportName},
        visit::FoldWith,
    },
};

use crate::{
    shared::{
        constants,
        enums::ModuleCycle,
        structures::named_import_source::{ImportSources, NamedImportSource},
        utils::common::increase_ident_count,
    },
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

            if self.state.stylex_import.iter().any(|import| match import {
                ImportSources::Regular(regular) => regular.eq(&declaration.to_string()),
                ImportSources::Named(named) => named.from.eq(&declaration.to_string()),
            }) {
                for specifier in &import_decl.specifiers {
                    match &specifier {
                        ImportSpecifier::Default(import_specifier) => {
                            self.state
                                .stylex_create_import
                                .insert(import_specifier.local.to_id());
                        }
                        ImportSpecifier::Namespace(import_specifier) => {
                            self.state
                                .stylex_create_import
                                .insert(import_specifier.local.to_id());
                        }
                        ImportSpecifier::Named(import_specifier) => {
                            println!("!!!!import_specifier: {:?}", import_specifier);
                            match &import_specifier.imported {
                                Some(imported) => match imported {
                                    ModuleExportName::Ident(ident) => {
                                        if ident.sym.as_str() == "create" {
                                            self.state
                                                .stylex_create_import
                                                .insert(import_specifier.local.to_id());
                                        } else {
                                            panic!(
                                                "{}",
                                                constants::messages::MUST_BE_DEFAULT_IMPORT
                                            )
                                        }
                                    }
                                    ModuleExportName::Str(_) => {
                                        panic!("{}", constants::messages::MUST_BE_DEFAULT_IMPORT)
                                    }
                                },
                                None => {
                                    let named_custom_imports = self
                                        .state
                                        .stylex_import
                                        .clone()
                                        .into_iter()
                                        .filter(|import| import.is_named_export())
                                        .map(|import| match import {
                                            ImportSources::Named(named) => named,
                                            _ => panic!("Not named import"),
                                        })
                                        .collect::<Vec<NamedImportSource>>();

                                    let mut found_custom_import = false;

                                    if named_custom_imports.len() > 0 {
                                        let named_import = named_custom_imports
                                            .iter()
                                            .find(|import| import.from.eq(&declaration.to_string()))
                                            .unwrap();

                                        if named_import
                                            .r#as
                                            .eq(&import_specifier.local.sym.as_str())
                                        {
                                            self.state
                                                .stylex_create_import
                                                .insert(import_specifier.local.to_id());
                                        }

                                        found_custom_import = true;
                                    }

                                    if !found_custom_import {
                                        match import_specifier.local.sym.as_str() {
                                            "create" => {
                                                self.state
                                                    .stylex_create_import
                                                    .insert(import_specifier.local.to_id());
                                            }
                                            _ => {
                                                panic!(
                                                    "{}",
                                                    constants::messages::MUST_BE_DEFAULT_IMPORT
                                                )
                                            }
                                        };
                                    }
                                }
                            }
                        }
                    };
                }
            }

            if self.state.stylex_create_import.len() == 0 {
                import_decl
            } else {
                import_decl.fold_children_with(self)
            }
        } else {
            import_decl
        }
    }
}
