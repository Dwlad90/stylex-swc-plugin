use swc_core::ecma::ast::{Expr, Id, Lit, MemberExpr, MemberProp, ObjectLit, Prop, PropOrSpread};

use crate::shared::{
  enums::{NonNullProp, NonNullProps, StyleVarsToKeep},
  structures::{
    evaluate_result::EvaluateResultValue, functions::FunctionMap, state_manager::StateManager,
  },
  utils::{common::increase_ident_count, css::stylex::evaluate::evaluate},
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

  let mut obj_name: Option<Id> = Option::None;
  let mut prop_name: Option<Id> = Option::None;

  if let Expr::Ident(ident) = object {
    let obj_ident_name = ident.sym.to_string();

    obj_name = Option::Some(ident.to_id());

    if state.style_map.contains_key(&obj_ident_name) {
      match property {
        MemberProp::Ident(ident) => {
          // let prop_ident_name = ident.sym.to_string();

          prop_name = Option::Some(ident.to_id());
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
    let evaluate_result = evaluate(&Box::new(Expr::Member(member.clone())), state, fns);

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
                    Expr::Lit(Lit::Null(_)) => Option::None,
                    _ => Option::Some(
                      key_value
                        .key
                        .as_ident()
                        .expect("Key not an ident")
                        .to_id()
                        .clone(),
                    ),
                  },
                  _ => unimplemented!(),
                },
              })
              .collect::<Vec<Id>>();

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
        Some(prop_name) => NonNullProp::Id(prop_name.clone()),
        None => NonNullProp::True,
      },
      style_non_null_props,
    );

    state.style_vars_to_keep.insert(Box::new(style_var_to_keep));
  }
}
