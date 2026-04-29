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

#[cfg(test)]
mod unspanned_fast_path_tests {
  use super::super::{create_hash, stable_hash, stable_hash_unspanned, to_base36};
  use swc_core::{
    common::{DUMMY_SP, FileName, SourceMap, SyntaxContext, input::StringInput, sync::Lrc},
    ecma::{
      ast::{
        ArrayLit, ArrowExpr, AssignProp, AwaitExpr, BigInt, BinExpr, BinaryOp, BlockStmtOrExpr,
        Bool, Callee, ComputedPropName, CondExpr, Expr, ExprOrSpread, Ident, IdentName, Import,
        ImportPhase, JSXText, KeyValueProp, Lit, MemberExpr, MemberProp, MetaPropExpr,
        MetaPropKind, NewExpr, Null, Number, ObjectLit, OptCall, OptChainBase, OptChainExpr,
        ParenExpr, Pat, PrivateName, Prop, PropName, PropOrSpread, Regex, SeqExpr, SpreadElement,
        Str, Super, SuperProp, SuperPropExpr, TaggedTpl, ThisExpr, Tpl, TplElement, UnaryExpr,
        UnaryOp, UpdateExpr, UpdateOp, YieldExpr,
      },
      utils::drop_span,
    },
  };
  use swc_ecma_parser::{EsSyntax, Parser, Syntax, lexer::Lexer};

  #[test]
  fn base36_handles_zero_directly() {
    assert_eq!(to_base36(0), "0");
    assert_eq!(create_hash("Button.root"), create_hash("Button.root"));
  }

