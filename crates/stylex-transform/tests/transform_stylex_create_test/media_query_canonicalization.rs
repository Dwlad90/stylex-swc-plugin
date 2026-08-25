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

// The same ladder with ordering turned off, which is the documented way out of
// all of this: no rung is rewritten, so no contradictory branch is built and
// no wrapper appears. The authored spelling is what gets hashed.
//
// Asserted over the reported ladder rather than a two-query input, because
// opting out is only worth anything on the shape that would otherwise grow a
// wrapper -- a small input cannot tell a working opt-out from a rewrite that
// happened to be a no-op.
stylex_test!(
  a_disjoint_breakpoint_ladder_opted_out_hashes_the_authored_spelling,
  |tr| theme_import_transform_with(tr.comments.clone(), |b| {
    b.with_enable_media_query_order(false)
  }),
  LADDER_CODE
);

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

// Two entries of one conditional value map that canonicalize to the same query
// text, which retained contradictory branches are what make possible.
//
// The rewritten keys are written into a map rather than appended to a list, so
// the second entry to reach a key replaces the first entry's value and keeps
// that entry's position. One authored declaration is therefore absent from the
// output entirely — `red` here — and the rule count is four rather than five.
// That loss is faithful rather than incidental, and no diagnostic accompanies
// it, because the official compiler prints none.
//
// The ladder is chosen so the collision straddles a third key: `min-width:
// 200px` and `min-width: 300px` both contradict the trailing
// `min-width: 100px` and collapse to `not all`, while the `min-height` key
// between them survives on its own. That is what makes the surviving position
// observable — a collision between neighbours would land in the same place
// either way.
//
// Expectations are quoted from a run of `@stylexjs/babel-plugin@0.19.0`.
stylex_test_transform!(
  colliding_rewritten_keys_drop_a_declaration,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: {
          default: 'black',
          '@media (min-width: 200px)': 'red',
          '@media (min-height: 100px)': 'green',
          '@media (min-width: 300px)': 'blue',
          '@media (min-width: 100px)': 'purple',
        },
      },
    });
  "#,
  r#"
    import _inject from "@stylexjs/stylex/lib/stylex-inject";
    var _inject2 = _inject;
    import * as stylex from '@stylexjs/stylex';
    _inject2({
      ltr: ".x1mqxbix{color:black}",
      priority: 3000
    });
    _inject2({
      ltr: "@media not all{.x12vud9h.x12vud9h{color:blue}}",
      priority: 3200
    });
    _inject2({
      ltr: "@media (max-width: 99.99px) and (min-height: 100px){.xsllcrx.xsllcrx{color:green}}",
      priority: 3200
    });
    _inject2({
      ltr: "@media (min-width: 100px){.xr6za1w.xr6za1w{color:purple}}",
      priority: 3200
    });
    export const styles = {
      root: {
        kMwMTN: "x1mqxbix x12vud9h xsllcrx xr6za1w",
        $$css: true
      }
    };
  "#
);

// A rewritten media key beside other at-rules, and beside plain properties.
//
// At-rule sorting compares the final key text, and a rewritten key is much
// longer than the one an author wrote -- long enough that it could sort to a
// different place among its siblings than the authored spelling did. It does
// not. `@media not all` here is what `(min-width: 200px)` becomes once the
// later `(min-width: 100px)` is negated out of it, which is about as far from
// the authored text as a rewrite gets, and it still lands where it was.
//
// Plain properties on both sides pin the other half: a value map holding media
// keys does not migrate past the declarations around it.
//
// Quoted from a run of `@stylexjs/babel-plugin@0.19.0`, whose emitted order is
// identical rule for rule.
stylex_test_transform!(
  a_rewritten_media_key_sorts_where_the_authored_one_did,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        padding: '10px',
        color: {
          default: 'black',
          '@supports (display: grid)': 'green',
          '@media (min-width: 200px)': 'red',
          '@container (min-width: 400px)': 'teal',
          '@media (min-width: 100px)': 'blue',
        },
        margin: '2px',
      },
    });
  "#,
  r#"
    import _inject from "@stylexjs/stylex/lib/stylex-inject";
    var _inject2 = _inject;
    import * as stylex from '@stylexjs/stylex';
    _inject2({
      ltr: ".x7z7khe{padding:10px}",
      priority: 1000
    });
    _inject2({
      ltr: ".x1mqxbix{color:black}",
      priority: 3000
    });
    _inject2({
      ltr: "@supports (display: grid){.x19g4ih5.x19g4ih5{color:green}}",
      priority: 3030
    });
    _inject2({
      ltr: "@container (min-width: 400px){.x15pkjp4.x15pkjp4{color:teal}}",
      priority: 3300
    });
    _inject2({
      ltr: "@media not all{.x1jqaanj.x1jqaanj{color:red}}",
      priority: 3200
    });
    _inject2({
      ltr: "@media (min-width: 100px){.x18tmubq.x18tmubq{color:blue}}",
      priority: 3200
    });
    _inject2({
      ltr: ".xy3p2pi{margin:2px}",
      priority: 1000
    });
    export const styles = {
      root: {
        kmVPX3: "x7z7khe",
        kMwMTN: "x1mqxbix x19g4ih5 x15pkjp4 x1jqaanj x18tmubq",
        kogj98: "xy3p2pi",
        $$css: true
      }
    };
  "#
);
