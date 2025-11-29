use swc_core::{
  atoms::Atom,
  ecma::ast::{Expr, Lit, MemberExpr, MemberProp, ObjectLit, Prop, PropOrSpread},
};

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    style_vars_to_keep::{NonNullProp, NonNullProps, StyleVarsToKeep},
  },
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::{ast::convertors::lit_to_string, common::increase_ident_count, js::evaluate::evaluate},
};

pub(crate) fn member_expression(
  member: &MemberExpr,
  index: &mut i32,
  bail_out_index: &mut Option<i32>,
  non_null_props: &mut NonNullProps,
  state: &mut StateManager,
  fns: &FunctionMap,
) {
  let object = member.obj.as_ref();
  let property = &member.prop;

  let mut obj_name: Option<&Atom> = None;
  let mut prop_name: Option<Atom> = None;

  if let Expr::Ident(ident) = object {
    let obj_ident_name = &ident.sym;

    obj_name = Some(&ident.sym);

    if state.style_map.contains_key(&obj_ident_name.to_string()) {
      match property {
        MemberProp::Ident(ident) => {
          prop_name = Some(ident.sym.clone());
        }
        MemberProp::Computed(computed) => {
          if let Expr::Lit(lit) = computed.expr.as_ref() {
            prop_name = lit_to_string(lit).map(Atom::from);
          }
        }
        _ => {}
      }
    }
  }

  let style_non_null_props: NonNullProps;

  if let Some(bail_out_index) = bail_out_index
    && index > bail_out_index
  {
    *non_null_props = NonNullProps::True;
  }

  if let NonNullProps::True = non_null_props {
    style_non_null_props = NonNullProps::True;
  } else {
    let evaluate_result = evaluate(&Box::new(Expr::from(member.clone())), state, fns);

    let style_value = evaluate_result.value;
    let confident = evaluate_result.confident;

    if !confident {
      *non_null_props = NonNullProps::True;
      style_non_null_props = NonNullProps::True;
    } else {
      if let NonNullProps::True = non_null_props {
        style_non_null_props = NonNullProps::True;
      } else {
        style_non_null_props = non_null_props.clone();
      }

      if let NonNullProps::Vec(vec) = non_null_props
        && let Some(EvaluateResultValue::Expr(Expr::Object(ObjectLit { props, .. }))) = style_value
      {
        let namespaces = props
          .iter()
          .filter_map(|item| match item {
            PropOrSpread::Spread(_) => unimplemented!("Spread"),
            PropOrSpread::Prop(prop) => match prop.as_ref() {
              Prop::KeyValue(key_value) => match key_value.value.as_ref() {
                Expr::Lit(Lit::Null(_)) => None,
                _ => Some(
                  key_value
                    .key
                    .as_ident()
                    .map(|ident| &ident.sym)
                    .expect("Key not an ident"),
                ),
              },
              _ => unimplemented!(),
            },
          })
          .collect::<Vec<&Atom>>();

        vec.extend(namespaces.into_iter().cloned());
      }
    }
  }

  if let Some(obj_name) = obj_name {
    increase_ident_count(state, object.as_ident().expect("Object not an ident"));

    let style_var_to_keep = StyleVarsToKeep(
      obj_name.clone(),
      match prop_name {
        Some(prop_name) => NonNullProp::Atom(prop_name.clone()),
        None => NonNullProp::True,
      },
      style_non_null_props,
    );

    state.style_vars_to_keep.insert(style_var_to_keep);
  }
}
