//! A key the `env` option does not carry, and an `env` option nobody set.
//!
//! An unset option is an empty object, and a key an object does not carry reads
//! as `undefined`. So a fallback written beside the read applies, through the
//! namespace and through a named import alike.

use super::brand_env;
use crate::utils::prelude::*;

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
  a_fallback_applies_when_the_env_option_is_not_set,
  |tr| stylex_transform(tr.comments.clone(), None),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { env } from '@stylexjs/stylex';
    export const styles = stylex.create({
      namespace: { color: stylex.env.theme ?? 'red' },
      named: { color: env.theme ?? 'red' },
    });
  "#
);

// `.x1e2nbdu{color:red}`
stylex_test!(
  a_fallback_applies_for_a_key_the_env_option_has_not_got,
  |tr| stylex_transform(tr.comments.clone(), Some(brand_env())),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { color: stylex.env.brandSecondary ?? 'red' },
    });
  "#
);
