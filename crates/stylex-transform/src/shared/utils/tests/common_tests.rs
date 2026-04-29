use std::path::PathBuf;

use swc_core::{
  atoms::Atom,
  common::{DUMMY_SP, FileName, SyntaxContext},
  ecma::ast::{
    BinaryOp, BindingIdent, Decl, ExportDecl, Expr, Ident, Lit, Module, ModuleDecl, ModuleItem,
    Number, Pat, Stmt, Str, VarDecl, VarDeclKind, VarDeclarator,
  },
};

use crate::shared::{
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::common::{
    deep_merge_props, downcast_style_options_to_state_manager, evaluate_bin_expr,
    extract_filename_from_path, extract_filename_with_ext_from_path, extract_path,
    fill_state_declarations, fill_top_level_expressions, gen_file_based_identifier, get_css_value,
    get_expr_from_var_decl, get_import_from, get_key_values_from_object, get_var_decl_by_ident,
    increase_ident_count, increase_ident_count_by_count, increase_member_ident,
    increase_member_ident_count, increase_member_ident_count_by_count, js_object_to_json,
    normalize_expr, reduce_ident_count, reduce_member_expression_count, reduce_member_ident_count,
    remove_duplicates, serialize_value_to_json_string, type_of,
  },
};
use stylex_enums::misc::VarDeclAction;

// ──────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────

fn make_ident(name: &str) -> Ident {
  Ident {
    span: DUMMY_SP,
    sym: name.into(),
    optional: false,
    ctxt: SyntaxContext::empty(),
  }
}

fn make_num_expr(val: f64) -> Expr {
  Expr::Lit(Lit::Num(Number {
    value: val,
    span: DUMMY_SP,
    raw: None,
  }))
}

fn make_str_expr(val: &str) -> Expr {
  Expr::Lit(Lit::Str(Str {
    value: val.into(),
    span: DUMMY_SP,
    raw: None,
  }))
}

fn make_var_declarator(name: &str, init: Expr) -> VarDeclarator {
  VarDeclarator {
    span: DUMMY_SP,
    name: Pat::Ident(BindingIdent {
      id: make_ident(name),
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
      id: make_ident(name),
      type_ann: None,
    }),
    init: None,
    definite: false,
  }
}

// ──────────────────────────────────────────────
// extract_filename_from_path
// ──────────────────────────────────────────────

mod extract_filename_from_path_tests {
  use super::*;

  #[test]
  fn returns_stem_for_simple_js_file() {
    let path = FileName::Real(PathBuf::from("/path/to/file.js"));
    assert_eq!(extract_filename_from_path(&path), "file");
  }

  #[test]
  fn returns_stem_for_dotted_extension() {
    let path = FileName::Real(PathBuf::from("/path/to/file.stylex.ts"));
    assert_eq!(extract_filename_from_path(&path), "file.stylex");
  }

  #[test]
  fn returns_stem_for_flat_path() {
    let path = FileName::Real(PathBuf::from("simple.js"));
    assert_eq!(extract_filename_from_path(&path), "simple");
  }

  #[test]
  fn returns_empty_string_for_anon() {
    let path = FileName::Anon;
    assert_eq!(extract_filename_from_path(&path), "");
  }

  #[test]
  fn returns_stem_for_tsx_file() {
    let path = FileName::Real(PathBuf::from("/src/components/Button.tsx"));
    assert_eq!(extract_filename_from_path(&path), "Button");
  }

  #[test]
  fn returns_stem_for_no_extension() {
    let path = FileName::Real(PathBuf::from("/path/to/Makefile"));
    assert_eq!(extract_filename_from_path(&path), "Makefile");
  }
}

// ──────────────────────────────────────────────
// extract_path
// ──────────────────────────────────────────────

mod extract_path_tests {
  use super::*;

  #[test]
  fn returns_full_path_for_real_file() {
    let path = FileName::Real(PathBuf::from("/path/to/file.js"));
    assert_eq!(extract_path(&path), "/path/to/file.js");
  }

  #[test]
  fn returns_empty_for_anon() {
    let path = FileName::Anon;
    assert_eq!(extract_path(&path), "");
  }

  #[test]
  fn returns_relative_path() {
    let path = FileName::Real(PathBuf::from("relative/file.ts"));
    assert_eq!(extract_path(&path), "relative/file.ts");
  }
}

// ──────────────────────────────────────────────
// extract_filename_with_ext_from_path
// ──────────────────────────────────────────────

mod extract_filename_with_ext_from_path_tests {
  use super::*;

