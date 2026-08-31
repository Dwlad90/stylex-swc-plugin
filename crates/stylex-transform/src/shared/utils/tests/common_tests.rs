use swc_core::{
  common::{BytePos, DUMMY_SP, Span, SyntaxContext},
  ecma::ast::{
    BindingIdent, Decl, ExportDecl, Expr, Lit, Module, ModuleDecl, ModuleItem, Number, Pat, Stmt,
    Str, VarDecl, VarDeclKind, VarDeclarator,
  },
};

use crate::shared::{
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::common::{
    assign_props, downcast_style_options_to_state_manager, fill_state_declarations,
    fill_top_level_expressions, gen_file_based_identifier, get_css_value, get_import_by_ident,
    get_var_decl_by_ident, js_object_to_json, remove_duplicates, serialize_value_to_json_string,
    type_of,
  },
};
use stylex_ast::ast::convertors::{
  create_number_expr, create_string_expr, get_expr_from_var_decl, get_key_values_from_object,
  normalize_expr,
};
use stylex_ast::ast::factories::create_ident;

// ──────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────

fn make_var_declarator(name: &str, init: Expr) -> VarDeclarator {
  VarDeclarator {
    span: DUMMY_SP,
    name: Pat::Ident(BindingIdent {
      id: create_ident(name),
      type_ann: None,
    }),
    init: Some(Box::new(init)),
    definite: false,
  }
}

fn make_var_declarator_no_init(name: &str) -> VarDeclarator {
  VarDeclarator {
    span: DUMMY_SP,
    name: Pat::Ident(BindingIdent {
      id: create_ident(name),
      type_ann: None,
    }),
    init: None,
    definite: false,
  }
}

// ──────────────────────────────────────────────
// get_expr_from_var_decl
// ──────────────────────────────────────────────

mod get_expr_from_var_decl_tests {
  use super::*;

  #[test]
  fn returns_init_expr_for_number() {
    let decl = make_var_declarator("x", create_number_expr(42.0));
    let expr = get_expr_from_var_decl(&decl);
    match expr {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 42.0),
      _ => panic!("Expected number literal"),
    }
  }

  #[test]
  fn returns_init_expr_for_string() {
    let decl = make_var_declarator("name", create_string_expr("hello"));
    let expr = get_expr_from_var_decl(&decl);
    match expr {
      Expr::Lit(Lit::Str(s)) => assert_eq!(s.value.as_str().unwrap(), "hello"),
      _ => panic!("Expected string literal"),
    }
  }

  #[test]
  #[should_panic]
  fn panics_when_no_init() {
    let decl = make_var_declarator_no_init("x");
    get_expr_from_var_decl(&decl);
  }
}

// ──────────────────────────────────────────────
// normalize_expr
// ──────────────────────────────────────────────

mod normalize_expr_tests {
  use super::*;
  use swc_core::ecma::ast::ParenExpr;

  #[test]
  fn returns_non_paren_expr_unchanged() {
    let expr = create_number_expr(5.0);
    let result = normalize_expr(&expr);
    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 5.0),
      _ => panic!("Expected number literal"),
    }
  }

  #[test]
  fn unwraps_paren_expr() {
    let inner = create_number_expr(10.0);
    let expr = Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(inner),
    });
    let result = normalize_expr(&expr);
    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 10.0),
      _ => panic!("Expected unwrapped number literal"),
    }
  }

  #[test]
  fn unwraps_nested_paren_expr() {
    let inner = create_string_expr("nested");
    let paren1 = Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(inner),
    });
    let expr = Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(paren1),
    });
    let result = normalize_expr(&expr);
    match result {
      Expr::Lit(Lit::Str(s)) => {
        assert_eq!(s.value.as_str().unwrap(), "nested")
      },
      _ => panic!("Expected unwrapped string literal"),
    }
  }
}

// ──────────────────────────────────────────────
// serialize_value_to_json_string
// ──────────────────────────────────────────────

mod serialize_value_to_json_string_tests {
  use super::*;

  #[test]
  fn serializes_number() {
    let result = serialize_value_to_json_string(42);
    assert_eq!(result, "42");
  }

  #[test]
  fn serializes_float() {
    let result = serialize_value_to_json_string(1.75);
    assert_eq!(result, "1.75");
  }

  #[test]
  fn serializes_boolean_true() {
    let result = serialize_value_to_json_string(true);
    assert_eq!(result, "true");
  }

  #[test]
  fn serializes_boolean_false() {
    let result = serialize_value_to_json_string(false);
    assert_eq!(result, "false");
  }

  #[test]
  fn serializes_plain_string() {
    let result = serialize_value_to_json_string("hello");
    assert_eq!(result, "hello");
  }

  #[test]
  fn serializes_numeric_string_as_number() {
    let result = serialize_value_to_json_string("123");
    assert_eq!(result, "123");
  }

  #[test]
  fn serializes_null() {
    let result = serialize_value_to_json_string::<Option<i32>>(None);
    assert_eq!(result, "null");
  }

  #[test]
  fn serializes_array() {
    let result = serialize_value_to_json_string(vec![1, 2, 3]);
    assert_eq!(result, "[1,2,3]");
  }

  #[test]
  fn serializes_empty_string() {
    // Empty string wrapped in quotes, but length <= 2, goes to else branch
    let result = serialize_value_to_json_string("");
    assert_eq!(result, "\"\"");
  }
}

// ──────────────────────────────────────────────
// gen_file_based_identifier
// ──────────────────────────────────────────────

mod gen_file_based_identifier_tests {
  use super::*;

  #[test]
  fn generates_identifier_without_key() {
    let result = gen_file_based_identifier("file.js", "styles", None);
    assert_eq!(result, "file.js//styles");
  }

  #[test]
  fn generates_identifier_with_key() {
    let result = gen_file_based_identifier("file.js", "styles", Some("color"));
    assert_eq!(result, "file.js//styles.color");
  }

  #[test]
  fn handles_empty_file_name() {
    let result = gen_file_based_identifier("", "export", None);
    assert_eq!(result, "//export");
  }

  #[test]
  fn handles_empty_export_name() {
    let result = gen_file_based_identifier("file.js", "", None);
    assert_eq!(result, "file.js//");
  }
}

// ──────────────────────────────────────────────
// remove_duplicates
// ──────────────────────────────────────────────

mod remove_duplicates_tests {
  use super::*;
  use swc_core::ecma::ast::{IdentName, KeyValueProp, Prop, PropName, PropOrSpread};

  fn make_kv_prop(key: &str, val: f64) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: key.into(),
      }),
      value: Box::new(create_number_expr(val)),
    })))
  }

  #[test]
  fn keeps_unique_props() {
    let props = vec![make_kv_prop("a", 1.0), make_kv_prop("b", 2.0)];
    let result = remove_duplicates(props);
    assert_eq!(result.len(), 2);
  }

  #[test]
  fn removes_duplicate_keeping_last() {
    let props = vec![
      make_kv_prop("a", 1.0),
      make_kv_prop("b", 2.0),
      make_kv_prop("a", 3.0),
    ];
    let result = remove_duplicates(props);
    assert_eq!(result.len(), 2);

    // Last "a" (value=3.0) should win, ordering: [b, a]
    if let PropOrSpread::Prop(prop) = &result[1]
      && let Prop::KeyValue(kv) = prop.as_ref()
      && let Expr::Lit(Lit::Num(n)) = kv.value.as_ref()
    {
      assert_eq!(n.value, 3.0);
    }
  }

  #[test]
  fn handles_empty_props() {
    let result = remove_duplicates(vec![]);
    assert!(result.is_empty());
  }

  #[test]
  fn handles_all_duplicates() {
    let props = vec![
      make_kv_prop("x", 1.0),
      make_kv_prop("x", 2.0),
      make_kv_prop("x", 3.0),
    ];
    let result = remove_duplicates(props);
    assert_eq!(result.len(), 1);
  }
}

