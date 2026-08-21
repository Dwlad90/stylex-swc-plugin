#[cfg(test)]
mod key_span_index {
  use std::cmp::Reverse;

  use rustc_hash::FxHashSet;
  use swc_core::{
    atoms::Atom,
    common::{BytePos, DUMMY_SP, FileName, SourceMap, Span, SyntaxContext, sync::Lrc},
    ecma::{
      ast::{
        CallExpr, Callee, EsVersion, Expr, ExprStmt, Ident, IdentName, KeyValueProp, Lit, Module,
        ModuleItem, Number, ObjectLit, Prop, PropName, PropOrSpread, Stmt,
      },
      parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer},
    },
  };

  use crate::shared::structures::key_span_index::{CandidateRank, KeySpanIndex, NamespaceKeyQuery};

  fn parse(source: &str) -> Module {
    let source_map: Lrc<SourceMap> = Default::default();
    let source_file = source_map.new_source_file(FileName::Anon.into(), source.to_owned());
    let lexer = Lexer::new(
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      EsVersion::EsNext,
      StringInput::from(&*source_file),
      None,
    );

    match Parser::new_from(lexer).parse_module() {
      Ok(module) => module,
      Err(error) => panic!("failed to parse the fixture: {:?}", error),
    }
  }

  fn keys(names: &[&str]) -> FxHashSet<Atom> {
    names.iter().map(|name| Atom::from(*name)).collect()
  }

  fn rank(
    namespace_value_overlap: usize,
    sibling_overlap: usize,
    distance_from_target: Option<u32>,
  ) -> CandidateRank {
    CandidateRank {
      namespace_value_overlap,
      sibling_overlap,
      distance_from_target: Reverse(distance_from_target),
    }
  }

  /// A lookup for `key`, spelled out rather than read off a compiled call, so a
  /// test says which signals it is giving the index.
  fn query<'a>(
    key: &'a str,
    siblings: &[&str],
    value_keys: &[&str],
    target_lo: Option<BytePos>,
  ) -> NamespaceKeyQuery<'a> {
    NamespaceKeyQuery {
      namespace_key: key,
      sibling_keys: keys(siblings),
      namespace_value_keys: keys(value_keys),
      target_lo,
    }
  }

  /// The line `source` holds `key` on, 1-based, so a resolved span can be
  /// asserted as a position an author would recognise rather than as a byte
  /// offset.
  fn line_of(source: &str, needle: &str) -> usize {
    let offset = match source.find(needle) {
      Some(offset) => offset,
      None => panic!("the fixture does not contain {needle}"),
    };

    source[..offset].matches('\n').count() + 1
  }

  /// The 1-based line of `span` within `source`.
  ///
  /// Each fixture is parsed into a source map of its own, whose first byte is
  /// `BytePos(1)`, so a span's offset into the file is one less than its `lo`.
  fn line_at(source: &str, span: Span) -> usize {
    assert!(!span.is_dummy(), "the lookup resolved no position");

    let offset = span.lo.0 as usize - 1;

    source[..offset].matches('\n').count() + 1
  }

  fn resolved_line(source: &str, key: &str, siblings: &[&str], value_keys: &[&str]) -> usize {
    let module = parse(source);
    let span = KeySpanIndex::build(&module).resolve(&query(key, siblings, value_keys, None));

    line_at(source, span)
  }

  #[test]
  fn resolves_a_namespace_key_to_the_line_it_is_written_on() {
    let source = "\
const styles = stylex.create({
  root: { color: 'red' },
  hovered: { color: 'blue' },
});
";

    assert_eq!(
      resolved_line(source, "hovered", &["root", "hovered"], &["color"]),
      line_of(source, "hovered:")
    );
  }

  #[test]
  fn a_key_no_object_argument_spells_resolves_to_nothing() {
    let module = parse("const styles = stylex.create({ root: { color: 'red' } });");

    let span = KeySpanIndex::build(&module).resolve(&query("missing", &["root"], &["color"], None));

    assert_eq!(span, DUMMY_SP, "a key nothing spells must not resolve");
  }

  #[test]
  fn the_namespace_value_keys_pick_between_two_calls_spelling_the_same_key() {
    let source = "\
const first = stylex.create({
  root: { color: 'red' },
});
const second = stylex.create({
  root: { display: 'flex', flexGrow: 1 },
});
";

    assert_eq!(
      resolved_line(source, "root", &["root"], &["display", "flexGrow"]),
      line_of(source, "root: { display")
    );
  }

  #[test]
  fn two_equally_good_candidates_resolve_to_nothing() {
    let source = "\
const first = stylex.create({
  root: { color: 'red' },
});
const second = stylex.create({
  root: { color: 'red' },
});
";
    let module = parse(source);

    let span = KeySpanIndex::build(&module).resolve(&query("root", &["root"], &["color"], None));

    assert_eq!(
      span, DUMMY_SP,
      "a tie must be refused rather than guessed: a wrong file:line is worse than none"
    );
  }

  #[test]
  fn the_position_nearest_the_compiled_call_breaks_a_tie() {
    let source = "\
const first = stylex.create({
  root: { color: 'red' },
});
const second = stylex.create({
  root: { color: 'red' },
});
";
    let module = parse(source);
    let index = KeySpanIndex::build(&module);

    // The two calls are indistinguishable by keys alone -- above, without a
    // target position, this same fixture refuses. Pointing the target at one
    // call's object argument is what tells them apart.
    let second_object_lo = match source.rfind("({") {
      Some(offset) => BytePos(offset as u32 + 2 + 1),
      None => panic!("the fixture no longer holds a call"),
    };

    let span = index.resolve(&query(
      "root",
      &["root"],
      &["color"],
      Some(second_object_lo),
    ));

    assert_eq!(
      line_at(source, span),
      // The second of the two `root:` lines.
      line_of(source, "root:") + 3,
      "the candidate nearest the compiled call must win a tie the keys cannot break"
    );
  }

  #[test]
  fn a_key_written_twice_in_one_object_is_one_candidate_not_two() {
    let source = "\
const styles = stylex.create({
  root: { color: 'red' },
  root: { color: 'blue' },
});
";

    // Two candidates would tie and refuse; one resolves, and it is the property
    // a runtime object literal keeps.
    assert_eq!(
      resolved_line(source, "root", &["root"], &["color"]),
      line_of(source, "root: { color: 'blue' }")
    );
  }

  #[test]
  fn a_duplicate_key_is_ranked_by_the_value_that_survives_it() {
    let source = "\
const first = stylex.create({
  root: { color: 'red' },
  root: someVar,
});
const second = stylex.create({
  root: { color: 'red' },
});
";

    // The first call spells `root` twice and shares two sibling keys with the
    // query, so it would win on sibling overlap alone. It loses because the
    // occurrence that survives -- the last one, as at runtime -- has no value
    // object to overlap with, and value overlap outranks siblings.
    assert_eq!(
      resolved_line(source, "root", &["root"], &["color"]),
      line_of(source, "root: { color: 'red' },\n});\n"),
      "a duplicate key must be ranked by the value of its last occurrence"
    );
  }

  /// A call at `call_lo` whose object argument carries no position of its own
  /// and binds `key` at `key_span` -- the shape only a synthesized module has,
  /// and the one the position fallback exists for.
  fn call_with_positionless_object(call_lo: u32, key: &str, key_span: Span) -> Stmt {
    let prop = KeyValueProp {
      key: PropName::Ident(IdentName::new(key.into(), key_span)),
      value: Box::new(Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: 1.0,
        raw: None,
      }))),
    };

    Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(Expr::Call(CallExpr {
        span: Span::new(BytePos(call_lo), BytePos(call_lo + 1)),
        callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
          "create".into(),
          DUMMY_SP,
          SyntaxContext::empty(),
        )))),
        args: vec![
          Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(prop)))],
          })
          .into(),
        ],
        type_args: None,
        ctxt: SyntaxContext::empty(),
      })),
    })
  }

  #[test]
  fn a_positionless_object_is_placed_by_its_call() {
    let near_key = Span::new(BytePos(20), BytePos(24));
    let module = Module {
      span: DUMMY_SP,
      shebang: None,
      body: vec![
        ModuleItem::Stmt(call_with_positionless_object(
          100,
          "root",
          Span::new(BytePos(10), BytePos(14)),
        )),
        ModuleItem::Stmt(call_with_positionless_object(200, "root", near_key)),
      ],
    };

    let span =
      KeySpanIndex::build(&module).resolve(&query("root", &["root"], &[], Some(BytePos(190))));

    assert_eq!(
      span, near_key,
      "with no object position to measure from, the candidate must be placed by \
       its call: the second one is written nearer the target"
    );
  }

  #[test]
  fn an_object_that_names_nothing_is_not_a_candidate() {
    let source = "\
const spread = stylex.create({
  ...base,
});
const styles = stylex.create({
  root: { color: 'red' },
});
";

    // The first call's object holds no namespace key at all, so it contributes
    // no candidate and cannot tie with, or outrank, the one that does.
    assert_eq!(
      resolved_line(source, "root", &["root"], &["color"]),
      line_of(source, "root: { color: 'red' }")
    );
  }

  #[test]
  fn rank_prefers_value_overlap_then_sibling_overlap_then_proximity() {
    // Namespace-value overlap dominates every other signal.
    assert!(rank(2, 0, Some(100)) > rank(1, 9, Some(0)));

    // Sibling-key overlap breaks value-overlap ties.
    assert!(rank(1, 3, Some(100)) > rank(1, 2, Some(0)));

    // Smaller distance to the target wins a full overlap tie.
    assert!(rank(1, 3, Some(5)) > rank(1, 3, Some(6)));

    // No target position outranks any measured distance (Option: None < Some).
    assert!(rank(1, 3, None) > rank(1, 3, Some(0)));

    // Identical signals rank equal, which a lookup reports as ambiguous.
    assert_eq!(rank(1, 3, Some(5)), rank(1, 3, Some(5)));
  }
}