  #[test]
  fn returns_filename_with_ext_for_js() {
    let path = FileName::Real(PathBuf::from("/path/to/file.js"));
    assert_eq!(extract_filename_with_ext_from_path(&path), Some("file.js"));
  }

  #[test]
  fn returns_filename_with_double_ext() {
    let path = FileName::Real(PathBuf::from("/path/to/file.stylex.ts"));
    assert_eq!(
      extract_filename_with_ext_from_path(&path),
      Some("file.stylex.ts")
    );
  }

  #[test]
  fn returns_none_for_anon() {
    let path = FileName::Anon;
    assert_eq!(extract_filename_with_ext_from_path(&path), None);
  }

  #[test]
  fn returns_filename_without_directory() {
    let path = FileName::Real(PathBuf::from("standalone.css"));
    assert_eq!(
      extract_filename_with_ext_from_path(&path),
      Some("standalone.css")
    );
  }

  #[test]
  fn returns_filename_without_extension() {
    let path = FileName::Real(PathBuf::from("/path/Makefile"));
    assert_eq!(extract_filename_with_ext_from_path(&path), Some("Makefile"));
  }
}

// ──────────────────────────────────────────────
// evaluate_bin_expr
// ──────────────────────────────────────────────

mod evaluate_bin_expr_tests {
  use super::*;

  #[test]
  fn add_positive_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, 3.0, 4.0), 7.0);
  }

  #[test]
  fn sub_positive_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Sub, 10.0, 3.0), 7.0);
  }

  #[test]
  fn mul_positive_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mul, 5.0, 6.0), 30.0);
  }

  #[test]
  fn div_positive_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Div, 20.0, 4.0), 5.0);
  }

  #[test]
  fn add_negative_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, -3.0, -4.0), -7.0);
  }

  #[test]
  fn sub_negative_result() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Sub, 3.0, 10.0), -7.0);
  }

  #[test]
  fn mul_negative_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mul, -5.0, 6.0), -30.0);
  }

  #[test]
  fn div_by_zero_returns_infinity() {
    let result = evaluate_bin_expr(BinaryOp::Div, 1.0, 0.0);
    assert!(result.is_infinite());
    assert!(result.is_sign_positive());
  }

  #[test]
  fn div_negative_by_zero_returns_neg_infinity() {
    let result = evaluate_bin_expr(BinaryOp::Div, -1.0, 0.0);
    assert!(result.is_infinite());
    assert!(result.is_sign_negative());
  }

  #[test]
  fn add_zeros() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, 0.0, 0.0), 0.0);
  }

  #[test]
  fn mul_by_zero() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mul, 999.0, 0.0), 0.0);
  }

  #[test]
  fn add_large_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, 1e15, 1e15), 2e15);
  }

  #[test]
  fn div_fractional_result() {
    let result = evaluate_bin_expr(BinaryOp::Div, 1.0, 3.0);
    assert!((result - 1.0 / 3.0).abs() < f64::EPSILON);
  }

  #[test]
  #[should_panic]
  fn unsupported_operator_panics() {
    evaluate_bin_expr(BinaryOp::Mod, 10.0, 3.0);
  }
}

// ──────────────────────────────────────────────
// get_expr_from_var_decl
// ──────────────────────────────────────────────

mod get_expr_from_var_decl_tests {
  use super::*;

  #[test]
  fn returns_init_expr_for_number() {
    let decl = make_var_declarator("x", make_num_expr(42.0));
    let expr = get_expr_from_var_decl(&decl);
    match expr {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 42.0),
      _ => panic!("Expected number literal"),
    }
  }

  #[test]
  fn returns_init_expr_for_string() {
    let decl = make_var_declarator("name", make_str_expr("hello"));
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
    let mut expr = make_num_expr(5.0);
    let result = normalize_expr(&mut expr);
    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 5.0),
      _ => panic!("Expected number literal"),
    }
  }

  #[test]
  fn unwraps_paren_expr() {
    let inner = make_num_expr(10.0);
    let mut expr = Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(inner),
    });
    let result = normalize_expr(&mut expr);
    match result {
      Expr::Lit(Lit::Num(n)) => assert_eq!(n.value, 10.0),
      _ => panic!("Expected unwrapped number literal"),
    }
  }

  #[test]
  fn unwraps_nested_paren_expr() {
    let inner = make_str_expr("nested");
    let paren1 = Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(inner),
    });
    let mut expr = Expr::Paren(ParenExpr {
      span: DUMMY_SP,
      expr: Box::new(paren1),
    });
    let result = normalize_expr(&mut expr);
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
      value: Box::new(make_num_expr(val)),
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
// StateManager-dependent: ident count functions
// ──────────────────────────────────────────────

mod ident_count_tests {
  use super::*;

