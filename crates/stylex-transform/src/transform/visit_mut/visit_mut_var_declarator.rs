use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;
use stylex_macros::stylex_panic;
use swc_core::{
  atoms::Atom,
  common::{EqIgnoreSpan, comments::Comments},
  ecma::{
    ast::{
      CallExpr, Callee, Expr, KeyValueProp, Lit, ObjectLit, ObjectPatProp, Pat, Prop, PropName,
      PropOrSpread, VarDeclarator,
    },
    visit::VisitMutWith,
  },
};

use stylex_enums::{
  style_vars_to_keep::{NonNullProp, NonNullProps},
  top_level_expression::TopLevelExpressionKind,
};
use stylex_structures::{
  style_vars_to_keep::StyleVarsToKeep, top_level_expression::TopLevelExpression,
};

use crate::{
  StyleXTransform,
  shared::{
    structures::state_manager::{DeclId, ImportKind},
    utils::{
      ast::{
        convertors::{convert_str_lit_to_string, expand_shorthand_prop},
        helpers::namespace_name_from_prop_key,
      },
      common::fill_state_declarations,
    },
  },
};
use stylex_atoms::transform::ATOMS_SOURCE;
use stylex_constants::constants::{
  api_names::STYLEX_CREATE,
  messages::{KEY_VALUE_EXPECTED, PROPERTY_NOT_FOUND, VAR_DECL_NAME_NOT_IDENT},
};
use stylex_enums::core::TransformationCycle;
use stylex_structures::named_import_source::ImportSources;

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn visit_mut_var_declarator_impl(&mut self, var_declarator: &mut VarDeclarator) {
    match self.state.cycle {
      TransformationCycle::Discover => {
        fill_state_declarations(&mut self.state, var_declarator);
        self.discover_commonjs_stylex_require(var_declarator);
        self.discover_commonjs_atoms_require(var_declarator);

        if let Some(Expr::Call(call)) = var_declarator.init.as_deref_mut()
          && let Some((declaration, member)) = self.process_declaration(call)
        {
          let declaration_name = declaration.0.as_str();

          if self
            .state
            .is_stylex_import_for_kinds(declaration_name, &[ImportKind::Create])
            && (member.as_str() == STYLEX_CREATE || member == declaration_name)
          {
            self.props_declaration = var_declarator.name.as_ident().map(|ident| ident.to_id());
          }
        }

        var_declarator.visit_mut_children_with(self);
      },
      TransformationCycle::Finalize => {
        let mut vars_to_keep: FxHashMap<DeclId, NonNullProps> = FxHashMap::default();

        for StyleVarsToKeep(var_id, namespace_name, _) in self.state.style_vars_to_keep.iter() {
          match vars_to_keep.entry(var_id.clone()) {
            Entry::Occupied(mut entry) => {
              let entry_value = entry.get_mut();

              if let NonNullProps::Vec(vec) = entry_value {
                match namespace_name {
                  NonNullProp::Atom(id) => {
                    vec.push(id.clone());
                  },
                  _ => {
                    *entry_value = NonNullProps::True;
                  },
                }
              }
            },
            Entry::Vacant(entry) => {
              let value = match namespace_name {
                NonNullProp::Atom(namespace_name) => {
                  NonNullProps::Vec(vec![namespace_name.clone()])
                },
                NonNullProp::True => NonNullProps::True,
              };

              entry.insert(value);
            },
          }
        }

        for (_, var_name) in self.state.style_vars.iter() {
          if !var_declarator.eq_ignore_span(var_name) {
            continue;
          };

          let top_level_expression = self.state.top_level_expressions.iter().find(
            |TopLevelExpression(_, expr, _)| match var_name.init.as_ref() {
              Some(init) => init.as_ref().eq_ignore_span(expr),
              None => {
                stylex_panic!(
                  "Variable declaration must have an initializer for top-level expression lookup."
                )
              },
            },
          );

          if let Some(TopLevelExpression(kind, _, _)) = top_level_expression
            && *kind == TopLevelExpressionKind::Stmt
            && let Some(object) = var_declarator
              .init
              .as_mut()
              .and_then(|var_decl| var_decl.as_mut_object())
          {
            let var_id = match var_name.name.as_ident() {
              Some(i) => i.id.to_id(),
              None => stylex_panic!("{}", VAR_DECL_NAME_NOT_IDENT),
            };

            let namespaces_to_keep = match vars_to_keep.get(&var_id) {
              Some(NonNullProps::Vec(vec)) => vec.clone(),
              _ => Vec::new(),
            };

            if !namespaces_to_keep.is_empty() {
              object.props = self.retain_object_props(object, &namespaces_to_keep, &var_id);
            }
          }
        }
      },
      _ => var_declarator.visit_mut_children_with(self),
    }
  }

  fn discover_commonjs_stylex_require(&mut self, var_declarator: &VarDeclarator) {
    let Some(call) = var_declarator.init.as_deref().and_then(Expr::as_call) else {
      return;
    };

    let Some(source_path) = get_stylex_require_source(call, &self.state) else {
      return;
    };

    let has_named_import_source = self.state.import_as(&source_path).is_some();

    let import_alias = self.state.import_as(&source_path).map(str::to_string);

    match &var_declarator.name {
      Pat::Ident(local) if !has_named_import_source => {
        self.state.insert_import_path(source_path);
        self
          .state
          .insert_stylex_import(ImportSources::Regular(local.id.sym.to_string()));
      },
      Pat::Ident(_) => {},
      Pat::Object(object) => {
        // Mirror the ES `import { foo } from 'stylex'` named-import path: when an
        // `import_sources: [{ from, as }]` alias is configured, a destructured
        // require whose imported name matches the alias should still register the
        // local binding as the StyleX namespace handle. Other destructured props
        // are skipped in that mode (the alias narrows what counts as a StyleX
        // import). Without an alias, fall back to the original behaviour of
        // tracking every prop that maps to a known `ImportKind`.
        self.state.insert_import_path(source_path);

        for prop in &object.props {
          let Some((imported_name, local_name, _local_id)) = destructured_require_prop(prop) else {
            continue;
          };

          match &import_alias {
            Some(alias) => {
              if imported_name == *alias {
                self
                  .state
                  .insert_stylex_import(ImportSources::Regular(local_name.to_string()));
              }
            },
            None => {
              if let Some(kind) = ImportKind::from_import_name(imported_name.as_str()) {
                self.state.insert_stylex_api_import(kind, local_name);
              }
            },
          }
        }
      },
      _ => {},
    }
  }

  /// Records `@stylexjs/atoms` imports introduced via CommonJS `require`.
  ///
  /// - `const css = require('@stylexjs/atoms')` → `css` maps to `"*"`
  /// - `const { color, padding: p } = require('@stylexjs/atoms')` → `color`
  ///   maps to `"color"` and `p` maps to `"padding"`
  fn discover_commonjs_atoms_require(&mut self, var_declarator: &VarDeclarator) {
    let Some(call) = var_declarator.init.as_deref().and_then(Expr::as_call) else {
      return;
    };

    if !is_atoms_require(call) {
      return;
    }

    match &var_declarator.name {
      Pat::Ident(local) => {
        self
          .state
          .atom_imports
          .insert(local.id.to_id(), "*".to_string());
      },
      Pat::Object(object) => {
        for prop in &object.props {
          if let Some((imported_name, _local_name, local_id)) = destructured_require_prop(prop) {
            self.state.atom_imports.insert(local_id, imported_name);
          }
        }
      },
      _ => {},
    }
  }

  fn retain_object_props(
    &self,
    object: &mut ObjectLit,
    namespace_to_keep: &[Atom],
    var_id: &DeclId,
  ) -> Vec<PropOrSpread> {
    let mut props: Vec<PropOrSpread> = Vec::with_capacity(object.props.len());

    for object_prop in object.props.iter() {
      let Some(prop) = object_prop.as_prop() else {
        return object.props.clone();
      };

      let Some(key_value) = prop.as_key_value() else {
        return object.props.clone();
      };

      if namespace_name_from_prop_key(&key_value.key).is_none() {
        return object.props.clone();
      }
    }

    for object_prop in object.props.iter_mut() {
      assert!(object_prop.is_prop(), "Spread properties are not supported");

      let prop = match object_prop.as_mut_prop() {
        Some(p) => p.as_mut(),
        None => stylex_panic!("{}", PROPERTY_NOT_FOUND),
      };

      let Some(KeyValueProp { key, .. }) = prop.as_key_value() else {
        return object.props.clone();
      };

      let Some(namespace_name) = namespace_name_from_prop_key(key) else {
        return object.props.clone();
      };

      if namespace_to_keep.contains(&namespace_name) {
        let key_id = NonNullProp::Atom(namespace_name);

        let all_nulls_to_keep = self
          .state
          .style_vars_to_keep
          .iter()
          .filter_map(|top_level_expression| {
            let StyleVarsToKeep(var, namespace_name, prop) = top_level_expression;

            if var == var_id && namespace_name == &key_id {
              Some(prop.clone())
            } else {
              None
            }
          })
          .collect::<Vec<NonNullProps>>();

        if !all_nulls_to_keep.contains(&NonNullProps::True) {
          let nulls_to_keep = all_nulls_to_keep
            .into_iter()
            .filter_map(|item| match item {
              NonNullProps::Vec(vec) => Some(vec),
              NonNullProps::True => None,
            })
            .flatten()
            .collect::<Vec<Atom>>();

          if let Some(style_object) = match prop.as_mut_key_value() {
            Some(kv) => kv,
            None => stylex_panic!("{}", KEY_VALUE_EXPECTED),
          }
          .value
          .as_mut_object()
          {
            retain_style_props(style_object, nulls_to_keep);
          }
        }

        props.push(object_prop.clone())
      }
    }

    props
  }
}