  #[test]
  fn unspanned_hash_fast_path_covers_supported_expression_shapes() {
    let expressions = vec![
      Expr::This(ThisExpr { span: DUMMY_SP }),
      Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems: vec![
          Some(expr_or_spread(ident("a"))),
          Some(ExprOrSpread {
            spread: Some(DUMMY_SP),
            expr: Box::new(ident("b")),
          }),
          None,
        ],
      }),
      Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: vec![
          PropOrSpread::Spread(SpreadElement {
            dot3_token: DUMMY_SP,
            expr: Box::new(ident("spread")),
          }),
          prop(Prop::Shorthand(ident_pat("short"))),
          prop(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(IdentName::new("ident".into(), DUMMY_SP)),
            value: Box::new(number(1.0)),
          })),
          prop(Prop::KeyValue(KeyValueProp {
            key: PropName::Str(Str {
              span: DUMMY_SP,
              value: "str".into(),
              raw: None,
            }),
            value: Box::new(string("value")),
          })),
          prop(Prop::KeyValue(KeyValueProp {
            key: PropName::Num(Number {
              span: DUMMY_SP,
              value: 1.0,
              raw: None,
            }),
            value: Box::new(bool_expr(true)),
          })),
          prop(Prop::KeyValue(KeyValueProp {
            key: PropName::Computed(ComputedPropName {
              span: DUMMY_SP,
              expr: Box::new(ident("computed")),
            }),
            value: Box::new(null()),
          })),
          prop(Prop::KeyValue(KeyValueProp {
            key: PropName::BigInt(BigInt {
              span: DUMMY_SP,
              value: Box::new(7u32.into()),
              raw: None,
            }),
            value: Box::new(regex("x", "g")),
          })),
          prop(Prop::Assign(AssignProp {
            span: DUMMY_SP,
            key: ident_pat("assigned"),
            value: Box::new(string("assigned-value")),
          })),
        ],
      }),
      Expr::Unary(UnaryExpr {
        span: DUMMY_SP,
        op: UnaryOp::Bang,
        arg: Box::new(bool_expr(false)),
      }),
      Expr::Update(UpdateExpr {
        span: DUMMY_SP,
        op: UpdateOp::PlusPlus,
        prefix: true,
        arg: Box::new(ident("counter")),
      }),
      Expr::Bin(BinExpr {
        span: DUMMY_SP,
        op: BinaryOp::Add,
        left: Box::new(number(1.0)),
        right: Box::new(number(2.0)),
      }),
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(ident("obj")),
        prop: MemberProp::Ident(IdentName::new("prop".into(), DUMMY_SP)),
      }),
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(ident("obj")),
        prop: MemberProp::PrivateName(PrivateName {
          span: DUMMY_SP,
          name: "secret".into(),
        }),
      }),
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(ident("obj")),
        prop: MemberProp::Computed(ComputedPropName {
          span: DUMMY_SP,
          expr: Box::new(string("prop")),
        }),
      }),
      Expr::SuperProp(SuperPropExpr {
        span: DUMMY_SP,
        obj: Super { span: DUMMY_SP },
        prop: SuperProp::Ident(IdentName::new("x".into(), DUMMY_SP)),
      }),
      Expr::SuperProp(SuperPropExpr {
        span: DUMMY_SP,
        obj: Super { span: DUMMY_SP },
        prop: SuperProp::Computed(ComputedPropName {
          span: DUMMY_SP,
          expr: Box::new(number(0.0)),
        }),
      }),
      Expr::Cond(CondExpr {
        span: DUMMY_SP,
        test: Box::new(ident("test")),
        cons: Box::new(string("yes")),
        alt: Box::new(string("no")),
      }),
      Expr::Call(swc_core::ecma::ast::CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(Box::new(ident("fn"))),
        args: vec![expr_or_spread(number(1.0))],
        type_args: None,
      }),
      Expr::Call(swc_core::ecma::ast::CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Super(Super { span: DUMMY_SP }),
        args: vec![],
        type_args: None,
      }),
      Expr::Call(swc_core::ecma::ast::CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Import(Import {
          span: DUMMY_SP,
          phase: ImportPhase::Evaluation,
        }),
        args: vec![expr_or_spread(string("./dep"))],
        type_args: None,
      }),
      Expr::New(NewExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Box::new(ident("Ctor")),
        args: Some(vec![expr_or_spread(number(1.0))]),
        type_args: None,
      }),
      Expr::New(NewExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Box::new(ident("Ctor")),
        args: None,
        type_args: None,
      }),
      Expr::Seq(SeqExpr {
        span: DUMMY_SP,
        exprs: vec![Box::new(ident("a")), Box::new(ident("b"))],
      }),
      ident("id"),
      string("str"),
      bool_expr(true),
      null(),
      number(1.0),
      Expr::Lit(Lit::BigInt(BigInt {
        span: DUMMY_SP,
        value: Box::new(10u32.into()),
        raw: None,
      })),
      regex("x", "gi"),
      Expr::Lit(Lit::JSXText(JSXText {
        span: DUMMY_SP,
        value: "jsx".into(),
        raw: "jsx".into(),
      })),
      tpl(vec![ident("value")]),
      Expr::TaggedTpl(TaggedTpl {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        tag: Box::new(ident("tag")),
        type_params: None,
        tpl: Box::new(tpl_node(vec![ident("value")])),
      }),
      Expr::Arrow(ArrowExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        params: vec![Pat::Ident(ident_pat("arg").into())],
        body: Box::new(BlockStmtOrExpr::Expr(Box::new(ident("arg")))),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
      }),
      Expr::Arrow(ArrowExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        params: vec![Pat::Expr(Box::new(ident("expr_pat")))],
        body: Box::new(BlockStmtOrExpr::Expr(Box::new(ident("expr_pat")))),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
      }),
      Expr::Yield(YieldExpr {
        span: DUMMY_SP,
        arg: Some(Box::new(ident("yielded"))),
        delegate: false,
      }),
      Expr::Yield(YieldExpr {
        span: DUMMY_SP,
        arg: None,
        delegate: true,
      }),
      Expr::MetaProp(MetaPropExpr {
        span: DUMMY_SP,
        kind: MetaPropKind::ImportMeta,
      }),
      Expr::Await(AwaitExpr {
        span: DUMMY_SP,
        arg: Box::new(ident("promise")),
      }),
      Expr::Paren(ParenExpr {
        span: DUMMY_SP,
        expr: Box::new(ident("inner")),
      }),
      Expr::OptChain(OptChainExpr {
        span: DUMMY_SP,
        optional: true,
        base: Box::new(OptChainBase::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(ident("obj")),
          prop: MemberProp::Ident(IdentName::new("prop".into(), DUMMY_SP)),
        })),
      }),
      Expr::OptChain(OptChainExpr {
        span: DUMMY_SP,
        optional: true,
        base: Box::new(OptChainBase::Call(OptCall {
          span: DUMMY_SP,
          ctxt: SyntaxContext::empty(),
          callee: Box::new(ident("maybeFn")),
          args: vec![expr_or_spread(number(1.0))],
          type_args: None,
        })),
      }),
    ];

    for expr in expressions {
      assert_eq!(stable_hash_unspanned(&expr), stable_hash_unspanned(&expr));
    }
  }

  #[test]
  fn unspanned_hash_falls_back_for_unsupported_shapes_without_changing_result() {
    let unsupported = vec![
      parse_expr("function named() {}"),
      parse_expr("class Foo {}"),
      parse_expr("({ get value() { return 1; } })"),
      parse_expr("(value) => { return value; }"),
      parse_expr("([value]) => value"),
      Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems: (0..129)
          .map(|_| Some(expr_or_spread(number(1.0))))
          .collect(),
      }),
      Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: (0..129)
          .map(|idx| {
            prop(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(IdentName::new(format!("key{idx}").into(), DUMMY_SP)),
              value: Box::new(number(idx as f64)),
            }))
          })
          .collect(),
      }),
    ];

    for expr in unsupported {
      assert_eq!(stable_hash_unspanned(&expr), stable_hash(&drop_span(expr)));
    }
  }

  fn parse_expr(source: &str) -> Expr {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Anon.into(), source.to_string());
    let lexer = Lexer::new(
      Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
      }),
      Default::default(),
      StringInput::from(&*fm),
      None,
    );
    let mut parser = Parser::new_from(lexer);

    match parser.parse_expr() {
      Ok(expr) => *expr,
      Err(error) => panic!("failed to parse expression `{source}`: {error:?}"),
    }
  }

  fn ident(sym: &str) -> Expr {
    Expr::Ident(ident_pat(sym))
  }

  fn ident_pat(sym: &str) -> Ident {
    Ident {
      span: DUMMY_SP,
      ctxt: SyntaxContext::empty(),
      sym: sym.into(),
      optional: false,
    }
  }

  fn number(value: f64) -> Expr {
    Expr::Lit(Lit::Num(Number {
      span: DUMMY_SP,
      value,
      raw: None,
    }))
  }

  fn string(value: &str) -> Expr {
    Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: value.into(),
      raw: None,
    }))
  }

  fn bool_expr(value: bool) -> Expr {
    Expr::Lit(Lit::Bool(Bool {
      span: DUMMY_SP,
      value,
    }))
  }

  fn null() -> Expr {
    Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))
  }

  fn regex(exp: &str, flags: &str) -> Expr {
    Expr::Lit(Lit::Regex(Regex {
      span: DUMMY_SP,
      exp: exp.into(),
      flags: flags.into(),
    }))
  }

  fn tpl(exprs: Vec<Expr>) -> Expr {
    Expr::Tpl(tpl_node(exprs))
  }

  fn tpl_node(exprs: Vec<Expr>) -> Tpl {
    Tpl {
      span: DUMMY_SP,
      exprs: exprs.into_iter().map(Box::new).collect(),
      quasis: vec![TplElement {
        span: DUMMY_SP,
        tail: true,
        cooked: Some("text".into()),
        raw: "text".into(),
      }],
    }
  }

  fn expr_or_spread(expr: Expr) -> ExprOrSpread {
    ExprOrSpread {
      spread: None,
      expr: Box::new(expr),
    }
  }

  fn prop(prop: Prop) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(prop))
  }
}
