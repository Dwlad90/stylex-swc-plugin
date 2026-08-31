//! What a `defineVars` group behaves like once it is inside the engine.
//!
//! The fold's own tests read a stylesheet, which can only show what an
//! expression came to. These read the value itself: a group answers a member it
//! never stored, holds none of them, says what it is, and nests exactly where the
//! guard said it does. Each is asked the way the printed source would ask it —
//! by running JavaScript against the value — rather than through an accessor of
//! this module's own.

use super::*;

use boa_engine::{Context, JsValue, Source};

use stylex_state::theme_ref::{ThemeRef, VarNaming};

/// The group every case below reads, under the identity the transform tests use.
fn group(context: &mut Context, prefixes: &[&str]) -> JsValue {
  let builder = match compile_var_group(context) {
    Ok(builder) => builder,
    Err(_) => panic!("the theme group traps did not compile"),
  };

  let prefixes = prefixes.iter().map(|name| Atom::from(*name)).collect();

  let naming = VarNaming::from_flags(false, false);

  let theme = ThemeRef::new("vars.stylex.js", "vars", "x");

  match var_group(&builder, &theme, naming, Some(&prefixes), context) {
    Ok(value) => value,
    Err(error) => panic!("the group would not build: {}", error),
  }
}

/// What `source` — an arrow of one parameter — answers when it is handed the
/// group, as a string.
fn asked(source: &str, prefixes: &[&str]) -> String {
  let mut context = Context::default();
  let group = group(&mut context, prefixes);

  let asked = match context.eval(Source::from_bytes(source)) {
    Ok(asked) => asked,
    Err(error) => panic!("`{}` did not compile: {}", source, error),
  };

  let Some(asked) = asked.as_callable() else {
    panic!("`{}` is not a function", source);
  };

  let answered = match asked.call(&JsValue::undefined(), &[group], &mut context) {
    Ok(answered) => answered,
    Err(error) => panic!("`{}` threw: {}", source, error),
  };

  match answered.to_string(&mut context) {
    Ok(text) => text.to_std_string_escaped(),
    Err(error) => panic!("`{}` answered something unreadable: {}", source, error),
  }
}

/// A name nobody declared answers a variable all the same, which is what makes a
/// group a proxy rather than an object.
#[test]
fn a_member_nobody_declared_answers_a_variable() {
  assert_eq!(asked("(g) => g.primary", &[]), "var(--x1ineb92)");

  let unwritten = asked("(g) => g.anythingAtAll", &[]);

  assert!(
    unwritten.starts_with("var(--x") && unwritten != "var(--x1ineb92)",
    "expected a variable of its own for a name nobody declared, got `{}`",
    unwritten
  );
}

/// Two names answer two variables, and one name answers the same variable twice.
#[test]
fn a_name_decides_the_variable_and_nothing_else_does() {
  assert_eq!(
    asked("(g) => g.primary", &[]),
    asked("(g) => g['primary']", &[])
  );
  assert_ne!(
    asked("(g) => g.primary", &[]),
    asked("(g) => g.secondary", &[])
  );
}

/// The group asked for its own text answers the variable-group hash, however the
/// language asks for it.
#[test]
fn a_group_answers_its_own_hash_however_it_is_asked() {
  for source in [
    "(g) => g.toString()",
    "(g) => String(g)",
    "(g) => `${g}`",
    "(g) => [g].join('')",
    "(g) => g + ''",
  ] {
    assert_eq!(asked(source, &[]), "xop34xu", "asked by `{}`", source);
  }
}

/// A group is an object that holds nothing, so every question about what it
/// *has* answers empty while every read answers a variable.
#[test]
fn a_group_holds_none_of_the_names_it_answers() {
  assert_eq!(asked("(g) => typeof g", &[]), "object");
  assert_eq!(asked("(g) => Object.keys(g).length", &[]), "0");
  assert_eq!(asked("(g) => 'primary' in g", &[]), "false");
  assert_eq!(asked("(g) => JSON.stringify({...g})", &[]), "{}");
}

