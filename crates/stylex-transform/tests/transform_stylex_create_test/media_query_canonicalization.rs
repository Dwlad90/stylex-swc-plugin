//! Media query canonicalization parity with `@stylexjs/babel-plugin`.
//!
//! Regression coverage for
//! https://github.com/Dwlad90/stylex-swc-plugin/issues/1250, where authored
//! media queries were emitted verbatim instead of canonicalized: conditions
//! reordered width-first, spacing normalized after the colon, and range syntax
//! (`(width >= 1460px)`) rewritten to the `min-width` form. The emitted CSS was
//! valid either way, but the class names diverged from the Babel plugin's, so
//! the two compilers could not be mixed across SSR and client.
//!
//! Expectations here are inline rather than snapshots: the canonical query
//! strings and the class hashes are the contract with
//! `@stylexjs/babel-plugin@0.19.0`, and a snapshot regeneration must not be
//! able to rewrite them.
//!
//! Both tests compile the issue's exact input, so the two expectations differ
//! only in what `enable_media_query_order` does to it. The opt-out's hashes are
//! the ones the issue reports from 0.18.3 — canonicalization off reproduces the
//! authored-verbatim output on purpose, matching the Babel plugin's opt-out.

use crate::utils::prelude::*;

/// The reproduction from issue #1250, verbatim.
const INPUT_CODE: &str = r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: {
        display: {
          default: 'none',
          '@media (max-height:120px) and (min-width: 720px)': 'block',
        },
      },
      b: {
        color: {
          default: 'red',
          '@media (width >= 1460px)': 'blue',
        },
      },
    });
  "#;

stylex_test!(authored_media_queries_are_canonicalized, INPUT_CODE);

stylex_test!(
  media_query_order_opt_out_keeps_queries_verbatim,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_enable_media_query_order(false)
      .with_runtime_injection()
  }),
  INPUT_CODE
);
