use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{
    BigInt, ComputedPropName, Expr, GetterProp, IdentName, KeyValueProp, Lit, Number, ObjectLit,
    Prop, PropName, PropOrSpread, SpreadElement, Str,
  },
};

use crate::ast::convertors::{convert_wtf8_to_atom, create_number_expr};
use crate::ast::factories::create_ident;
use crate::ast::objects::{assign_props, order_own_keys, remove_duplicates};

fn make_kv_prop(key: &str, val: f64) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key: PropName::Ident(IdentName {
      span: DUMMY_SP,
      sym: key.into(),
    }),
    value: Box::new(create_number_expr(val)),
  })))
}

fn make_kv_str_key_prop(key: &str, val: f64) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key: PropName::Str(Str {
      span: DUMMY_SP,
      value: key.into(),
      raw: None,
    }),
    value: Box::new(create_number_expr(val)),
  })))
}

fn make_shorthand_prop(name: &str) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::Shorthand(create_ident(name))))
}

fn make_spread_prop() -> PropOrSpread {
  PropOrSpread::Spread(SpreadElement {
    dot3_token: DUMMY_SP,
    expr: Box::new(create_number_expr(1.0)),
  })
}

fn make_kv_obj_prop(key: &str, inner_props: Vec<PropOrSpread>) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key: PropName::Ident(IdentName {
      span: DUMMY_SP,
      sym: key.into(),
    }),
    value: Box::new(Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: inner_props,
    })),
  })))
}

fn make_kv_str_key_obj_prop(key: &str, inner: Vec<PropOrSpread>) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key: PropName::Str(Str {
      span: DUMMY_SP,
      value: key.into(),
      raw: None,
    }),
    value: Box::new(Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: inner,
    })),
  })))
}

fn make_kv_num_key_obj_prop(key: f64, inner: Vec<PropOrSpread>) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key: PropName::Num(Number {
      span: DUMMY_SP,
      value: key,
      raw: None,
    }),
    value: Box::new(Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: inner,
    })),
  })))
}

fn make_bigint_obj_prop(val: u32, inner: Vec<PropOrSpread>) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key: PropName::BigInt(BigInt {
      span: DUMMY_SP,
      value: Box::new(val.into()),
      raw: None,
    }),
    value: Box::new(Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: inner,
    })),
  })))
}

fn make_kv_num_key_prop(key: f64, val: f64) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    key: PropName::Num(Number {
      span: DUMMY_SP,
      value: key,
      raw: None,
    }),
    value: Box::new(create_number_expr(val)),
  })))
}

fn make_getter_prop(key: &str) -> PropOrSpread {
  PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
    span: DUMMY_SP,
    key: PropName::Ident(IdentName {
      span: DUMMY_SP,
      sym: key.into(),
    }),
    type_ann: None,
    body: None,
  })))
}

/// The number a property carries, or `None` where it carries anything else.
fn number_value_of(prop: &PropOrSpread) -> Option<f64> {
  match prop {
    PropOrSpread::Prop(prop) => match prop.as_ref() {
      Prop::KeyValue(kv) => match kv.value.as_ref() {
        Expr::Lit(Lit::Num(number)) => Some(number.value),
        _ => None,
      },
      _ => None,
    },
    _ => None,
  }
}

/// The keys of the object a property carries.
fn inner_keys_of(prop: &PropOrSpread) -> Vec<String> {
  match prop {
    PropOrSpread::Prop(prop) => match prop.as_ref() {
      Prop::KeyValue(kv) => match kv.value.as_ref() {
        Expr::Object(object) => keys_of(&object.props),
        _ => vec![],
      },
      _ => vec![],
    },
    _ => vec![],
  }
}

fn keys_of(props: &[PropOrSpread]) -> Vec<String> {
  props
    .iter()
    .map(|prop| match prop {
      PropOrSpread::Prop(prop) => match prop.as_ref() {
        Prop::KeyValue(kv) => match &kv.key {
          PropName::Ident(ident) => ident.sym.to_string(),
          PropName::Str(strng) => convert_wtf8_to_atom(&strng.value).to_string(),
          PropName::Num(num) => num.value.to_string(),
          PropName::BigInt(big_int) => big_int.value.to_string(),
          PropName::Computed(_) => String::from("<computed>"),
        },
        Prop::Shorthand(ident) => ident.sym.to_string(),
        _ => String::from("<other>"),
      },
      PropOrSpread::Spread(_) => String::from("<spread>"),
    })
    .collect()
}

