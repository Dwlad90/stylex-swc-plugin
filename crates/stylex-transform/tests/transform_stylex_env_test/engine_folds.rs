//! `env` read inside a method call that the engine folds, in a callback and
//! outside one.
//!
//! The `env` object crosses into the engine as an object of its entries,
//! through the namespace and through a named import. A key the option does not
//! carry reads as `undefined`, and a function entry is called with the values
//! its arguments hold.

use crate::utils::ast::convert_expr_to_str_wrapper;
use crate::utils::prelude::*;
use stylex_ast::ast::convertors::{create_number_expr, create_string_expr};
use stylex_ast::ast::factories::{create_key_value_prop, create_object_lit};
use stylex_structures::stylex_env::JSFunction;
use swc_core::ecma::ast::Expr;

/// The option the cases that set one read.
fn an_env() -> IndexMap<String, EnvEntry> {
  let mut env = IndexMap::new();

  env.insert(
    "color".to_string(),
    EnvEntry::Expr(create_string_expr("blue")),
  );
  env.insert("n".to_string(), EnvEntry::Expr(create_number_expr(3.0)));
  env.insert(
    "theme".to_string(),
    EnvEntry::Expr(Expr::Object(create_object_lit(vec![
      create_key_value_prop("fg", create_string_expr("green")),
    ]))),
  );
  env.insert(
    "tint".to_string(),
    EnvEntry::Function(JSFunction::new(|args: Vec<Expr>| {
      let color = args
        .first()
        .and_then(convert_expr_to_str_wrapper)
        .unwrap_or_default();

      create_string_expr(&format!("light-dark({}, white)", color))
    })),
  );

  env
}

fn stylex_transform(comments: TestComments, env: Option<IndexMap<String, EnvEntry>>) -> impl Pass {
  build_test_transform(comments, |b| {
    let b = b.with_runtime_injection();

    match env {
      Some(env) => b.with_env(env),
      None => b,
    }
  })
}

// `.x1e2nbdu{color:red}`
stylex_test!(
  a_fallback_applies_in_a_callback_when_the_option_is_not_set,
  |tr| stylex_transform(tr.comments.clone(), None),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { env } from '@stylexjs/stylex';
    export const styles = stylex.create({
      namespace: { color: ['a'].map(() => stylex.env.color ?? 'red')[0] },
      named: { color: ['a'].map(() => env.color ?? 'red')[0] },
    });
  "#
);

// `.xju2f9n{color:blue}`
stylex_test!(
  an_env_value_is_read_in_a_callback,
  |tr| stylex_transform(tr.comments.clone(), Some(an_env())),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { env } from '@stylexjs/stylex';
    export const styles = stylex.create({
      namespace: { color: ['a'].map(() => stylex.env.color ?? 'red')[0] },
      named: { color: ['a'].map(() => env.color ?? 'red')[0] },
    });
  "#
);

// `@keyframes xl7tn2a-B{from{color:blue;}}`, `.x4xwbg3{animation-name:xl7tn2a-B}`
stylex_test!(
  an_env_value_is_read_in_a_keyframes_call_in_a_callback,
  |tr| stylex_transform(tr.comments.clone(), Some(an_env())),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: ['a']
          .map(() => stylex.keyframes({ from: { color: stylex.env.color ?? 'red' } }))
          .join(),
      },
    });
  "#
);

// `.x57vt4w{font-family:a,}`, `.x16cqjx{font-family:a,b}`,
// `.x1xeqiy{font-family:aundefined}`
stylex_test!(
  a_missing_key_reads_as_undefined_in_a_method_call,
  |tr| stylex_transform(tr.comments.clone(), None),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      element: { fontFamily: ['a', stylex.env.nope].join() },
      separator: { fontFamily: ['a', 'b'].join(stylex.env.nope) },
      argument: { fontFamily: 'a'.concat(stylex.env.nope) },
    });
  "#
);

// `.x1prwzq3{color:green}`, `.xyf84xx{color:color,n,theme,tint}`
stylex_test!(
  an_env_object_value_and_the_keys_are_read_in_a_callback,
  |tr| stylex_transform(tr.comments.clone(), Some(an_env())),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      nested: { color: ['a'].map(() => stylex.env.theme.fg)[0] },
      keys: { color: ['a'].map(() => Object.keys(stylex.env).join())[0] },
    });
  "#
);

// `.x1a63eun{color:light-dark(red,white) light-dark(blue,white)}`,
// `.x11hcj94{color:light-dark(red,white)}`
stylex_test!(
  an_env_function_is_called_in_a_callback,
  |tr| stylex_transform(tr.comments.clone(), Some(an_env())),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { env } from '@stylexjs/stylex';
    export const styles = stylex.create({
      namespace: { color: ['red', 'blue'].map((c) => stylex.env.tint(c)).join(' ') },
      named: { color: ['red'].map((c) => env.tint(c))[0] },
      outside: { color: [stylex.env.tint('red')].join() },
    });
  "#
);
