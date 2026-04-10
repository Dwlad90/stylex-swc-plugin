#[cfg(test)]
mod tests {
  use std::path::PathBuf;

  use swc_core::{
    atoms::Atom,
    common::{DUMMY_SP, FileName, SyntaxContext},
    ecma::ast::{
      BinaryOp, BindingIdent, Decl, ExportDecl, Expr, Ident, Lit, Module, ModuleDecl, ModuleItem,
      Number, Pat, Stmt, Str, VarDecl, VarDeclKind, VarDeclarator,
    },
  };

  use crate::shared::structures::state_manager::StateManager;
  use crate::shared::utils::common::{
    _md5_hash, deep_merge_props, evaluate_bin_expr, extract_filename_from_path,
    extract_filename_with_ext_from_path, extract_path, fill_state_declarations,
    fill_top_level_expressions, gen_file_based_identifier, get_expr_from_var_decl,
    get_import_from, increase_ident_count, increase_ident_count_by_count,
    increase_member_ident_count, increase_member_ident_count_by_count, js_object_to_json,
    normalize_expr, reduce_ident_count, reduce_member_ident_count, remove_duplicates,
    serialize_value_to_json_string, type_of,
  };

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
  // _md5_hash
  // ──────────────────────────────────────────────

  mod md5_hash_tests {
    use super::*;

    #[test]
    fn deterministic_same_input_same_output() {
      let hash1 = _md5_hash("hello", 8);
      let hash2 = _md5_hash("hello", 8);
      assert_eq!(hash1, hash2);
    }

    #[test]
    fn different_inputs_produce_different_hashes() {
      let hash1 = _md5_hash("hello", 8);
      let hash2 = _md5_hash("world", 8);
      assert_ne!(hash1, hash2);
    }

    #[test]
    fn empty_string_produces_valid_hash() {
      let hash = _md5_hash("", 8);
      assert_eq!(hash.len(), 8);
      assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn respects_length_parameter() {
      let hash_short = _md5_hash("test", 4);
      let hash_long = _md5_hash("test", 16);
      assert_eq!(hash_short.len(), 4);
      assert_eq!(hash_long.len(), 16);
    }

    #[test]
    fn length_exceeding_hash_returns_full_hash() {
      let hash = _md5_hash("test", 100);
      // MD5 hex is always 32 characters
      assert_eq!(hash.len(), 32);
    }

    #[test]
    fn works_with_numeric_input() {
      let hash = _md5_hash(42, 8);
      assert_eq!(hash.len(), 8);
    }

    #[test]
    fn works_with_boolean_input() {
      let hash = _md5_hash(true, 8);
      assert_eq!(hash.len(), 8);
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
      assert_eq!(
        state.member_object_ident_count_map.get(&atom),
        Some(&1)
      );
    }

    #[test]
    fn increase_member_ident_count_increments_existing() {
      let mut state = StateManager::default();
      let atom: Atom = "obj".into();
      increase_member_ident_count(&mut state, &atom);
      increase_member_ident_count(&mut state, &atom);
      assert_eq!(
        state.member_object_ident_count_map.get(&atom),
        Some(&2)
      );
    }

    #[test]
    fn increase_member_ident_count_by_count_adds_correct_amount() {
      let mut state = StateManager::default();
      let atom: Atom = "member".into();
      increase_member_ident_count_by_count(&mut state, &atom, 5);
      assert_eq!(
        state.member_object_ident_count_map.get(&atom),
        Some(&5)
      );
    }

    #[test]
    fn increase_member_ident_count_by_count_accumulates() {
      let mut state = StateManager::default();
      let atom: Atom = "member".into();
      increase_member_ident_count_by_count(&mut state, &atom, 3);
      increase_member_ident_count_by_count(&mut state, &atom, 2);
      assert_eq!(
        state.member_object_ident_count_map.get(&atom),
        Some(&5)
      );
    }

    #[test]
    fn reduce_member_ident_count_decrements() {
      let mut state = StateManager::default();
      let atom: Atom = "member".into();
      increase_member_ident_count_by_count(&mut state, &atom, 3);
      reduce_member_ident_count(&mut state, &atom);
      assert_eq!(
        state.member_object_ident_count_map.get(&atom),
        Some(&2)
      );
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
      let props = vec![
        make_shorthand_prop("x"),
        make_kv_prop("x", 5.0),
      ];
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

    fn make_renamed_import(
      local: &str,
      imported: &str,
      source: &str,
    ) -> ImportDecl {
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
      state.top_imports.push(make_named_import("stylex", "@stylexjs/stylex"));
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
      state.top_imports.push(make_default_import("stylex", "@stylexjs/stylex"));
      let ident = make_ident("stylex");
      let result = get_import_from(&state, &ident);
      assert!(result.is_some());
    }

    #[test]
    fn finds_namespace_import() {
      let mut state = StateManager::default();
      state.top_imports.push(make_namespace_import("ns", "module"));
      let ident = make_ident("ns");
      let result = get_import_from(&state, &ident);
      assert!(result.is_some());
    }

    #[test]
    fn finds_renamed_import_by_original_name() {
      let mut state = StateManager::default();
      state
        .top_imports
        .push(make_renamed_import("localName", "create", "@stylexjs/stylex"));
      let ident = make_ident("create");
      let result = get_import_from(&state, &ident);
      assert!(result.is_some());
    }

    #[test]
    fn finds_renamed_import_by_local_name() {
      let mut state = StateManager::default();
      state
        .top_imports
        .push(make_renamed_import("localName", "create", "@stylexjs/stylex"));
      let ident = make_ident("localName");
      let result = get_import_from(&state, &ident);
      assert!(result.is_some());
    }

    #[test]
    fn does_not_match_wrong_ident() {
      let mut state = StateManager::default();
      state.top_imports.push(make_named_import("stylex", "@stylexjs/stylex"));
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
}
