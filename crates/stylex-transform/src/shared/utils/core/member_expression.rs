use stylex_constants::constants::messages::{OBJECT_KEY_MUST_BE_IDENT, SPREAD_NOT_SUPPORTED};
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::{
  atoms::Atom,
  ecma::{
    ast::{Expr, Lit, MemberExpr, ObjectLit, Prop, PropOrSpread},
    visit::{VisitMut, noop_visit_mut_type},
  },
};

use stylex_enums::style_vars_to_keep::{NonNullProp, NonNullProps};
use stylex_structures::style_vars_to_keep::StyleVarsToKeep;

use stylex_ast::ast::keys::namespace_name_from_member_prop;

use stylex_evaluator::evaluate::evaluate;
use stylex_state::{
  evaluate_result_value::EvaluateResultValue,
  functions::FunctionMap,
  state_manager::{DeclId, StateManager},
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

  let mut obj_id: Option<DeclId> = None;
  let mut prop_name: Option<Atom> = None;

  if let Expr::Ident(ident) = object
    && state.is_style_var_ident(ident)
  {
    obj_id = Some(ident.to_id());
    prop_name = namespace_name_from_member_prop(property);
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
        let namespaces = props.iter().filter_map(|item| match item {
          PropOrSpread::Spread(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
          PropOrSpread::Prop(prop) => match prop.as_ref() {
            Prop::KeyValue(key_value) => match key_value.value.as_ref() {
              Expr::Lit(Lit::Null(_)) => None,
              _ => Some(match key_value.key.as_ident().map(|ident| &ident.sym) {
                Some(sym) => sym,
                None => stylex_panic!("{}", OBJECT_KEY_MUST_BE_IDENT),
              }),
            },
            _ => stylex_unimplemented!(
              "This property variant is not supported in member expression evaluation."
            ),
          },
        });

        vec.extend(namespaces.cloned());
      }
    }
  }

  if let Some(obj_id) = obj_id {
    let style_var_to_keep = StyleVarsToKeep(
      obj_id,
      match prop_name {
        Some(prop_name) => NonNullProp::Atom(prop_name),
        None => NonNullProp::True,
      },
      style_non_null_props,
    );

    state.style_vars_to_keep.insert(style_var_to_keep);
  }
}

/// Walks the member expressions of a `stylex.props`-family call argument and
/// records which style variables and namespaces the runtime still needs.
pub(crate) struct MemberTransform<'a> {
  pub(crate) index: i32,
  pub(crate) bail_out_index: Option<i32>,
  pub(crate) non_null_props: NonNullProps,
  pub(crate) state: &'a mut StateManager,
  pub(crate) functions: &'a FunctionMap,
}

impl VisitMut for MemberTransform<'_> {
  noop_visit_mut_type!();

  fn visit_mut_member_expr(&mut self, member: &mut MemberExpr) {
    member_expression(
      member,
      &mut self.index,
      &mut self.bail_out_index,
      &mut self.non_null_props,
      &mut *self.state,
      self.functions,
    );
  }
}
