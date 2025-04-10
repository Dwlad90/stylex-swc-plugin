use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;
use swc_core::{
  atoms::Atom,
  common::{EqIgnoreSpan, comments::Comments},
  ecma::{
    ast::{Expr, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, VarDeclarator},
    visit::FoldWith,
  },
};

use crate::{
  StyleXTransform,
  shared::{
    enums::{
      core::TransformationCycle,
      data_structures::{
        style_vars_to_keep::{NonNullProp, NonNullProps, StyleVarsToKeep},
        top_level_expression::{TopLevelExpression, TopLevelExpressionKind},
      },
    },
    utils::{ast::convertors::transform_shorthand_to_key_values, common::fill_state_declarations},
  },
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn fold_var_declarator_impl(
    &mut self,
    mut var_declarator: VarDeclarator,
  ) -> VarDeclarator {
    match self.state.cycle {
      TransformationCycle::StateFilling => {
        fill_state_declarations(&mut self.state, &var_declarator);

        if let Some(Expr::Call(call)) = var_declarator.init.as_deref_mut() {
          if let Some((declaration, member)) = self.process_declaration(call) {
            let stylex_imports = self.state.stylex_import_stringified();

            if let Some(declaration_string) = stylex_imports
              .into_iter()
              .find(|item| item.eq(declaration.0.as_str()))
              .or_else(|| {
                self
                  .state
                  .stylex_create_import
                  .iter()
                  .find(|decl| decl.eq_ignore_span(&&declaration.0.clone()))
                  .map(|decl| decl.to_string())
              })
            {
              if self.state.cycle == TransformationCycle::StateFilling
                && (member.as_str() == "create" || member.eq(declaration_string.as_str()))
              {
                self.props_declaration = var_declarator.name.as_ident().map(|ident| ident.to_id());
              }
            }
          }
        }

        var_declarator.fold_children_with(self)
      }
      TransformationCycle::Cleaning => {
        {
          let mut vars_to_keep: FxHashMap<Atom, NonNullProps> = FxHashMap::default();

          for StyleVarsToKeep(var_name, namespace_name, _) in
            self.state.style_vars_to_keep.clone().into_iter()
          {
            match vars_to_keep.entry(var_name) {
              Entry::Occupied(mut entry) => {
                let entry_value = entry.get_mut();

                if let NonNullProps::Vec(vec) = entry_value {
                  match namespace_name {
                    NonNullProp::Atom(id) => {
                      vec.push(id);
                    }
                    _ => {
                      *entry_value = NonNullProps::True;
                    }
                  }
                }
              }
              Entry::Vacant(entry) => {
                let value = match namespace_name {
                  NonNullProp::Atom(namespace_name) => NonNullProps::Vec(vec![namespace_name]),
                  NonNullProp::True => NonNullProps::True,
                };

                entry.insert(value);
              }
            }
          }

          for (_, var_name) in self.state.style_vars.clone().into_iter() {
            if !var_declarator.eq_ignore_span(&var_name) {
              continue;
            };

            let top_level_expression = self.state.top_level_expressions.clone().into_iter().find(
              |TopLevelExpression(_, expr, _)| {
                var_name
                  .init
                  .clone()
                  .unwrap()
                  .eq_ignore_span(&Box::new(expr.clone()))
              },
            );

            if let Some(TopLevelExpression(kind, _, _)) = top_level_expression {
              if kind == TopLevelExpressionKind::Stmt {
                if let Some(object) = var_declarator
                  .init
                  .as_mut()
                  .and_then(|var_decl| var_decl.as_object())
                {
                  let namespaces_to_keep =
                    match vars_to_keep.get(&var_name.name.as_ident().unwrap().sym) {
                      Some(NonNullProps::Vec(vec)) => vec.clone(),
                      _ => Vec::new(),
                    };

                  if !namespaces_to_keep.is_empty() {
                    let mut new_object = object.clone();

                    let props =
                      self.retain_object_props(&mut new_object, namespaces_to_keep, &var_name);
                    new_object.props = props;

                    *var_declarator.init.as_mut().unwrap() = Box::new(Expr::Object(new_object));
                  }
                }
              }
            }
          }
        }

        var_declarator
      }
      TransformationCycle::Skip | TransformationCycle::InjectStyles => var_declarator,
      _ => var_declarator.fold_children_with(self),
    }
  }

  fn retain_object_props(
    &self,
    object: &mut ObjectLit,
    namespace_to_keep: Vec<Atom>,
    var_name: &VarDeclarator,
  ) -> Vec<PropOrSpread> {
    let mut props: Vec<PropOrSpread> = vec![];

    for object_prop in object.props.iter_mut() {
      assert!(object_prop.is_prop(), "Spread properties are not supported");

      let prop = object_prop
        .as_mut_prop()
        .expect("Property not found")
        .as_mut();

      if let Some(KeyValueProp { key, .. }) = prop.as_key_value() {
        let key_as_ident = match key {
          PropName::Ident(ident) => Some(ident),
          _ => None,
        };

        if let Some(key_as_string) = key_as_ident {
          if namespace_to_keep.contains(&key_as_string.sym) {
            let var_id = &var_name.name.as_ident().unwrap().sym;
            let key_id = NonNullProp::Atom(key_as_ident.unwrap().clone().sym);

            let all_nulls_to_keep = self
              .state
              .style_vars_to_keep
              .iter()
              .filter_map(|top_level_expression| {
                let StyleVarsToKeep(var, namespace_name, prop) = top_level_expression;

                if var_id.eq_ignore_span(var) && namespace_name.eq(&key_id.clone()) {
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

              if let Some(style_object) = prop
                .as_mut_key_value()
                .expect("Prop not a key value")
                .value
                .as_mut_object()
              {
                retain_style_props(style_object, nulls_to_keep);
              }
            }

            props.push(object_prop.clone())
          }
        }
      }
    }

    props
  }
}

fn retain_style_props(style_object: &mut ObjectLit, nulls_to_keep: Vec<Atom>) {
  style_object.props.retain(|prop| match prop {
    PropOrSpread::Prop(prop) => {
      let mut prop = prop.clone();

      transform_shorthand_to_key_values(&mut prop);

      if let Prop::KeyValue(key_value) = &*prop {
        if key_value
          .value
          .as_lit()
          .and_then(|lit| match lit {
            Lit::Null(_) => Some(()),
            _ => None,
          })
          .is_some()
          && matches!(key_value.key, PropName::Ident(_))
        {
          if let PropName::Ident(ident) = &key_value.key {
            return nulls_to_keep.contains(&ident.sym);
          }
        }
      }

      true
    }
    PropOrSpread::Spread(_) => true,
  });
}