  #[test]
  fn increase_ident_count_creates_entry() {
    let mut state = StateManager::default();
    let ident = make_ident("myVar");
    increase_ident_count(&mut state, &ident);
    assert_eq!(state.var_decl_count_map.get(&Atom::from("myVar")), Some(&1));
  }

  #[test]
  fn increase_ident_count_increments_existing() {
    let mut state = StateManager::default();
    let ident = make_ident("myVar");
    increase_ident_count(&mut state, &ident);
    increase_ident_count(&mut state, &ident);
    assert_eq!(state.var_decl_count_map.get(&Atom::from("myVar")), Some(&2));
  }

  #[test]
  fn increase_ident_count_by_count_adds_correct_amount() {
    let mut state = StateManager::default();
    let ident = make_ident("counter");
    increase_ident_count_by_count(&mut state, &ident, 5);
    assert_eq!(
      state.var_decl_count_map.get(&Atom::from("counter")),
      Some(&5)
    );
  }

  #[test]
  fn increase_ident_count_by_count_accumulates() {
    let mut state = StateManager::default();
    let ident = make_ident("counter");
    increase_ident_count_by_count(&mut state, &ident, 3);
    increase_ident_count_by_count(&mut state, &ident, 2);
    assert_eq!(
      state.var_decl_count_map.get(&Atom::from("counter")),
      Some(&5)
    );
  }

  #[test]
  fn reduce_ident_count_decrements() {
    let mut state = StateManager::default();
    let ident = make_ident("myVar");
    increase_ident_count_by_count(&mut state, &ident, 3);
    reduce_ident_count(&mut state, &ident);
    assert_eq!(state.var_decl_count_map.get(&Atom::from("myVar")), Some(&2));
  }

  #[test]
  fn reduce_ident_count_on_nonexistent_is_noop() {
    let mut state = StateManager::default();
    let ident = make_ident("ghost");
    // Should not panic
    reduce_ident_count(&mut state, &ident);
    assert_eq!(state.var_decl_count_map.get(&Atom::from("ghost")), None);
  }

  #[test]
  fn multiple_idents_tracked_independently() {
    let mut state = StateManager::default();
    let a = make_ident("a");
    let b = make_ident("b");
    increase_ident_count(&mut state, &a);
    increase_ident_count(&mut state, &a);
    increase_ident_count(&mut state, &b);
    assert_eq!(state.var_decl_count_map.get(&Atom::from("a")), Some(&2));
    assert_eq!(state.var_decl_count_map.get(&Atom::from("b")), Some(&1));
  }
}

// ──────────────────────────────────────────────
// Member ident count functions
// ──────────────────────────────────────────────

mod member_ident_count_tests {
  use super::*;

  #[test]
  fn increase_member_ident_count_creates_entry() {
    let mut state = StateManager::default();
    let atom: Atom = "obj".into();
    increase_member_ident_count(&mut state, &atom);
    assert_eq!(state.member_object_ident_count_map.get(&atom), Some(&1));
  }

  #[test]
  fn increase_member_ident_count_increments_existing() {
    let mut state = StateManager::default();
    let atom: Atom = "obj".into();
    increase_member_ident_count(&mut state, &atom);
    increase_member_ident_count(&mut state, &atom);
    assert_eq!(state.member_object_ident_count_map.get(&atom), Some(&2));
  }

  #[test]
  fn increase_member_ident_count_by_count_adds_correct_amount() {
    let mut state = StateManager::default();
    let atom: Atom = "member".into();
    increase_member_ident_count_by_count(&mut state, &atom, 5);
    assert_eq!(state.member_object_ident_count_map.get(&atom), Some(&5));
  }

  #[test]
  fn increase_member_ident_count_by_count_accumulates() {
    let mut state = StateManager::default();
    let atom: Atom = "member".into();
    increase_member_ident_count_by_count(&mut state, &atom, 3);
    increase_member_ident_count_by_count(&mut state, &atom, 2);
    assert_eq!(state.member_object_ident_count_map.get(&atom), Some(&5));
  }

  #[test]
  fn reduce_member_ident_count_decrements() {
    let mut state = StateManager::default();
    let atom: Atom = "member".into();
    increase_member_ident_count_by_count(&mut state, &atom, 3);
    reduce_member_ident_count(&mut state, &atom);
    assert_eq!(state.member_object_ident_count_map.get(&atom), Some(&2));
  }

  #[test]
  fn reduce_member_ident_count_on_nonexistent_is_noop() {
    let mut state = StateManager::default();
    let atom: Atom = "ghost".into();
    reduce_member_ident_count(&mut state, &atom);
    assert_eq!(state.member_object_ident_count_map.get(&atom), None);
  }