// ──────────────────────────────────────────────
// fill_state_declarations
// ──────────────────────────────────────────────

mod fill_state_declarations_tests {
  use super::*;

  #[test]
  fn adds_declaration_to_empty_state() {
    let mut state = StateManager::default();
    let decl = make_var_declarator("x", create_number_expr(1.0));
    fill_state_declarations(&mut state, &decl);
    assert_eq!(state.declarations.len(), 1);
  }

  #[test]
  fn does_not_add_duplicate_declaration() {
    let mut state = StateManager::default();
    let decl = make_var_declarator("x", create_number_expr(1.0));
    fill_state_declarations(&mut state, &decl);
    fill_state_declarations(&mut state, &decl);
    assert_eq!(state.declarations.len(), 1);
  }

  #[test]
  fn adds_different_declarations() {
    let mut state = StateManager::default();
    let decl1 = make_var_declarator("x", create_number_expr(1.0));
    let decl2 = make_var_declarator("y", create_number_expr(2.0));
    fill_state_declarations(&mut state, &decl1);
    fill_state_declarations(&mut state, &decl2);
    assert_eq!(state.declarations.len(), 2);
  }

  /// `var m = f(); var m = f();` — two declarations that read the same but sit
  /// at different positions. Collapsing them loses the second one, and a
  /// transform that pins a call to its declarator by span then finds nothing
  /// to rewrite, leaving the call in the output.
  #[test]
  fn adds_identical_declarations_from_different_positions() {
    let mut state = StateManager::default();

    let first = VarDeclarator {
      span: Span {
        lo: BytePos(1),
        hi: BytePos(10),
      },
      ..make_var_declarator("m", create_number_expr(1.0))
    };
    let second = VarDeclarator {
      span: Span {
        lo: BytePos(20),
        hi: BytePos(30),
      },
      ..make_var_declarator("m", create_number_expr(1.0))
    };

    fill_state_declarations(&mut state, &first);
    fill_state_declarations(&mut state, &second);

    assert_eq!(state.declarations.len(), 2);

    // Re-recording either one on a later discovery pass is still a no-op.
    fill_state_declarations(&mut state, &first);
    fill_state_declarations(&mut state, &second);

    assert_eq!(state.declarations.len(), 2);
  }
}

// ──────────────────────────────────────────────
// fill_top_level_expressions
// ──────────────────────────────────────────────

mod fill_top_level_expressions_tests {
  use super::*;

  #[test]
  fn handles_empty_module() {
    let mut state = StateManager::default();
    let module = Module {
      span: DUMMY_SP,
      body: vec![],
      shebang: None,
    };
    fill_top_level_expressions(&module, &mut state);
    assert!(state.top_level_expressions.is_empty());
    assert!(state.declarations.is_empty());
  }

  #[test]
  fn captures_exported_var_decl() {
    let mut state = StateManager::default();

    let decl = VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Ident(BindingIdent {
        id: create_ident("styles"),
        type_ann: None,
      }),
      init: Some(Box::new(create_number_expr(42.0))),
      definite: false,
    };

    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
        span: DUMMY_SP,
        decl: Decl::Var(Box::new(VarDecl {
          span: DUMMY_SP,
          kind: VarDeclKind::Const,
          declare: false,
          decls: vec![decl],
          ctxt: SyntaxContext::empty(),
        })),
      }))],
      shebang: None,
    };

    fill_top_level_expressions(&module, &mut state);
    assert_eq!(state.top_level_expressions.len(), 1);
    assert_eq!(state.declarations.len(), 1);
  }

  #[test]
  fn captures_plain_var_stmt() {
    let mut state = StateManager::default();

    let decl = VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Ident(BindingIdent {
        id: create_ident("localVar"),
        type_ann: None,
      }),
      init: Some(Box::new(create_string_expr("value"))),
      definite: false,
    };

    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![decl],
        ctxt: SyntaxContext::empty(),
      }))))],
      shebang: None,
    };

    fill_top_level_expressions(&module, &mut state);
    assert_eq!(state.top_level_expressions.len(), 1);
    assert_eq!(state.declarations.len(), 1);
  }

  #[test]
  fn skips_var_decl_without_init() {
    let mut state = StateManager::default();

    let decl = make_var_declarator_no_init("noInit");

    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![decl],
        ctxt: SyntaxContext::empty(),
      }))))],
      shebang: None,
    };

    fill_top_level_expressions(&module, &mut state);
    assert!(state.top_level_expressions.is_empty());
    assert!(state.declarations.is_empty());
  }

  #[test]
  fn captures_multiple_decls_in_one_statement() {
    let mut state = StateManager::default();

    let decl1 = make_var_declarator("a", create_number_expr(1.0));
    let decl2 = make_var_declarator("b", create_number_expr(2.0));

    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![decl1, decl2],
        ctxt: SyntaxContext::empty(),
      }))))],
      shebang: None,
    };

    fill_top_level_expressions(&module, &mut state);
    assert_eq!(state.top_level_expressions.len(), 2);
    assert_eq!(state.declarations.len(), 2);
  }

  #[test]
  fn captures_default_export() {
    let mut state = StateManager::default();
    use swc_core::ecma::ast::ExportDefaultExpr;

    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(
        ExportDefaultExpr {
          span: DUMMY_SP,
          expr: Box::new(create_number_expr(99.0)),
        },
      ))],
      shebang: None,
    };

    fill_top_level_expressions(&module, &mut state);
    assert_eq!(state.top_level_expressions.len(), 1);
    // Default exports don't add to declarations
    assert!(state.declarations.is_empty());
  }
}

// ──────────────────────────────────────────────
// fill_top_level_expressions - ParenExpr branch
// ──────────────────────────────────────────────

mod fill_top_level_expressions_paren_tests {
  use super::*;
  use swc_core::ecma::ast::{ExportDefaultExpr, ParenExpr};

  #[test]
  fn captures_paren_wrapped_default_export() {
    let mut state = StateManager::default();

    let inner = create_number_expr(99.0);
    let paren = Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(inner),
    });

    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(
        ExportDefaultExpr {
          span: DUMMY_SP,
          expr: Box::new(paren),
        },
      ))],
      shebang: None,
    };

    fill_top_level_expressions(&module, &mut state);
    assert_eq!(state.top_level_expressions.len(), 1);
    assert!(state.declarations.is_empty());
  }
}

// ──────────────────────────────────────────────
// remove_duplicates - additional branches
// ──────────────────────────────────────────────

mod remove_duplicates_extra_tests {
  use super::*;
  use swc_core::ecma::ast::{IdentName, KeyValueProp, Prop, PropName, PropOrSpread, SpreadElement};

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

  #[test]
  fn deduplicates_shorthand_props() {
    let props = vec![
      make_shorthand_prop("a"),
      make_shorthand_prop("b"),
      make_shorthand_prop("a"),
    ];
    let result = remove_duplicates(props);
    assert_eq!(result.len(), 2);
  }

