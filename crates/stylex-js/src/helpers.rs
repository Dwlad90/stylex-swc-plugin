use stylex_constants::constants::common::{
  INVALID_METHODS, MUTATING_ARRAY_METHODS, MUTATING_OBJECT_METHODS, VALID_CALLEES,
};
use stylex_macros::stylex_panic;
use swc_core::{
  atoms::Atom,
  ecma::ast::{AssignTarget, Expr, Lit, MemberProp, SimpleAssignTarget, UnaryOp},
};

pub fn is_valid_callee(callee: &Expr) -> bool {
  if let Expr::Ident(ident) = callee {
    VALID_CALLEES.contains(ident.sym.as_ref())
  } else {
    false
  }
}

pub fn get_callee_name(callee: &Expr) -> &str {
  match callee {
    Expr::Ident(ident) => &ident.sym,
    _ => stylex_panic!("The function being called must be a static identifier."),
  }
}

pub fn is_invalid_method(prop: &MemberProp) -> bool {
  match prop {
    MemberProp::Ident(ident_prop) => INVALID_METHODS.contains(&*ident_prop.sym),
    _ => false,
  }
}

/// Checks if a member property represents a mutating object method (Object.assign, etc.)
pub fn is_mutating_object_method(prop: &MemberProp) -> bool {
  if let MemberProp::Ident(ident_prop) = prop {
    MUTATING_OBJECT_METHODS.contains(&*ident_prop.sym)
  } else {
    false
  }
}

/// Checks if a member property represents a mutating array method (push, pop, splice, etc.)
pub fn is_mutating_array_method(prop: &MemberProp) -> bool {
  if let MemberProp::Ident(ident_prop) = prop {
    MUTATING_ARRAY_METHODS.contains(&*ident_prop.sym)
  } else {
    false
  }
}

/// Checks if an expression represents a mutation operation
/// Returns true if any of the following conditions are met:
/// - Assignment to a member expression (e.g., `a.x = 1` or `a[0] = 1`)
/// - Update expression on a member (e.g., `++a.x` or `a[0]++`)
/// - Delete operation on a member (e.g., `delete a.x`)
pub fn is_mutation_expr(expr: &Expr) -> bool {
  match expr {
    // Check for assignment to member: a.x = 1 or a[0] = 1
    Expr::Assign(assign)
      if matches!(
        &assign.left,
        AssignTarget::Simple(SimpleAssignTarget::Member(member)) if member.obj.is_ident()
      ) =>
    {
      true
    },

    // Check for update on member: ++a.x or a[0]++
    Expr::Update(update) if matches!(&*update.arg, Expr::Member(member) if member.obj.is_ident()) => {
      true
    },

    // Check for delete on member: delete a.x
    Expr::Unary(unary)
      if unary.op == UnaryOp::Delete
        && matches!(&*unary.arg, Expr::Member(member) if member.obj.is_ident()) =>
    {
      true
    },

    _ => false,
  }
}

pub fn get_method_name(prop: &MemberProp) -> &str {
  match prop {
    MemberProp::Ident(ident_prop) => &ident_prop.sym,
    _ => stylex_panic!("The method name in a call expression must be a static identifier."),
  }
}

