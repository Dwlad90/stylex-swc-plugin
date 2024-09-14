use swc_core::{
  common::comments::Comments,
  ecma::ast::{ImportDecl, ImportNamedSpecifier, ImportSpecifier, ModuleExportName},
};

use crate::{
  shared::{
    constants::messages::MUST_BE_DEFAULT_IMPORT, enums::core::TransformationCycle,
    structures::named_import_source::ImportSources,
  },
  ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_import_decl_impl(&mut self, import_decl: ImportDecl) -> ImportDecl {
    if self.state.cycle == TransformationCycle::Skip {
      return import_decl;
    }

    if self.state.cycle == TransformationCycle::Initializing {
      if import_decl.type_only {
        return import_decl;
      }

      let src = &import_decl.src;
      let declaration = &src.value;

      let import_sources = self.state.import_sources_stringified();

      self.state.top_imports.push(import_decl.clone());

      if import_sources.contains(&declaration.to_string()) {
        let source_path = import_decl.src.value.to_string();

        for specifier in &import_decl.specifiers {
          match &specifier {
            ImportSpecifier::Default(import_specifier) => {
              if self.state.import_as(&import_decl.src.value).is_none() {
                let local_name = import_specifier.local.sym.to_string();

                self.state.import_paths.insert(source_path.clone());

                self
                  .state
                  .stylex_import
                  .insert(ImportSources::Regular(local_name));
              };
            }
            ImportSpecifier::Namespace(import_specifier) => {
              if self.state.import_as(&import_decl.src.value).is_none() {
                let local_name = import_specifier.local.sym.to_string();

                self.state.import_paths.insert(source_path.clone());

                self
                  .state
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
                    ModuleExportName::Str(strng) => strng.value.to_string(),
                  };

                  self.fill_stylex_create_import(
                    &source_path,
                    imported_name,
                    &local_name,
                    import_specifier,
                  );
                }
                None => {
                  let imported_name = import_specifier.local.sym.to_string();

                  self.fill_stylex_create_import(
                    &source_path,
                    imported_name,
                    &local_name,
                    import_specifier,
                  );
                }
              }
            }
          };
        }
      }

      import_decl
    } else {
      import_decl
    }
  }

  fn fill_stylex_create_import(
    &mut self,
    source_path: &str,
    imported_name: String,
    local_name: &str,
    import_specifier: &ImportNamedSpecifier,
  ) {
    if let Some(source_path) = self.state.import_as(source_path) {
      if source_path.eq(&imported_name) {
        self.state.import_paths.insert(source_path.clone());

        self
          .state
          .stylex_import
          .insert(ImportSources::Regular(local_name.to_string()));
      }
    }

    if self.state.import_as(source_path).is_none() {
      self.state.import_paths.insert(source_path.to_string());

      let local_name_ident_atom = import_specifier.local.clone().sym;

      match imported_name.as_str() {
        "create" => {
          self
            .state
            .stylex_create_import
            .insert(local_name_ident_atom);
        }
        "props" => {
          self.state.stylex_props_import.insert(local_name_ident_atom);
        }
        "attrs" => {
          self.state.stylex_attrs_import.insert(local_name_ident_atom);
        }
        "keyframes" => {
          self
            .state
            .stylex_keyframes_import
            .insert(local_name_ident_atom);
        }
        "include" => {
          self
            .state
            .stylex_include_import
            .insert(local_name_ident_atom);
        }
        "firstThatWorks" => {
          self
            .state
            .stylex_first_that_works_import
            .insert(local_name_ident_atom);
        }
        "defineVars" => {
          self
            .state
            .stylex_define_vars_import
            .insert(local_name_ident_atom);
        }
        "createTheme" => {
          self
            .state
            .stylex_create_theme_import
            .insert(local_name_ident_atom);
        }
        "types" => {
          self.state.stylex_types_import.insert(local_name_ident_atom);
        }
        _ => {
          unreachable!("{}", MUST_BE_DEFAULT_IMPORT)
        }
      }
    }
  }
}
