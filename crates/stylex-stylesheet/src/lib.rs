//! The StyleX stylesheet step: the rules a compile emits, assembled into one
//! sheet.
//!
//! A port of `processStylexRules` from `@stylexjs/babel-plugin`
//! (`packages/@stylexjs/babel-plugin/src/index.js`), function by function and
//! in its order, so that a change upstream maps onto one function here. The
//! commit the port follows is named in `CONTEXT.md` (the one `@stylexjs/babel-plugin`
//! 0.19.1 was published from); the output is the same string that version
//! writes, byte for byte. The matrix test in `tests/` holds the order against
//! the plugin's own answers.
//!
//! This crate holds the sort order of the sheet: the media-query width key a
//! rule's at-rule chain earns, the comparator over such keys, and the
//! declared property name the comparator reads first. The assembly itself,
//! `process_stylex_rules`, follows in the next change.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod collation;
pub mod width_sort_key;
