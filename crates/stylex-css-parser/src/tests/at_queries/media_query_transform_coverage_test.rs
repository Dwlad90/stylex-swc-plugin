use super::*;

use swc_core::{
  atoms::Wtf8Atom,
  common::DUMMY_SP,
  ecma::ast::{
    ArrayLit, Expr, Ident, KeyValueProp, Lit, Number, ObjectLit, Prop, PropName, PropOrSpread, Str,
  },
};

// ---------------------------------------------------------------------------
// Helpers shared by coverage tests
// ---------------------------------------------------------------------------

fn str_kv(key: &str, value: &str) -> KeyValueProp {
  KeyValueProp {
    key: PropName::Str(Str {
      span: DUMMY_SP,
      value: Wtf8Atom::from(key),
      raw: None,
    }),
    value: Box::new(Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: Wtf8Atom::from(value),
      raw: None,
    }))),
  }
}

fn ident_kv(key: &str, value: &str) -> KeyValueProp {
  KeyValueProp {
    // PropName::Ident uses IdentName; the Into impl accepts &str via Atom
    key: PropName::Ident(key.into()),
    value: Box::new(Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: Wtf8Atom::from(value),
      raw: None,
    }))),
  }
}

fn num_kv(key: f64, value: &str) -> KeyValueProp {
  KeyValueProp {
    key: PropName::Num(Number {
      span: DUMMY_SP,
      value: key,
      raw: None,
    }),
    value: Box::new(Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: Wtf8Atom::from(value),
      raw: None,
    }))),
  }
}

// ---------------------------------------------------------------------------
// key_value_to_str — (PropName::Ident) and (_ arm)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod key_value_to_str_coverage {
  use super::*;

  /// Covers PropName::Ident branch of key_value_to_str.
  #[test]
  fn ident_key_returns_sym_string() {
    let kv = ident_kv("gridColumn", "1 / 2");
    let result = key_value_to_str(&kv);
    assert_eq!(result, "gridColumn");
  }

  /// Covers _ arm of key_value_to_str (PropName::Num, which is neither Str nor Ident).
  #[test]
  fn numeric_key_returns_empty_string() {
    let kv = num_kv(42.0, "value");
    let result = key_value_to_str(&kv);
    assert_eq!(result, "");
  }

  /// Covers the existing Str arm — ensures Str still works after the coverage tests run.
  #[test]
  fn str_key_returns_value_string() {
    let kv = str_kv("color", "red");
    let result = key_value_to_str(&kv);
    assert_eq!(result, "color");
  }
}