/// A key that is not a string is not a member name, so it answers nothing —
/// which is what keeps the language's own protocols working around the group.
#[test]
fn a_symbol_key_is_not_a_member() {
  assert_eq!(
    asked("(g) => g[Symbol.iterator] === undefined", &[]),
    "true"
  );
  assert_eq!(
    asked("(g) => g[Symbol.toPrimitive] === undefined", &[]),
    "true"
  );
}

/// The marker every reader of a group in this compiler asks for.
#[test]
fn a_group_says_what_it_is() {
  assert_eq!(asked("(g) => g.__IS_PROXY", &[]), "true");
}

/// A path the guard named nests: the first name answers a stand-in and the
/// second answers the variable the whole path names — one token, not a read of a
/// read.
#[test]
fn a_named_prefix_nests_and_the_last_name_answers() {
  assert_eq!(
    asked("(g) => g.brand.primary", &["brand"]),
    "var(--x1tr9ywo)"
  );
  assert_eq!(asked("(g) => g.brand.__IS_PROXY", &["brand"]), "true");
  assert_eq!(asked("(g) => typeof g.brand", &["brand"]), "object");
}

/// A stand-in answers its own path's variable when it is asked for text, so a
/// prefix used as a value reads exactly as the same name would without one.
#[test]
fn a_stand_in_answers_the_variable_its_own_path_names() {
  assert_eq!(
    asked("(g) => String(g.brand)", &["brand"]),
    asked("(g) => g.brand", &[])
  );
}

/// A prefix nobody named is an ordinary member, so the second name is read off
/// the variable's *text* — which is why the guard has to name them.
#[test]
fn an_unnamed_prefix_is_an_ordinary_member() {
  assert_eq!(asked("(g) => g.brand.primary", &[]), "undefined");
  assert_eq!(asked("(g) => g.brand.length", &[]), "15");
}

/// Nesting goes as deep as the guard named, and every level is its own token.
#[test]
fn nesting_goes_as_deep_as_the_paths_that_were_named() {
  let deep = &["a", "a.b", "a.b.c", "a.b.c.d"];

  assert_eq!(asked("(g) => g.a.b.c.d.e", deep), "var(--xuxm88k)");
  assert_ne!(asked("(g) => g.a.b", deep), asked("(g) => g.a.b.c", deep));

  // One level short of the path written: the last named prefix answers a
  // stand-in, the name after it answers a variable rather than a further one, and
  // the name after *that* is read off the variable's text.
  assert_eq!(asked("(g) => g.a.b.c.d", &["a", "a.b"]), "undefined");
}

/// A prefix is a path and not a name, so the same name at another level is
/// unaffected by it.
#[test]
fn a_prefix_names_a_path_rather_than_a_name() {
  assert_eq!(asked("(g) => typeof g.b", &["a.b"]), "string");
  assert_eq!(asked("(g) => typeof g.a.b", &["a", "a.b"]), "object");
}

/// A key an author spelled as a variable of their own is answered as written at
/// any level.
#[test]
fn a_key_spelled_as_a_variable_is_answered_as_written() {
  assert_eq!(asked("(g) => g['--custom']", &[]), "var(--custom)");
}

/// A group carries no state between reads, so reading a name a thousand times
/// answers the same variable a thousand times.
#[test]
fn a_group_answers_the_same_variable_however_often_it_is_read() {
  assert_eq!(
    asked(
      "(g) => new Set(Array.from({ length: 1000 }, () => g.primary)).size",
      &[]
    ),
    "1"
  );
}

/// A key long past anything an author writes is still one variable, because the
/// derivation hashes the key rather than carrying it.
#[test]
fn an_enormous_key_still_answers_one_variable() {
  assert_eq!(
    asked("(g) => g['k'.repeat(100000)].length < 32", &[]),
    "true"
  );
}
