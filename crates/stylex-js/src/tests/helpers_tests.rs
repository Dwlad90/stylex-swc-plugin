// Tests for the JavaScript-semantics predicates and the identifier extractors
// beside them.
// Source: crates/stylex-js/src/helpers.rs

use super::*;
use swc_core::{
  atoms::{
    Wtf8Atom,
    wtf8::{CodePoint, Wtf8Buf},
  },
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

  assert!(!is_invalid_method(&push));
  assert!(!is_mutating_object_method(&push));

  assert!(!is_invalid_method(&map));
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

#[test]
#[should_panic(expected = "String value contains invalid UTF-8 encoding.")]
fn id_prop_extraction_panics_for_invalid_utf8() {
  // A lone surrogate: valid WTF-8 storage, invalid UTF-8 decoding.
  let mut lone_surrogate = Wtf8Buf::new();

  match CodePoint::from_u32(0xd800) {
    Some(code_point) => lone_surrogate.push(code_point),
    None => panic!("U+D800 is within the code-point range."),
  }

  let computed_invalid = MemberProp::Computed(ComputedPropName {
    span: DUMMY_SP,
    expr: Box::new(Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: Wtf8Atom::new(lone_surrogate),
      raw: None,
    }))),
  });

  let _ = is_id_prop(&computed_invalid);
}
