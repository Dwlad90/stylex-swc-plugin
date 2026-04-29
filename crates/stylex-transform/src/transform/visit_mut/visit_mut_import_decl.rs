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
    if self.state.cycle == TransformationCycle::Skip {
      return;
    }

    if self.state.cycle == TransformationCycle::Initializing {
      if import_decl.type_only {
        return;
      }

      let src = &import_decl.src;
      let declaration = &src.value;

      let import_sources = self.state.import_sources_stringified();

      self.state.top_imports.push(drop_span(import_decl.clone()));

      let source_path = convert_atom_to_string(&import_decl.src.value);

      for specifier in &import_decl.specifiers {
        match &specifier {
          ImportSpecifier::Default(import_specifier) => {
            let import_specifier_ident = import_specifier.local.clone();

            let import_specifier_string = import_specifier_ident.sym.to_string();
            if !self
              .state
              .import_specifiers
              .contains(&import_specifier_string)
            {
              self.state.import_specifiers.push(import_specifier_string);
            }

            if import_sources.contains(&convert_atom_to_string(declaration))
              && self
                .state
                .import_as(&convert_atom_to_string(&import_decl.src.value))
                .is_none()
            {
              let local_name = import_specifier.local.sym.to_string();

              self.state.import_paths.insert(source_path.clone());

              self
                .state
                .stylex_import
                .insert(ImportSources::Regular(local_name));
            }
          },
          ImportSpecifier::Namespace(import_specifier) => {
            let import_specifier_ident = import_specifier.local.clone();

            let import_specifier_string = import_specifier_ident.sym.to_string();

            if !self
              .state
              .import_specifiers
              .contains(&import_specifier_string)
            {
              self.state.import_specifiers.push(import_specifier_string);
            }

            if import_sources.contains(&convert_atom_to_string(declaration))
              && self
                .state
                .import_as(&convert_atom_to_string(&import_decl.src.value))
                .is_none()
            {
              let local_name = import_specifier.local.sym.to_string();

              self.state.import_paths.insert(source_path.clone());

              self
                .state
                .stylex_import
                .insert(ImportSources::Regular(local_name));
            }
          },
          ImportSpecifier::Named(import_specifier) => {
            let import_specifier_ident = import_specifier.local.clone();

            let import_specifier_string = import_specifier_ident.sym.to_string();

            if !self
              .state
              .import_specifiers
              .contains(&import_specifier_string)
            {
              self.state.import_specifiers.push(import_specifier_string);
            }

            if import_sources.contains(&convert_atom_to_string(declaration)) {
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
    if let Some(source_path) = self.state.import_as(source_path)
      && source_path.eq(&imported_name)
    {
      self.state.import_paths.insert(source_path.clone());

      self
        .state
        .stylex_import
        .insert(ImportSources::Regular(local_name.to_string()));
    }

    if self.state.import_as(source_path).is_none() {
      self.state.import_paths.insert(source_path.to_string());

      let local_name_ident_atom = import_specifier.local.clone().sym;

      if let Some(kind) = ImportKind::from_import_name(imported_name.as_str()) {
        self
          .state
          .insert_stylex_api_import(kind, local_name_ident_atom);
      }
    }
  }
}