  #[test]
  fn deduplicates_str_key_props() {
    let props = vec![
      make_kv_str_key_prop("color", 1.0),
      make_kv_str_key_prop("color", 2.0),
    ];
    let result = remove_duplicates(props);
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn skips_spread_elements() {
    let props = vec![make_kv_prop("a", 1.0), make_spread_prop()];
    let result = remove_duplicates(props);
    // Spread is skipped (continue), only "a" remains
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn mixed_shorthand_and_kv_props() {
    let props = vec![make_shorthand_prop("x"), make_kv_prop("x", 5.0)];
    let result = remove_duplicates(props);
    // "x" appears twice but last wins
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn skips_non_kv_non_shorthand_props() {
    use swc_core::ecma::ast::{GetterProp, PropName};
    let getter_prop = PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
      span: DUMMY_SP,
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "val".into(),
      }),
      type_ann: None,
      body: None,
    })));
    let props = vec![make_kv_prop("a", 1.0), getter_prop];
    let result = remove_duplicates(props);
    // Getter is skipped (continue), only "a" remains
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn keeps_numeric_key_props() {
    let num_key_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Num(Number {
        span: DUMMY_SP,
        value: 42.0,
        raw: None,
      }),
      value: Box::new(create_number_expr(1.0)),
    })));
    let props = vec![num_key_prop];
    let result = remove_duplicates(props);
    // A numeric key names a property like any other, so it is kept. It used to
    // be dropped: the key reader had no arm for it, and a declaration written
    // `{ 42: 1 }` vanished from the object it was written in.
    assert_eq!(result.len(), 1);
  }
}

// ──────────────────────────────────────────────
// assign_props
// ──────────────────────────────────────────────

mod assign_props_tests {
  use super::*;
  use swc_core::ecma::ast::{
    IdentName, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, SpreadElement,
  };

  fn make_kv_prop(key: &str, val: f64) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: key.into(),
      }),
      value: Box::new(create_number_expr(val)),
    })))
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

  fn make_spread() -> PropOrSpread {
    PropOrSpread::Spread(SpreadElement {
      dot3_token: DUMMY_SP,
      expr: Box::new(create_number_expr(0.0)),
    })
  }

  #[test]
  fn merges_non_overlapping_props() {
    let old = vec![make_kv_prop("a", 1.0)];
    let new = vec![make_kv_prop("b", 2.0)];
    let result = assign_props(old, new);
    assert_eq!(result.len(), 2);
  }

  #[test]
  fn overlapping_object_key_merges_nested() {
    let inner_old = vec![make_kv_prop("x", 1.0)];
    let inner_new = vec![make_kv_prop("y", 2.0)];
    let old = vec![make_kv_obj_prop("shared", inner_old)];
    let new = vec![make_kv_obj_prop("shared", inner_new)];
    let result = assign_props(old, new);
    // After dedup, "shared" appears once but both old and new versions are merged
    assert!(!result.is_empty());
  }

  #[test]
  fn non_kv_old_props_appended() {
    use swc_core::ecma::ast::GetterProp;
    let getter = PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
      span: DUMMY_SP,
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "val".into(),
      }),
      type_ann: None,
      body: None,
    })));
    let old = vec![getter];
    let new = vec![make_kv_prop("a", 1.0)];
    let result = assign_props(old, new);
    assert!(!result.is_empty());
  }

  #[test]
  fn spread_old_props_appended() {
    let old = vec![make_spread()];
    let new = vec![make_kv_prop("a", 1.0)];
    let result = assign_props(old, new);
    assert!(!result.is_empty());
  }

  #[test]
  fn empty_old_returns_new() {
    let new = vec![make_kv_prop("a", 1.0)];
    let result = assign_props(vec![], new);
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn overlapping_non_object_value_appended() {
    // When old and new share a key but old value is not an object
    let old = vec![make_kv_prop("x", 1.0)];
    let new = vec![make_kv_prop("x", 2.0)];
    let result = assign_props(old, new);
    // Last wins via remove_duplicates
    assert_eq!(result.len(), 1);
  }
}

// ──────────────────────────────────────────────
// get_import_by_ident
// ──────────────────────────────────────────────

mod get_import_by_ident_tests {
  use super::*;
  use swc_core::ecma::ast::{
    Ident, ImportDecl, ImportDefaultSpecifier, ImportNamedSpecifier, ImportSpecifier,
    ImportStarAsSpecifier, ModuleExportName,
  };

  /// An identifier at a given syntax context. Zero is the context every ident
  /// the parser produces carries before the resolver runs, so it doubles as
  /// "context does not matter to this test"; anything else is a distinct scope.
  ///
  /// `SyntaxContext::from_u32` rather than `apply_mark`, which would need
  /// `GLOBALS` installed for what is only "some other context than that one".
  fn ident_at(name: &str, ctxt: u32) -> Ident {
    Ident {
      span: DUMMY_SP,
      ctxt: SyntaxContext::from_u32(ctxt),
      sym: name.into(),
      optional: false,
    }
  }

  /// One import declaration over the specifiers handed in. Every case here is
  /// some choice of source and specifier list, so they share one builder --
  /// otherwise a case that needs a shape no helper covers grows a fifteen-line
  /// literal of its own and the shapes stop being comparable at a glance.
  fn import_from(source: &str, specifiers: Vec<ImportSpecifier>) -> ImportDecl {
    ImportDecl {
      span: DUMMY_SP,
      specifiers,
      src: Box::new(Str {
        span: DUMMY_SP,
        value: source.into(),
        raw: None,
      }),
      type_only: false,
      with: None,
      phase: Default::default(),
    }
  }

  /// `import { local }` -- the specifier whose comparison #1266 was about.
  fn named(local: &str, ctxt: u32) -> ImportSpecifier {
    ImportSpecifier::Named(ImportNamedSpecifier {
      span: DUMMY_SP,
      local: ident_at(local, ctxt),
      imported: None,
      is_type_only: false,
    })
  }

  /// `import { imported as local }`.
  fn aliased(local: &str, imported: &str, ctxt: u32) -> ImportSpecifier {
    ImportSpecifier::Named(ImportNamedSpecifier {
      span: DUMMY_SP,
      local: ident_at(local, ctxt),
      imported: Some(ModuleExportName::Ident(ident_at(imported, ctxt))),
      is_type_only: false,
    })
  }

  /// `import { "imported" as local }`, whose imported name need not be a legal
  /// identifier.
  fn str_named(local: &str, imported: &str, ctxt: u32) -> ImportSpecifier {
    ImportSpecifier::Named(ImportNamedSpecifier {
      span: DUMMY_SP,
      local: ident_at(local, ctxt),
      imported: Some(ModuleExportName::Str(Str {
        span: DUMMY_SP,
        value: imported.into(),
        raw: None,
      })),
      is_type_only: false,
    })
  }

  /// `import local from` and `import * as local from`.
  fn default_of(local: &str, ctxt: u32) -> ImportSpecifier {
    ImportSpecifier::Default(ImportDefaultSpecifier {
      span: DUMMY_SP,
      local: ident_at(local, ctxt),
    })
  }

  fn namespace_of(local: &str, ctxt: u32) -> ImportSpecifier {
    ImportSpecifier::Namespace(ImportStarAsSpecifier {
      span: DUMMY_SP,
      local: ident_at(local, ctxt),
    })
  }

