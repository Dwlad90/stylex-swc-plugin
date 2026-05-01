use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{ImportDecl, ImportNamedSpecifier, ImportSpecifier, ModuleExportName},
    utils::drop_span,
  },
};

use crate::{
  StyleXTransform,
  shared::{structures::state_manager::ImportKind, utils::ast::convertors::convert_atom_to_string},
};
use stylex_enums::core::TransformationCycle;
use stylex_structures::named_import_source::ImportSources;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_import_decl_impl(&mut self, import_decl: &mut ImportDecl) {
    if self.state.cycle == TransformationCycle::Initializing {
      if import_decl.type_only {
        return;
      }

      self.state.top_imports.push(drop_span(import_decl.clone()));

      let source_path = convert_atom_to_string(&import_decl.src.value);
      let is_import_source = self.state.is_import_source(&source_path);
      let has_named_import_source = self.state.import_as(&source_path).is_some();

      for specifier in &import_decl.specifiers {
        match &specifier {
          ImportSpecifier::Default(import_specifier) => {
            if is_import_source && !has_named_import_source {
              let local_name = import_specifier.local.sym.to_string();

              self.state.insert_import_path(source_path.clone());

              self
                .state
                .insert_stylex_import(ImportSources::Regular(local_name));
            }
          },
          ImportSpecifier::Namespace(import_specifier) => {
            if is_import_source && !has_named_import_source {
              let local_name = import_specifier.local.sym.to_string();

              self.state.insert_import_path(source_path.clone());

              self
                .state
                .insert_stylex_import(ImportSources::Regular(local_name));
            }
          },
          ImportSpecifier::Named(import_specifier) => {
            if is_import_source {
              let local_name = import_specifier.local.sym.to_string();

              match &import_specifier.imported {
                Some(imported) => {
                  let imported_name = match imported {
                    ModuleExportName::Ident(ident) => ident.sym.to_string(),
                    ModuleExportName::Str(strng) => convert_atom_to_string(&strng.value),
                  };

                  self.fill_stylex_create_import(
                    &source_path,
                    imported_name,
                    &local_name,
                    import_specifier,
                  );
                },
                None => {
                  let imported_name = import_specifier.local.sym.to_string();

                  self.fill_stylex_create_import(
                    &source_path,
                    imported_name,
                    &local_name,
                    import_specifier,
                  );
                },
              }
            }
          },
        };
      }
    }
  }

  fn fill_stylex_create_import(
    &mut self,
    source_path: &str,
    imported_name: String,
    local_name: &str,
    import_specifier: &ImportNamedSpecifier,
  ) {
    let import_alias = self.state.import_as(source_path);
    let has_import_alias = import_alias.is_some();
    let import_alias_matches = import_alias.is_some_and(|alias| alias.eq(&imported_name));

    if import_alias_matches {
      self.state.insert_import_path(imported_name.clone());

      self
        .state
        .insert_stylex_import(ImportSources::Regular(local_name.to_string()));
    }

    if !has_import_alias {
      self.state.insert_import_path(source_path.to_string());

      let local_name_ident_atom = import_specifier.local.clone().sym;

      if let Some(kind) = ImportKind::from_import_name(imported_name.as_str()) {
        self
          .state
          .insert_stylex_api_import(kind, local_name_ident_atom);
      }
    }
  }
}
