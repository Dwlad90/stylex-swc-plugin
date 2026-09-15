//! The finalize sweep, over the two objects it walks.
//!
//! A style variable holds a map of namespaces, and each namespace holds the
//! declarations compiled out of it. Both objects were written by
//! `convert_object_to_ast` a few phases earlier, which writes key-value props
//! under names it chose, so no module can put a spread, a method or a computed
//! key in front of the sweep. The objects are therefore built here rather than
//! compiled, which is also what lets one test drive the whole sweep: llvm-cov
//! scores a function on its best-covered instantiation, and the cases the
//! compiled suite reaches are in a different one.

use std::rc::Rc;

use swc_core::{
  atoms::Atom,
  common::{GLOBALS, Globals, comments::SingleThreadedComments},
  ecma::ast::{Expr, ObjectLit, PropOrSpread},
};

use stylex_ast::ast::convertors::create_null_expr;
use stylex_ast::ast::factories::{
  create_ident, create_key_value_prop, create_object_expression, create_spread_prop,
  create_string_key_value_prop,
};
use stylex_enums::style_vars_to_keep::{NonNullProp, NonNullProps};
use stylex_state::state_manager::DeclId;
use stylex_structures::style_vars_to_keep::StyleVarsToKeep;

use super::super::visit_mut_var_declarator::retain_style_props;
use crate::StyleXTransform;

/// The object `props` make up.
fn object_of(props: Vec<PropOrSpread>) -> ObjectLit {
  match create_object_expression(props) {
    Expr::Object(object) => object,
    other => panic!("the factory answered something other than an object: {other:?}"),
  }
}

/// One compiled namespace: a declaration of `null` under each name given.
fn namespace_of(name: &str, null_declarations: &[&str]) -> PropOrSpread {
  let declarations = null_declarations
    .iter()
    .map(|declared| create_key_value_prop(declared, create_null_expr()))
    .collect();

  create_key_value_prop(name, Expr::Object(object_of(declarations)))
}

fn transform() -> StyleXTransform<Rc<SingleThreadedComments>> {
  StyleXTransform::test(Rc::new(SingleThreadedComments::default())).build()
}

fn declaration_id() -> DeclId {
  create_ident("styles").to_id()
}

/// The name each prop of `object` is written under.
fn prop_names(props: &[PropOrSpread]) -> Vec<String> {
  props
    .iter()
    .filter_map(|prop| prop.as_prop()?.as_key_value()?.key.as_ident().cloned())
    .map(|key| key.sym.to_string())
    .collect()
}

/// The whole sweep over one style variable: a namespace still read keeps the
/// null declarations recorded against it and loses the others, a namespace
/// recorded as kept whole loses none, and a namespace nothing reads goes.
#[test]
fn the_sweep_keeps_the_namespaces_and_nulls_still_read() {
  GLOBALS.set(&Globals::default(), || {
    let mut transform = transform();
    let var_id = declaration_id();

    // `read` keeps the `color` declaration alone; `whole` is recorded as kept
    // in full, so both of its declarations stay.
    transform.state.style_vars_to_keep.insert(StyleVarsToKeep(
      var_id.clone(),
      NonNullProp::Atom(Atom::from("read")),
      NonNullProps::Vec(vec![Atom::from("color")]),
    ));
    transform.state.style_vars_to_keep.insert(StyleVarsToKeep(
      var_id.clone(),
      NonNullProp::Atom(Atom::from("whole")),
      NonNullProps::True,
    ));

    let mut object = object_of(vec![
      namespace_of("read", &["color", "margin"]),
      namespace_of("whole", &["color", "margin"]),
      namespace_of("unread", &["color"]),
    ]);

    let swept = transform.retain_object_props(
      &mut object,
      &[Atom::from("read"), Atom::from("whole")],
      &var_id,
    );

    assert_eq!(
      prop_names(&swept),
      vec!["read".to_string(), "whole".to_string()],
      "the namespace nothing reads is gone"
    );

    let declarations = |index: usize| match &swept[index] {
      PropOrSpread::Prop(prop) => match prop.as_key_value().map(|key_value| &key_value.value) {
        Some(value) => prop_names(
          &value
            .as_object()
            .expect("a namespace holds an object")
            .props,
        ),
        None => panic!("a namespace is a key-value prop"),
      },
      PropOrSpread::Spread(_) => panic!("a namespace is a key-value prop"),
    };

    assert_eq!(
      declarations(0),
      vec!["color".to_string()],
      "the namespace read by name keeps the declaration named with it"
    );
    assert_eq!(
      declarations(1),
      vec!["color".to_string(), "margin".to_string()],
      "the namespace kept whole keeps every declaration"
    );
  });
}

/// A prop the sweep cannot name leaves the whole object as it is, rather than
/// dropping the namespaces beside it that it could name.
#[test]
fn a_spread_leaves_the_namespace_map_alone() {
  GLOBALS.set(&Globals::default(), || {
    let mut object = object_of(vec![
      namespace_of("base", &["color"]),
      create_spread_prop(Expr::Ident(create_ident("other"))),
    ]);
    let written = object.props.clone();

    let swept =
      transform().retain_object_props(&mut object, &[Atom::from("base")], &declaration_id());

    assert_eq!(
      swept, written,
      "the object the sweep cannot read is answered as it was written"
    );
  });
}

/// A namespace the module still reads is kept whatever its value is. Only an
/// object of declarations has nulls to sweep out of it.
#[test]
fn a_namespace_that_holds_no_object_is_kept_as_it_is() {
  GLOBALS.set(&Globals::default(), || {
    let mut object = object_of(vec![
      create_string_key_value_prop("base", "already a class name"),
      namespace_of("unread", &["color"]),
    ]);

    let swept =
      transform().retain_object_props(&mut object, &[Atom::from("base")], &declaration_id());

    assert_eq!(swept.len(), 1, "the namespace asked for is the one kept");
  });
}

/// Inside one namespace: a declaration of `null` under a name the module no
/// longer reads goes, one it still reads stays, and everything that is not a
/// declaration of `null` under a name the sweep reads -- a class name, the
/// `$$css` marker, a string key, a prop that is no property at all -- stays
/// with it.
#[test]
fn the_null_sweep_drops_only_the_nulls_nothing_reads() {
  GLOBALS.set(&Globals::default(), || {
    let mut style_object = object_of(vec![
      create_key_value_prop("color", create_null_expr()),
      create_key_value_prop("margin", create_null_expr()),
      create_key_value_prop("padding", Expr::Ident(create_ident("className"))),
      create_string_key_value_prop("$$css", "true"),
      // A name that has to be quoted is a string key, which the sweep reads as
      // no declaration of its own -- a minified key is written this way.
      create_key_value_prop("color-kMwMTN", create_null_expr()),
      create_spread_prop(Expr::Ident(create_ident("other"))),
    ]);

    retain_style_props(&mut style_object, vec![Atom::from("color")]);

    assert_eq!(
      prop_names(&style_object.props),
      vec![
        "color".to_string(),
        "padding".to_string(),
        "$$css".to_string(),
      ],
      "the null nothing reads went; the null still read and the values stayed"
    );
    assert_eq!(
      style_object.props.len(),
      5,
      "the string-keyed null and the spread stayed too"
    );
  });
}