  /// The source a lookup answered with, as authored text. Reads the atom
  /// directly rather than comparing `Debug` renderings, which would pass for the
  /// wrong reason the moment the atom's formatting changes.
  fn source_of<'a>(found: Option<(&'a ImportDecl, &ImportSpecifier)>) -> Option<&'a str> {
    found.and_then(|(import, _)| import.src.value.as_str())
  }

  #[test]
  fn finds_named_import_by_local() {
    let mut state = StateManager::default();
    state.push_top_import(import_from("@stylexjs/stylex", vec![named("stylex", 0)]));
    let ident = create_ident("stylex");
    let result = get_import_by_ident(&ident, &state);
    assert!(result.is_some());
  }

  #[test]
  fn returns_none_when_not_found() {
    let state = StateManager::default();
    let ident = create_ident("nonexistent");
    let result = get_import_by_ident(&ident, &state);
    assert!(result.is_none());
  }

  #[test]
  fn finds_default_import() {
    let mut state = StateManager::default();
    state.push_top_import(import_from(
      "@stylexjs/stylex",
      vec![default_of("stylex", 0)],
    ));
    let ident = create_ident("stylex");
    let result = get_import_by_ident(&ident, &state);
    assert!(result.is_some());
  }

  #[test]
  fn finds_namespace_import() {
    let mut state = StateManager::default();
    state.push_top_import(import_from("module", vec![namespace_of("ns", 0)]));
    let ident = create_ident("ns");
    let result = get_import_by_ident(&ident, &state);
    assert!(result.is_some());
  }

  #[test]
  fn does_not_match_a_renamed_import_by_the_name_it_was_aliased_away_from() {
    let mut state = StateManager::default();
    state.push_top_import(import_from(
      "@stylexjs/stylex",
      vec![aliased("localName", "create", 0)],
    ));

    // `import { create as localName }` leaves `create` unbound in this module,
    // so a reference spelled that way names something else or nothing at all.
    // Answering the import for it resolved a binding no scope holds.
    let ident = create_ident("create");
    let result = get_import_by_ident(&ident, &state);
    assert!(result.is_none());
  }

  #[test]
  fn finds_renamed_import_by_local_name() {
    let mut state = StateManager::default();
    state.push_top_import(import_from(
      "@stylexjs/stylex",
      vec![aliased("localName", "create", 0)],
    ));
    let ident = create_ident("localName");
    let result = get_import_by_ident(&ident, &state);
    assert!(result.is_some());
  }

  #[test]
  fn does_not_match_wrong_ident() {
    let mut state = StateManager::default();
    state.push_top_import(import_from("@stylexjs/stylex", vec![named("stylex", 0)]));
    let ident = create_ident("wrongName");
    let result = get_import_by_ident(&ident, &state);
    assert!(result.is_none());
  }

  #[test]
  fn does_not_match_a_string_named_specifier_by_its_imported_name() {
    let mut state = StateManager::default();
    state.push_top_import(import_from(
      "module",
      vec![str_named("localName", "strExport", 0)],
    ));

    // The same answer for the string-named spelling, and the one that was
    // reachable in practice: an imported name that *is* a legal identifier
    // matched a reference to it by symbol alone, across every scope.
    let ident = create_ident("strExport");
    assert!(get_import_by_ident(&ident, &state).is_none());

    // The local binding is what the declaration introduces, and it still
    // answers -- a lookup that said `None` to everything would pass the
    // assertion above on its own.
    assert!(get_import_by_ident(&create_ident("localName"), &state).is_some());
  }

  // ──────────────────────────────────────────────
  // Shadowing: the lookup answers about a binding, not a name (#1266)
  //
  // Every case here fixes two references with the same symbol at *different*
  // syntax contexts, which is what a shadowing binding looks like once the
  // resolver has run. Written against the lookup rather than the transform
  // because a context is the whole question and the transform has to build a
  // whole module to ask it.
  // ──────────────────────────────────────────────

  #[test]
  fn does_not_match_a_named_import_shadowed_by_another_binding() {
    let mut state = StateManager::default();
    state.push_top_import(import_from("zIndex.stylex.js", vec![named("zIndex", 1)]));

    // The arrow parameter in `(zIndex) => ({ zIndex })`: same symbol, its own
    // context. Resolving it to the import is #1266.
    let shadowing_param = ident_at("zIndex", 2);

    assert!(get_import_by_ident(&shadowing_param, &state).is_none());
  }

  #[test]
  fn finds_a_named_import_from_its_own_context() {
    let mut state = StateManager::default();
    state.push_top_import(import_from("zIndex.stylex.js", vec![named("zIndex", 1)]));

    // The other half of the same question: a genuine reference to the import
    // still resolves. A fix that answered `None` for everything would pass the
    // test above on its own.
    assert!(get_import_by_ident(&ident_at("zIndex", 1), &state).is_some());
  }

  #[test]
  fn does_not_match_an_aliased_import_shadowed_by_another_binding() {
    let mut state = StateManager::default();
    state.push_top_import(import_from(
      "zIndex.stylex.js",
      vec![aliased("zi", "zIndex", 1)],
    ));

    // `import { zIndex as zi }` shadowed by a parameter `zi`, which failed
    // identically to the unaliased shape.
    assert!(get_import_by_ident(&ident_at("zi", 2), &state).is_none());
  }

  #[test]
  fn matches_only_the_specifier_whose_context_agrees() {
    let mut state = StateManager::default();
    state.push_top_import(import_from("first.stylex.js", vec![named("shadowed", 1)]));
    state.push_top_import(import_from("second.stylex.js", vec![named("shadowed", 2)]));

    // Two declarations cannot bind one name in valid source, but the lookup
    // scans a flat list and must not answer by position.
    let reference = ident_at("shadowed", 2);

    assert_eq!(
      source_of(get_import_by_ident(&reference, &state)),
      Some("second.stylex.js")
    );
  }

  #[test]
  fn matches_one_specifier_of_a_declaration_without_matching_its_siblings() {
    let mut state = StateManager::default();
    state.push_top_import(import_from(
      "tokens.stylex.js",
      vec![named("spacing", 1), named("zIndex", 1)],
    ));

    // The answer is per declaration, but the match is per specifier: a
    // declaration that binds a live `spacing` does not thereby answer for a
    // shadowed `zIndex`. Both are the same declaration, so a lookup that
    // stopped at "does this declaration mention the name" would say yes to
    // both.
    assert!(get_import_by_ident(&ident_at("spacing", 1), &state).is_some());
    assert!(get_import_by_ident(&ident_at("zIndex", 1), &state).is_some());
    assert!(get_import_by_ident(&ident_at("zIndex", 2), &state).is_none());
    assert!(get_import_by_ident(&ident_at("nothing", 1), &state).is_none());
  }

  #[test]
  fn a_shadowed_default_or_namespace_import_was_already_context_aware() {
    let mut state = StateManager::default();
    state.push_top_import(import_from("theme.stylex.js", vec![default_of("theme", 1)]));
    state.push_top_import(import_from(
      "tokens.stylex.js",
      vec![namespace_of("tokens", 1)],
    ));

    // The two arms the named one now matches. Pinned so a later edit cannot
    // regress all three to a name match at once.
    assert!(get_import_by_ident(&ident_at("theme", 2), &state).is_none());
    assert!(get_import_by_ident(&ident_at("tokens", 2), &state).is_none());
    assert!(get_import_by_ident(&ident_at("theme", 1), &state).is_some());
    assert!(get_import_by_ident(&ident_at("tokens", 1), &state).is_some());
  }

  #[test]
  fn a_string_named_specifier_answers_only_for_its_local_binding() {
    let mut state = StateManager::default();
    state.push_top_import(import_from(
      "tokens.stylex.js",
      vec![str_named("spacing", "spacing-lg", 1)],
    ));

    // The imported name is compared against nothing now, at any context.
    // `spacing-lg` is not a legal identifier, so the only way a reference could
    // ever have carried that symbol was through the specifier itself.
    assert!(get_import_by_ident(&ident_at("spacing-lg", 9), &state).is_none());
    assert!(get_import_by_ident(&ident_at("spacing-lg", 1), &state).is_none());

    // Its local binding still resolves, and a parameter shadowing that local
    // binding still does not.
    assert!(get_import_by_ident(&ident_at("spacing", 1), &state).is_some());
    assert!(get_import_by_ident(&ident_at("spacing", 2), &state).is_none());
  }

  #[test]
  fn a_non_ascii_local_name_matches_by_binding_too() {
    let mut state = StateManager::default();
    // `zÍndex` is a different identifier that merely looks like `zIndex`, and
    // the lookup compares interned atoms, so it must not match one for the
    // other. A unicode-escaped spelling needs no case of its own here: the
    // lexer folds `\u007AIndex` to the atom `zIndex` long before this, so at
    // this seam the two spellings are one value. The escape is exercised where
    // it can still differ -- as authored source, in the parity corpus.
    state.push_top_import(import_from("zIndex.stylex.js", vec![named("zIndex", 1)]));
    state.push_top_import(import_from("accented.stylex.js", vec![named("zÍndex", 1)]));

    assert!(get_import_by_ident(&ident_at("zIndex", 1), &state).is_some());
    assert!(get_import_by_ident(&ident_at("zÍndex", 1), &state).is_some());
    assert!(get_import_by_ident(&ident_at("zÍndex", 2), &state).is_none());
  }

  #[test]
  fn answers_with_the_specifier_that_bound_the_name() {
    let mut state = StateManager::default();
    state.push_top_import(import_from(
      "tokens.stylex.js",
      vec![default_of("theme", 1), named("spacing", 1)],
    ));

    // One declaration can bind both kinds, and the chain refuses a default
    // where it resolves a named one. Which specifier answered is therefore part
    // of the answer, not something the caller can re-derive from the
    // declaration -- searching it again by name is what could come back empty.
    let (_, default_specifier) = match get_import_by_ident(&ident_at("theme", 1), &state) {
      Some(found) => found,
      None => panic!("the default specifier binds `theme`"),
    };
    assert!(matches!(default_specifier, ImportSpecifier::Default(_)));

    let (_, named_specifier) = match get_import_by_ident(&ident_at("spacing", 1), &state) {
      Some(found) => found,
      None => panic!("the named specifier binds `spacing`"),
    };
    assert!(matches!(named_specifier, ImportSpecifier::Named(_)));
  }

  #[test]
  fn an_empty_import_declaration_answers_for_nothing() {
    let mut state = StateManager::default();
    state.push_top_import(import_from("side-effect.css", vec![]));

    // `import './side-effect.css'` binds no name at all. `Iterator::any` over
    // no specifiers is `false`, which is the answer -- pinned because a
    // refactor to `all` would make it `true` and resolve every reference to it.
    assert!(get_import_by_ident(&ident_at("anything", 0), &state).is_none());
    assert!(get_import_by_ident(&create_ident("anything"), &state).is_none());
  }
}

