#[cfg(test)]
mod state_manager {
  use swc_core::{
    atoms::Atom,
    common::{BytePos, DUMMY_SP, EqIgnoreSpan, Span, SyntaxContext},
    ecma::ast::{
      ArrayLit, BindingIdent, CallExpr, Callee, Decl, Expr, ExprOrSpread, ExprStmt, Ident,
      ImportDecl, ImportDefaultSpecifier, ImportNamedSpecifier, ImportPhase, ImportSpecifier,
      ImportStarAsSpecifier, Lit, ModuleDecl, ModuleItem, ObjectLit, ObjectPat, Pat, Stmt, Str,
      VarDecl, VarDeclKind, VarDeclarator,
    },
  };

  use crate::state_manager::{InsertionSlot, StateManager, flush_pending_insertions};
  use stylex_enums::declaration_type::DeclarationType;
  use stylex_enums::top_level_expression::TopLevelExpressionKind;
  use stylex_structures::ceiling::Ceiling;
  use stylex_structures::evaluation_depth::MAX_EVALUATION_DEPTH;
  use stylex_structures::fold_ceilings::{MAX_FOLDED_CHARACTERS, MAX_FOLDED_ENTRIES};
  use stylex_structures::top_level_expression::TopLevelExpression;
  use stylex_utils::hash::stable_hash_unspanned;

  fn ident(name: &str) -> Ident {
    Ident {
      span: DUMMY_SP,
      sym: name.into(),
      optional: false,
      ctxt: SyntaxContext::empty(),
    }
  }

  fn ident_expr(name: &str) -> Expr {
    Expr::Ident(ident(name))
  }

