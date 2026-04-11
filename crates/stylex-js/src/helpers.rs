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
#[path = "tests/helpers_tests.rs"]
mod tests;
