use swc_core::{
  common::comments::Comments,
  ecma::{
    ast::{ImportDecl, ImportNamedSpecifier, ImportSpecifier, ModuleExportName},
    utils::drop_span,
  },
};

use crate::{
  StyleXTransform,
  shared::{
    enums::core::TransformationCycle, structures::named_import_source::ImportSources,
    utils::ast::convertors::atom_to_string,
  },
};

impl<C> StyleXTransform<C>
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

      self.state.top_imports.push(drop_span(import_decl.clone()));

      let source_path = atom_to_string(&import_decl.src.value);

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

            if import_sources.contains(&atom_to_string(declaration))
              && self
                .state
                .import_as(&atom_to_string(&import_decl.src.value))
                .is_none()
            {
              let local_name = import_specifier.local.sym.to_string();

              self.state.import_paths.insert(source_path.clone());

              self
                .state
                .stylex_import
                .insert(ImportSources::Regular(local_name));
            }
          }
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

            if import_sources.contains(&atom_to_string(declaration))
              && self
                .state
                .import_as(&atom_to_string(&import_decl.src.value))
                .is_none()
            {
              let local_name = import_specifier.local.sym.to_string();

              self.state.import_paths.insert(source_path.clone());

              self
                .state
                .stylex_import
                .insert(ImportSources::Regular(local_name));
            }
          }
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

            if import_sources.contains(&atom_to_string(declaration)) {
              let local_name = import_specifier.local.sym.to_string();

              match &import_specifier.imported {
                Some(imported) => {
                  let imported_name = match imported {
                    ModuleExportName::Ident(ident) => ident.sym.to_string(),
                    ModuleExportName::Str(strng) => atom_to_string(&strng.value),
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
          }
        };
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
        "keyframes" => {
          self
            .state
            .stylex_keyframes_import
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
        "defineConsts" => {
          self
            .state
            .stylex_define_consts_import
            .insert(local_name_ident_atom);
        }
        "createTheme" => {
          self
            .state
            .stylex_create_theme_import
            .insert(local_name_ident_atom);
        }
        "positionTry" => {
          self
            .state
            .stylex_position_try_import
            .insert(local_name_ident_atom);
        }
        "viewTransitionClass" => {
          self
            .state
            .stylex_view_transition_class_import
            .insert(local_name_ident_atom);
        }
        "types" => {
          self.state.stylex_types_import.insert(local_name_ident_atom);
        }
        "when" => {
          self.state.stylex_when_import.insert(local_name_ident_atom);
        }
        "defaultMarker" => {
          self
            .state
            .stylex_default_marker_import
            .insert(local_name_ident_atom);
        }
        _ => {}
      }
    }
  }
}