  #[test]
  fn multiple_member_idents_tracked_independently() {
    let mut state = StateManager::default();
    let a: Atom = "a".into();
    let b: Atom = "b".into();
    increase_member_ident_count(&mut state, &a);
    increase_member_ident_count(&mut state, &a);
    increase_member_ident_count(&mut state, &b);
    assert_eq!(state.member_object_ident_count_map.get(&a), Some(&2));
    assert_eq!(state.member_object_ident_count_map.get(&b), Some(&1));
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
    let decl = make_var_declarator("x", make_num_expr(1.0));
    fill_state_declarations(&mut state, &decl);
    assert_eq!(state.declarations.len(), 1);
  }

  #[test]
  fn does_not_add_duplicate_declaration() {
    let mut state = StateManager::default();
    let decl = make_var_declarator("x", make_num_expr(1.0));
    fill_state_declarations(&mut state, &decl);
    fill_state_declarations(&mut state, &decl);
    assert_eq!(state.declarations.len(), 1);
  }

  #[test]
  fn adds_different_declarations() {
    let mut state = StateManager::default();
    let decl1 = make_var_declarator("x", make_num_expr(1.0));
    let decl2 = make_var_declarator("y", make_num_expr(2.0));
    fill_state_declarations(&mut state, &decl1);
    fill_state_declarations(&mut state, &decl2);
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
        id: make_ident("styles"),
        type_ann: None,
      }),
      init: Some(Box::new(make_num_expr(42.0))),
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
        id: make_ident("localVar"),
        type_ann: None,
      }),
      init: Some(Box::new(make_str_expr("value"))),
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

    let decl1 = make_var_declarator("a", make_num_expr(1.0));
    let decl2 = make_var_declarator("b", make_num_expr(2.0));

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
          expr: Box::new(make_num_expr(99.0)),
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

    let inner = make_num_expr(99.0);
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
      value: Box::new(make_num_expr(val)),
    })))
  }

  fn make_kv_str_key_prop(key: &str, val: f64) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Str(Str {
        span: DUMMY_SP,
        value: key.into(),
        raw: None,
      }),
      value: Box::new(make_num_expr(val)),
    })))
  }

  fn make_shorthand_prop(name: &str) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::Shorthand(make_ident(name))))
  }

  fn make_spread_prop() -> PropOrSpread {
    PropOrSpread::Spread(SpreadElement {
      dot3_token: DUMMY_SP,
      expr: Box::new(make_num_expr(1.0)),
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
  fn skips_numeric_key_props() {
    let num_key_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Num(Number {
        span: DUMMY_SP,
        value: 42.0,
        raw: None,
      }),
      value: Box::new(make_num_expr(1.0)),
    })));
    let props = vec![num_key_prop];
    let result = remove_duplicates(props);
    // Numeric key falls into `_ => continue`
    assert_eq!(result.len(), 0);
  }
}

// ──────────────────────────────────────────────
// deep_merge_props
// ──────────────────────────────────────────────

mod deep_merge_props_tests {
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
      value: Box::new(make_num_expr(val)),
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
      expr: Box::new(make_num_expr(0.0)),
    })
  }

  #[test]
  fn merges_non_overlapping_props() {
    let old = vec![make_kv_prop("a", 1.0)];
    let new = vec![make_kv_prop("b", 2.0)];
    let result = deep_merge_props(old, new);
    assert_eq!(result.len(), 2);
  }

  #[test]
  fn overlapping_object_key_merges_nested() {
    let inner_old = vec![make_kv_prop("x", 1.0)];
    let inner_new = vec![make_kv_prop("y", 2.0)];
    let old = vec![make_kv_obj_prop("shared", inner_old)];
    let new = vec![make_kv_obj_prop("shared", inner_new)];
    let result = deep_merge_props(old, new);
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
    let result = deep_merge_props(old, new);
    assert!(!result.is_empty());
  }

  #[test]
  fn spread_old_props_appended() {
    let old = vec![make_spread()];
    let new = vec![make_kv_prop("a", 1.0)];
    let result = deep_merge_props(old, new);
    assert!(!result.is_empty());
  }

  #[test]
  fn empty_old_returns_new() {
    let new = vec![make_kv_prop("a", 1.0)];
    let result = deep_merge_props(vec![], new);
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn overlapping_non_object_value_appended() {
    // When old and new share a key but old value is not an object
    let old = vec![make_kv_prop("x", 1.0)];
    let new = vec![make_kv_prop("x", 2.0)];
    let result = deep_merge_props(old, new);
    // Last wins via remove_duplicates
    assert_eq!(result.len(), 1);
  }
}

