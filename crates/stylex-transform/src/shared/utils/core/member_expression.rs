use std::rc::Rc;

use swc_core::{
  atoms::Atom,
  ecma::{
    ast::{Expr, Lit, MemberExpr, ObjectLit},
    visit::{Visit, noop_visit_type},
  },
};

use stylex_enums::style_vars_to_keep::{NonNullProp, NonNullProps};
use stylex_structures::style_vars_to_keep::StyleVarsToKeep;

use stylex_ast::ast::keys::{namespace_name_from_member_prop, try_namespace_name_from_prop_key};

use stylex_evaluator::evaluate::evaluate_with_functions;
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
  fns: &Rc<FunctionMap>,
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
    let evaluate_result =
      evaluate_with_functions(&Expr::from(member.clone()), state, Rc::clone(fns));

    let style_value = evaluate_result.value;
    let confident = evaluate_result.confident;

    if !confident {
      *non_null_props = NonNullProps::True;
      style_non_null_props = NonNullProps::True;
    } else {
      // Not `True` here: the branch above took that case, and nothing since has
      // written to the counter.
      style_non_null_props = non_null_props.clone();

      if let NonNullProps::Vec(vec) = non_null_props
        && let Some(EvaluateResultValue::Expr(Expr::Object(ObjectLit { props, .. }))) = style_value
      {
        // The evaluator rebuilds every object it folds, so each property that
        // arrives here is a key-value pair: a spread is already merged away.
        // That is asserted where the object is written, by
        // `every_written_object_carries_key_value_properties_only` in the
        // evaluator, not argued from here. What is left to decide is the value:
        // a property declared as absent names nothing the runtime still needs.
        //
        // The key is read by name and not by shape. The reference lists the
        // keys of the folded object, which names a quoted key as readily as a
        // bare one, so a namespace spelled `'--my-color'` would otherwise be
        // missing from the keep list and swept away.
        let namespaces = props.iter().filter_map(|item| {
          item
            .as_prop()
            .and_then(|prop| prop.as_key_value())
            .filter(|key_value| !matches!(key_value.value.as_ref(), Expr::Lit(Lit::Null(_))))
            .and_then(|key_value| try_namespace_name_from_prop_key(&key_value.key))
        });

        vec.extend(namespaces);
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
///
/// A reader, not a writer: [`member_expression`] takes the node by shared
/// reference and writes only to the state and to the three counters here. The
/// walk says so through `Visit`, which is what keeps it independent of the
/// hoisting walk that runs beside it.
pub(crate) struct MemberTransform<'a> {
  pub(crate) index: i32,
  pub(crate) bail_out_index: Option<i32>,
  pub(crate) non_null_props: NonNullProps,
  pub(crate) state: &'a mut StateManager,
  pub(crate) functions: &'a Rc<FunctionMap>,
}

impl Visit for MemberTransform<'_> {
  noop_visit_type!();

  // Deliberately does not walk the children of a member expression. The index
  // counts one step per member expression the argument holds, and a nested one
  // would count twice and move the bail-out point.
  fn visit_member_expr(&mut self, member: &MemberExpr) {
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

#[cfg(test)]
#[path = "tests/member_expression_tests.rs"]
mod tests;