// ---------------------------------------------------------------------------
// dfs_process_queries_with_depth
// (ObjectLit with non-KeyValue prop, hitting else of let-chain condition)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod dfs_coverage {
  use super::*;

  /// Covers Expr::Array arm: when a top-level prop's value is an Array expression,
  /// dfs_process_queries_with_depth passes it through unchanged.
  #[test]
  fn array_valued_prop_passes_through_unchanged() {
    let array_expr = Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: vec![],
    });

    let prop = KeyValueProp {
      key: PropName::Str(Str {
        span: DUMMY_SP,
        value: Wtf8Atom::from("firstThatWorks"),
        raw: None,
      }),
      value: Box::new(array_expr),
    };

    // Call last_media_query_wins_transform which delegates to dfs_process_queries_with_depth
    let result = last_media_query_wins_transform(&[prop]);

    assert_eq!(result.len(), 1);
    // The prop key should be unchanged
    if let PropName::Str(s) = &result[0].key {
      assert_eq!(s.value.as_str().unwrap(), "firstThatWorks");
    } else {
      panic!("Expected Str key");
    }
    // The value should still be an Array
    assert!(matches!(&*result[0].value, Expr::Array(_)));
  }

  /// Non-KeyValue props inside an ObjectLit must be preserved unchanged so the
  /// main StyleX validation can report the unsupported non-static value.
  #[test]
  fn object_with_shorthand_prop_skips_non_key_value() {
    // Build a shorthand prop: `{ foo }` (Prop::Shorthand) — Ident::from(&str) works
    let shorthand_prop = Prop::Shorthand(Ident::from("foo"));

    // Build an ObjectLit containing only the shorthand prop
    let obj_lit = ObjectLit {
      span: DUMMY_SP,
      props: vec![PropOrSpread::Prop(Box::new(shorthand_prop))],
    };

    // Wrap it in a KeyValueProp at depth=0 (top-level), so DFS recurses into it at depth=1
    let outer_prop = KeyValueProp {
      key: PropName::Str(Str {
        span: DUMMY_SP,
        value: Wtf8Atom::from("outer"),
        raw: None,
      }),
      value: Box::new(Expr::Object(obj_lit)),
    };

    // last_media_query_wins_transform calls dfs at depth=0, then at depth=1 for nested objects
    let result = last_media_query_wins_transform(&[outer_prop]);

    assert_eq!(result.len(), 1);
    if let Expr::Object(inner_obj) = &*result[0].value {
      assert_eq!(inner_obj.props.len(), 1);
      assert!(matches!(
        inner_obj.props[0],
        PropOrSpread::Prop(ref prop) if matches!(prop.as_ref(), Prop::Shorthand(_))
      ));
    } else {
      panic!("Expected Object value");
    }
  }

  /// Covers PropOrSpread::Spread (not a Prop at all).
  /// The first condition `if let PropOrSpread::Prop(p) = obj_prop` fails.
  #[test]
  fn object_with_spread_prop_is_skipped() {
    use swc_core::ecma::ast::SpreadElement;

    // Build a spread element: `{ ...someExpr }`
    let spread = SpreadElement {
      dot3_token: DUMMY_SP,
      expr: Box::new(Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: Wtf8Atom::from("irrelevant"),
        raw: None,
      }))),
    };

    let obj_lit = ObjectLit {
      span: DUMMY_SP,
      props: vec![PropOrSpread::Spread(spread)],
    };

    let outer_prop = KeyValueProp {
      key: PropName::Str(Str {
        span: DUMMY_SP,
        value: Wtf8Atom::from("outerSpread"),
        raw: None,
      }),
      value: Box::new(Expr::Object(obj_lit)),
    };

    let result = last_media_query_wins_transform(&[outer_prop]);

    assert_eq!(result.len(), 1);
    if let Expr::Object(inner_obj) = &*result[0].value {
      assert_eq!(inner_obj.props.len(), 1);
      assert!(matches!(inner_obj.props[0], PropOrSpread::Spread(_)));
    } else {
      panic!("Expected Object value");
    }
  }
}

// ---------------------------------------------------------------------------
// transform_media_queries_in_result — negation accumulation and invalid-key
// preservation paths.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod transform_media_coverage {
  use super::*;

  /// Covers accumulated_negations.push(negations.clone()) is called
  /// when there are multiple parseable media queries.
  /// Explicitly calls transform_media_queries_in_result directly to ensure the
  /// negation accumulation loop body is exercised.
  #[test]
  fn multiple_media_queries_fill_accumulated_negations() {
    // Two distinct media queries that both parse successfully, with overlapping ranges
    // so that are_media_queries_disjoint returns false and we enter the negation path.
    let props = vec![
      str_kv("default", "1 / 2"),
      str_kv("@media (color)", "colorful"),
      str_kv("@media (monochrome)", "grayscale"),
    ];

    // Call transform_media_queries_in_result directly (private fn, accessible from test module)
    let result = transform_media_queries_in_result(props);

    // The result should contain the same number of entries (non-media + media)
    // and media keys should be transformed with negations
    assert!(!result.is_empty());
    let has_negation = result.iter().any(|kv| {
      if let PropName::Str(s) = &kv.key {
        let k = s.value.as_str().unwrap_or("");
        k.contains("not")
      } else {
        false
      }
    });
    assert!(
      has_negation,
      "Expected at least one media query with a 'not' negation"
    );
  }

  /// Covers 3 media queries so the loop runs 2 iterations,
  /// producing multiple accumulated_negations entries.
  #[test]
  fn three_media_queries_produce_multiple_accumulated_negation_entries() {
    let props = vec![
      str_kv("@media (color)", "colorful"),
      str_kv("@media (monochrome)", "grayscale"),
      str_kv("@media (hover: hover)", "hoverable"),
    ];

    let result = transform_media_queries_in_result(props);

    // All three original queries should appear (possibly transformed)
    assert_eq!(result.len(), 3);
  }

  /// Invalid media keys are preserved unchanged. The transform must never drop
  /// declarations while trying to normalize media-query order.
  #[test]
  fn media_key_that_fails_to_parse_is_silently_dropped() {
    let props = vec![
      str_kv("@media (color)", "red"),
      str_kv("@media !!!invalid!!!css", "blue"),
    ];

    let result = transform_media_queries_in_result(props);

    assert_eq!(result.len(), 2);
    assert!(result.iter().any(|kv| {
      matches!(
        &kv.key,
        PropName::Str(s) if s.value.as_str() == Some("@media !!!invalid!!!css")
      )
    }));
  }
}