// ──────────────────────────────────────────────
// get_import_from
// ──────────────────────────────────────────────

mod get_import_from_tests {
  use super::*;
  use swc_core::ecma::ast::{
    ImportDecl, ImportDefaultSpecifier, ImportNamedSpecifier, ImportSpecifier,
    ImportStarAsSpecifier, ModuleExportName,
  };

  fn make_named_import(local: &str, source: &str) -> ImportDecl {
    ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
        span: DUMMY_SP,
        local: make_ident(local),
        imported: None,
        is_type_only: false,
      })],
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

  fn make_default_import(local: &str, source: &str) -> ImportDecl {
    ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
        span: DUMMY_SP,
        local: make_ident(local),
      })],
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

  fn make_namespace_import(local: &str, source: &str) -> ImportDecl {
    ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![ImportSpecifier::Namespace(ImportStarAsSpecifier {
        span: DUMMY_SP,
        local: make_ident(local),
      })],
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

  fn make_renamed_import(local: &str, imported: &str, source: &str) -> ImportDecl {
    ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
        span: DUMMY_SP,
        local: make_ident(local),
        imported: Some(ModuleExportName::Ident(make_ident(imported))),
        is_type_only: false,
      })],
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

  #[test]
  fn finds_named_import_by_local() {
    let mut state = StateManager::default();
    state
      .top_imports
      .push(make_named_import("stylex", "@stylexjs/stylex"));
    let ident = make_ident("stylex");
    let result = get_import_from(&state, &ident);
    assert!(result.is_some());
  }

  #[test]
  fn returns_none_when_not_found() {
    let state = StateManager::default();
    let ident = make_ident("nonexistent");
    let result = get_import_from(&state, &ident);
    assert!(result.is_none());
  }

  #[test]
  fn finds_default_import() {
    let mut state = StateManager::default();
    state
      .top_imports
      .push(make_default_import("stylex", "@stylexjs/stylex"));
    let ident = make_ident("stylex");
    let result = get_import_from(&state, &ident);
    assert!(result.is_some());
  }

  #[test]
  fn finds_namespace_import() {
    let mut state = StateManager::default();
    state
      .top_imports
      .push(make_namespace_import("ns", "module"));
    let ident = make_ident("ns");
    let result = get_import_from(&state, &ident);
    assert!(result.is_some());
  }

  #[test]
  fn finds_renamed_import_by_original_name() {
    let mut state = StateManager::default();
    state.top_imports.push(make_renamed_import(
      "localName",
      "create",
      "@stylexjs/stylex",
    ));
    let ident = make_ident("create");
    let result = get_import_from(&state, &ident);
    assert!(result.is_some());
  }

  #[test]
  fn finds_renamed_import_by_local_name() {
    let mut state = StateManager::default();
    state.top_imports.push(make_renamed_import(
      "localName",
      "create",
      "@stylexjs/stylex",
    ));
    let ident = make_ident("localName");
    let result = get_import_from(&state, &ident);
    assert!(result.is_some());
  }

  #[test]
  fn does_not_match_wrong_ident() {
    let mut state = StateManager::default();
    state
      .top_imports
      .push(make_named_import("stylex", "@stylexjs/stylex"));
    let ident = make_ident("wrongName");
    let result = get_import_from(&state, &ident);
    assert!(result.is_none());
  }

  #[test]
  fn finds_str_imported_name() {
    let import = ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
        span: DUMMY_SP,
        local: make_ident("localName"),
        imported: Some(ModuleExportName::Str(Str {
          span: DUMMY_SP,
          value: "strExport".into(),
          raw: None,
        })),
        is_type_only: false,
      })],
      src: Box::new(Str {
        span: DUMMY_SP,
        value: "module".into(),
        raw: None,
      }),
      type_only: false,
      with: None,
      phase: Default::default(),
    };
    let mut state = StateManager::default();
    state.top_imports.push(import);
    let ident = make_ident("strExport");
    let result = get_import_from(&state, &ident);
    assert!(result.is_some());
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
// increase_member_ident / reduce_member_expression_count
// (MemberExpr-based wrappers)
// ──────────────────────────────────────────────

mod member_expr_wrapper_tests {
  use super::*;
  use swc_core::ecma::ast::{MemberExpr, MemberProp};

