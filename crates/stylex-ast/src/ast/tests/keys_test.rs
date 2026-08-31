//! Tests for the object and member key readers.

use crate::ast::keys::{
  collect_object_lit_keys, namespace_name_from_member_prop, namespace_name_from_prop_key,
  prop_as_key_value,
};
use swc_core::{
  atoms::Atom,
  common::DUMMY_SP,
  ecma::ast::{
    ArrowExpr, BigInt, BigIntValue, BindingIdent, BlockStmtOrExpr, ComputedPropName, Expr,
    GetterProp, Ident, IdentName, KeyValueProp, Lit, MemberProp, MethodProp, Number, ObjectLit,
    Pat, PrivateName, Prop, PropName, PropOrSpread, SpreadElement, Str, Tpl, TplElement,
  },
};

fn ident_name(name: &str) -> IdentName {
  IdentName {
    span: DUMMY_SP,
    sym: Atom::new(name),
  }
}

fn string_lit(value: &str) -> Str {
  Str {
    span: DUMMY_SP,
    value: Atom::new(value).into(),
    raw: None,
  }
}

fn number_lit(value: f64) -> Number {
  Number {
    span: DUMMY_SP,
    value,
    raw: None,
  }
}

fn big_int_lit(value: i64) -> BigInt {
  BigInt {
    span: DUMMY_SP,
    value: Box::new(BigIntValue::from(value)),
    raw: None,
  }
}

fn computed(expr: Expr) -> ComputedPropName {
  ComputedPropName {
    span: DUMMY_SP,
    expr: Box::new(expr),
  }
}

/// A template literal with no expressions, which reads back as its one quasi.
fn static_tpl(text: &str) -> Tpl {
  Tpl {
    span: DUMMY_SP,
    exprs: vec![],
    quasis: vec![TplElement {
      span: DUMMY_SP,
      tail: true,
      cooked: Some(Atom::new(text).into()),
      raw: Atom::new(text),
    }],
  }
}

/// A template literal with an interpolation, so nothing static can be read off
/// it.
fn dynamic_tpl() -> Tpl {
  Tpl {
    span: DUMMY_SP,
    exprs: vec![Box::new(Expr::Ident(Ident::new_no_ctxt(
      Atom::new("value"),
      DUMMY_SP,
    )))],
    quasis: vec![
      TplElement {
        span: DUMMY_SP,
        tail: false,
        cooked: Some(Atom::new("a").into()),
        raw: Atom::new("a"),
      },
      TplElement {
        span: DUMMY_SP,
        tail: true,
        cooked: Some(Atom::new("b").into()),
        raw: Atom::new("b"),
      },
    ],
  }
}

fn key_value_prop(key: PropName, value: Expr) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key,
    value: Box::new(value),
  })))
}

fn object_of(props: Vec<PropOrSpread>) -> ObjectLit {
  ObjectLit {
    span: DUMMY_SP,
    props,
  }
}

// ---------- namespace_name_from_prop_key ----------

#[test]
fn reads_an_identifier_key() {
  let key = PropName::Ident(ident_name("root"));

  assert_eq!(namespace_name_from_prop_key(&key), Some(Atom::new("root")));
}

#[test]
fn reads_a_string_key() {
  let key = PropName::Str(string_lit("a key"));

  assert_eq!(namespace_name_from_prop_key(&key), Some(Atom::new("a key")));
}

#[test]
fn reads_a_number_key_the_way_javascript_spells_it() {
  let key = PropName::Num(number_lit(42.0));

  assert_eq!(namespace_name_from_prop_key(&key), Some(Atom::new("42")));
}

#[test]
fn reads_a_big_int_key() {
  let key = PropName::BigInt(big_int_lit(9_007_199_254_740_993));

  assert_eq!(
    namespace_name_from_prop_key(&key),
    Some(Atom::new("9007199254740993"))
  );
}

#[test]
fn reads_a_computed_string_key() {
  let key = PropName::Computed(computed(Expr::Lit(Lit::Str(string_lit("root")))));

  assert_eq!(namespace_name_from_prop_key(&key), Some(Atom::new("root")));
}