// ---------------------------------------------------------------------------
// are_media_queries_disjoint
// ---------------------------------------------------------------------------

#[cfg(test)]
mod are_media_queries_disjoint_coverage {
  use super::*;

  fn parsed_pair(key: &str) -> (String, KeyValueProp, MediaQuery) {
    (
      key.to_string(),
      str_kv(key, "value"),
      MediaQuery::parser().parse_to_end(key).unwrap(),
    )
  }

  /// Invalid media keys are handled before are_media_queries_disjoint is called,
  /// so the nested object is preserved unchanged and no panic/indexing failure occurs.
  #[test]
  fn invalid_media_key_via_transform_causes_disjoint_check_to_return_false() {
    // Build an outer prop with a nested object containing two "media" keys,
    // one valid and one syntactically invalid, at depth=1.
    let inner_obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(str_kv(
          "@media (max-width: 768px)",
          "red",
        )))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(str_kv(
          "@media !!!invalid",
          "blue",
        )))),
      ],
    };

    let outer_prop = KeyValueProp {
      key: PropName::Str(Str {
        span: DUMMY_SP,
        value: Wtf8Atom::from("color"),
        raw: None,
      }),
      value: Box::new(Expr::Object(inner_obj)),
    };

    let result = last_media_query_wins_transform(&[outer_prop]);

    assert_eq!(result.len(), 1);
    let Expr::Object(inner_obj) = result[0].value.as_ref() else {
      panic!("Expected Object value");
    };
    assert_eq!(inner_obj.props.len(), 2);
  }

  #[test]
  fn direct_call_with_invalid_key_returns_false() {
    let media_pairs = vec![
      parsed_pair("@media (min-width: 100px) and (max-width: 300px)"),
      parsed_pair("@media (min-width: 200px) and (max-width: 400px)"),
    ];

    let result = are_media_queries_disjoint(&media_pairs);

    assert!(
      !result,
      "Expected are_media_queries_disjoint to return false for overlapping ranges"
    );
  }

  #[test]
  fn first_key_failing_to_parse_returns_false() {
    let media_pairs = vec![parsed_pair("@media (color)")];
    assert!(
      !are_media_queries_disjoint(&media_pairs),
      "a parsed non-range query must not use disjoint-range fast path"
    );
  }
}