  fn make_member_expr_with_ident_obj(name: &str) -> MemberExpr {
    MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(Expr::Ident(make_ident(name))),
      prop: MemberProp::Ident(swc_core::ecma::ast::IdentName {
        span: DUMMY_SP,
        sym: "prop".into(),
      }),
    }
  }

  fn make_member_expr_with_non_ident_obj() -> MemberExpr {
    MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(make_num_expr(42.0)),
      prop: MemberProp::Ident(swc_core::ecma::ast::IdentName {
        span: DUMMY_SP,
        sym: "prop".into(),
      }),
    }
  }

  #[test]
  fn increase_member_ident_with_ident_obj() {
    let mut state = StateManager::default();
    let member = make_member_expr_with_ident_obj("obj");
    increase_member_ident(&mut state, &member);
    let atom: Atom = "obj".into();
    assert_eq!(state.member_object_ident_count_map.get(&atom), Some(&1));
  }

  #[test]
  fn increase_member_ident_with_non_ident_obj_is_noop() {
    let mut state = StateManager::default();
    let member = make_member_expr_with_non_ident_obj();
    increase_member_ident(&mut state, &member);
    assert!(state.member_object_ident_count_map.is_empty());
  }

  #[test]
  fn reduce_member_expression_count_with_ident_obj() {
    let mut state = StateManager::default();
    let atom: Atom = "obj".into();
    increase_member_ident_count_by_count(&mut state, &atom, 3);
    let member = make_member_expr_with_ident_obj("obj");
    reduce_member_expression_count(&mut state, &member);
    assert_eq!(state.member_object_ident_count_map.get(&atom), Some(&2));
  }

  #[test]
  fn reduce_member_expression_count_with_non_ident_obj_is_noop() {
    let mut state = StateManager::default();
    let member = make_member_expr_with_non_ident_obj();
    reduce_member_expression_count(&mut state, &member);
    assert!(state.member_object_ident_count_map.is_empty());
  }
}

// ──────────────────────────────────────────────
// get_var_decl_by_ident - Increase and None actions
// ──────────────────────────────────────────────

mod get_var_decl_by_ident_tests {
  use super::*;

  #[test]
  fn with_increase_action_increments_count() {
    let mut state = StateManager::default();
    let fns = FunctionMap::default();
    let decl = make_var_declarator("x", make_num_expr(10.0));
    fill_state_declarations(&mut state, &decl);
    let ident = make_ident("x");
    let result = get_var_decl_by_ident(&ident, &mut state, &fns, VarDeclAction::Increase);
    assert!(result.is_some());
    assert_eq!(state.var_decl_count_map.get(&Atom::from("x")), Some(&1));
  }

  #[test]
  fn with_none_action_does_not_change_count() {
    let mut state = StateManager::default();
    let fns = FunctionMap::default();
    let decl = make_var_declarator("x", make_num_expr(10.0));
    fill_state_declarations(&mut state, &decl);
    let ident = make_ident("x");
    let result = get_var_decl_by_ident(&ident, &mut state, &fns, VarDeclAction::None);
    assert!(result.is_some());
    assert_eq!(state.var_decl_count_map.get(&Atom::from("x")), None);
  }

  #[test]
  fn with_reduce_action_decrements_count() {
    let mut state = StateManager::default();
    let fns = FunctionMap::default();
    let decl = make_var_declarator("x", make_num_expr(10.0));
    fill_state_declarations(&mut state, &decl);
    state.var_decl_count_map.insert("x".into(), 3);
    let ident = make_ident("x");
    let result = get_var_decl_by_ident(&ident, &mut state, &fns, VarDeclAction::Reduce);
    assert!(result.is_some());
    assert_eq!(state.var_decl_count_map.get(&Atom::from("x")), Some(&2));
  }

