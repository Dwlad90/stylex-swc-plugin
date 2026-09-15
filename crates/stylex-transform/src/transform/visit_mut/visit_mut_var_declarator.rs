use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;
use stylex_state::state_writers::fill_state_declarations;
use swc_core::{
  atoms::Atom,
  common::comments::Comments,
  ecma::{
    ast::{
      CallExpr, Callee, Lit, ObjectLit, ObjectPatProp, Pat, Prop, PropName, PropOrSpread,
      VarDeclarator,
    },
    visit::VisitMutWith,
  },
};

use stylex_ast::ast::convertors::{
  convert_str_lit_to_string, expand_shorthand_prop, init_call, normalize_expr,
};
use stylex_enums::{
  style_vars_to_keep::{NonNullProp, NonNullProps},
  top_level_expression::TopLevelExpressionKind,
};
use stylex_structures::{
  style_vars_to_keep::StyleVarsToKeep, top_level_expression::TopLevelExpression,
};

use crate::StyleXTransform;
use stylex_ast::ast::keys::namespace_name_from_prop_key;
use stylex_atoms::transform::ATOMS_SOURCE;
use stylex_enums::core::TransformationCycle;
use stylex_state::state_manager::{DeclId, ImportKind};
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

        // The style variable this declarator is, and the top-level expression
        // it was declared as -- both found by the name they are bound to rather
        // than by a walk of every style variable and every recorded expression
        // in the module, which ran once per declarator the finalize cycle
        // visits.
        if let Some((binding, init)) = self.state.matching_style_var(var_declarator) {
          let declared_as_a_statement = matches!(
            self.state.find_top_level_expr_named(&binding.sym, init),
            Some(TopLevelExpression(TopLevelExpressionKind::Stmt, _, _))
          );

          // Read before the object below is reached for, which needs the
          // declarator to itself.
          let var_id = binding.id.to_id();

          if declared_as_a_statement
            && let Some(object) = var_declarator
              .init
              .as_mut()
              .and_then(|var_decl| var_decl.as_mut_object())
          {
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
    let Some(call) = init_call(var_declarator) else {
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
    let Some(call) = init_call(var_declarator) else {
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

  /// The props of a compiled style object, swept down to the namespaces the
  /// module still reads.
  ///
  /// The object was written by `convert_object_to_ast` a few phases back, and
  /// that step writes every prop as a key-value under a name -- asserted where
  /// the object is built, by `writes_every_prop_as_a_key_value_under_a_name`.
  /// A prop that is not leaves the whole object as it is, because a sweep
  /// cannot tell what such an entry carries.
  fn retain_object_props(
    &self,
    object: &mut ObjectLit,
    namespace_to_keep: &[Atom],
    var_id: &DeclId,
  ) -> Vec<PropOrSpread> {
    // The namespace each prop names, read once. A `None` here answers for the
    // whole object, so the sweep below runs over props it has already read.
    let mut namespace_names = Vec::with_capacity(object.props.len());

    for object_prop in object.props.iter() {
      match object_prop
        .as_prop()
        .and_then(|prop| prop.as_key_value())
        .and_then(|key_value| namespace_name_from_prop_key(&key_value.key))
      {
        Some(namespace_name) => namespace_names.push(namespace_name),
        None => return object.props.clone(),
      }
    }

    let mut props: Vec<PropOrSpread> = Vec::with_capacity(object.props.len());

    for (object_prop, namespace_name) in object.props.iter_mut().zip(namespace_names) {
      if !namespace_to_keep.contains(&namespace_name) {
        continue;
      }

      let key_id = NonNullProp::Atom(namespace_name);

      // What this namespace keeps of its null declarations: every list recorded
      // against it, unless one entry keeps the namespace whole, in which case
      // nothing is swept out of it.
      let mut keeps_every_null = false;
      let mut nulls_to_keep: Vec<Atom> = Vec::new();

      for StyleVarsToKeep(var, recorded_name, prop) in self.state.style_vars_to_keep.iter() {
        if var != var_id || recorded_name != &key_id {
          continue;
        }

        match prop {
          NonNullProps::Vec(vec) => nulls_to_keep.extend(vec.iter().cloned()),
          NonNullProps::True => keeps_every_null = true,
        }
      }

      if !keeps_every_null
        && let Some(style_object) = object_prop
          .as_mut_prop()
          .and_then(|prop| prop.as_mut_key_value())
          .and_then(|key_value| key_value.value.as_mut_object())
      {
        retain_style_props(style_object, nulls_to_keep);
      }

      props.push(object_prop.clone())
    }

    props
  }
}

/// The module a `require` call names, or nothing where the call is not one.
///
/// The callee and the argument are both read through their parentheses. A
/// parenthesis is not a different call and not a different string, so
/// `(require)('@stylexjs/stylex')` and `require(('@stylexjs/stylex'))` name the
/// module the bare spelling names. Read bare, the import is never registered
/// and the whole module reaches the runtime unstyled, with no error for the
/// author to read.
///
/// One reader for both the StyleX source and the atoms source, so the two
/// cannot come to answer a spelling differently.
fn required_module(call: &CallExpr) -> Option<String> {
  let is_require_call = matches!(
    &call.callee,
    Callee::Expr(callee) if normalize_expr(callee).as_ident().is_some_and(|ident| ident.sym == "require")
  );

  if !is_require_call {
    return None;
  }

  let first_arg = call.args.first()?;

  if first_arg.spread.is_some() {
    return None;
  }

  match normalize_expr(&first_arg.expr).as_lit()? {
    Lit::Str(strng) => Some(convert_str_lit_to_string(strng)),
    _ => None,
  }
}

fn get_stylex_require_source(
  call: &CallExpr,
  state: &stylex_state::state_manager::StateManager,
) -> Option<String> {
  let source_path = required_module(call)?;

  state.is_import_source(&source_path).then_some(source_path)
}

/// Whether a call expression is `require('@stylexjs/atoms')`.
fn is_atoms_require(call: &CallExpr) -> bool {
  required_module(call).is_some_and(|source_path| source_path == ATOMS_SOURCE)
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

#[cfg(test)]
#[path = "tests/style_var_sweep_test.rs"]
mod tests;
