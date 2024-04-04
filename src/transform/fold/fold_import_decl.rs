use core::panic;

use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{ImportDecl, ImportNamedSpecifier, ImportSpecifier, ModuleExportName},
        visit::FoldWith,
    },
};

use crate::{
    shared::{constants, enums::ModuleCycle, structures::named_import_source::ImportSources},
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

            let import_sources = self.state.import_sources_stringified();
            dbg!(&import_decl, &import_sources, &declaration);
            // panic!("declaration: {:?}", declaration);

            if import_sources.contains(&declaration.to_string()) {
                let source_path = import_decl.src.value.to_string();

                for specifier in &import_decl.specifiers {
                    match &specifier {
                        ImportSpecifier::Default(import_specifier) => {
                            if self.state.import_as(&import_decl.src.value).is_none() {
                                let local_name = import_specifier.local.sym.to_string();

                                println!("!!!! import_decl: {:?}", source_path);
                                println!("!!!! import_specifier: {:?}", local_name);

                                self.state.import_paths.insert(source_path.clone());

                                self.state
                                    .stylex_import
                                    .insert(ImportSources::Regular(local_name));
                            };
                        }
                        ImportSpecifier::Namespace(import_specifier) => {
                            if self.state.import_as(&import_decl.src.value).is_none() {
                                let local_name = import_specifier.local.sym.to_string();

                                self.state.import_paths.insert(source_path.clone());

                                self.state
                                    .stylex_import
                                    .insert(ImportSources::Regular(local_name));
                            }
                        }
                        ImportSpecifier::Named(import_specifier) => {
                            let local_name = import_specifier.local.sym.to_string();

                            match &import_specifier.imported {
                                Some(imported) => {
                                    let imported_name = match imported {
                                        ModuleExportName::Ident(ident) => ident.sym.to_string(),
                                        ModuleExportName::Str(str) => str.value.to_string(),
                                    };

                                    self.fill_stylex_create_import(
                                        &source_path,
                                        imported_name,
                                        &local_name,
                                        import_specifier,
                                    );

                                    // panic!(
                                    //     "!!!imported_name: {:?}, local_name: {:?}",
                                    //     imported_name, local_name
                                    // );

                                    // match imported {
                                    //     ModuleExportName::Ident(ident) => {
                                    //         if ident.sym.as_str() == "create" {
                                    //             self.state
                                    //                 .stylex_create_import
                                    //                 .insert(import_specifier.local.to_id());
                                    //         } else {
                                    //             panic!(
                                    //                 "{}",
                                    //                 constants::messages::MUST_BE_DEFAULT_IMPORT
                                    //             )
                                    //         }
                                    //     }
                                    //     ModuleExportName::Str(_) => {
                                    //         panic!(
                                    //             "{}",
                                    //             constants::messages::MUST_BE_DEFAULT_IMPORT
                                    //         )
                                    //     }
                                    // }
                                }
                                None => {
                                    let imported_name = import_specifier.local.sym.to_string();

                                    self.fill_stylex_create_import(
                                        &source_path,
                                        imported_name,
                                        &local_name,
                                        import_specifier,
                                    );

                                    // let named_custom_imports = self
                                    //     .state
                                    //     .stylex_import
                                    //     .clone()
                                    //     .into_iter()
                                    //     .filter(|import| import.is_named_export())
                                    //     .map(|import| match import {
                                    //         ImportSources::Named(named) => named,
                                    //         _ => panic!("Not named import"),
                                    //     })
                                    //     .collect::<Vec<NamedImportSource>>();

                                    // let mut found_custom_import = false;

                                    // if named_custom_imports.len() > 0 {
                                    //     let named_import = named_custom_imports
                                    //         .iter()
                                    //         .find(|import| import.from.eq(&declaration.to_string()))
                                    //         .unwrap();

                                    //     if named_import
                                    //         .r#as
                                    //         .eq(&import_specifier.local.sym.as_str())
                                    //     {
                                    //         self.state
                                    //             .stylex_create_import
                                    //             .insert(import_specifier.local.to_id());
                                    //     }

                                    //     found_custom_import = true;
                                    // }

                                    // if !found_custom_import {
                                    //     match import_specifier.local.sym.as_str() {
                                    //         "create" => {
                                    //             self.state
                                    //                 .stylex_create_import
                                    //                 .insert(import_specifier.local.to_id());
                                    //         }
                                    //         _ => {
                                    //             panic!(
                                    //                 "{}",
                                    //                 constants::messages::MUST_BE_DEFAULT_IMPORT
                                    //             )
                                    //         }
                                    //     };
                                    // }
                                }
                            }
                        }
                    };
                }
            }

            if self.state.import_paths.len() == 0 {
                import_decl
            } else {
                import_decl.fold_children_with(self)
            }
        } else {
            import_decl
        }
    }

    fn fill_stylex_create_import(
        &mut self,
        source_path: &String,
        imported_name: String,
        local_name: &String,
        import_specifier: &ImportNamedSpecifier,
    ) {
        if let Some(source_path) = self.state.import_as(source_path) {
            if source_path.eq(&imported_name) {
                self.state.import_paths.insert(source_path.clone());

                self.state
                    .stylex_import
                    .insert(ImportSources::Regular(local_name.clone()));
            }
        }

        if self.state.import_as(source_path).is_none() {
            self.state.import_paths.insert(source_path.clone());

            let local_name_ident = import_specifier.local.to_id();

            match imported_name.as_str() {
                "create" => {
                    self.state.stylex_create_import.insert(local_name_ident);
                }
                "props" => {
                    self.state.stylex_props_import.insert(local_name_ident);
                }
                "attrs" => {
                    self.state.stylex_attrs_import.insert(local_name_ident);
                }
                "keyframes" => {
                    self.state.stylex_keyframes_import.insert(local_name_ident);
                }
                "include" => {
                    self.state.stylex_include_import.insert(local_name_ident);
                }
                "firstThatWorks" => {
                    todo!("firstThatWorks");
                }
                "defineVars" => {
                    self.state
                        .stylex_define_vars_import
                        .insert(local_name_ident);
                }
                "createTheme" => {
                    todo!("createTheme");
                }
                "types" => {
                    todo!("types");
                }
                _ => {
                    panic!("{}", constants::messages::MUST_BE_DEFAULT_IMPORT)
                }
            }
        }
    }
}