// ──────────────────────────────────────────────
// serialize_value_to_json_string - JS object path
// ──────────────────────────────────────────────

mod serialize_value_to_json_string_extra_tests {
  use super::*;

  #[test]
  fn serializes_js_object_like_string() {
    // A string that starts with '{' and does NOT contain `":`
    // triggers js_object_to_json
    let result = serialize_value_to_json_string("{color: red, size: big}");
    assert!(result.contains('"'));
  }

  #[test]
  fn serializes_json_like_string_passthrough() {
    // A string that starts with '{' and contains `":` is NOT treated
    // as a JS object; it falls through to the plain remove_quotes path
    let result = serialize_value_to_json_string(r#"{"key":"value"}"#);
    assert!(result.contains("key"));
  }
}

// ──────────────────────────────────────────────
// js_object_to_json
// ──────────────────────────────────────────────

mod js_object_to_json_tests {
  use super::*;

  #[test]
  fn converts_js_object_keys_to_quoted_json() {
    let input = "{color: red}";
    let result = js_object_to_json(input);
    assert!(result.contains('"'));
  }

  #[test]
  fn handles_empty_object() {
    let input = "{}";
    let result = js_object_to_json(input);
    assert_eq!(result, "{}");
  }
}

// ──────────────────────────────────────────────
// type_of
// ──────────────────────────────────────────────

mod type_of_tests {
  use super::*;

  #[test]
  fn returns_type_name_for_i32() {
    let result = type_of(42_i32);
    assert_eq!(result, "i32");
  }

  #[test]
  fn returns_type_name_for_string() {
    let result = type_of(String::from("hello"));
    assert!(result.contains("String"));
  }

  #[test]
  fn returns_type_name_for_bool() {
    let result = type_of(true);
    assert_eq!(result, "bool");
  }
}

// ──────────────────────────────────────────────
// get_var_decl_by_ident - Increase and None actions
// ──────────────────────────────────────────────

mod get_var_decl_by_ident_tests {
  use super::*;

  #[test]
  fn returns_var_decl_for_known_ident() {
    let mut state = StateManager::default();
    let fns = FunctionMap::default();
    let decl = make_var_declarator("x", create_number_expr(10.0));
    fill_state_declarations(&mut state, &decl);
    let ident = create_ident("x");
    let result = get_var_decl_by_ident(&ident, &mut state, &fns);
    assert!(result.is_some());
  }

  #[test]
  fn returns_none_for_unknown_ident() {
    let mut state = StateManager::default();
    let fns = FunctionMap::default();
    let ident = create_ident("nonexistent");
    let result = get_var_decl_by_ident(&ident, &mut state, &fns);
    assert!(result.is_none());
  }
}

// ──────────────────────────────────────────────
// get_key_values_from_object
// ──────────────────────────────────────────────

mod get_key_values_from_object_tests {
  use super::*;
  use swc_core::ecma::ast::{IdentName, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread};

  #[test]
  fn returns_empty_for_empty_object() {
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![],
    };
    let result = get_key_values_from_object(&obj);
    assert!(result.is_empty());
  }

  #[test]
  fn extracts_key_values() {
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(IdentName {
            span: DUMMY_SP,
            sym: "color".into(),
          }),
          value: Box::new(create_string_expr("red")),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(IdentName {
            span: DUMMY_SP,
            sym: "size".into(),
          }),
          value: Box::new(create_number_expr(12.0)),
        }))),
      ],
    };
    let result = get_key_values_from_object(&obj);
    assert_eq!(result.len(), 2);
  }

  #[test]
  fn expands_shorthand_props() {
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![PropOrSpread::Prop(Box::new(Prop::Shorthand(create_ident(
        "color",
      ))))],
    };
    let result = get_key_values_from_object(&obj);
    assert_eq!(result.len(), 1);
  }

  #[test]
  #[should_panic]
  fn panics_on_getter_prop() {
    use swc_core::ecma::ast::GetterProp;
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
        span: DUMMY_SP,
        key: PropName::Ident(IdentName {
          span: DUMMY_SP,
          sym: "val".into(),
        }),
        type_ann: None,
        body: None,
      })))],
    };
    get_key_values_from_object(&obj);
  }
}

