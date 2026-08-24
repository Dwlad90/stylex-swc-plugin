//! Computed media query bounds carry the digits the official compiler emits.
//!
//! Regression coverage for
//! https://github.com/Dwlad90/stylex-swc-plugin/issues/1267. The
//! last-media-query-wins transform derives each range's upper bound as
//! `next - 0.01`, and that subtraction was being done — and stored — in single
//! precision, so `28.81 - 0.01` printed as `28.8` where the official compiler
//! prints `28.799999999999997`. The breakpoint landed at a different width, and
//! because the query string feeds the class hash, the class name differed too:
//! output from the two compilers could not be mixed across an SSR and client
//! boundary.
//!
//! Runtime injection is on so the query text sits beside the class name. Both
//! halves are pinned deliberately — a text-only assertion would pass through a
//! change that altered hashing, and a class name alone cannot show which query
//! produced it.
//!
//! Every expectation here was captured from `@stylexjs/babel-plugin@0.19.0`
//! run over the same source, not derived by reasoning about floating point.

use crate::utils::prelude::*;

// The reproduction from issue #1267, verbatim. Four breakpoints at fractional
// `rem` values: the two middle rules are the ones whose derived upper bound
// was wrong, and the first and last are here to show the chain around them is
// undisturbed.
stylex_test!(
  fractional_rem_breakpoints_derive_babels_upper_bounds,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        minHeight: {
          default: '100px',
          '@media (min-width: 25rem)': '200px',
          '@media (min-width: 28.81rem)': '300px',
          '@media (min-width: 32.88rem)': '400px',
        },
      },
    });
  "#
);

// A strict range query rewrites to a `min-`/`max-` pair by nudging each end by
// 0.01, and that nudge is computed at the same width. `400.5 + 0.01` is
// `400.51` and `900.25 - 0.01` is `900.24` in double precision; a single
// precision nudge moved both.
stylex_test!(
  strict_range_queries_nudge_in_double_precision,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { color: { default: 'red', '@media (width > 400.5px)': 'blue' } },
      b: { color: { default: 'red', '@media (400.5px < width < 900.25px)': 'blue' } },
    });
  "#
);

// A default and four breakpoints, none of them on a round number, so that every
// derived bound in the chain is asserted rather than only the ones that happen to
// survive single precision. Adding a fractional breakpoint must not silently
// move the output of its neighbours.
stylex_test!(
  every_bound_in_a_long_fractional_chain_matches,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        width: {
          default: '1px',
          '@media (min-width: 1.1rem)': '2px',
          '@media (min-width: 2.2rem)': '3px',
          '@media (min-width: 3.3rem)': '4px',
          '@media (min-width: 4.4rem)': '5px',
        },
      },
    });
  "#
);

// A breakpoint past the exponential threshold. The transform re-serializes the
// bound it derived rather than echoing the authored text, so this is the one
// end-to-end seam where the number's *spelling* is the compiler's own choice:
// the official compiler writes `1e+21px`, and Rust's default formatting wrote
// twenty-two digits. Confirmed against `@stylexjs/babel-plugin@0.19.0`, which
// emits `(min-width: 1e+21px) and (max-width: 2e+21px)` for this source.
stylex_test!(
  breakpoints_past_the_exponential_threshold_spell_the_bound_as_javascript_does,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        minHeight: {
          default: '0px',
          '@media (min-width: 1e21px)': '10px',
          '@media (min-width: 2e21px)': '20px',
        },
      },
    });
  "#
);

// A fractional aspect-ratio reaches the stylesheet at the width it was written.
//
// The same widening as the bounds above, one field over: a media fraction was
// held as two `i32`s, so `16.5/9` reprinted as `16 / 9` -- a different shape of
// screen -- and anything past `i32::MAX` saturated onto `2147483647`.
//
// At the transform level rather than only in the parser, because that is where
// the field's reachability is the claim: the transform reparses and reprints
// every `@media` key nested one level down, and it does so even where there is
// nothing to negate. So a fraction held at the wrong width did not stay inside
// the parser, and only a fixture carrying the emitted rule can show that.
//
// The second query is the saturating half of the same bug.
stylex_test!(
  a_fractional_aspect_ratio_reaches_the_stylesheet_intact,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: {
        color: {
          default: 'red',
          '@media (aspect-ratio: 16.5/9)': 'blue',
          '@media (aspect-ratio: 3000000000/1)': 'green',
        },
      },
    });
  "#
);
