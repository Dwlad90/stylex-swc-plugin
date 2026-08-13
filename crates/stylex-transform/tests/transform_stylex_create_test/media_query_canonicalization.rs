//! Media query canonicalization.
//!
//! Regression coverage for
//! https://github.com/Dwlad90/stylex-swc-plugin/issues/1250, where authored
//! media queries were emitted verbatim instead of canonicalized: conditions
//! reordered width-first, spacing normalized after the colon, and range syntax
//! (`(width >= 1460px)`) rewritten to the `min-width` form. The emitted CSS was
//! valid either way, but the query string feeds the class hash, so equivalent
//! queries written differently produced different class names.
//!
//! The snapshots carry both halves of the contract: the canonical `@media`
//! strings and the class names they hash to. Runtime injection is on so the
//! rule text sits beside the class, which is what makes a rehash visible —
//! a class name alone cannot show which query produced it.
//!
//! Both tests compile the issue's exact input, so the two snapshots differ only
//! in what `enable_media_query_order` does to it. With it off the authored keys
//! pass through untouched, reproducing the verbatim output the issue reports
//! from 0.18.3 on purpose.

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