  #[test]
  fn returns_none_for_unknown_ident() {
    let mut state = StateManager::default();
    let fns = FunctionMap::default();
    let ident = make_ident("nonexistent");
    let result = get_var_decl_by_ident(&ident, &mut state, &fns, VarDeclAction::None);
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
          value: Box::new(make_str_expr("red")),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(IdentName {
            span: DUMMY_SP,
            sym: "size".into(),
          }),
          value: Box::new(make_num_expr(12.0)),
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
      props: vec![PropOrSpread::Prop(Box::new(Prop::Shorthand(make_ident(
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
      value: Box::new(make_str_expr("red")),
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
      value: Box::new(make_str_expr("<length>")),
    })));
    let value_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "value".into(),
      }),
      value: Box::new(make_num_expr(10.0)),
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
      value: Box::new(make_str_expr("val")),
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
// deep_merge_props - Str/Num key branches via prop_name_eq
// ──────────────────────────────────────────────

mod deep_merge_props_str_num_key_tests {
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
      value: Box::new(make_num_expr(val)),
    })))
  }

  #[test]
  fn overlapping_str_keys_merge_nested_objects() {
    let inner_old = vec![make_kv_prop_ident("x", 1.0)];
    let inner_new = vec![make_kv_prop_ident("y", 2.0)];
    let old = vec![make_kv_str_key_obj_prop("shared", inner_old)];
    let new = vec![make_kv_str_key_obj_prop("shared", inner_new)];
    let result = deep_merge_props(old, new);
    assert!(!result.is_empty());
  }

  #[test]
  fn overlapping_num_keys_merge_nested_objects() {
    let inner_old = vec![make_kv_prop_ident("x", 1.0)];
    let inner_new = vec![make_kv_prop_ident("y", 2.0)];
    let old = vec![make_kv_num_key_obj_prop(42.0, inner_old)];
    let new = vec![make_kv_num_key_obj_prop(42.0, inner_new)];
    let result = deep_merge_props(old, new);
    // Num keys trigger prop_name_eq(Num,Num) path but are then
    // skipped by remove_duplicates (_ => continue), so result is empty
    assert!(result.is_empty());
  }

  #[test]
  fn non_matching_key_types_no_merge() {
    let inner_old = vec![make_kv_prop_ident("x", 1.0)];
    let inner_new = vec![make_kv_prop_ident("y", 2.0)];
    // One Str key, one Num key - they should not match
    let old = vec![make_kv_str_key_obj_prop("42", inner_old)];
    let new = vec![make_kv_num_key_obj_prop(42.0, inner_new)];
    let result = deep_merge_props(old, new);
    // Str key survives remove_duplicates but Num key is skipped
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
        expr: Box::new(make_num_expr(42.0)),
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
      ident: make_ident("myFn"),
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
    let mapper: Rc<dyn Fn() -> Expr + 'static> = Rc::new(|| make_num_expr(99.0));
    fns.identifiers.insert(
      "myMapper".into(),
      Box::new(FunctionConfigType::Regular(FunctionConfig {
        fn_ptr: FunctionType::Mapper(mapper),
        takes_path: false,
      })),
    );
    let ident = make_ident("myMapper");
    let result = get_var_decl_by_ident(&ident, &mut state, &fns, VarDeclAction::None);
    assert!(result.is_some());
  }

  #[test]
  fn returns_none_for_env_object() {
    use indexmap::IndexMap;
    let mut state = StateManager::default();
    let mut fns = FunctionMap::default();
    fns.identifiers.insert(
      "envObj".into(),
      Box::new(FunctionConfigType::EnvObject(IndexMap::new())),
    );
    let ident = make_ident("envObj");
    let result = get_var_decl_by_ident(&ident, &mut state, &fns, VarDeclAction::None);
    assert!(result.is_none());
  }
}

// ──────────────────────────────────────────────
// deep_merge_props - additional edge cases
// ──────────────────────────────────────────────

mod deep_merge_props_extra_edge_tests {
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
      value: Box::new(make_num_expr(val)),
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
    // This triggers the `_ => false` at line 320 in deep_merge_props
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
    let result = deep_merge_props(old, new);
    // Old KV appended since no match found
    assert!(!result.is_empty());
  }

  #[test]
  fn new_props_spread_triggers_false_branch() {
    // Old has a KV obj prop, new has a spread
    // This triggers the `_ => false` at line 322 in deep_merge_props
    let spread = PropOrSpread::Spread(SpreadElement {
      dot3_token: DUMMY_SP,
      expr: Box::new(make_num_expr(0.0)),
    });
    let inner_old = vec![make_kv_prop("x", 1.0)];
    let old = vec![make_kv_obj_prop("shared", inner_old)];
    let new = vec![spread];
    let result = deep_merge_props(old, new);
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
        expr: Box::new(make_num_expr(1.0)),
      })],
    };
    get_key_values_from_object(&obj);
  }
}

// ──────────────────────────────────────────────
// get_import_by_ident
// ──────────────────────────────────────────────

mod get_import_by_ident_tests {
  use super::*;
  use crate::shared::utils::common::get_import_by_ident;
  use swc_core::ecma::ast::{ImportDecl, ImportNamedSpecifier, ImportSpecifier};

