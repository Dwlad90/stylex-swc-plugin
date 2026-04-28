#[cfg(test)]
mod create_hash_tests {
  use crate::hash::{create_hash, create_key_hash};

  #[test]
  fn returns_consistent_hash() {
    let hash1 = create_hash("hello");
    let hash2 = create_hash("hello");
    assert_eq!(hash1, hash2);
  }

  #[test]
  fn different_inputs_produce_different_hashes() {
    assert_ne!(create_hash("hello"), create_hash("world"));
  }

  #[test]
  fn returns_non_empty_string() {
    assert!(!create_hash("test").is_empty());
  }

  #[test]
  fn handles_empty_string() {
    let hash = create_hash("");
    assert!(!hash.is_empty());
  }

  #[test]
  fn handles_unicode_input() {
    let hash = create_hash("日本語");
    assert!(!hash.is_empty());
  }

  #[test]
  fn handles_long_input() {
    let long = "a".repeat(10000);
    let hash = create_hash(&long);
    assert!(!hash.is_empty());
  }

  #[test]
  fn matches_radix_fmt_base36_output() {
    for input in ["", "hello", "world", "日本語", "a very long input string"] {
      let raw = murmur2::murmur2(input.as_bytes(), 1);
      assert_eq!(create_hash(input), radix_fmt::radix(raw, 36).to_string());
    }
  }

  #[test]
  fn create_key_hash_matches_joined_key_hash() {
    assert_eq!(
      create_key_hash("Button.stylex", "root"),
      create_hash("Button.stylex.root")
    );
  }
}

#[cfg(test)]
mod stable_hash_tests {
  use crate::hash::{stable_hash, stable_hash_unspanned};
  use swc_core::{
    common::{BytePos, Span, SyntaxContext},
    ecma::ast::{Expr, Ident, IdentName, MemberExpr, MemberProp},
  };

  #[test]
  fn returns_consistent_hash_for_same_value() {
    assert_eq!(stable_hash(&42u64), stable_hash(&42u64));
  }

  #[test]
  fn different_values_produce_different_hashes() {
    assert_ne!(stable_hash(&1u64), stable_hash(&2u64));
  }

  #[test]
  fn works_with_strings() {
    assert_eq!(stable_hash(&"test"), stable_hash(&"test"));
    assert_ne!(stable_hash(&"a"), stable_hash(&"b"));
  }

  #[test]
  fn works_with_tuples() {
    assert_eq!(stable_hash(&(1, 2)), stable_hash(&(1, 2)));
    assert_ne!(stable_hash(&(1, 2)), stable_hash(&(2, 1)));
  }

  #[test]
  fn unspanned_expr_hash_ignores_nested_spans() {
    let expr_a = member_expr("foo", 1, "bar", 4);
    let expr_b = member_expr("foo", 10, "bar", 40);

    assert_eq!(
      stable_hash_unspanned(&expr_a),
      stable_hash_unspanned(&expr_b)
    );
  }

  #[test]
  fn unspanned_expr_hash_preserves_structure() {
    let expr_a = member_expr("foo", 1, "bar", 4);
    let expr_b = member_expr("foo", 1, "baz", 4);

    assert_ne!(
      stable_hash_unspanned(&expr_a),
      stable_hash_unspanned(&expr_b)
    );
  }

  fn member_expr(obj: &str, obj_start: u32, prop: &str, prop_start: u32) -> Expr {
    Expr::Member(MemberExpr {
      span: span(obj_start, prop_start + 3),
      obj: Box::new(Expr::Ident(ident(obj, obj_start))),
      prop: MemberProp::Ident(IdentName::new(
        prop.into(),
        span(prop_start, prop_start + 3),
      )),
    })
  }

  fn ident(sym: &str, start: u32) -> Ident {
    Ident {
      span: span(start, start + sym.len() as u32),
      ctxt: SyntaxContext::empty(),
      sym: sym.into(),
      optional: false,
    }
  }

  fn span(start: u32, end: u32) -> Span {
    Span::new(BytePos(start), BytePos(end))
  }
}

#[cfg(test)]
mod create_short_hash_tests {
  use crate::hash::create_short_hash;

  #[test]
  fn returns_consistent_hash() {
    assert_eq!(create_short_hash("hello"), create_short_hash("hello"));
  }

  #[test]
  fn different_inputs_produce_different_hashes() {
    assert_ne!(create_short_hash("hello"), create_short_hash("world"));
  }

  #[test]
  fn returns_non_empty_string() {
    assert!(!create_short_hash("test").is_empty());
  }

  #[test]
  fn produces_short_output() {
    // base62 encoded, mod 62^5, should be at most 5 chars
    assert!(create_short_hash("test").len() <= 5);
  }
}

#[cfg(test)]
mod hash_f64_tests {
  use crate::hash::hash_f64;

  #[test]
  fn returns_consistent_hash_for_same_value() {
    assert_eq!(hash_f64(1.23456), hash_f64(1.23456));
  }

  #[test]
  fn different_values_produce_different_hashes() {
    assert_ne!(hash_f64(1.0), hash_f64(2.0));
  }

  #[test]
  fn zero_and_neg_zero_differ() {
    // In IEEE 754, 0.0 and -0.0 have different bit patterns
    assert_ne!(hash_f64(0.0), hash_f64(-0.0));
  }

  #[test]
  fn handles_special_values() {
    let _ = hash_f64(f64::INFINITY);
    let _ = hash_f64(f64::NEG_INFINITY);
    let _ = hash_f64(f64::NAN);
  }
}
