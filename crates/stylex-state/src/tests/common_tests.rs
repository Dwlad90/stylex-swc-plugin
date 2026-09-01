use swc_core::{
  common::{BytePos, DUMMY_SP, Span, SyntaxContext},
  ecma::ast::{
    BindingIdent, Decl, ExportDecl, Expr, Lit, Module, ModuleDecl, ModuleItem, Number, Pat, Stmt,
    Str, VarDecl, VarDeclKind, VarDeclarator,
  },
};

use crate::{
  common::{
    assign_props, downcast_style_options_to_state_manager, fill_state_declarations,
    fill_top_level_expressions, gen_file_based_identifier, get_css_value, js_object_to_json,
    remove_duplicates, serialize_value_to_json_string, type_of,
  },
  state_manager::StateManager,
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