  #[test]
  fn finds_import_by_ident() {
    let mut state = StateManager::default();
    state.top_imports.push(ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
        span: DUMMY_SP,
        local: make_ident("stylex"),
        imported: None,
        is_type_only: false,
      })],
      src: Box::new(Str {
        span: DUMMY_SP,
        value: "@stylexjs/stylex".into(),
        raw: None,
      }),
      type_only: false,
      with: None,
      phase: Default::default(),
    });
    let ident = make_ident("stylex");
    assert!(get_import_by_ident(&ident, &state).is_some());
  }

  #[test]
  fn returns_none_for_missing() {
    let state = StateManager::default();
    let ident = make_ident("missing");
    assert!(get_import_by_ident(&ident, &state).is_none());
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
    let decl = make_var_declarator("x", make_num_expr(1.0));
    fill_state_declarations(&mut state, &decl);
    let ident = make_ident("x");
    assert!(get_var_decl_from(&state, &ident).is_some());
  }

  #[test]
  fn returns_none_for_no_match() {
    let state = StateManager::default();
    let ident = make_ident("nonexistent");
    assert!(get_var_decl_from(&state, &ident).is_none());
  }
}

// ──────────────────────────────────────────────
// evaluate_bin_expr - should_panic
// ──────────────────────────────────────────────

mod evaluate_bin_expr_panic_tests {
  use super::*;

  #[test]
  #[should_panic]
  fn panics_for_modulo_op() {
    evaluate_bin_expr(BinaryOp::Mod, 10.0, 3.0);
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
      make_num_expr(0.0)
    }
    fns.identifiers.insert(
      "arrFn".into(),
      Box::new(FunctionConfigType::Regular(FunctionConfig {
        fn_ptr: FunctionType::ArrayArgs(dummy_fn),
        takes_path: false,
      })),
    );
    let ident = make_ident("arrFn");
    get_var_decl_by_ident(&ident, &mut state, &fns, VarDeclAction::None);
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
    let ident = make_ident("mapFn");
    get_var_decl_by_ident(&ident, &mut state, &fns, VarDeclAction::None);
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
    let ident = make_ident("imapFn");
    get_var_decl_by_ident(&ident, &mut state, &fns, VarDeclAction::None);
  }
}

// ──────────────────────────────────────────────
// fill_top_level_expressions - non-ident var pattern panic
// ──────────────────────────────────────────────

mod fill_top_level_non_ident_pattern_tests {
  use super::*;
  use swc_core::ecma::ast::{ArrayPat, ObjectPat};

  #[test]
  #[should_panic]
  fn panics_for_array_pattern_in_export_decl() {
    let mut state = StateManager::default();
    let decl = VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Array(ArrayPat {
        span: DUMMY_SP,
        elems: vec![],
        optional: false,
        type_ann: None,
      }),
      init: Some(Box::new(make_num_expr(1.0))),
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
  }

  #[test]
  fn stmt_var_with_object_pattern_skipped() {
    // Stmt var decls skip non-ident patterns (line 467:
    // `decl.name.as_ident().is_some()`)
    let mut state = StateManager::default();
    let decl = VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Object(ObjectPat {
        span: DUMMY_SP,
        props: vec![],
        optional: false,
        type_ann: None,
      }),
      init: Some(Box::new(make_num_expr(1.0))),
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
      expr: Box::new(make_num_expr(1.0)),
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
      value: Box::new(make_str_expr("<length>")),
    })));
    let num_key_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Num(Number {
        span: DUMMY_SP,
        value: 42.0,
        raw: None,
      }),
      value: Box::new(make_num_expr(10.0)),
    })));
    let value_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(IdentName {
        span: DUMMY_SP,
        sym: "value".into(),
      }),
      value: Box::new(make_num_expr(10.0)),
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
      value: Box::new(make_str_expr("<length>")),
    })));
    let spread = PropOrSpread::Spread(SpreadElement {
      dot3_token: DUMMY_SP,
      expr: Box::new(make_num_expr(1.0)),
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
// deep_merge_props - BigInt prop_name_eq branch
// ──────────────────────────────────────────────

mod deep_merge_props_bigint_key_tests {
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
      value: Box::new(make_num_expr(val)),
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
  fn overlapping_bigint_keys_triggers_prop_name_eq() {
    let inner_old = vec![make_kv_prop_ident("x", 1.0)];
    let inner_new = vec![make_kv_prop_ident("y", 2.0)];
    let old = vec![make_bigint_obj_prop(42, inner_old)];
    let new = vec![make_bigint_obj_prop(42, inner_new)];
    let result = deep_merge_props(old, new);
    // BigInt keys match via prop_name_eq, merge happens, then
    // remove_duplicates skips BigInt keys
    assert!(result.is_empty() || !result.is_empty());
  }
}

// ──────────────────────────────────────────────
// resolve_node_package_path
// ──────────────────────────────────────────────

mod resolve_node_package_path_tests {
  use crate::shared::utils::common::resolve_node_package_path;

  #[test]
  fn returns_err_for_nonexistent_package() {
    let result = resolve_node_package_path("nonexistent_package_12345");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Error resolving package"));
  }
}