mod remove_duplicates_tests {
  use super::*;

  #[test]
  fn keeps_every_key_declared_once() {
    let props = vec![make_kv_prop("a", 1.0), make_kv_prop("b", 2.0)];

    assert_eq!(keys_of(&remove_duplicates(props)), vec!["a", "b"]);
  }

  #[test]
  fn keeps_the_last_value_a_repeated_key_takes_in_the_place_it_last_took() {
    let props = vec![
      make_kv_prop("a", 1.0),
      make_kv_prop("b", 2.0),
      make_kv_prop("a", 3.0),
    ];

    let result = remove_duplicates(props);

    assert_eq!(keys_of(&result), vec!["b", "a"]);
    assert_eq!(number_value_of(&result[1]), Some(3.0));
  }

  #[test]
  fn handles_no_properties_at_all() {
    assert!(remove_duplicates(vec![]).is_empty());
  }

  #[test]
  fn one_key_declared_three_times_is_one_property() {
    let props = vec![
      make_kv_prop("x", 1.0),
      make_kv_prop("x", 2.0),
      make_kv_prop("x", 3.0),
    ];

    let result = remove_duplicates(props);

    assert_eq!(keys_of(&result), vec!["x"]);
    assert_eq!(number_value_of(&result[0]), Some(3.0));
  }

  #[test]
  fn reads_a_shorthand_key() {
    let props = vec![
      make_shorthand_prop("a"),
      make_shorthand_prop("b"),
      make_shorthand_prop("a"),
    ];

    assert_eq!(keys_of(&remove_duplicates(props)), vec!["b", "a"]);
  }

  #[test]
  fn reads_a_string_key() {
    let props = vec![
      make_kv_str_key_prop("color", 1.0),
      make_kv_str_key_prop("color", 2.0),
    ];

    assert_eq!(keys_of(&remove_duplicates(props)), vec!["color"]);
  }

  #[test]
  fn a_shorthand_and_a_key_value_under_one_name_are_one_property() {
    let props = vec![make_shorthand_prop("x"), make_kv_prop("x", 5.0)];

    assert_eq!(keys_of(&remove_duplicates(props)), vec!["x"]);
  }

  #[test]
  fn drops_a_spread_because_it_names_no_key() {
    let props = vec![make_kv_prop("a", 1.0), make_spread_prop()];

    assert_eq!(keys_of(&remove_duplicates(props)), vec!["a"]);
  }

  #[test]
  fn drops_a_getter_because_it_names_no_readable_key() {
    let props = vec![make_kv_prop("a", 1.0), make_getter_prop("val")];

    assert_eq!(keys_of(&remove_duplicates(props)), vec!["a"]);
  }

  #[test]
  fn keeps_a_numeric_key() {
    let props = vec![make_kv_num_key_prop(42.0, 1.0)];

    // A numeric key names a property like any other, so it is kept. It used to
    // be dropped: the key reader had no arm for it, and a declaration written
    // `{ 42: 1 }` vanished from the object it was written in.
    assert_eq!(keys_of(&remove_duplicates(props)), vec!["42"]);
  }
}

mod assign_props_tests {
  use super::*;

  #[test]
  fn keeps_both_where_no_key_is_repeated() {
    let old = vec![make_kv_prop("a", 1.0)];
    let new = vec![make_kv_prop("b", 2.0)];

    assert_eq!(keys_of(&assign_props(old, new)), vec!["a", "b"]);
  }

  #[test]
  fn a_repeated_key_takes_the_later_value_in_the_place_it_first_took() {
    let old = vec![make_kv_prop("a", 1.0), make_kv_prop("b", 2.0)];
    let new = vec![make_kv_prop("a", 3.0)];

    let result = assign_props(old, new);

    assert_eq!(keys_of(&result), vec!["a", "b"]);
    assert_eq!(number_value_of(&result[0]), Some(3.0));
  }