  fn string_expr(value: &str) -> Expr {
    Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: value.into(),
      raw: None,
    }))
  }

  fn expr_stmt(value: &str) -> ModuleItem {
    ModuleItem::Stmt(Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(string_expr(value)),
    }))
  }

  fn import_stmt(source: &str) -> ModuleItem {
    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![],
      src: Box::new(Str {
        span: DUMMY_SP,
        value: source.into(),
        raw: None,
      }),
      type_only: false,
      with: None,
      phase: ImportPhase::Evaluation,
    }))
  }

  fn var_declarator(name: &str, init: Expr) -> VarDeclarator {
    VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Ident(BindingIdent {
        id: ident(name),
        type_ann: None,
      }),
      init: Some(Box::new(init)),
      definite: false,
    }
  }

  fn var_decl_item(name: &str, init: Expr) -> ModuleItem {
    ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
      span: DUMMY_SP,
      ctxt: SyntaxContext::empty(),
      kind: VarDeclKind::Const,
      declare: false,
      decls: vec![var_declarator(name, init)],
    }))))
  }

  fn item_label(item: &ModuleItem) -> String {
    match item {
      ModuleItem::Stmt(Stmt::Expr(expr_stmt)) => match expr_stmt.expr.as_lit() {
        Some(Lit::Str(value)) => value.value.as_str().unwrap_or("").to_string(),
        _ => "expr".to_string(),
      },
      ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => {
        let decl = &var_decl.decls[0];
        match decl.name.as_ident() {
          Some(binding) => format!("var:{}", binding.id.sym),
          None => "var:pattern".to_string(),
        }
      },
      ModuleItem::ModuleDecl(ModuleDecl::Import(import)) => {
        format!("import:{}", import.src.value.as_str().unwrap_or(""))
      },
      _ => "other".to_string(),
    }
  }

  fn item_labels(items: &[ModuleItem]) -> Vec<String> {
    items.iter().map(item_label).collect()
  }

  #[test]
  fn flush_pending_insertions_places_each_slot_in_emit_order() {
    let mut state = StateManager::default();
    let styles_init = Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![],
    });
    let before_decl_hash = stable_hash_unspanned(&styles_init);
    let mut body = vec![
      expr_stmt("use strict"),
      import_stmt("existing"),
      var_decl_item("styles", styles_init),
    ];

    state.queue_insertion(InsertionSlot::AfterImports, expr_stmt("after_imports"));
    state.queue_insertion(
      InsertionSlot::BeforeDecl(before_decl_hash),
      expr_stmt("before_decl"),
    );
    state.queue_insertion(InsertionSlot::ThemeImports, import_stmt("theme"));
    state.queue_insertion(InsertionSlot::BeforeImports, expr_stmt("before_imports"));

    flush_pending_insertions(&mut state, &mut body, true);

    assert_eq!(
      item_labels(&body),
      vec![
        "use strict",
        "before_imports",
        "import:theme",
        "import:existing",
        "after_imports",
        "before_decl",
        "var:styles",
      ]
    );
  }

  #[test]
  fn flush_pending_insertions_drops_runtime_gated_slots_when_runtime_injection_is_disabled() {
    let mut state = StateManager::default();
    let styles_init = Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![],
    });
    let before_decl_hash = stable_hash_unspanned(&styles_init);
    let mut body = vec![
      expr_stmt("use strict"),
      import_stmt("existing"),
      var_decl_item("styles", styles_init),
    ];

    state.queue_insertion(InsertionSlot::AfterImports, expr_stmt("after_imports"));
    state.queue_insertion(
      InsertionSlot::BeforeDecl(before_decl_hash),
      expr_stmt("before_decl"),
    );
    state.queue_insertion(InsertionSlot::ThemeImports, import_stmt("theme"));
    state.queue_insertion(InsertionSlot::BeforeImports, expr_stmt("before_imports"));

    flush_pending_insertions(&mut state, &mut body, false);

    assert_eq!(
      item_labels(&body),
      vec![
        "use strict",
        "import:existing",
        "after_imports",
        "var:styles",
      ]
    );
  }

  /// A zero-argument call, the shape every `defineMarker()` shares, carrying
  /// only the position that tells two of them apart.
  fn call_at(span: Span) -> CallExpr {
    CallExpr {
      span,
      ctxt: SyntaxContext::empty(),
      callee: Callee::Expr(Box::new(ident_expr("defineMarker"))),
      args: vec![],
      type_args: None,
    }
  }

  fn span_at(lo: u32, hi: u32) -> Span {
    Span {
      lo: BytePos(lo),
      hi: BytePos(hi),
    }
  }

  #[test]
  fn find_top_level_expr_by_span_pins_the_entry_recorded_from_that_call() {
    let mut state = StateManager::default();

    let first = call_at(span_at(1, 10));
    let second = call_at(span_at(20, 30));

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::NamedExport,
      Expr::Call(first.clone()),
      Some("first".into()),
    ));
    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      Expr::Call(second.clone()),
      Some("second".into()),
    ));

    // The two calls are equal ignoring spans, so only the span tells the
    // entries apart.
    assert_eq!(
      state.find_top_level_expr_by_span(&second).map(|tpe| &tpe.0),
      Some(&TopLevelExpressionKind::Stmt)
    );
    assert_eq!(
      state.find_top_level_expr_by_span(&first).map(|tpe| &tpe.0),
      Some(&TopLevelExpressionKind::NamedExport)
    );
    assert!(
      state
        .find_top_level_expr_by_span(&call_at(span_at(40, 50)))
        .is_none()
    );
  }

  #[test]
  fn find_top_level_expr_by_span_ignores_a_non_call_entry_at_the_same_position() {
    let mut state = StateManager::default();

    let call = call_at(span_at(1, 10));

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::NamedExport,
      Expr::Ident(Ident {
        span: span_at(1, 10),
        sym: "notACall".into(),
        optional: false,
        ctxt: SyntaxContext::empty(),
      }),
      Some("notACall".into()),
    ));

    // The span is only ever consulted to tell two recorded *calls* apart, so a
    // position match on some other expression is not a match.
    assert!(state.find_top_level_expr_by_span(&call).is_none());
  }

  #[test]
  fn find_top_level_expr_by_span_never_matches_a_spanless_call() {
    let mut state = StateManager::default();

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::NamedExport,
      Expr::Call(call_at(DUMMY_SP)),
      Some("synthesized".into()),
    ));

    assert!(
      state
        .find_top_level_expr_by_span(&call_at(DUMMY_SP))
        .is_none()
    );
  }

  #[test]
  fn find_call_declaration_index_by_span_pins_the_declarator_that_call_initialises() {
    let mut state = StateManager::default();

    let first = call_at(span_at(1, 10));
    let second = call_at(span_at(20, 30));

    state
      .declarations
      .push(var_declarator("first", Expr::Call(first.clone())));
    state
      .declarations
      .push(var_declarator("second", Expr::Call(second.clone())));
    state
      .declarations
      .push(var_declarator("notACall", ident_expr("other")));

    assert_eq!(state.find_call_declaration_index_by_span(&second), Some(1));
    assert_eq!(state.find_call_declaration_index_by_span(&first), Some(0));
    assert_eq!(
      state.find_call_declaration_index_by_span(&call_at(span_at(40, 50))),
      None
    );
  }

  #[test]
  fn find_call_declaration_by_span_reads_the_declarator_at_that_position() {
    let mut state = StateManager::default();

    let first = call_at(span_at(1, 10));
    let second = call_at(span_at(20, 30));

    state
      .declarations
      .push(var_declarator("first", Expr::Call(first)));
    state
      .declarations
      .push(var_declarator("second", Expr::Call(second.clone())));

    assert_eq!(
      bound_name(state.find_call_declaration_by_span(&second)),
      Some("second")
    );
    assert!(
      state
        .find_call_declaration_by_span(&call_at(span_at(40, 50)))
        .is_none()
    );
  }

  #[test]
  fn find_call_declaration_index_by_span_never_matches_a_spanless_call() {
    let mut state = StateManager::default();

    state
      .declarations
      .push(var_declarator("synthesized", Expr::Call(call_at(DUMMY_SP))));

    assert_eq!(
      state.find_call_declaration_index_by_span(&call_at(DUMMY_SP)),
      None
    );
  }

  /// The options struct holds each ceiling as a bare `usize`, which a
  /// struct-update literal can set to anything -- so what the three accessors
  /// answer is bracketed again where it is read rather than trusted as held.
  /// Without that bracket a zero would refuse every fold the compiler runs to do
  /// its own work, and a `usize::MAX` would be no ceiling at all.
  ///
  /// Asserted through the accessors rather than through `Ceiling::clamped`,
  /// because the claim is that these three readings are wired to it: a `clamped`
  /// pinned only in its own crate stays green when a call site here drops it.
  #[test]
  fn a_ceiling_held_by_the_options_struct_is_read_back_inside_its_bracket() {
    /// One reading: the option a project writes, the field the options struct
    /// holds it in, the accessor that spends it, and the ceiling that brackets
    /// it. Named rather than a tuple, so the loop below reads as the claim.
    struct Reading {
      option: &'static str,
      hold: fn(&mut StateManager, usize),
      spend: fn(&StateManager) -> usize,
      ceiling: &'static Ceiling,
    }

    let readings = [
      Reading {
        option: "maxEvaluationDepth",
        hold: |state, held| state.options.max_evaluation_depth = held,
        spend: StateManager::evaluation_ceiling,
        ceiling: &MAX_EVALUATION_DEPTH,
      },
      Reading {
        option: "maxFoldedCharacters",
        hold: |state, held| state.options.max_folded_characters = held,
        spend: StateManager::character_ceiling,
        ceiling: &MAX_FOLDED_CHARACTERS,
      },
      Reading {
        option: "maxFoldedEntries",
        hold: |state, held| state.options.max_folded_entries = held,
        spend: StateManager::entry_ceiling,
        ceiling: &MAX_FOLDED_ENTRIES,
      },
    ];

    for reading in readings {
      let ceiling = reading.ceiling;

      for (held, expected) in [
        (0, 1),
        (usize::MAX, ceiling.limit),
        (ceiling.limit + 1, ceiling.limit),
        (ceiling.default, ceiling.default),
      ] {
        let mut state = StateManager::default();
        (reading.hold)(&mut state, held);

        assert_eq!(
          (reading.spend)(&state),
          expected,
          "{}: held {held}",
          reading.option
        );
      }
    }
  }

  /// A call carrying `argument`, so two calls can differ in content rather than
  /// only in position.
  fn call_of(name: &str, argument: &str) -> CallExpr {
    CallExpr {
      span: DUMMY_SP,
      ctxt: SyntaxContext::empty(),
      callee: Callee::Expr(Box::new(ident_expr(name))),
      args: vec![ExprOrSpread {
        spread: None,
        expr: Box::new(string_expr(argument)),
      }],
      type_args: None,
    }
  }

  /// A call that carries both content and a position, so the two lookups can be
  /// asked about the same node.
  fn call_of_at(name: &str, argument: &str, span: Span) -> CallExpr {
    CallExpr {
      span,
      ..call_of(name, argument)
    }
  }

  /// The name a found declarator binds, for assertions that only care about it.
  fn bound_name(declarator: Option<&VarDeclarator>) -> Option<&str> {
    declarator
      .and_then(|decl| decl.name.as_ident())
      .map(|ident| ident.sym.as_str())
  }

  fn declarator_at(name: &str, span: Span, init: Expr) -> VarDeclarator {
    VarDeclarator {
      span,
      ..var_declarator(name, init)
    }
  }

  #[test]
  fn find_call_declaration_pins_the_declarator_that_call_initialises() {
    let mut state = StateManager::default();

    let first = call_of("create", "first");
    let second = call_of("create", "second");

    state.push_declaration(var_declarator("first", Expr::Call(first)));
    state.push_declaration(var_declarator("second", Expr::Call(second.clone())));
    state.push_declaration(var_declarator("plain", string_expr("no call here")));

    assert_eq!(
      bound_name(state.find_call_declaration(&second)),
      Some("second")
    );
    assert!(
      state
        .find_call_declaration(&call_of("create", "absent"))
        .is_none()
    );
  }

  #[test]
  fn find_call_declaration_answers_with_the_earliest_of_equal_declarators() {
    let mut state = StateManager::default();

    let call = call_of("create", "shared");

    state.push_declaration(var_declarator("earlier", Expr::Call(call.clone())));
    state.push_declaration(var_declarator("later", Expr::Call(call.clone())));

    // Two declarators can hold structurally equal calls, and the walk this
    // replaced answered with whichever came first in the module.
    assert_eq!(
      bound_name(state.find_call_declaration(&call)),
      Some("earlier")
    );
  }

  /// The two lookups answer different questions and must keep disagreeing.
  ///
  /// `find_call_declaration` is keyed on what a call *is*, so two calls that
  /// differ only in position collapse onto the earliest declarator holding one.
  /// `find_call_declaration_by_span` is keyed on where a call was *written*, so
  /// the same pair resolves to one declarator each. Routing either through the
  /// other would look like a tidy-up and would silently break whichever
  /// question it stopped answering.
  #[test]
  fn a_call_is_found_by_content_collapsed_and_by_position_apart() {
    let mut state = StateManager::default();

    let earlier = call_of_at("create", "shared", span_at(1, 10));
    let later = call_of_at("create", "shared", span_at(20, 30));

    state.push_declaration(var_declarator("earlier", Expr::Call(earlier.clone())));
    state.push_declaration(var_declarator("later", Expr::Call(later.clone())));

    for call in [&earlier, &later] {
      assert_eq!(
        bound_name(state.find_call_declaration(call)),
        Some("earlier")
      );
    }

    assert_eq!(
      bound_name(state.find_call_declaration_by_span(&earlier)),
      Some("earlier")
    );
    assert_eq!(
      bound_name(state.find_call_declaration_by_span(&later)),
      Some("later")
    );
  }

  /// The same disagreement at a width no hand-written module reaches, so the
  /// index answering from a bucket and the walk answering from a position are
  /// both exercised past the point where a linear fallback would be visible.
  #[test]
  fn content_and_position_keep_disagreeing_across_five_thousand_equal_calls() {
    const DECLARATORS: usize = 5_000;

    let mut state = StateManager::default();

    let calls: Vec<CallExpr> = (0..DECLARATORS)
      .map(|index| {
        let start = (index as u32 + 1) * 100;
        call_of_at("create", "shared", span_at(start, start + 10))
      })
      .collect();

    for (index, call) in calls.iter().enumerate() {
      state.push_declaration(var_declarator(
        &format!("styles{index}"),
        Expr::Call(call.clone()),
      ));
    }

    for index in [0usize, 1, 2_499, DECLARATORS - 1] {
      assert_eq!(
        bound_name(state.find_call_declaration(&calls[index])),
        Some("styles0"),
        "content lookup at {index}"
      );
      assert_eq!(
        bound_name(state.find_call_declaration_by_span(&calls[index])),
        Some(format!("styles{index}").as_str()),
        "positional lookup at {index}"
      );
    }

    // A position no declarator was written at is not the last one seen.
    assert!(
      state
        .find_call_declaration_by_span(&call_of_at("create", "shared", span_at(7, 8)))
        .is_none()
    );
  }

  #[test]
  fn find_call_declaration_ignores_a_declarator_with_no_initializer() {
    let mut state = StateManager::default();

    state.push_declaration(VarDeclarator {
      init: None,
      ..var_declarator("bare", string_expr("unused"))
    });

    assert!(
      state
        .find_call_declaration(&call_of("create", "anything"))
        .is_none()
    );
  }

  #[test]
  fn replacing_an_initializer_moves_the_declarator_to_the_call_it_now_holds() {
    let mut state = StateManager::default();

    let original = call_of("create", "original");
    let replacement = call_of("create", "replacement");

    state.push_declaration(var_declarator("styles", Expr::Call(original.clone())));
    state.set_declaration_init(0, Expr::Call(replacement.clone()));

    // The key the declarator was recorded under stops being true, so the call
    // it no longer holds must stop finding it.
    assert!(state.find_call_declaration(&original).is_none());
    assert!(state.find_call_declaration(&replacement).is_some());
  }

  #[test]
  fn replacing_an_initializer_with_a_non_call_leaves_it_findable_by_nothing() {
    let mut state = StateManager::default();

    let original = call_of("defineMarker", "marker");

    state.push_declaration(var_declarator("marker", Expr::Call(original.clone())));
    state.set_declaration_init(0, string_expr("a compiled marker object"));

    assert!(state.find_call_declaration(&original).is_none());
  }

  #[test]
  fn replacing_an_initializer_with_the_same_call_keeps_it_findable() {
    let mut state = StateManager::default();

    let call = call_of("create", "unchanged");

    state.push_declaration(var_declarator("styles", Expr::Call(call.clone())));
    state.set_declaration_init(0, Expr::Call(call.clone()));

    // Forgetting and recording happen in that order, so a replacement that
    // reads the same must not leave the record dropped.
    assert!(state.find_call_declaration(&call).is_some());
  }

  #[test]
  fn replacing_an_initializer_out_of_range_changes_nothing() {
    let mut state = StateManager::default();

    let call = call_of("create", "only");

    state.push_declaration(var_declarator("styles", Expr::Call(call.clone())));
    state.set_declaration_init(99, string_expr("nowhere"));

    assert!(state.find_call_declaration(&call).is_some());
  }

  #[test]
  fn find_top_level_expr_pins_the_entry_that_is_that_call() {
    let mut state = StateManager::default();

    let create = call_of("create", "styles");
    let theme = call_of("createTheme", "theme");

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      Expr::Call(create),
      Some("styles".into()),
    ));
    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::NamedExport,
      Expr::Call(theme.clone()),
      Some("theme".into()),
    ));

    assert_eq!(
      state.find_top_level_expr(&theme).map(|tpe| tpe.0),
      Some(TopLevelExpressionKind::NamedExport)
    );
    assert!(
      state
        .find_top_level_expr(&call_of("create", "absent"))
        .is_none()
    );
  }

  #[test]
  fn find_top_level_expr_ignores_an_entry_that_merely_holds_the_call() {
    let mut state = StateManager::default();

    let call = call_of("create", "styles");

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems: vec![Some(ExprOrSpread {
          spread: None,
          expr: Box::new(Expr::Call(call.clone())),
        })],
      }),
      Some("lotsOfStyles".into()),
    ));

    // The entry *is* the array, not the call inside it -- which is the whole
    // reason `has_top_level_expr` takes a predicate for the shapes that hold a
    // call without being one.
    assert!(state.find_top_level_expr(&call).is_none());
    assert!(state.has_top_level_expr(&call, |tpe| matches!(tpe.1, Expr::Array(_))));
    assert!(!state.has_top_level_expr(&call, |_| false));
  }

  /// The arrays a module records have to follow the list they come from, in
  /// both directions -- including the case where the last one is rewritten into
  /// something else.
  #[test]
  fn the_top_level_arrays_follow_the_list_they_come_from() {
    let mut state = StateManager::default();

    let held = call_of_at("create", "held", span_at(12, 30));
    let outside = call_of_at("create", "outside", span_at(90, 99));
    let array = |span| {
      Expr::Array(ArrayLit {
        span,
        elems: vec![Some(ExprOrSpread {
          spread: None,
          expr: Box::new(Expr::Call(held.clone())),
        })],
      })
    };

    assert!(!state.holds_call_in_top_level_array(&held));

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      Expr::Call(held.clone()),
      Some("styles".into()),
    ));

    // A call is not an array, whatever it holds.
    assert!(!state.holds_call_in_top_level_array(&held));

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      array(span_at(10, 40)),
      Some("lotsOfStyles".into()),
    ));

    assert!(state.holds_call_in_top_level_array(&held));
    // A call the array does not hold answers no, however many arrays the module
    // writes. That is the whole of what containment decides.
    assert!(!state.holds_call_in_top_level_array(&outside));

    // A second array over the same call, so the answer has something to come
    // back to.
    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      array(span_at(50, 80)),
      Some("moreStyles".into()),
    ));

    state.set_top_level_expr(1, string_expr("no longer an array"));
    // The second array does not hold the call, so nothing does.
    assert!(!state.holds_call_in_top_level_array(&held));

    state.set_top_level_expr(2, array(span_at(10, 40)));
    assert!(state.holds_call_in_top_level_array(&held));

    state.set_top_level_expr(2, string_expr("nor is this"));
    assert!(!state.holds_call_in_top_level_array(&held));

    // And a replacement out of range records nothing.
    state.set_top_level_expr(99, array(span_at(10, 40)));
    assert!(!state.holds_call_in_top_level_array(&held));
  }

  /// A synthesized call was written nowhere, so no recorded array can hold it.
  /// A dummy span reads as position zero, which an array starting at zero would
  /// otherwise contain.
  #[test]
  fn a_span_less_call_is_held_by_no_top_level_array() {
    let mut state = StateManager::default();

    let synthesized = call_of("create", "styles");

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      Expr::Array(ArrayLit {
        span: span_at(0, 40),
        elems: vec![],
      }),
      Some("lotsOfStyles".into()),
    ));

    assert!(!state.holds_call_in_top_level_array(&synthesized));
  }

  /// A bucket a lookup can stop early in is one nothing has moved. Rewriting an
  /// entry back to the expression it started with re-records it at the back, so
  /// the bucket stops being ascending -- and the answer has to stay the earliest
  /// entry the module writes, not the first the bucket holds.
  #[test]
  fn a_bucket_an_entry_moved_back_into_still_answers_with_the_earliest() {
    let mut state = StateManager::default();

    let shared = call_of("create", "shared");
    let other = call_of("create", "other");

    for name in ["first", "second", "third"] {
      state.push_top_level_expression(TopLevelExpression(
        TopLevelExpressionKind::Stmt,
        Expr::Call(shared.clone()),
        Some(name.into()),
      ));
    }

    // Out of the bucket and back into it, which leaves position 0 behind
    // positions 1 and 2 in it.
    state.set_top_level_expr(0, Expr::Call(other));
    state.set_top_level_expr(0, Expr::Call(shared.clone()));

    assert_eq!(
      state
        .find_top_level_expr(&shared)
        .and_then(|tpe| tpe.2.as_ref())
        .map(Atom::as_str),
      Some("first")
    );
  }

  #[test]
  fn has_top_level_expr_answers_from_the_index_before_the_predicate() {
    let mut state = StateManager::default();

    let call = call_of("create", "styles");

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      Expr::Call(call.clone()),
      Some("styles".into()),
    ));

    assert!(state.has_top_level_expr(&call, |_| panic!("the predicate must not be reached")));
  }

  #[test]
  fn replacing_a_top_level_expression_moves_it_to_the_call_it_now_is() {
    let mut state = StateManager::default();

    let original = call_of("create", "original");
    let replacement = call_of("create", "replacement");

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      Expr::Call(original.clone()),
      Some("styles".into()),
    ));
    state.set_top_level_expr(0, Expr::Call(replacement.clone()));

    assert!(state.find_top_level_expr(&original).is_none());
    assert!(state.find_top_level_expr(&replacement).is_some());
  }

  #[test]
  fn find_top_level_expr_named_answers_for_the_name_it_binds() {
    let mut state = StateManager::default();

    let shared = Expr::Call(call_of("create", "shared"));

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::NamedExport,
      shared.clone(),
      Some("first".into()),
    ));
    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      shared.clone(),
      Some("second".into()),
    ));

    // Both entries read the same, so only the name tells them apart -- where a
    // walk comparing expressions answered with whichever came first for either.
    assert_eq!(
      state
        .find_top_level_expr_named(&"second".into(), &shared)
        .map(|tpe| tpe.0),
      Some(TopLevelExpressionKind::Stmt)
    );
    assert_eq!(
      state
        .find_top_level_expr_named(&"first".into(), &shared)
        .map(|tpe| tpe.0),
      Some(TopLevelExpressionKind::NamedExport)
    );
  }

  #[test]
  fn find_top_level_expr_named_refuses_a_name_whose_expression_has_moved_on() {
    let mut state = StateManager::default();

    let recorded = Expr::Call(call_of("create", "recorded"));

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      recorded,
      Some("styles".into()),
    ));

    assert!(
      state
        .find_top_level_expr_named(&"styles".into(), &string_expr("something else"))
        .is_none()
    );
    assert!(
      state
        .find_top_level_expr_named(&"absent".into(), &string_expr("anything"))
        .is_none()
    );
  }

  #[test]
  fn holds_declaration_dedups_on_position_then_content() {
    let mut state = StateManager::default();

    let first = declarator_at("styles", span_at(1, 10), string_expr("value"));
    let same_position_other_content = declarator_at("styles", span_at(1, 10), string_expr("other"));
    let same_content_other_position =
      declarator_at("styles", span_at(20, 30), string_expr("value"));

    state.push_declaration(first.clone());

    assert!(state.holds_declaration(&first));
    assert!(!state.holds_declaration(&same_position_other_content));
    // Two declarations that merely read alike stay two entries, because a
    // lookup that pins a call to its declarator by span needs them to.
    assert!(!state.holds_declaration(&same_content_other_position));
  }

  #[test]
  fn holds_declaration_tells_synthesized_declarators_apart_by_content() {
    let mut state = StateManager::default();

    let first = declarator_at("a", DUMMY_SP, string_expr("first"));
    let second = declarator_at("b", DUMMY_SP, string_expr("second"));

    state.push_declaration(first.clone());

    // Every synthesized declarator shares `DUMMY_SP`, so content is all that
    // separates them.
    assert!(state.holds_declaration(&first));
    assert!(!state.holds_declaration(&second));
  }

  #[test]
  fn matching_style_var_answers_for_the_name_the_declarator_binds() {
    let mut state = StateManager::default();

    let declarator = var_declarator("styles", Expr::Call(call_of("create", "styles")));

    state.insert_style_var("styles".to_string(), declarator.clone());

    assert!(state.matching_style_var(&declarator).is_some());
    // A declarator of another name is another style variable, however it reads.
    assert!(
      state
        .matching_style_var(&var_declarator(
          "other",
          Expr::Call(call_of("create", "styles"))
        ))
        .is_none()
    );
    // And the recorded name having moved on is not a match either.
    assert!(
      state
        .matching_style_var(&var_declarator("styles", string_expr("something else")))
        .is_none()
    );
  }

  #[test]
  fn matching_style_var_refuses_a_declarator_bound_to_a_pattern() {
    let mut state = StateManager::default();

    let init = Expr::Call(call_of("create", "styles"));

    state.insert_style_var("styles".to_string(), var_declarator("styles", init.clone()));

    let pattern_bound = VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Object(ObjectPat {
        span: DUMMY_SP,
        props: vec![],
        optional: false,
        type_ann: None,
      }),
      init: Some(Box::new(init)),
      definite: false,
    };

    assert!(state.matching_style_var(&pattern_bound).is_none());
  }

  #[test]
  fn replacing_a_style_var_initializer_moves_it_to_the_call_it_now_holds() {
    let mut state = StateManager::default();

    let original = call_of("create", "original");
    let replacement = call_of("create", "replacement");

    state.insert_style_var(
      "styles".to_string(),
      var_declarator("styles", Expr::Call(original)),
    );
    state.set_style_var_init("styles".to_string(), Expr::Call(replacement.clone()));

    assert_eq!(
      state.style_vars["styles"]
        .init
        .as_deref()
        .and_then(Expr::as_call)
        .map(|call| call.eq_ignore_span(&replacement)),
      Some(true)
    );
  }

  #[test]
  fn inserting_a_style_var_twice_forgets_the_call_the_first_one_held() {
    let mut state = StateManager::default();

    let first = call_of("create", "first");
    let second = call_of("create", "second");

    state.insert_style_var(
      "styles".to_string(),
      var_declarator("styles", Expr::Call(first)),
    );
    state.insert_style_var(
      "styles".to_string(),
      var_declarator("styles", Expr::Call(second.clone())),
    );

    assert!(
      state
        .matching_style_var(&var_declarator("styles", Expr::Call(second)))
        .is_some()
    );
  }

  #[test]
  fn declared_as_tells_a_class_from_a_function_and_keeps_the_first_position() {
    let mut state = StateManager::default();

    let class_name = Ident {
      span: span_at(1, 10),
      ..ident("Widget")
    };
    let later_spelling = Ident {
      span: span_at(50, 60),
      ..ident("Widget")
    };

    state.add_class_name_declaration(class_name);
    state.add_class_name_declaration(later_spelling);
    state.add_function_name_declaration(ident("render"));

    assert!(matches!(
      state.declared_as(&ident("Widget")),
      Some(DeclarationType::Class)
    ));
    assert!(matches!(
      state.declared_as(&ident("render")),
      Some(DeclarationType::Function)
    ));
    assert!(state.declared_as(&ident("unbound")).is_none());
    // First writer wins, which is the position a used-before-declaration
    // diagnostic is about.
    assert_eq!(
      state.class_name_declaration(&ident("Widget")),
      Some(span_at(1, 10))
    );
  }

  #[test]
  fn a_declaration_resolves_only_for_the_binding_its_own_context_names() {
    let mut state = StateManager::default();

    let outer = ident("Widget");
    // A context of its own, spelled directly rather than through a `Mark`,
    // which needs a `GLOBALS` scope this unit test has no other use for.
    let shadowed = Ident {
      ctxt: SyntaxContext::from_u32(1),
      ..ident("Widget")
    };

    state.add_class_name_declaration(outer.clone());

    assert!(matches!(
      state.declared_as(&outer),
      Some(DeclarationType::Class)
    ));
    // A name shadowing the class declares something else, and resolving it to
    // the declaration it shadows is what keying on the symbol alone would do.
    assert!(state.declared_as(&shadowed).is_none());
  }

  /// Far past any authored module, to show the lookups answer from a key rather
  /// than by walking what has been recorded.
  #[test]
  fn the_lookups_stay_exact_across_ten_thousand_declarations() {
    let mut state = StateManager::default();

    let calls: Vec<CallExpr> = (0..10_000)
      .map(|index| call_of("create", &format!("styles{index}")))
      .collect();

    for (index, call) in calls.iter().enumerate() {
      let name = format!("styles{index}");

      state.push_declaration(var_declarator(&name, Expr::Call(call.clone())));
      state.push_top_level_expression(TopLevelExpression(
        TopLevelExpressionKind::Stmt,
        Expr::Call(call.clone()),
        Some(name.as_str().into()),
      ));
    }

    for index in [0usize, 1, 4_999, 9_998, 9_999] {
      assert_eq!(
        bound_name(state.find_call_declaration(&calls[index])),
        Some(format!("styles{index}").as_str())
      );
      assert_eq!(
        state
          .find_top_level_expr(&calls[index])
          .and_then(|tpe| tpe.2.as_ref())
          .map(Atom::as_str),
        Some(format!("styles{index}").as_str())
      );
    }

    assert!(
      state
        .find_call_declaration(&call_of("create", "styles10000"))
        .is_none()
    );
  }

  /// A named import of `names` from `source`.
  fn named_import(source: &str, names: &[&str]) -> ImportDecl {
    ImportDecl {
      span: DUMMY_SP,
      specifiers: names
        .iter()
        .map(|name| {
          ImportSpecifier::Named(ImportNamedSpecifier {
            span: DUMMY_SP,
            local: ident(name),
            imported: None,
            is_type_only: false,
          })
        })
        .collect(),
      src: Box::new(Str {
        span: DUMMY_SP,
        value: source.into(),
        raw: None,
      }),
      type_only: false,
      with: None,
      phase: ImportPhase::Evaluation,
    }
  }

  #[test]
  fn find_top_level_expr_named_answers_for_a_redeclared_name() {
    let mut state = StateManager::default();

    let first = Expr::Call(call_of("create", "first"));
    let second = Expr::Call(call_of("create", "second"));

    // `var styles = ...; var styles = ...;` is legal, so one name can bind two
    // entries and the second must stay findable.
    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::NamedExport,
      first.clone(),
      Some("styles".into()),
    ));
    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      second.clone(),
      Some("styles".into()),
    ));

    assert_eq!(
      state
        .find_top_level_expr_named(&"styles".into(), &second)
        .map(|tpe| tpe.0),
      Some(TopLevelExpressionKind::Stmt)
    );
    assert_eq!(
      state
        .find_top_level_expr_named(&"styles".into(), &first)
        .map(|tpe| tpe.0),
      Some(TopLevelExpressionKind::NamedExport)
    );
  }

  #[test]
  fn find_top_level_expr_named_ignores_an_entry_bound_to_another_name() {
    let mut state = StateManager::default();

    let shared = Expr::Call(call_of("create", "shared"));

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::NamedExport,
      shared.clone(),
      Some("exported".into()),
    ));
    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      shared.clone(),
      Some("styles".into()),
    ));

    // Both entries read the same, and the exported one comes first. The answer
    // is the entry this name binds, not whichever the module spells first --
    // the kind belongs to a declarator, and reading another's is what the walk
    // this replaced did.
    assert_eq!(
      state
        .find_top_level_expr_named(&"styles".into(), &shared)
        .map(|tpe| tpe.0),
      Some(TopLevelExpressionKind::Stmt)
    );
  }

  #[test]
  fn replacing_out_of_range_entries_changes_nothing() {
    let mut state = StateManager::default();

    let call = call_of("create", "only");

    state.push_top_level_expression(TopLevelExpression(
      TopLevelExpressionKind::Stmt,
      Expr::Call(call.clone()),
      Some("styles".into()),
    ));
    state.insert_style_var(
      "styles".to_string(),
      var_declarator("styles", Expr::Call(call.clone())),
    );

    state.set_top_level_expr(99, string_expr("nowhere"));
    state.set_style_var_init("absent".to_string(), string_expr("nowhere"));

    assert!(state.find_top_level_expr(&call).is_some());
    assert!(
      state
        .matching_style_var(&var_declarator("styles", Expr::Call(call)))
        .is_some()
    );
  }

  #[test]
  fn import_binding_answers_for_the_binding_a_specifier_declares() {
    let mut state = StateManager::default();

    state.push_top_import(named_import("theme.stylex.js", &["spacing", "colors"]));
    state.push_top_import(named_import("other.stylex.js", &["radii"]));

    assert_eq!(
      state
        .import_binding(&ident("colors"))
        .and_then(|(import, _)| import.src.value.as_str()),
      Some("theme.stylex.js")
    );
    assert_eq!(
      state
        .import_binding(&ident("radii"))
        .and_then(|(import, _)| import.src.value.as_str()),
      Some("other.stylex.js")
    );
    assert!(state.import_binding(&ident("unbound")).is_none());
  }

  /// `import * as stylex` binds through another arm of `local_binding_of` than
  /// the named form the helper above builds, and the namespace form is what
  /// every StyleX module actually writes.
  #[test]
  fn import_binding_answers_for_a_namespace_specifier() {
    let mut state = StateManager::default();

    state.push_top_import(ImportDecl {
      specifiers: vec![ImportSpecifier::Namespace(ImportStarAsSpecifier {
        span: DUMMY_SP,
        local: ident("stylex"),
      })],
      ..named_import("@stylexjs/stylex", &[])
    });

    assert!(state.import_binding(&ident("stylex")).is_some());
    assert!(state.import_binding(&ident("other")).is_none());
  }

  /// `import stylex from` is the third arm, and the only one whose binding is
  /// the declaration's own default.
  #[test]
  fn import_binding_answers_for_a_default_specifier() {
    let mut state = StateManager::default();

    state.push_top_import(ImportDecl {
      specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
        span: DUMMY_SP,
        local: ident("stylex"),
      })],
      ..named_import("@stylexjs/stylex", &[])
    });

    assert!(state.import_binding(&ident("stylex")).is_some());
    assert!(state.import_binding(&ident("other")).is_none());
  }

  #[test]
  fn import_binding_refuses_a_reference_from_another_scope() {
    let mut state = StateManager::default();

    state.push_top_import(named_import("theme.stylex.js", &["spacing"]));

    // A parameter named after an imported theme carries a context of its own,
    // and resolving it to the import it shadows is the bug this keying avoids.
    let shadowing = Ident {
      ctxt: SyntaxContext::from_u32(1),
      ..ident("spacing")
    };

    assert!(state.import_binding(&ident("spacing")).is_some());
    assert!(state.import_binding(&shadowing).is_none());
  }

  #[test]
  fn import_binding_answers_with_the_earliest_of_two_imports_of_one_name() {
    let mut state = StateManager::default();

    state.push_top_import(named_import("first.stylex.js", &["spacing"]));
    state.push_top_import(named_import("second.stylex.js", &["spacing"]));

    assert_eq!(
      state
        .import_binding(&ident("spacing"))
        .and_then(|(import, _)| import.src.value.as_str()),
      Some("first.stylex.js")
    );
  }

  /// Far past any authored module, to show the lookup answers from a key rather
  /// than by walking every specifier the module imports.
  #[test]
  fn import_binding_stays_exact_across_ten_thousand_specifiers() {
    let mut state = StateManager::default();

    let names: Vec<String> = (0..10_000).map(|index| format!("token{index}")).collect();

    for chunk in names.chunks(100) {
      let borrowed: Vec<&str> = chunk.iter().map(String::as_str).collect();

      state.push_top_import(named_import("tokens.stylex.js", &borrowed));
    }

    for index in [0usize, 1, 4_999, 9_999] {
      assert!(state.import_binding(&ident(&names[index])).is_some());
    }

    assert!(state.import_binding(&ident("token10000")).is_none());
  }

  mod css_property_seen_tests {
    use crate::state_manager::StateManager;

    #[test]
    fn reads_back_what_the_mutable_accessor_wrote() {
      let mut state = StateManager::default();

      assert!(state.css_property_seen().is_empty());

      state
        .css_property_seen_mut()
        .insert("marginInlineStart".to_string(), "10px".to_string());

      assert_eq!(
        state.css_property_seen().get("marginInlineStart"),
        Some(&"10px".to_string())
      );
    }
  }
}
