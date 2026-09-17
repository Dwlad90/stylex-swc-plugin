//! Calls whose callee is not a name the compiler can read.
//!
//! A dynamic import and a call to the base class of a constructor are both
//! calls with no callee expression. Neither names a `stylex` API, so every
//! handler answers nothing and the module keeps the call exactly as written,
//! beside the calls that do compile.
//!
//! Every output below was measured against `@stylexjs/babel-plugin@0.19.0`
//! under the parity harness's options and agrees with it.

use crate::utils::prelude::*;

fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| b.with_runtime_injection())
}

// A dynamic import is a call with no callee expression.
stylex_test!(
  a_dynamic_import_beside_a_props_call_is_left_alone,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({ x: { color: 'red' } });
    export const load = () => import('./other.js');
    export const props = stylex.props(styles.x);
  "#
);

// A call to the base class of a constructor is the other one.
stylex_test!(
  a_base_class_call_beside_a_props_call_is_left_alone,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({ x: { color: 'red' } });
    export class Box extends Object {
      constructor() {
        super();
      }
    }
    export const props = stylex.props(styles.x);
  "#
);