  #[test]
  fn a_repeated_key_replaces_a_nested_object_rather_than_merging_into_it() {
    let old = vec![make_kv_obj_prop("shared", vec![make_kv_prop("x", 1.0)])];
    let new = vec![make_kv_obj_prop("shared", vec![make_kv_prop("y", 2.0)])];

    let result = assign_props(old, new);

    // `Object.assign` is shallow, so the nested `x` is gone rather than merged
    // in. A deep merge here quietly kept declarations the source had replaced.
    assert_eq!(keys_of(&result), vec!["shared"]);
    assert_eq!(inner_keys_of(&result[0]), vec!["y"]);
  }

  #[test]
  fn a_string_key_and_the_numeric_spelling_of_it_are_one_key() {
    let old = vec![make_kv_str_key_obj_prop("42", vec![make_kv_prop("x", 1.0)])];
    let new = vec![make_kv_num_key_obj_prop(42.0, vec![make_kv_prop("y", 2.0)])];

    let result = assign_props(old, new);

    // `{ '42': x }` and `{ 42: y }` name one property in the language, so they
    // collide and the later one wins.
    assert_eq!(keys_of(&result), vec!["42"]);
    assert_eq!(inner_keys_of(&result[0]), vec!["y"]);
  }

  #[test]
  fn a_repeated_numeric_key_is_one_key() {
    let old = vec![make_kv_num_key_obj_prop(42.0, vec![make_kv_prop("x", 1.0)])];
    let new = vec![make_kv_num_key_obj_prop(42.0, vec![make_kv_prop("y", 2.0)])];

    // This used to answer an empty object -- the key was matched by the merge
    // and then dropped by the deduplication, which read no name for it.
    assert_eq!(keys_of(&assign_props(old, new)), vec!["42"]);
  }

  #[test]
  fn a_repeated_big_integer_key_is_one_key() {
    let old = vec![make_bigint_obj_prop(42, vec![make_kv_prop("x", 1.0)])];
    let new = vec![make_bigint_obj_prop(42, vec![make_kv_prop("y", 2.0)])];

    // `{ 42n: x }` names the property `"42"`, so the two collide.
    assert_eq!(keys_of(&assign_props(old, new)), vec!["42"]);
  }

  #[test]
  fn a_repeated_string_key_is_one_key() {
    let old = vec![make_kv_str_key_obj_prop(
      "shared",
      vec![make_kv_prop("x", 1.0)],
    )];
    let new = vec![make_kv_str_key_obj_prop(
      "shared",
      vec![make_kv_prop("y", 2.0)],
    )];

    assert_eq!(keys_of(&assign_props(old, new)), vec!["shared"]);
  }

  #[test]
  fn no_old_properties_leaves_the_new_ones_as_they_are() {
    let new = vec![make_kv_prop("a", 1.0)];

    assert_eq!(keys_of(&assign_props(vec![], new)), vec!["a"]);
  }

  // A property that names no readable key collides with nothing: it keeps its
  // place, on either side of the merge.

  #[test]
  fn an_old_getter_keeps_its_place() {
    let old = vec![make_getter_prop("val")];
    let new = vec![make_kv_prop("a", 1.0)];

    assert_eq!(keys_of(&assign_props(old, new)), vec!["<other>", "a"]);
  }

  #[test]
  fn an_old_spread_keeps_its_place() {
    let old = vec![make_spread_prop()];
    let new = vec![make_kv_prop("a", 1.0)];

    assert_eq!(keys_of(&assign_props(old, new)), vec!["<spread>", "a"]);
  }

  #[test]
  fn a_new_getter_keeps_its_place_even_under_a_name_the_old_side_declares() {
    let old = vec![make_kv_obj_prop("shared", vec![make_kv_prop("x", 1.0)])];
    let new = vec![make_getter_prop("shared")];

    assert_eq!(keys_of(&assign_props(old, new)), vec!["shared", "<other>"]);
  }

  #[test]
  fn a_new_spread_keeps_its_place() {
    let old = vec![make_kv_obj_prop("shared", vec![make_kv_prop("x", 1.0)])];
    let new = vec![make_spread_prop()];

    assert_eq!(keys_of(&assign_props(old, new)), vec!["shared", "<spread>"]);
  }
}