pub fn is_id_prop(prop: &MemberProp) -> Option<&Atom> {
  if let MemberProp::Computed(comp_prop) = prop
    && let Expr::Lit(Lit::Str(strng)) = comp_prop.expr.as_ref()
  {
    return strng.value.as_atom();
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;
  use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{
      AssignExpr, AssignOp, ComputedPropName, Ident, IdentName, MemberExpr, Number,
      SimpleAssignTarget, Str, UnaryExpr, UpdateExpr, UpdateOp,
    },
  };

  fn ident_expr(name: &str) -> Expr {
    Expr::Ident(Ident::new(name.into(), DUMMY_SP, Default::default()))
  }

  fn member_ident(name: &str) -> MemberProp {
    MemberProp::Ident(IdentName::new(name.into(), DUMMY_SP))
  }

  fn member_expr_with_ident_obj(prop: &str) -> MemberExpr {
    MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(ident_expr("obj")),
      prop: member_ident(prop),
    }
  }

  #[test]
  fn valid_callee_detection() {
    assert!(is_valid_callee(&ident_expr("Math")));
    assert!(!is_valid_callee(&ident_expr("console")));
    assert!(!is_valid_callee(&Expr::Lit(Lit::Num(Number {
      span: DUMMY_SP,
      value: 1.0,
      raw: None,
    }))));
  }

  #[test]
  fn get_callee_name_for_identifier() {
    assert_eq!(get_callee_name(&ident_expr("Array")), "Array");
  }

  #[test]
  #[should_panic(expected = "The function being called must be a static identifier.")]
  fn get_callee_name_panics_for_non_identifier() {
    let _ = get_callee_name(&Expr::Unary(UnaryExpr {
      span: DUMMY_SP,
      op: UnaryOp::Minus,
      arg: Box::new(ident_expr("x")),
    }));
  }

  #[test]
  fn method_sets_detect_expected_members() {
    let assign = member_ident("assign");
    let push = member_ident("push");
    let map = member_ident("map");

    assert!(is_invalid_method(&assign));
    assert!(is_mutating_object_method(&assign));
    assert!(!is_mutating_array_method(&assign));

    assert!(is_mutating_array_method(&push));
    assert!(!is_invalid_method(&push));
    assert!(!is_mutating_object_method(&push));

    assert!(!is_invalid_method(&map));
    assert!(!is_mutating_array_method(&map));
    assert!(!is_mutating_object_method(&map));
  }

  #[test]
  fn computed_props_are_not_classified_as_identifier_methods() {
    let computed = MemberProp::Computed(ComputedPropName {
      span: DUMMY_SP,
      expr: Box::new(Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: "assign".into(),
        raw: None,
      }))),
    });

    assert!(!is_invalid_method(&computed));
    assert!(!is_mutating_array_method(&computed));
    assert!(!is_mutating_object_method(&computed));
  }

  #[test]
  fn mutation_expr_detection() {
    let assign_member = Expr::Assign(AssignExpr {
      span: DUMMY_SP,
      op: AssignOp::Assign,
      left: AssignTarget::Simple(SimpleAssignTarget::Member(member_expr_with_ident_obj("x"))),
      right: Box::new(Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: 1.0,
        raw: None,
      }))),
    });
    assert!(is_mutation_expr(&assign_member));

    let update_member = Expr::Update(UpdateExpr {
      span: DUMMY_SP,
      op: UpdateOp::PlusPlus,
      prefix: false,
      arg: Box::new(Expr::Member(member_expr_with_ident_obj("x"))),
    });
    assert!(is_mutation_expr(&update_member));

    let delete_member = Expr::Unary(UnaryExpr {
      span: DUMMY_SP,
      op: UnaryOp::Delete,
      arg: Box::new(Expr::Member(member_expr_with_ident_obj("x"))),
    });
    assert!(is_mutation_expr(&delete_member));

    let assign_non_ident_object = Expr::Assign(AssignExpr {
      span: DUMMY_SP,
      op: AssignOp::Assign,
      left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Lit(Lit::Num(Number {
          span: DUMMY_SP,
          value: 0.0,
          raw: None,
        }))),
        prop: member_ident("x"),
      })),
      right: Box::new(Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: 1.0,
        raw: None,
      }))),
    });
    assert!(!is_mutation_expr(&assign_non_ident_object));
  }

  #[test]
  fn get_method_name_returns_identifier_name() {
    assert_eq!(get_method_name(&member_ident("splice")), "splice");
  }

  #[test]
  #[should_panic(expected = "The method name in a call expression must be a static identifier.")]
  fn get_method_name_panics_for_non_identifier() {
    let computed = MemberProp::Computed(ComputedPropName {
      span: DUMMY_SP,
      expr: Box::new(Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: 1.0,
        raw: None,
      }))),
    });
    let _ = get_method_name(&computed);
  }

  #[test]
  fn id_prop_extraction_for_computed_string() {
    let computed_string = MemberProp::Computed(ComputedPropName {
      span: DUMMY_SP,
      expr: Box::new(Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: "id".into(),
        raw: None,
      }))),
    });
    assert_eq!(is_id_prop(&computed_string).map(Atom::as_ref), Some("id"));

    let computed_number = MemberProp::Computed(ComputedPropName {
      span: DUMMY_SP,
      expr: Box::new(Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: 1.0,
        raw: None,
      }))),
    });
    assert_eq!(is_id_prop(&computed_number), None);

    assert_eq!(is_id_prop(&member_ident("plain")), None);
  }
}
