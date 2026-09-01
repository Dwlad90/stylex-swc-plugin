use swc_core::{
  common::{BytePos, DUMMY_SP, Span, SyntaxContext},
  ecma::ast::{
    BindingIdent, Decl, ExportDecl, Expr, Module, ModuleDecl, ModuleItem, Pat, Stmt, Str, VarDecl,
    VarDeclKind, VarDeclarator,
  },
};

use crate::{
  state_manager::StateManager,
  state_writers::{fill_state_declarations, fill_top_level_expressions},
};
use stylex_ast::ast::convertors::{create_number_expr, create_string_expr};
use stylex_ast::ast::factories::create_ident;

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