// ──────────────────────────────────────────────
// get_css_value
// ──────────────────────────────────────────────

mod get_css_value_tests {
  use super::*;
  use swc_core::ecma::ast::{IdentName, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread};

  #[test]
  fn returns_value_directly_when_not_object() {
    let kv = KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "color".into(),
      }),
      value: Box::new(create_string_expr("red")),
    };
    let (expr, css_type) = get_css_value(kv);
    assert!(css_type.is_none());
    assert!(expr.is_lit());
  }

  #[test]
  fn returns_value_from_syntax_object() {
    let syntax_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "syntax".into(),
      }),
      value: Box::new(create_string_expr("<length>")),
    })));
    let value_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "value".into(),
      }),
      value: Box::new(create_number_expr(10.0)),
    })));
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![syntax_prop, value_prop],
    };
    let kv = KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "width".into(),
      }),
      value: Box::new(Expr::Object(obj)),
    };
    let (expr, css_type) = get_css_value(kv);
    assert!(css_type.is_some());
    assert!(expr.is_lit());
  }

  #[test]
  fn returns_object_when_no_syntax_key() {
    let some_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "notSyntax".into(),
      }),
      value: Box::new(create_string_expr("val")),
    })));
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![some_prop],
    };
    let kv = KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "width".into(),
      }),
      value: Box::new(Expr::Object(obj)),
    };
    let (expr, css_type) = get_css_value(kv);
    assert!(css_type.is_none());
    assert!(expr.is_object());
  }

  #[test]
  fn returns_empty_object_unchanged() {
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![],
    };
    let kv = KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "width".into(),
      }),
      value: Box::new(Expr::Object(obj)),
    };
    let (expr, css_type) = get_css_value(kv);
    assert!(css_type.is_none());
    assert!(expr.is_object());
  }
}

// ──────────────────────────────────────────────
// assign_props - Str/Num key branches
// ──────────────────────────────────────────────

mod assign_props_str_num_key_tests {
  use super::*;
  use swc_core::ecma::ast::{IdentName, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread};

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

  fn make_kv_prop_ident(key: &str, val: f64) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: key.into(),
      }),
      value: Box::new(create_number_expr(val)),
    })))
  }

  #[test]
  fn overlapping_str_keys_merge_nested_objects() {
    let inner_old = vec![make_kv_prop_ident("x", 1.0)];
    let inner_new = vec![make_kv_prop_ident("y", 2.0)];
    let old = vec![make_kv_str_key_obj_prop("shared", inner_old)];
    let new = vec![make_kv_str_key_obj_prop("shared", inner_new)];
    let result = assign_props(old, new);
    assert!(!result.is_empty());
  }

  #[test]
  fn overlapping_num_keys_merge_nested_objects() {
    let inner_old = vec![make_kv_prop_ident("x", 1.0)];
    let inner_new = vec![make_kv_prop_ident("y", 2.0)];
    let old = vec![make_kv_num_key_obj_prop(42.0, inner_old)];
    let new = vec![make_kv_num_key_obj_prop(42.0, inner_new)];
    let result = assign_props(old, new);
    // A numeric key is a key like any other: the two collide and the later one
    // wins. This used to answer an empty object -- the key was matched by the
    // merge and then dropped by the deduplication, which read no name for it.
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn a_string_key_and_the_numeric_spelling_of_it_are_one_key() {
    let inner_old = vec![make_kv_prop_ident("x", 1.0)];
    let inner_new = vec![make_kv_prop_ident("y", 2.0)];
    let old = vec![make_kv_str_key_obj_prop("42", inner_old)];
    let new = vec![make_kv_num_key_obj_prop(42.0, inner_new)];
    let result = assign_props(old, new);
    // `{ '42': x }` and `{ 42: y }` name one property in the language, so they
    // collide and the later one wins.
    assert_eq!(result.len(), 1);
  }
}

// ──────────────────────────────────────────────
// fill_top_level_expressions - additional branches
// ──────────────────────────────────────────────

mod fill_top_level_expressions_extra_tests {
  use super::*;
  use swc_core::ecma::ast::{ExportAll, ImportDecl};

  #[test]
  fn ignores_import_decl_items() {
    let mut state = StateManager::default();
    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![],
        src: Box::new(Str {
          span: DUMMY_SP,
          value: "module".into(),
          raw: None,
        }),
        type_only: false,
        with: None,
        phase: Default::default(),
      }))],
      shebang: None,
    };
    fill_top_level_expressions(&module, &mut state);
    assert!(state.top_level_expressions.is_empty());
  }

  #[test]
  fn ignores_export_all_items() {
    let mut state = StateManager::default();
    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::ModuleDecl(ModuleDecl::ExportAll(ExportAll {
        span: DUMMY_SP,
        src: Box::new(Str {
          span: DUMMY_SP,
          value: "module".into(),
          raw: None,
        }),
        type_only: false,
        with: None,
      }))],
      shebang: None,
    };
    fill_top_level_expressions(&module, &mut state);
    assert!(state.top_level_expressions.is_empty());
  }

  #[test]
  fn ignores_expression_stmts() {
    use swc_core::ecma::ast::ExprStmt;
    let mut state = StateManager::default();
    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::Stmt(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(create_number_expr(42.0)),
      }))],
      shebang: None,
    };
    fill_top_level_expressions(&module, &mut state);
    assert!(state.top_level_expressions.is_empty());
  }

  #[test]
  fn skips_exported_var_decl_without_init() {
    let mut state = StateManager::default();
    let decl = make_var_declarator_no_init("noInit");

    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
        span: DUMMY_SP,
        decl: Decl::Var(Box::new(VarDecl {
          span: DUMMY_SP,
          kind: VarDeclKind::Const,
          declare: false,
          decls: vec![decl],
          ctxt: SyntaxContext::empty(),
        })),
      }))],
      shebang: None,
    };

    fill_top_level_expressions(&module, &mut state);
    assert!(state.top_level_expressions.is_empty());
  }

  #[test]
  fn ignores_non_var_export_decls() {
    use swc_core::ecma::ast::{FnDecl, Function};
    let mut state = StateManager::default();
    let fn_decl = Decl::Fn(FnDecl {
      ident: create_ident("myFn"),
      declare: false,
      function: Box::new(Function {
        params: vec![],
        decorators: vec![],
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        body: None,
        is_generator: false,
        is_async: false,
        type_params: None,
        return_type: None,
      }),
    });
    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
        span: DUMMY_SP,
        decl: fn_decl,
      }))],
      shebang: None,
    };
    fill_top_level_expressions(&module, &mut state);
    assert!(state.top_level_expressions.is_empty());
  }
}

// ──────────────────────────────────────────────
// downcast_style_options_to_state_manager
// ──────────────────────────────────────────────

mod downcast_style_options_tests {
  use super::*;