#[test]
fn reads_a_computed_number_key() {
  let key = PropName::Computed(computed(Expr::Lit(Lit::Num(number_lit(7.0)))));

  assert_eq!(namespace_name_from_prop_key(&key), Some(Atom::new("7")));
}

#[test]
fn reads_a_computed_big_int_key() {
  let key = PropName::Computed(computed(Expr::Lit(Lit::BigInt(big_int_lit(12)))));

  assert_eq!(namespace_name_from_prop_key(&key), Some(Atom::new("12")));
}

#[test]
fn reads_a_computed_static_template_key() {
  let key = PropName::Computed(computed(Expr::Tpl(static_tpl("root"))));

  assert_eq!(namespace_name_from_prop_key(&key), Some(Atom::new("root")));
}

#[test]
fn refuses_a_computed_template_key_with_an_interpolation() {
  let key = PropName::Computed(computed(Expr::Tpl(dynamic_tpl())));

  assert_eq!(namespace_name_from_prop_key(&key), None);
}

#[test]
fn refuses_a_computed_key_whose_literal_has_no_name() {
  let key = PropName::Computed(computed(Expr::Lit(Lit::Null(swc_core::ecma::ast::Null {
    span: DUMMY_SP,
  }))));

  assert_eq!(namespace_name_from_prop_key(&key), None);
}

#[test]
fn refuses_a_computed_key_that_is_not_a_literal() {
  let key = PropName::Computed(computed(Expr::Ident(Ident::new_no_ctxt(
    Atom::new("name"),
    DUMMY_SP,
  ))));

  assert_eq!(namespace_name_from_prop_key(&key), None);
}

// ---------- namespace_name_from_member_prop ----------

#[test]
fn reads_an_identifier_member() {
  let prop = MemberProp::Ident(ident_name("color"));

  assert_eq!(
    namespace_name_from_member_prop(&prop),
    Some(Atom::new("color"))
  );
}

#[test]
fn reads_a_computed_member() {
  let prop = MemberProp::Computed(computed(Expr::Lit(Lit::Str(string_lit("color")))));

  assert_eq!(
    namespace_name_from_member_prop(&prop),
    Some(Atom::new("color"))
  );
}

#[test]
fn refuses_a_private_member() {
  let prop = MemberProp::PrivateName(PrivateName {
    span: DUMMY_SP,
    name: Atom::new("secret"),
  });

  assert_eq!(namespace_name_from_member_prop(&prop), None);
}

// ---------- prop_as_key_value ----------

#[test]
fn accepts_a_key_value_property() {
  let prop = key_value_prop(
    PropName::Ident(ident_name("root")),
    Expr::Lit(Lit::Num(number_lit(1.0))),
  );

  assert!(prop_as_key_value(&prop).is_some());
}

#[test]
fn refuses_a_spread() {
  let prop = PropOrSpread::Spread(SpreadElement {
    dot3_token: DUMMY_SP,
    expr: Box::new(Expr::Ident(Ident::new_no_ctxt(Atom::new("rest"), DUMMY_SP))),
  });

  assert!(prop_as_key_value(&prop).is_none());
}

#[test]
fn refuses_a_shorthand() {
  let prop = PropOrSpread::Prop(Box::new(Prop::Shorthand(Ident::new_no_ctxt(
    Atom::new("root"),
    DUMMY_SP,
  ))));

  assert!(prop_as_key_value(&prop).is_none());
}

#[test]
fn refuses_a_getter() {
  let prop = PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
    span: DUMMY_SP,
    key: PropName::Ident(ident_name("root")),
    type_ann: None,
    body: None,
  })));

  assert!(prop_as_key_value(&prop).is_none());
}

#[test]
fn refuses_a_method() {
  let prop = PropOrSpread::Prop(Box::new(Prop::Method(MethodProp {
    key: PropName::Ident(ident_name("root")),
    function: Box::default(),
  })));

  assert!(prop_as_key_value(&prop).is_none());
}