fn get_stylex_require_source(
  call: &CallExpr,
  state: &crate::shared::structures::state_manager::StateManager,
) -> Option<String> {
  let is_require_call = matches!(
    &call.callee,
    Callee::Expr(callee) if callee.as_ident().is_some_and(|ident| ident.sym == "require")
  );

  if !is_require_call {
    return None;
  }

  let first_arg = call.args.first()?;

  if first_arg.spread.is_some() {
    return None;
  }

  let source_path = match first_arg.expr.as_lit()? {
    Lit::Str(strng) => convert_str_lit_to_string(strng),
    _ => return None,
  };

  state.is_import_source(&source_path).then_some(source_path)
}

/// Whether a call expression is `require('@stylexjs/atoms')`.
fn is_atoms_require(call: &CallExpr) -> bool {
  let is_require_call = matches!(
    &call.callee,
    Callee::Expr(callee) if callee.as_ident().is_some_and(|ident| ident.sym == "require")
  );

  if !is_require_call {
    return false;
  }

  let Some(first_arg) = call.args.first() else {
    return false;
  };

  if first_arg.spread.is_some() {
    return false;
  }

  matches!(
    first_arg.expr.as_lit(),
    Some(Lit::Str(strng)) if convert_str_lit_to_string(strng) == ATOMS_SOURCE
  )
}

