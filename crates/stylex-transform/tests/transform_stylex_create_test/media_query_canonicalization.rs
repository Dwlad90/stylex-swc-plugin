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
const ISSUE_1250_INPUT: &str = r#"
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

stylex_test_transform!(
  authored_media_queries_are_canonicalized,
  ISSUE_1250_INPUT,
  r#"
    import _inject from "@stylexjs/stylex/lib/stylex-inject";
    var _inject2 = _inject;
    import * as stylex from '@stylexjs/stylex';
    _inject2({
      ltr: ".x1s85apg{display:none}",
      priority: 3000
    });
    _inject2({
      ltr: "@media (min-width: 720px) and (max-height: 120px){.x1gcnmh1.x1gcnmh1{display:block}}",
      priority: 3200
    });
    _inject2({
      ltr: ".x1e2nbdu{color:red}",
      priority: 3000
    });
    _inject2({
      ltr: "@media (min-width: 1460px){.xju9v9y.xju9v9y{color:blue}}",
      priority: 3200
    });
    export const styles = {
      a: {
        k1xSpc: "x1s85apg x1gcnmh1",
        $$css: true
      },
      b: {
        kMwMTN: "x1e2nbdu xju9v9y",
        $$css: true
      }
    };
  "#
);

stylex_test_transform!(
  media_query_order_opt_out_keeps_queries_verbatim,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_enable_media_query_order(false)
      .with_runtime_injection()
  }),
  ISSUE_1250_INPUT,
  r#"
    import _inject from "@stylexjs/stylex/lib/stylex-inject";
    var _inject2 = _inject;
    import * as stylex from '@stylexjs/stylex';
    _inject2({
      ltr: ".x1s85apg{display:none}",
      priority: 3000
    });
    _inject2({
      ltr: "@media (max-height:120px) and (min-width: 720px){.x4ob7n2.x4ob7n2{display:block}}",
      priority: 3200
    });
    _inject2({
      ltr: ".x1e2nbdu{color:red}",
      priority: 3000
    });
    _inject2({
      ltr: "@media (width >= 1460px){.xy2bn39.xy2bn39{color:blue}}",
      priority: 3200
    });
    export const styles = {
      a: {
        k1xSpc: "x1s85apg x4ob7n2",
        $$css: true
      },
      b: {
        kMwMTN: "x1e2nbdu xy2bn39",
        $$css: true
      }
    };
  "#
);
