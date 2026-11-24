//! The `env` option read inside an expression the engine folds.
//!
//! The `env` object crosses into the engine as an object of its entries,
//! through the namespace (`stylex.env`) and through a named import (`env`). A
//! key the option does not carry reads as `undefined`. A function entry
//! crosses as a native function that calls the option's function.

use std::rc::Rc;

use indexmap::IndexMap;
use stylex_ast::ast::convertors::{create_number_expr, create_string_expr};
use stylex_ast::ast::factories::{create_key_value_prop, create_object_lit};
use stylex_state::{
  functions::{FunctionMap, FunctionType},
  state_manager::{ImportKind, StateManager},
};
use stylex_structures::{
  named_import_source::ImportSources,
  stylex_env::{EnvEntry, JSFunction},
  stylex_options::StyleXOptions,
};
use swc_core::common::{GLOBALS, Globals};
use swc_core::ecma::ast::{Expr, Lit};

use super::evaluate;
use super::source_evaluation::*;
use crate::evaluate_result::EvaluateResult;

/// The `env` option every case reads, unless it sets one of its own.
fn an_env() -> IndexMap<String, EnvEntry> {
  let mut env = IndexMap::default();

  env.insert("color".into(), EnvEntry::Expr(create_string_expr("blue")));
  env.insert("size".into(), EnvEntry::Expr(create_number_expr(4.0)));
  env.insert(
    "theme".into(),
    EnvEntry::Expr(Expr::Object(create_object_lit(vec![
      create_key_value_prop("fg", create_string_expr("green")),
    ]))),
  );
  env.insert(
    "tint".into(),
    EnvEntry::Function(JSFunction::new(|args| match args.first() {
      Some(Expr::Lit(Lit::Str(text))) => {
        create_string_expr(&format!("{}!", text.value.to_string_lossy()))
      },
      _ => create_string_expr("none"),
    })),
  );

  env
}

/// Evaluates `source` with the `env` option set to `env`, through a namespace
/// import `stylex` and a named import `env`, under `options`.
fn evaluated_under(
  mut options: StyleXOptions,
  env: IndexMap<String, EnvEntry>,
  fns: FunctionMap,
  source: &str,
) -> Box<EvaluateResult> {
  options.core.env = Rc::new(env);

  GLOBALS.set(&Globals::new(), || {
    let mut state = StateManager::new(options);
    let mut fns = fns;

    state.insert_stylex_import(ImportSources::Regular("stylex".into()));
    state.insert_stylex_api_import(ImportKind::Env, "env".into());
    state.apply_stylex_env(&mut fns);

    evaluate(&parse_expr(source), &mut state, &fns)
  })
}

/// The text `source` folds to, with [`an_env`] set.
#[track_caller]
fn folded(source: &str) -> String {
  folded_text_of(
    evaluated_under(
      StyleXOptions::default(),
      an_env(),
      a_function_fold(),
      source,
    ),
    source,
  )
}

/// The reason `source` refused with, with [`an_env`] set.
#[track_caller]
fn refusal(source: &str) -> String {
  let result = evaluated_under(
    StyleXOptions::default(),
    an_env(),
    a_function_fold(),
    source,
  );

  assert_refused(&result, source);

  result.reason.unwrap_or_default()
}

#[test]
fn an_env_entry_is_read_in_a_callback() {
  for (source, expected) in [
    ("['a'].map(() => stylex.env.color ?? 'red')[0]", "blue"),
    ("['a'].map(() => stylex.env.nope ?? 'red')[0]", "red"),
    ("['a'].map(() => env.color ?? 'red')[0]", "blue"),
    ("['a'].map(() => env.nope ?? 'red')[0]", "red"),
    ("['a'].map(() => stylex.env.theme.fg)[0]", "green"),
    ("['a'].map(() => String(stylex.env.size + 1))[0]", "5"),
    (
      "['a'].map(() => Object.keys(env).join())[0]",
      "color,size,theme,tint",
    ),
  ] {
    assert_eq!(folded(source), expected, "{source}");
  }
}

/// A missing key is `undefined`, outside a callback as well.
#[test]
fn a_missing_key_reads_as_undefined() {
  for (source, expected) in [
    ("['a', stylex.env.nope].join()", "a,"),
    ("['a', 'b'].join(stylex.env.nope)", "a,b"),
    ("'a'.concat(stylex.env.nope)", "aundefined"),
    ("'a'.concat(env.nope)", "aundefined"),
  ] {
    assert_eq!(folded(source), expected, "{source}");
  }
}

/// The namespace holds the `env` object beside the compiler functions, so one
/// fold reads both through one name.
#[test]
fn the_env_object_and_a_compiler_function_share_the_namespace() {
  let mut fns = FunctionMap::default();

  let mut members = rustc_hash::FxHashMap::default();

  members.insert(
    "kf".into(),
    Box::new(folded_entry(
      FunctionType::StylexExprFn(|expr, _| expr),
      false,
    )),
  );
  fns.member_expressions.insert(
    ImportSources::Regular("stylex".to_string()),
    Box::new(members),
  );

  for source in [
    "['a'].map(() => stylex.kf(stylex.env.color ?? 'red')).join()",
    "['a'].map(() => stylex.env.color + stylex.kf('')).join()",
  ] {
    let result = evaluated_under(StyleXOptions::default(), an_env(), fns.clone(), source);

    assert_eq!(folded_text_of(result, source), "blue", "{source}");
  }
}

/// A function entry is called with the values its arguments hold. As the
/// outermost call it stays with the dispatch below, which calls it too.
#[test]
fn a_function_entry_is_called_in_the_engine() {
  for (source, expected) in [
    ("['a', 'b'].map((c) => stylex.env.tint(c)).join()", "a!,b!"),
    ("['a'].map(() => env.tint()).join()", "none"),
    ("[env.tint('a')].join()", "a!"),
    ("env.tint('a')", "a!"),
    ("stylex.env.tint('a')", "a!"),
  ] {
    assert_eq!(folded(source), expected, "{source}");
  }
}

/// An argument with no value form is a throw, and the throw is the refusal.
#[test]
fn a_function_entry_refuses_an_argument_with_no_value_form() {
  let reason = refusal("['a'].map(() => env.tint(() => 1)).join()");

  assert!(reason.contains("function"), "got {reason}");
}

/// The values are measured as they are carried, so a value past the ceiling
/// refuses the fold.
#[test]
fn an_env_value_past_the_character_ceiling_refuses() {
  let mut env = IndexMap::default();

  env.insert(
    "long".into(),
    EnvEntry::Expr(create_string_expr("abcdefghij")),
  );

  let source = "['a'].map(() => env.long).join()";
  let result = evaluated_under(a_character_ceiling_of(4), env, a_function_fold(), source);

  assert_refused(&result, source);
}

/// A name the callback binds is the callback's, whatever the namespace holds.
#[test]
fn a_parameter_shadows_the_namespace() {
  assert_eq!(
    folded("[{ env: { color: 'x' } }].map((stylex) => stylex.env.color).join()"),
    "x"
  );
}
