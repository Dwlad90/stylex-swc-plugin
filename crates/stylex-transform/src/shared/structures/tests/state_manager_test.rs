#[cfg(test)]
mod state_manager {
  use rustc_hash::FxHashSet;
  use swc_core::{
    common::{BytePos, DUMMY_SP, Span, SyntaxContext},
    ecma::ast::{
      AssignPatProp, BindingIdent, CallExpr, Callee, Decl, Expr, ExprStmt, Ident, IdentName,
      ImportDecl, ImportPhase, Lit, MemberExpr, MemberProp, Module, ModuleDecl, ModuleItem,
      ObjectLit, ObjectPat, ObjectPatProp, Pat, Prop, PropName, PropOrSpread, Stmt, Str, VarDecl,
      VarDeclKind, VarDeclarator,
    },
  };

  use crate::shared::structures::state_manager::{
    DeclId, InsertionSlot, StateManager, build_decl_use_graph, compute_live_set,
    flush_pending_insertions,
  };
  use stylex_enums::top_level_expression::TopLevelExpressionKind;
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

  fn id(name: &str) -> DeclId {
    ident(name).to_id()
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

  fn module(body: Vec<ModuleItem>) -> Module {
    Module {
      span: DUMMY_SP,
      body,
      shebang: None,
    }
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

  fn assert_set_eq(actual: &FxHashSet<DeclId>, expected: &[DeclId]) {
    let expected: FxHashSet<DeclId> = expected.iter().cloned().collect();
    assert_eq!(actual, &expected);
  }

  #[test]
  fn compute_live_set_handles_cycles_self_refs_transitive_and_disconnected_graphs() {
    let mut self_ref_state = StateManager::default();
    self_ref_state
      .decl_uses
      .insert(id("A"), [id("A")].into_iter().collect());
    self_ref_state.roots.insert(id("A"));
    assert_set_eq(&compute_live_set(&self_ref_state), &[id("A")]);

    let mut cycle_state = StateManager::default();
    cycle_state
      .decl_uses
      .insert(id("A"), [id("B")].into_iter().collect());
    cycle_state
      .decl_uses
      .insert(id("B"), [id("A")].into_iter().collect());
    cycle_state.roots.insert(id("A"));
    assert_set_eq(&compute_live_set(&cycle_state), &[id("A"), id("B")]);

    let mut transitive_state = StateManager::default();
    transitive_state
      .decl_uses
      .insert(id("A"), [id("B")].into_iter().collect());
    transitive_state
      .decl_uses
      .insert(id("B"), [id("C")].into_iter().collect());
    transitive_state.roots.insert(id("A"));
    assert_set_eq(
      &compute_live_set(&transitive_state),
      &[id("A"), id("B"), id("C")],
    );

    let mut disconnected_state = StateManager::default();
    disconnected_state
      .decl_uses
      .insert(id("A"), FxHashSet::default());
    disconnected_state
      .decl_uses
      .insert(id("B"), [id("C")].into_iter().collect());
    disconnected_state.roots.insert(id("A"));
    disconnected_state.roots.insert(id("B"));
    assert_set_eq(
      &compute_live_set(&disconnected_state),
      &[id("A"), id("B"), id("C")],
    );

    let mut empty_roots_state = StateManager::default();
    empty_roots_state
      .decl_uses
      .insert(id("A"), [id("B")].into_iter().collect());
    assert!(compute_live_set(&empty_roots_state).is_empty());
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

  #[test]
  fn build_decl_use_graph_excludes_property_keys_and_member_properties() {
    let mut state = StateManager::default();
    let object_init = Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(
        swc_core::ecma::ast::KeyValueProp {
          key: PropName::Ident(IdentName::new("foo".into(), DUMMY_SP)),
          value: Box::new(ident_expr("bar")),
        },
      )))],
    });
    let member_init = Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(ident_expr("obj")),
      prop: MemberProp::Ident(IdentName::new("baz".into(), DUMMY_SP)),
    });
    let module = module(vec![
      var_decl_item("x", object_init),
      var_decl_item("y", member_init),
    ]);

    build_decl_use_graph(&module, &mut state);

    assert_set_eq(state.decl_uses.get(&id("x")).unwrap(), &[id("bar")]);
    assert_set_eq(state.decl_uses.get(&id("y")).unwrap(), &[id("obj")]);
  }

  #[test]
  fn build_decl_use_graph_routes_non_ident_destructuring_uses_to_roots() {
    let mut state = StateManager::default();
    let destructuring_decl = VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Object(ObjectPat {
        span: DUMMY_SP,
        props: vec![ObjectPatProp::Assign(AssignPatProp {
          span: DUMMY_SP,
          key: BindingIdent {
            id: ident("a"),
            type_ann: None,
          },
          value: None,
        })],
        optional: false,
        type_ann: None,
      }),
      init: Some(Box::new(ident_expr("b"))),
      definite: false,
    };
    let module = module(vec![ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(
      VarDecl {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![destructuring_decl],
      },
    ))))]);

    build_decl_use_graph(&module, &mut state);

    assert!(state.decl_uses.is_empty());
    assert!(state.roots.contains(&id("b")));
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

    state.top_level_expressions.push(TopLevelExpression(
      TopLevelExpressionKind::NamedExport,
      Expr::Call(first.clone()),
      Some("first".into()),
    ));
    state.top_level_expressions.push(TopLevelExpression(
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

    state.top_level_expressions.push(TopLevelExpression(
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

    state.top_level_expressions.push(TopLevelExpression(
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
      state
        .find_call_declaration_by_span(&second)
        .and_then(|decl| decl.name.as_ident())
        .map(|ident| ident.sym.as_str()),
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
}
