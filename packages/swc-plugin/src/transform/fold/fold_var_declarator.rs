use std::{
  collections::{hash_map::Entry, HashMap},
  ops::Deref,
};

use swc_core::{
  common::{comments::Comments, EqIgnoreSpan},
  ecma::{
    ast::{Expr, Id, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, VarDeclarator},
    visit::FoldWith,
  },
};

use crate::{
  shared::{
    enums::{
      ModuleCycle, NonNullProp, NonNullProps, StyleVarsToKeep, TopLevelExpression,
      TopLevelExpressionKind,
    },
    utils::common::transform_shorthand_to_key_values,
  },
  ModuleTransformVisitor,
};

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn fold_var_declarator_impl(
    &mut self,
    mut var_declarator: VarDeclarator,
  ) -> VarDeclarator {
    // Get the declarations from the VarDecl struct
    // let var_declarator_id = var_declarator.clone().name.as_ident().unwrap().to_id();
    // let stylex_var_declarator = self.declaration.clone().unwrap();

    if self.cycle != ModuleCycle::Initializing
      && self.cycle != ModuleCycle::TransformEnter
      && self.cycle != ModuleCycle::TransformExit
      && self.cycle != ModuleCycle::PreCleaning
    {
      if self.cycle == ModuleCycle::Cleaning {
        let mut vars_to_keep: HashMap<Id, NonNullProps> = HashMap::new();

        for StyleVarsToKeep(var_name, namespace_name, _) in self
          .state
          .style_vars_to_keep
          .clone()
          .into_iter()
          .map(|item| *item)
        {
          match vars_to_keep.entry(var_name.clone()) {
            Entry::Occupied(mut entry) => {
              if let NonNullProps::Vec(vec) = entry.get_mut() {
                if let NonNullProp::Id(id) = &namespace_name {
                  vec.push(id.clone());
                }
              }
            }
            Entry::Vacant(entry) => {
              let value = match namespace_name {
                NonNullProp::Id(namespace_name) => NonNullProps::Vec(vec![namespace_name]),
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

          let var_decl = self.state.top_level_expressions.clone().into_iter().find(
            |TopLevelExpression(_, expr, _)| {
              var_name.init.clone().unwrap().eq(&Box::new(expr.clone()))
            },
          );
          if let Option::Some(TopLevelExpression(kind, _, _)) = var_decl {
            if TopLevelExpressionKind::Stmt == kind {
              if let Some(object) = var_declarator.init.as_mut() {
                if let Some(mut object) = object.as_object().cloned() {
                  let namespaces_to_keep =
                    match vars_to_keep.get(&var_name.name.as_ident().unwrap().to_id()) {
                      Some(e) => match e {
                        NonNullProps::Vec(vec) => vec.clone(),
                        NonNullProps::True => vec![],
                      },
                      None => vec![],
                    };

                  //   &namespaces_to_keep,
                  //   &vars_to_keep,
                  //   &var_name.name,
                  //   &self.state.style_vars_to_keep
                  // );

                  // todo!();

                  if !namespaces_to_keep.is_empty() {
                    let props =
                      self.retain_object_props(&object, namespaces_to_keep, var_name.as_ref());

                    // if props.is_empty(){
                    //   panic!("No properties to keep");
                    // }

                    object.props = props;

                    var_declarator.init = Option::Some(Box::new(Expr::Object(object)));
                  }
                }
              }
            }
          }
        }
      }

      return var_declarator;
    }

    if let Some(Expr::Call(call)) = var_declarator.init.as_deref() {
      if let Some((declaration, member)) = self.process_declaration(call) {
        let stylex_imports = self.state.stylex_import_stringified();

        let declaration_string = stylex_imports
          .into_iter()
          .find(|item| item.eq(declaration.0.as_str()))
          .or_else(|| {
            self
              .state
              .stylex_create_import
              .iter()
              .find(|decl| decl.eq_ignore_span(&&Box::new(declaration.clone())))
              .map(|decl| decl.0.to_string())
          });

        if let Some(declaration_string) = declaration_string {
          if self.cycle == ModuleCycle::Initializing
            && (member.as_str() == "create" || member.eq(declaration_string.as_str()))
          {
            self.props_declaration = var_declarator.name.as_ident().map(|ident| ident.to_id());
          }
        }
      }
    }

    // Call the fold_children_with method on the VarDecl struct
    var_declarator.fold_children_with(self)
  }

  fn retain_object_props(
    &mut self,
    object: &ObjectLit,
    namespace_to_keep: Vec<Id>,
    var_name: &VarDeclarator,
  ) -> Vec<PropOrSpread> {
    let props = object
      .props
      .clone()
      .into_iter()
      .filter_map(|mut object_prop| {
        assert!(object_prop.is_prop(), "Spread properties are not supported");

        let prop = object_prop.as_mut_prop().unwrap().as_mut();

        if let Some(KeyValueProp { key, .. }) = prop.as_key_value() {
          let key_as_ident = match key {
            PropName::Ident(ident) => Option::Some(ident),
            // PropName::Str(str) => {
            //     Option::Some(str.value.to_string())
            // }
            // PropName::Num(num) => {
            //     Option::Some(num.value.to_string())
            // }
            // PropName::Computed(_) => Option::None,
            // PropName::BigInt(big_int) => {
            //     Option::Some(big_int.value.to_string())
            // }
            _ => None,
          };

          if let Some(key_as_string) = key_as_ident {
            if namespace_to_keep.contains(&key_as_string.to_id()) {
              let all_nulls_to_keep = self
                .state
                .style_vars_to_keep
                .clone()
                .into_iter()
                .filter(|top_level_expression| {
                  let StyleVarsToKeep(v, namespace_name, _) = top_level_expression.as_ref();

                  var_name.name.clone().as_ident().unwrap().to_id().eq(v)
                    && namespace_name.eq(&NonNullProp::Id(key_as_ident.unwrap().to_id()))
                })
                .map(|a| a.2)
                .collect::<Vec<NonNullProps>>();

              if !all_nulls_to_keep.contains(&NonNullProps::True) {
                let nulls_to_keep = all_nulls_to_keep
                  .into_iter()
                  .filter_map(|item| match item {
                    NonNullProps::Vec(vec) => Option::Some(vec),
                    NonNullProps::True => Option::None,
                  })
                  .flatten()
                  .collect::<Vec<Id>>();

                if let Some(style_object) = prop
                  .as_mut_key_value()
                  .expect("Prop not a key value")
                  .value
                  .as_mut_object()
                {
                  retain_style_props(style_object, nulls_to_keep);
                }
              }

              return Some(object_prop);
            }
          }
        }

        Option::None
      })
      .collect::<Vec<PropOrSpread>>();

    props
  }
}

fn retain_style_props(style_object: &mut ObjectLit, nulls_to_keep: Vec<Id>) {
  style_object.props.retain(|prop| match prop {
    PropOrSpread::Prop(prop) => {
      let mut prop = prop.clone();
      transform_shorthand_to_key_values(&mut prop);

      match prop.deref() {
        Prop::KeyValue(key_value) => {
          let value = key_value.value.clone();

          if let Some(Lit::Null(_)) = value.as_lit() {
            let style_key_as_ident = match key_value.key.clone() {
              PropName::Ident(ident) => Option::Some(ident),
              _ => todo!("PropName not an ident"),
            };

            style_key_as_ident.map_or(false, |style_key_as_string| {
              nulls_to_keep.contains(&style_key_as_string.to_id())
            })
          } else {
            true
          }
        }
        _ => true,
      }
    }
    PropOrSpread::Spread(_) => true,
  });
}
