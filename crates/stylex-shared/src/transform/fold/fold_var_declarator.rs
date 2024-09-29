use std::collections::{hash_map::Entry, HashMap};

use swc_core::{
  atoms::Atom,
  common::{comments::Comments, EqIgnoreSpan},
  ecma::{
    ast::{Expr, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, VarDeclarator},
    visit::FoldWith,
  },
};

use crate::{
  shared::{
    enums::{
      core::TransformationCycle,
      data_structures::{
        style_vars_to_keep::{NonNullProp, NonNullProps, StyleVarsToKeep},
        top_level_expression::{TopLevelExpression, TopLevelExpressionKind},
      },
    },
    utils::ast::convertors::transform_shorthand_to_key_values,
  },
  StyleXTransform,
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
          let mut vars_to_keep: HashMap<Atom, NonNullProps> = HashMap::new();

          for StyleVarsToKeep(var_name, namespace_name, _) in self
            .state
            .style_vars_to_keep
            .clone()
            .into_iter()
            .map(|item| *item)
          {
            match vars_to_keep.entry(var_name) {
              Entry::Occupied(mut entry) => {
                if let NonNullProps::Vec(vec) = entry.get_mut() {
                  if let NonNullProp::Atom(id) = namespace_name {
                    vec.push(id);
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
            if var_declarator.name != var_name.name {
              continue;
            };

            if let Some(TopLevelExpression(kind, _, _)) =
              self.state.top_level_expressions.clone().into_iter().find(
                |TopLevelExpression(_, expr, _)| {
                  var_name.init.clone().unwrap().eq(&Box::new(expr.clone()))
                },
              )
            {
              if TopLevelExpressionKind::Stmt == kind {
                if let Some(object) = var_declarator.init.as_mut() {
                  if let Some(mut object) = object.as_object().cloned() {
                    let namespaces_to_keep =
                      match vars_to_keep.get(&var_name.name.as_ident().unwrap().sym) {
                        Some(NonNullProps::Vec(vec)) => vec.clone(),
                        _ => vec![],
                      };

                    if !namespaces_to_keep.is_empty() {
                      let props = self.retain_object_props(
                        &mut object,
                        namespaces_to_keep,
                        var_name.as_ref(),
                      );

                      object.props = props;

                      var_declarator.init = Some(Box::new(Expr::from(object)));
                    }
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
    &mut self,
    object: &mut ObjectLit,
    namespace_to_keep: Vec<Atom>,
    var_name: &VarDeclarator,
  ) -> Vec<PropOrSpread> {
    let mut props: Vec<PropOrSpread> = vec![];

    for object_prop in object.props.iter_mut() {
      assert!(object_prop.is_prop(), "Spread properties are not supported");

      let prop = object_prop.as_mut_prop().unwrap().as_mut();

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
                let StyleVarsToKeep(var, namespace_name, prop) = top_level_expression.as_ref();

                if var_id.eq(var) && namespace_name.eq(&key_id.clone()) {
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