// ---------- collect_object_lit_keys ----------

#[test]
fn collects_nothing_from_an_empty_object() {
  let object = object_of(vec![]);

  assert!(collect_object_lit_keys(&object).next().is_none());
}

#[test]
fn collects_keys_in_source_order() {
  let object = object_of(vec![
    key_value_prop(
      PropName::Ident(ident_name("root")),
      Expr::Lit(Lit::Num(number_lit(1.0))),
    ),
    key_value_prop(
      PropName::Str(string_lit("nested")),
      Expr::Lit(Lit::Num(number_lit(2.0))),
    ),
  ]);

  assert_eq!(
    collect_object_lit_keys(&object).collect::<Vec<_>>(),
    vec![Atom::new("root"), Atom::new("nested")]
  );
}

#[test]
fn counts_a_key_written_twice_as_two_properties() {
  let object = object_of(vec![
    key_value_prop(
      PropName::Ident(ident_name("root")),
      Expr::Lit(Lit::Num(number_lit(1.0))),
    ),
    key_value_prop(
      PropName::Ident(ident_name("root")),
      Expr::Lit(Lit::Num(number_lit(2.0))),
    ),
  ]);

  assert_eq!(
    collect_object_lit_keys(&object).collect::<Vec<_>>(),
    vec![Atom::new("root"), Atom::new("root")]
  );
}

#[test]
fn skips_properties_that_have_no_readable_key() {
  let object = object_of(vec![
    PropOrSpread::Spread(SpreadElement {
      dot3_token: DUMMY_SP,
      expr: Box::new(Expr::Ident(Ident::new_no_ctxt(Atom::new("rest"), DUMMY_SP))),
    }),
    key_value_prop(
      PropName::Computed(computed(Expr::Ident(Ident::new_no_ctxt(
        Atom::new("dynamic"),
        DUMMY_SP,
      )))),
      Expr::Lit(Lit::Num(number_lit(1.0))),
    ),
    key_value_prop(
      PropName::Ident(ident_name("root")),
      Expr::Lit(Lit::Num(number_lit(2.0))),
    ),
  ]);

  assert_eq!(
    collect_object_lit_keys(&object).collect::<Vec<_>>(),
    vec![Atom::new("root")]
  );
}

/// A large object, to show the readers stay linear and do not overflow a
/// recursion limit on width.
#[test]
fn collects_every_key_of_a_very_wide_object() {
  const WIDTH: usize = 10_000;

  let object = object_of(
    (0..WIDTH)
      .map(|index| {
        key_value_prop(
          PropName::Num(number_lit(index as f64)),
          Expr::Lit(Lit::Num(number_lit(index as f64))),
        )
      })
      .collect(),
  );

  let keys = collect_object_lit_keys(&object).collect::<Vec<_>>();

  assert_eq!(keys.len(), WIDTH);
  assert_eq!(keys.first(), Some(&Atom::new("0")));
  assert_eq!(keys.last(), Some(&Atom::new("9999")));
}

/// An arrow value carries no key of its own, so the property is read by its key
/// alone.
#[test]
fn reads_a_key_whose_value_is_a_function() {
  let object = object_of(vec![key_value_prop(
    PropName::Ident(ident_name("dynamic")),
    Expr::Arrow(ArrowExpr {
      span: DUMMY_SP,
      params: vec![Pat::Ident(BindingIdent {
        id: Ident::new_no_ctxt(Atom::new("value"), DUMMY_SP),
        type_ann: None,
      })],
      body: Box::new(BlockStmtOrExpr::Expr(Box::new(Expr::Ident(
        Ident::new_no_ctxt(Atom::new("value"), DUMMY_SP),
      )))),
      is_async: false,
      is_generator: false,
      type_params: None,
      return_type: None,
      ctxt: Default::default(),
    }),
  )]);

  assert_eq!(
    collect_object_lit_keys(&object).collect::<Vec<_>>(),
    vec![Atom::new("dynamic")]
  );
}