fn destructured_require_prop(
  prop: &ObjectPatProp,
) -> Option<(String, swc_core::atoms::Atom, swc_core::ecma::ast::Id)> {
  match prop {
    ObjectPatProp::Assign(assign) => {
      let name = assign.key.id.sym.clone();
      Some((name.to_string(), name, assign.key.id.to_id()))
    },
    ObjectPatProp::KeyValue(key_value) => {
      let imported_name = namespace_name_from_prop_key(&key_value.key)?;
      let (local_name, local_id) = local_binding_from_pat(&key_value.value)?;

      Some((imported_name.to_string(), local_name, local_id))
    },
    ObjectPatProp::Rest(_) => None,
  }
}

fn local_binding_from_pat(pat: &Pat) -> Option<(swc_core::atoms::Atom, swc_core::ecma::ast::Id)> {
  match pat {
    Pat::Ident(local) => Some((local.id.sym.clone(), local.id.to_id())),
    Pat::Assign(assign) => local_binding_from_pat(&assign.left),
    _ => None,
  }
}

fn retain_style_props(style_object: &mut ObjectLit, nulls_to_keep: Vec<Atom>) {
  style_object.props.retain(|prop| match prop {
    PropOrSpread::Prop(prop) => {
      let mut prop = prop.clone();

      expand_shorthand_prop(&mut prop);

      if let Prop::KeyValue(key_value) = &*prop
        && key_value
          .value
          .as_lit()
          .and_then(|lit| match lit {
            Lit::Null(_) => Some(()),
            _ => None,
          })
          .is_some()
        && matches!(key_value.key, PropName::Ident(_))
        && let PropName::Ident(ident) = &key_value.key
      {
        return nulls_to_keep.contains(&ident.sym);
      }

      true
    },
    PropOrSpread::Spread(_) => true,
  });
}