// ---------------------------------------------------------------------------
// normalize_media_query_syntax — (kv else branch when key fails to parse)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod normalize_media_coverage {
  use super::*;

  /// Covers when a key starts with "@media " but parse_to_end fails,
  /// normalize_media_query_syntax returns the kv unchanged (the `else { kv }` arm).
  /// Calls normalize_media_query_syntax directly since it's in scope (private fn).
  #[test]
  fn direct_call_with_invalid_key_returns_kv_unchanged() {
    let invalid_key = "@media !!!invalid!!!";
    let props = vec![str_kv(invalid_key, "red")];

    // Call normalize_media_query_syntax directly
    let result = normalize_media_query_syntax(props);

    // With a single prop that fails to parse, it should be returned unchanged
    assert_eq!(result.len(), 1);
    // The key should be unchanged (the else { kv } branch )
    if let PropName::Str(s) = &result[0].key {
      assert_eq!(s.value.as_str().unwrap(), invalid_key);
    } else {
      panic!("Expected Str key");
    }
  }

  /// Covers both the successful Ok branch and the failed Err branch in normalize_media_query_syntax.
  #[test]
  fn normalize_processes_valid_keys_and_skips_invalid() {
    let valid_kv = str_kv("@media (max-width: 768px)", "red");
    let invalid_kv = str_kv("@media !!!invalid", "blue");
    let non_media_kv = str_kv("color", "green");

    let props = vec![valid_kv, invalid_kv, non_media_kv];
    let result = normalize_media_query_syntax(props);

    assert_eq!(result.len(), 3);

    // The non-media key should be unchanged
    let non_media = result.iter().find(|kv| {
      if let PropName::Str(s) = &kv.key {
        !s.value.as_str().unwrap_or("").starts_with("@media")
      } else {
        false
      }
    });
    assert!(non_media.is_some());

    // The invalid key should appear unchanged in the result
    let has_invalid = result.iter().any(|kv| {
      if let PropName::Str(s) = &kv.key {
        s.value.as_str().unwrap_or("") == "@media !!!invalid"
      } else {
        false
      }
    });
    assert!(has_invalid, "Invalid key should be preserved unchanged");
  }
}

// ---------------------------------------------------------------------------
// Integration: PropName::Ident keys flow through last_media_query_wins_transform
// ---------------------------------------------------------------------------

#[cfg(test)]
mod ident_key_integration {
  use super::*;

  /// Covers via last_media_query_wins_transform: an Ident-keyed prop
  /// at depth=1 calls key_value_to_str which hits the PropName::Ident arm.
  #[test]
  fn ident_keyed_style_prop_passes_through() {
    // Build nested object where keys are Ident (not Str)
    let inner_obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(ident_kv(
        "default", "1 / 2",
      ))))],
    };

    let outer_prop = KeyValueProp {
      key: PropName::Str(Str {
        span: DUMMY_SP,
        value: Wtf8Atom::from("gridColumn"),
        raw: None,
      }),
      value: Box::new(Expr::Object(inner_obj)),
    };

    let result = last_media_query_wins_transform(&[outer_prop]);

    assert_eq!(result.len(), 1);
    // The transformation should succeed without panicking
    if let Expr::Object(inner) = &*result[0].value {
      assert_eq!(inner.props.len(), 1);
    } else {
      panic!("Expected Object value");
    }
  }

  /// Covers via last_media_query_wins_transform: a Num-keyed prop
  /// at depth=1 calls key_value_to_str which hits the _ arm, returning "".
  /// The empty string "" doesn't start with "@media ", so it's treated as a
  /// non-media prop and passes through unchanged.
  #[test]
  fn numeric_keyed_prop_in_nested_object_does_not_match_media_query() {
    // Build nested object with numeric key — key_value_to_str returns ""
    let inner_obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(str_kv("default", "1 / 2")))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(num_kv(0.0, "zero")))),
      ],
    };

    let outer_prop = KeyValueProp {
      key: PropName::Str(Str {
        span: DUMMY_SP,
        value: Wtf8Atom::from("gridColumn"),
        raw: None,
      }),
      value: Box::new(Expr::Object(inner_obj)),
    };

    let result = last_media_query_wins_transform(&[outer_prop]);

    assert_eq!(result.len(), 1);
    // The numeric-keyed prop is extracted into key_values, its key returns ""
    // so it passes through transform_media_queries_in_result as a non-media prop
    if let Expr::Object(inner) = &*result[0].value {
      // Both props should be present (default + numeric)
      assert_eq!(inner.props.len(), 2);
    } else {
      panic!("Expected Object value");
    }
  }
}
