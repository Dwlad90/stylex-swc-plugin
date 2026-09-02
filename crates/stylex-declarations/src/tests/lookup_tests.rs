use swc_core::{
  common::{DUMMY_SP, SyntaxContext},
  ecma::ast::{BindingIdent, Expr, Pat, Str, VarDeclarator},
};

use crate::lookup::{get_import_by_ident, get_var_decl_by_ident};
use stylex_ast::ast::convertors::create_number_expr;
use stylex_ast::ast::factories::create_ident;
use stylex_state::state_writers::fill_state_declarations;
use stylex_state::{functions::FunctionMap, state_manager::StateManager};

// ──────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────

/// One declarator over the initializer handed in. Every case here needs a
/// declarator for the state to record, and none of them cares about the name
/// pattern beyond it being an identifier.
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
// get_var_decl_by_ident - FunctionMap branches
// ──────────────────────────────────────────────

mod get_var_decl_by_ident_function_map_tests {
  use super::*;
  use std::rc::Rc;
  use stylex_state::functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType};

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
// get_var_decl_from
// ──────────────────────────────────────────────

mod get_var_decl_from_tests {
  use super::*;
  use crate::lookup::get_var_decl_from;

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
// get_var_decl_by_ident - FunctionMap panic branches
// ──────────────────────────────────────────────

mod get_var_decl_by_ident_fn_map_panic_tests {
  use super::*;
  use stylex_state::functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType};

  /// The three arms below refuse for three different reasons, so each asserts
  /// the message it is refused with: a bare `#[should_panic]` passes on any
  /// panic and cannot tell one arm from another.
  #[test]
  #[should_panic(expected = "Function type not supported:")]
  fn panics_for_non_mapper_regular_function() {
    let mut state = StateManager::default();
    let mut fns = FunctionMap::default();
    fn dummy_fn(
      _args: Vec<Expr>,
      _state: &mut dyn stylex_types::traits::StyleOptions,
      _fns: &stylex_state::functions::FunctionMap,
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
  #[should_panic(expected = "Map values are not supported in this context.")]
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
  #[should_panic(expected = "IndexMap values are not supported in this context.")]
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
// get_var_decl_parts_by_ident
// ──────────────────────────────────────────────

mod get_var_decl_parts_by_ident_tests {
  use super::*;
  use crate::lookup::get_var_decl_parts_by_ident;
  use std::rc::Rc;
  use stylex_state::functions::{FunctionConfig, FunctionConfigType, FunctionType};
  use swc_core::common::{BytePos, Span};
  use swc_core::ecma::ast::Lit;

  /// A declarator whose span is distinguishable from `DUMMY_SP`, so a test can
  /// tell the span it asked for from the one every synthesized node carries.
  fn make_spanned_declarator(name: &str, init: Expr, lo: u32, hi: u32) -> VarDeclarator {
    VarDeclarator {
      span: Span::new(BytePos(lo), BytePos(hi)),
      name: Pat::Ident(BindingIdent {
        id: create_ident(name),
        type_ann: None,
      }),
      init: Some(Box::new(init)),
      definite: false,
    }
  }

  #[test]
  fn answers_the_span_and_the_initializer_the_state_recorded() {
    let mut state = StateManager::default();
    let fns = FunctionMap::default();
    let decl = make_spanned_declarator("x", create_number_expr(7.0), 10, 20);
    fill_state_declarations(&mut state, &decl);

    let (span, init) = get_var_decl_parts_by_ident(&create_ident("x"), &mut state, &fns)
      .expect("Expected the recorded declaration");

    assert_eq!(span.lo, BytePos(10));
    assert_eq!(span.hi, BytePos(20));
    match init.as_deref() {
      Some(Expr::Lit(Lit::Num(number))) => assert_eq!(number.value, 7.0),
      other => panic!("Expected the recorded initializer, got {:?}", other),
    }
  }

  #[test]
  fn answers_none_where_nothing_binds_the_name() {
    let mut state = StateManager::default();
    let fns = FunctionMap::default();

    assert!(get_var_decl_parts_by_ident(&create_ident("absent"), &mut state, &fns).is_none());
  }

  #[test]
  fn falls_through_to_the_declarator_the_function_map_builds() {
    let mut state = StateManager::default();
    let mut fns = FunctionMap::default();
    let mapper: Rc<dyn Fn() -> Expr + 'static> = Rc::new(|| create_number_expr(99.0));
    fns.identifiers.insert(
      "mapped".into(),
      Box::new(FunctionConfigType::Regular(FunctionConfig {
        fn_ptr: FunctionType::Mapper(mapper),
        takes_path: false,
      })),
    );

    let (_, init) = get_var_decl_parts_by_ident(&create_ident("mapped"), &mut state, &fns)
      .expect("Expected the synthesized declaration");

    match init.as_deref() {
      Some(Expr::Lit(Lit::Num(number))) => assert_eq!(number.value, 99.0),
      other => panic!("Expected the mapped initializer, got {:?}", other),
    }
  }

  #[test]
  fn a_recorded_declaration_wins_over_a_function_of_the_same_name() {
    // The state hit is probed first, and the two answers differ, so which one
    // comes back says which path ran.
    let mut state = StateManager::default();
    let mut fns = FunctionMap::default();
    fill_state_declarations(
      &mut state,
      &make_spanned_declarator("both", create_number_expr(1.0), 30, 40),
    );
    let mapper: Rc<dyn Fn() -> Expr + 'static> = Rc::new(|| create_number_expr(2.0));
    fns.identifiers.insert(
      "both".into(),
      Box::new(FunctionConfigType::Regular(FunctionConfig {
        fn_ptr: FunctionType::Mapper(mapper),
        takes_path: false,
      })),
    );

    let (span, init) = get_var_decl_parts_by_ident(&create_ident("both"), &mut state, &fns)
      .expect("Expected the recorded declaration");

    assert_eq!(span.lo, BytePos(30));
    match init.as_deref() {
      Some(Expr::Lit(Lit::Num(number))) => assert_eq!(number.value, 1.0),
      other => panic!("Expected the recorded initializer, got {:?}", other),
    }
  }
}
