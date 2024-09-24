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
  utils::{common::increase_ident_count, js::evaluate::evaluate},
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

  let mut obj_name: Option<Atom> = None;
  let mut prop_name: Option<&Atom> = None;

  if let Expr::Ident(ident) = object {
    let obj_ident_name = ident.sym.to_string();

    obj_name = Some(ident.sym.clone());

    if state.style_map.contains_key(&obj_ident_name) {
      match property {
        MemberProp::Ident(ident) => {
          prop_name = Some(&ident.sym);
        }
        MemberProp::Computed(computed) => {
          assert!(
            !computed.expr.is_lit(),
            "Computed not implemented yet for non literal expressions"
          );
        }
        _ => {}
      }
    }
  }

  let style_non_null_props: NonNullProps;

  if let Some(bail_out_index) = bail_out_index {
    if index > bail_out_index {
      *non_null_props = NonNullProps::True;
    }
  }

  if let NonNullProps::True = non_null_props {
    style_non_null_props = NonNullProps::True;
  } else {
    let evaluate_result = evaluate(&Box::new(Expr::from(member.clone())), state, fns);

    if !evaluate_result.confident {
      *non_null_props = NonNullProps::True;
      style_non_null_props = NonNullProps::True;
    } else {
      if let NonNullProps::True = non_null_props {
        style_non_null_props = NonNullProps::True;
      } else {
        style_non_null_props = non_null_props.clone();
      }

      if let NonNullProps::Vec(vec) = non_null_props {
        if let Some(EvaluateResultValue::Expr(expr)) = evaluate_result.value.map(|v| *v) {
          if let Expr::Object(ObjectLit { props, .. }) = expr.as_ref() {
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
                        .map(|ident| ident.sym.clone())
                        .expect("Key not an ident"),
                    ),
                  },
                  _ => unimplemented!(),
                },
              })
              .collect::<Vec<Atom>>();

            vec.extend(namespaces);
          }
        }
      }
    }
  }

  if let Some(obj_name) = obj_name {
    increase_ident_count(state, object.as_ident().expect("Object not an ident"));

    let style_var_to_keep = StyleVarsToKeep(
      obj_name,
      match prop_name {
        Some(prop_name) => NonNullProp::Atom(prop_name.clone()),
        None => NonNullProp::True,
      },
      style_non_null_props,
    );

    state.style_vars_to_keep.insert(Box::new(style_var_to_keep));
  }
}