mod order_own_keys_tests {
  use super::*;

  #[test]
  fn puts_every_array_index_first_in_ascending_order() {
    let props = vec![
      make_kv_prop("color", 1.0),
      make_kv_str_key_prop("2", 2.0),
      make_kv_str_key_prop("0", 3.0),
      make_kv_prop("opacity", 4.0),
      make_kv_str_key_prop("1", 5.0),
    ];

    assert_eq!(
      keys_of(&order_own_keys(props)),
      vec!["0", "1", "2", "color", "opacity"]
    );
  }

  #[test]
  fn keeps_the_insertion_order_of_the_string_keys() {
    let props = vec![
      make_kv_prop("z", 1.0),
      make_kv_prop("a", 2.0),
      make_kv_prop("m", 3.0),
    ];

    assert_eq!(keys_of(&order_own_keys(props)), vec!["z", "a", "m"]);
  }

  #[test]
  fn reads_a_numeric_key_as_the_index_it_spells() {
    let props = vec![
      make_kv_prop("color", 1.0),
      make_kv_num_key_obj_prop(3.0, vec![]),
      make_kv_str_key_prop("1", 2.0),
    ];

    assert_eq!(keys_of(&order_own_keys(props)), vec!["1", "3", "color"]);
  }

  #[test]
  fn a_key_that_is_not_the_canonical_spelling_is_a_string_key() {
    // `00`, `01` and `+0` all round-trip to a different string, so the
    // language enumerates them in insertion order rather than as indices.
    let props = vec![
      make_kv_str_key_prop("00", 1.0),
      make_kv_str_key_prop("1", 2.0),
      make_kv_str_key_prop("01", 3.0),
      make_kv_str_key_prop("+0", 4.0),
    ];

    assert_eq!(keys_of(&order_own_keys(props)), vec!["1", "00", "01", "+0"]);
  }

  #[test]
  fn the_largest_unsigned_value_is_not_an_index() {
    // 2^32 - 1 is one past the last index an array can hold, so it enumerates
    // as a string key. One below it is an index.
    let props = vec![
      make_kv_str_key_prop("4294967295", 1.0),
      make_kv_str_key_prop("4294967294", 2.0),
    ];

    assert_eq!(
      keys_of(&order_own_keys(props)),
      vec!["4294967294", "4294967295"]
    );
  }

  #[test]
  fn a_key_too_large_for_an_index_is_a_string_key() {
    let props = vec![
      make_kv_str_key_prop("4294967296", 1.0),
      make_kv_str_key_prop("0", 2.0),
    ];

    assert_eq!(keys_of(&order_own_keys(props)), vec!["0", "4294967296"]);
  }

  #[test]
  fn returns_the_properties_untouched_where_no_key_is_an_index() {
    let props = vec![make_kv_prop("a", 1.0), make_shorthand_prop("b")];

    assert_eq!(keys_of(&order_own_keys(props)), vec!["a", "b"]);
  }

  #[test]
  fn a_property_with_no_readable_key_keeps_its_place_among_the_string_keys() {
    let props = vec![
      make_kv_prop("a", 1.0),
      make_spread_prop(),
      make_kv_str_key_prop("0", 2.0),
    ];

    assert_eq!(keys_of(&order_own_keys(props)), vec!["0", "a", "<spread>"]);
  }

  #[test]
  fn a_computed_key_reads_as_no_key_at_all() {
    let computed = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Computed(ComputedPropName {
        span: DUMMY_SP,
        expr: Box::new(create_number_expr(0.0)),
      }),
      value: Box::new(create_number_expr(1.0)),
    })));
    let props = vec![computed, make_kv_str_key_prop("0", 2.0)];

    assert_eq!(keys_of(&order_own_keys(props)), vec!["0", "<computed>"]);
  }

  #[test]
  fn the_empty_key_is_a_string_key() {
    let props = vec![
      make_kv_str_key_prop("", 1.0),
      make_kv_str_key_prop("0", 2.0),
    ];

    assert_eq!(keys_of(&order_own_keys(props)), vec!["0", ""]);
  }

  #[test]
  fn handles_no_properties_at_all() {
    assert!(order_own_keys(vec![]).is_empty());
  }
}
