//! Which spellings of a property read the escaping-property rule answers for.
//!
//! Two callers ask -- the walk in front of the engine and the dispatch below it
//! -- so the answer lives in one function, and the spellings it accepts are
//! asserted once here rather than twice over. The two that carry no name are the
//! half no module can reach: a private name is grammatical only inside a class
//! body, and a computed key that is not a static string is a value rather than a
//! name.

use super::*;

use stylex_ast::ast::convertors::create_string_expr;
use swc_core::common::{DUMMY_SP, SyntaxContext};
use swc_core::ecma::ast::{ComputedPropName, Ident, IdentName, PrivateName};

/// `.name`, the spelling an author writes.
fn named(property: &str) -> MemberProp {
  MemberProp::Ident(IdentName {
    span: DUMMY_SP,
    sym: property.into(),
  })
}

/// `[key]`, whatever the key is written as.
fn computed(key: Expr) -> MemberProp {
  MemberProp::Computed(ComputedPropName {
    span: DUMMY_SP,
    expr: Box::new(key),
  })
}

#[test]
fn a_named_read_of_an_escaping_property_is_answered() {
  for property in ESCAPING_PROPERTIES {
    assert_eq!(escaping_property_named(&named(property)), Some(property));
  }
}

/// A key written as a string spells the read a name spells, so it answers the
/// same.
#[test]
fn a_string_key_spelling_one_is_answered() {
  assert_eq!(
    escaping_property_named(&computed(create_string_expr("constructor"))),
    Some("constructor")
  );
}

/// A read of any other property is not this rule's, in either spelling.
#[test]
fn a_read_of_any_other_property_is_not_answered() {
  assert!(escaping_property_named(&named("length")).is_none());
  assert!(escaping_property_named(&computed(create_string_expr("trim"))).is_none());
}

/// A key that is a value rather than a name carries nothing to compare against,
/// so the rule does not fire and the key is walked as the value it is.
#[test]
fn a_key_with_no_static_name_is_not_answered() {
  let name = Expr::Ident(Ident {
    span: DUMMY_SP,
    sym: "key".into(),
    optional: false,
    ctxt: SyntaxContext::empty(),
  });

  assert!(escaping_property_named(&computed(name)).is_none());
}

/// A private name belongs to a class body, which no value a fold carries has --
/// so it names no property this rule could be about.
#[test]
fn a_private_name_is_not_answered() {
  let private = MemberProp::PrivateName(PrivateName {
    span: DUMMY_SP,
    name: "constructor".into(),
  });

  assert!(escaping_property_named(&private).is_none());
}