  #[test]
  fn downcasts_state_manager_successfully() {
    let mut state = StateManager::default();
    let sm = downcast_style_options_to_state_manager(&mut state);
    // Verify we get a valid StateManager back
    assert!(sm.declarations.is_empty());
  }
}

// ──────────────────────────────────────────────
// get_var_decl_by_ident - FunctionMap branches
// ──────────────────────────────────────────────

mod get_var_decl_by_ident_function_map_tests {
  use super::*;
  use crate::shared::structures::functions::{
    FunctionConfig, FunctionConfigType, FunctionMap, FunctionType,
  };
  use std::rc::Rc;

  #[test]
  fn returns_var_decl_from_mapper_function() {
    let mut state = StateManager::default();
    let mut fns = FunctionMap::default();
    let mapper: Rc<dyn Fn() -> Expr + 'static> = Rc::new(|| create_number_expr(99.0));
    fns.identifiers.insert(
      "myMapper".into(),
      Box::new(FunctionConfigType::Regular(FunctionConfig {
        fn_ptr: FunctionType::Mapper(mapper),
        takes_path: false,
      })),
    );
    let ident = create_ident("myMapper");
    let result = get_var_decl_by_ident(&ident, &mut state, &fns);
    assert!(result.is_some());
  }

  #[test]
  fn returns_none_for_env_object() {
    use indexmap::IndexMap;
    let mut state = StateManager::default();
    let mut fns = FunctionMap::default();
    fns.identifiers.insert(
      "envObj".into(),
      Box::new(FunctionConfigType::EnvObject(IndexMap::new().into())),
    );
    let ident = create_ident("envObj");
    let result = get_var_decl_by_ident(&ident, &mut state, &fns);
    assert!(result.is_none());
  }
}

// ──────────────────────────────────────────────
// assign_props - additional edge cases
// ──────────────────────────────────────────────

mod assign_props_extra_edge_tests {
  use super::*;
  use swc_core::ecma::ast::{
    GetterProp, IdentName, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, SpreadElement,
  };

  fn make_kv_prop(key: &str, val: f64) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: key.into(),
      }),
      value: Box::new(create_number_expr(val)),
    })))
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

  #[test]
  fn new_props_non_kv_triggers_false_branch() {
    // Old has a KV obj prop, new has a getter with matching key
    // This triggers the `_ => false` at line 320 in assign_props
    let getter = PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
      span: DUMMY_SP,
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "shared".into(),
      }),
      type_ann: None,
      body: None,
    })));
    let inner_old = vec![make_kv_prop("x", 1.0)];
    let old = vec![make_kv_obj_prop("shared", inner_old)];
    let new = vec![getter];
    let result = assign_props(old, new);
    // Old KV appended since no match found
    assert!(!result.is_empty());
  }

  #[test]
  fn new_props_spread_triggers_false_branch() {
    // Old has a KV obj prop, new has a spread
    // This triggers the `_ => false` at line 322 in assign_props
    let spread = PropOrSpread::Spread(SpreadElement {
      dot3_token: DUMMY_SP,
      expr: Box::new(create_number_expr(0.0)),
    });
    let inner_old = vec![make_kv_prop("x", 1.0)];
    let old = vec![make_kv_obj_prop("shared", inner_old)];
    let new = vec![spread];
    let result = assign_props(old, new);
    assert!(!result.is_empty());
  }
}

// ──────────────────────────────────────────────
// get_key_values_from_object - spread should_panic
// ──────────────────────────────────────────────

mod get_key_values_from_object_spread_tests {
  use super::*;
  use swc_core::ecma::ast::{ObjectLit, PropOrSpread, SpreadElement};

  #[test]
  #[should_panic]
  fn panics_on_spread_element() {
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![PropOrSpread::Spread(SpreadElement {
        dot3_token: DUMMY_SP,
        expr: Box::new(create_number_expr(1.0)),
      })],
    };
    get_key_values_from_object(&obj);
  }
}

// ──────────────────────────────────────────────
// get_var_decl_from
// ──────────────────────────────────────────────

mod get_var_decl_from_tests {
  use super::*;
  use crate::shared::utils::common::get_var_decl_from;

  #[test]
  fn finds_matching_declaration() {
    let mut state = StateManager::default();
    let decl = make_var_declarator("x", create_number_expr(1.0));
    fill_state_declarations(&mut state, &decl);
    let ident = create_ident("x");
    assert!(get_var_decl_from(&state, &ident).is_some());
  }

  #[test]
  fn returns_none_for_no_match() {
    let state = StateManager::default();
    let ident = create_ident("nonexistent");
    assert!(get_var_decl_from(&state, &ident).is_none());
  }
}

// ──────────────────────────────────────────────
// get_expr_from_var_decl - should_panic
// ──────────────────────────────────────────────

mod get_expr_from_var_decl_panic_tests {
  use super::*;

  #[test]
  #[should_panic]
  fn panics_when_init_is_none() {
    let decl = make_var_declarator_no_init("x");
    get_expr_from_var_decl(&decl);
  }
}

// ──────────────────────────────────────────────
// get_var_decl_by_ident - FunctionMap panic branches
// ──────────────────────────────────────────────

mod get_var_decl_by_ident_fn_map_panic_tests {
  use super::*;
  use crate::shared::structures::functions::{
    FunctionConfig, FunctionConfigType, FunctionMap, FunctionType,
  };

  #[test]
  #[should_panic]
  fn panics_for_non_mapper_regular_function() {
    let mut state = StateManager::default();
    let mut fns = FunctionMap::default();
    fn dummy_fn(
      _args: Vec<Expr>,
      _state: &mut dyn stylex_types::traits::StyleOptions,
      _fns: &crate::shared::structures::functions::FunctionMap,
    ) -> Expr {
      create_number_expr(0.0)
    }
    fns.identifiers.insert(
      "arrFn".into(),
      Box::new(FunctionConfigType::Regular(FunctionConfig {
        fn_ptr: FunctionType::ArrayArgs(dummy_fn),
        takes_path: false,
      })),
    );
    let ident = create_ident("arrFn");
    get_var_decl_by_ident(&ident, &mut state, &fns);
  }

  #[test]
  #[should_panic]
  fn panics_for_map_function_config() {
    let mut state = StateManager::default();
    let mut fns = FunctionMap::default();
    fns.identifiers.insert(
      "mapFn".into(),
      Box::new(FunctionConfigType::Map(Default::default())),
    );
    let ident = create_ident("mapFn");
    get_var_decl_by_ident(&ident, &mut state, &fns);
  }

  #[test]
  #[should_panic]
  fn panics_for_indexmap_function_config() {
    let mut state = StateManager::default();
    let mut fns = FunctionMap::default();
    fns.identifiers.insert(
      "imapFn".into(),
      Box::new(FunctionConfigType::IndexMap(Default::default())),
    );
    let ident = create_ident("imapFn");
    get_var_decl_by_ident(&ident, &mut state, &fns);
  }
}

// ──────────────────────────────────────────────
// fill_top_level_expressions - non-ident var patterns
// ──────────────────────────────────────────────

mod fill_top_level_non_ident_pattern_tests {
  use super::*;
  use swc_core::ecma::ast::{ArrayPat, CallExpr, Callee, ObjectPat};

