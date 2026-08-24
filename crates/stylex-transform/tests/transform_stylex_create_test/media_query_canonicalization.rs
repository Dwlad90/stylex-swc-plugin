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

// A style-level `@media` key wrapping a block of properties, authored in a form
// canonicalization would rewrite: conditions height-first, no space after the
// colon, and a range query. The transform only rewrites keys nested at least one
// level below the style object, so both keys must survive verbatim — pinning
// that means the snapshot can tell pass-through apart from canonicalization,
// which a key that was already canonical cannot.
stylex_test!(
  style_level_media_keys_are_left_verbatim,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: {
        '@media (max-height:120px) and (min-width: 720px)': {
          display: 'block',
        },
      },
      b: {
        '@media (width >= 1460px)': {
          color: 'blue',
        },
      },
    });
  "#
);

stylex_test!(
  media_query_order_opt_out_keeps_queries_verbatim,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_enable_media_query_order(false)
      .with_runtime_injection()
  }),
  INPUT_CODE
);

/// The reproduction from issue #1268, verbatim: a ladder of exclusive
/// `min-width`/`max-width` breakpoints ending in a `max-width`-only rung, whose
/// values are variables defined in a separate module.
///
/// Every rung is disjoint from the next, so the negation chain
/// last-media-query-wins builds distributes into branches that all contradict.
/// A contradiction is kept rather than dropped: it prints as `not all`, and the
/// disjunction nesting around it stays in the key. The key text is what the
/// class name hashes, so the wrapper is not cosmetic — two of the seven class
/// names below depend on it.
///
/// The expected output is quoted from row `r01` of the ticket 02 divergence
/// table, a recorded run of `@stylexjs/babel-plugin@0.19.0`.
const LADDER_CODE: &str = r#"
    import * as stylex from '@stylexjs/stylex';
    import { colors } from 'colors.stylex.js';
    export const styles = stylex.create({
      root: {
        color: {
          default: colors.base,
          '@media (min-width: 1440px)': colors.xl,
          '@media (min-width: 1200px) and (max-width: 1439px)': colors.lg,
          '@media (min-width: 1024px) and (max-width: 1199px)': colors.md,
          '@media (min-width: 768px) and (max-width: 1023px)': colors.sm,
          '@media (min-width: 480px) and (max-width: 767px)': colors.xs,
          '@media (max-width: 479px)': colors.xxs,
        },
      },
    });
  "#;

stylex_test_transform!(
  a_disjoint_breakpoint_ladder_keeps_its_contradictory_branches,
  |tr| theme_import_transform(tr.comments.clone()),
  LADDER_CODE,
  r#"
    import _inject from "@stylexjs/stylex/lib/stylex-inject";
    var _inject2 = _inject;
    import "colors.stylex.js";
    import * as stylex from '@stylexjs/stylex';
    import { colors } from 'colors.stylex.js';
    _inject2({
      ltr: ".x1fm9ujy{color:var(--x1g366na)}",
      priority: 3000
    });
    _inject2({
      ltr: "@media ((not all) or (not all)) or ((not all) or ((min-width: 1440px))){.x11g08g8.x11g08g8{color:var(--x1gey9a0)}}",
      priority: 3200
    });
    _inject2({
      ltr: "@media (not all) or ((min-width: 1200px) and (max-width: 1439px)){.x1qsezja.x1qsezja{color:var(--xo0k5im)}}",
      priority: 3200
    });
    _inject2({
      ltr: "@media (min-width: 1024px) and (max-width: 1199px){.xqw8h0p.xqw8h0p{color:var(--x1ncemq0)}}",
      priority: 3200
    });
    _inject2({
      ltr: "@media (min-width: 768px) and (max-width: 1023px){.x1bhj7sf.x1bhj7sf{color:var(--x5eudzp)}}",
      priority: 3200
    });
    _inject2({
      ltr: "@media (min-width: 480px) and (max-width: 767px){.x1p9ejzw.x1p9ejzw{color:var(--xtqmhjj)}}",
      priority: 3200
    });
    _inject2({
      ltr: "@media (max-width: 479px){.xw70vyp.xw70vyp{color:var(--xuk8yok)}}",
      priority: 3200
    });
    export const styles = {
      root: {
        kMwMTN: "x1fm9ujy x11g08g8 x1qsezja xqw8h0p x1bhj7sf x1p9ejzw xw70vyp",
        $$css: true
      }
    };
  "#
);