  /// `export const [ a ] = expr;` exports no single name to record, so it is
  /// skipped rather than rejected — it is ordinary JavaScript, and the APIs
  /// that do require a name report that against the call themselves.
  #[test]
  fn export_decl_with_array_pattern_skipped() {
    let mut state = StateManager::default();
    let decl = VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Array(ArrayPat {
        span: DUMMY_SP,
        elems: vec![],
        optional: false,
        type_ann: None,
      }),
      init: Some(Box::new(create_number_expr(1.0))),
      definite: false,
    };
    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
        span: DUMMY_SP,
        decl: Decl::Var(Box::new(VarDecl {
          span: DUMMY_SP,
          kind: VarDeclKind::Const,
          declare: false,
          decls: vec![decl],
          ctxt: SyntaxContext::empty(),
        })),
      }))],
      shebang: None,
    };
    fill_top_level_expressions(&module, &mut state);

    assert!(state.top_level_expressions.is_empty());
    assert!(state.declarations.is_empty());
  }

  /// A statement declarator bound to a pattern declares no single name, so it
  /// contributes no top-level expression either.
  #[test]
  fn stmt_var_with_object_pattern_skipped() {
    let mut state = StateManager::default();
    let decl = VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Object(ObjectPat {
        span: DUMMY_SP,
        props: vec![],
        optional: false,
        type_ann: None,
      }),
      init: Some(Box::new(create_number_expr(1.0))),
      definite: false,
    };
    let module = Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![decl],
        ctxt: SyntaxContext::empty(),
      }))))],
      shebang: None,
    };
    fill_top_level_expressions(&module, &mut state);
    // Object pattern is skipped, no expressions added
    assert!(state.top_level_expressions.is_empty());
  }

  /// A pattern-bound declarator initialised by a call keeps the call's
  /// position, which is what marks it program level.
  #[test]
  fn export_decl_with_pattern_records_the_initializing_call_position() {
    let call_span = Span {
      lo: BytePos(11),
      hi: BytePos(30),
    };

    let mut state = StateManager::default();
    fill_top_level_expressions(
      &pattern_bound_export(Expr::Call(make_call_expr(call_span))),
      &mut state,
    );

    assert!(state.top_level_expressions.is_empty());
    assert_eq!(
      state
        .pattern_bound_top_level_calls
        .iter()
        .copied()
        .collect::<Vec<_>>(),
      vec![call_span]
    );
  }

  /// A span-less call identifies no position, so there is nothing to record —
  /// see `StateManager::find_top_level_expr_by_span` for why a dummy span is
  /// never a match.
  #[test]
  fn export_decl_with_pattern_ignores_a_span_less_call() {
    let mut state = StateManager::default();
    fill_top_level_expressions(
      &pattern_bound_export(Expr::Call(make_call_expr(DUMMY_SP))),
      &mut state,
    );

    assert!(state.pattern_bound_top_level_calls.is_empty());
  }

  /// A pattern-bound declarator initialised by anything else records nothing.
  #[test]
  fn export_decl_with_pattern_ignores_a_non_call_initializer() {
    let mut state = StateManager::default();
    fill_top_level_expressions(&pattern_bound_export(create_number_expr(1.0)), &mut state);

    assert!(state.pattern_bound_top_level_calls.is_empty());
  }

  fn make_call_expr(span: Span) -> CallExpr {
    CallExpr {
      span,
      ctxt: SyntaxContext::empty(),
      callee: Callee::Expr(Box::new(Expr::Ident(create_ident("defineMarker")))),
      args: vec![],
      type_args: None,
    }
  }

  /// `export const { a } = <init>;`
  fn pattern_bound_export(init: Expr) -> Module {
    let decl = VarDeclarator {
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

    Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
        span: DUMMY_SP,
        decl: Decl::Var(Box::new(VarDecl {
          span: DUMMY_SP,
          kind: VarDeclKind::Const,
          declare: false,
          decls: vec![decl],
          ctxt: SyntaxContext::empty(),
        })),
      }))],
      shebang: None,
    }
  }
}

// ──────────────────────────────────────────────
// get_css_value - spread and non-KV panics
// ──────────────────────────────────────────────

mod get_css_value_panic_tests {
  use super::*;
  use swc_core::ecma::ast::{
    GetterProp, IdentName, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, SpreadElement,
  };

  #[test]
  #[should_panic]
  fn panics_on_spread_in_css_value_object() {
    let spread = PropOrSpread::Spread(SpreadElement {
      dot3_token: DUMMY_SP,
      expr: Box::new(create_number_expr(1.0)),
    });
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![spread],
    };
    let kv = KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "width".into(),
      }),
      value: Box::new(Expr::Object(obj)),
    };
    get_css_value(kv);
  }

  #[test]
  #[should_panic]
  fn panics_on_getter_in_css_value_object() {
    let getter = PropOrSpread::Prop(Box::new(Prop::Getter(GetterProp {
      span: DUMMY_SP,
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "val".into(),
      }),
      type_ann: None,
      body: None,
    })));
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![getter],
    };
    let kv = KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "width".into(),
      }),
      value: Box::new(Expr::Object(obj)),
    };
    get_css_value(kv);
  }

  #[test]
  #[should_panic]
  fn syntax_obj_with_num_key_prop_hits_false_branch() {
    // A syntax obj with a non-ident (Num) key causes the find closure
    // to fall through to the `false` return path. The conversion to
    // BaseCSSType then panics for the unsupported numeric key.
    let syntax_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "syntax".into(),
      }),
      value: Box::new(create_string_expr("<length>")),
    })));
    let num_key_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Num(Number {
        span: DUMMY_SP,
        value: 42.0,
        raw: None,
      }),
      value: Box::new(create_number_expr(10.0)),
    })));
    let value_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "value".into(),
      }),
      value: Box::new(create_number_expr(10.0)),
    })));
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![syntax_prop, num_key_prop, value_prop],
    };
    let kv = KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "width".into(),
      }),
      value: Box::new(Expr::Object(obj)),
    };
    get_css_value(kv);
  }

  #[test]
  #[should_panic]
  fn panics_on_spread_inside_syntax_obj_find() {
    let syntax_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "syntax".into(),
      }),
      value: Box::new(create_string_expr("<length>")),
    })));
    let spread = PropOrSpread::Spread(SpreadElement {
      dot3_token: DUMMY_SP,
      expr: Box::new(create_number_expr(1.0)),
    });
    let obj = ObjectLit {
      span: DUMMY_SP,
      props: vec![syntax_prop, spread],
    };
    let kv = KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "width".into(),
      }),
      value: Box::new(Expr::Object(obj)),
    };
    get_css_value(kv);
  }
}

// ──────────────────────────────────────────────
// assign_props - BigInt keys
// ──────────────────────────────────────────────

mod assign_props_bigint_key_tests {
  use super::*;
  use swc_core::ecma::ast::{
    BigInt, IdentName, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread,
  };

  fn make_kv_prop_ident(key: &str, val: f64) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: key.into(),
      }),
      value: Box::new(create_number_expr(val)),
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

  #[test]
  fn overlapping_bigint_keys_are_one_key() {
    let inner_old = vec![make_kv_prop_ident("x", 1.0)];
    let inner_new = vec![make_kv_prop_ident("y", 2.0)];
    let old = vec![make_bigint_obj_prop(42, inner_old)];
    let new = vec![make_bigint_obj_prop(42, inner_new)];
    let result = assign_props(old, new);
    // `{ 42n: x }` names the property `"42"`, so the two collide and the later
    // one wins.
    assert_eq!(result.len(), 1);
  }
}
