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
//! `process::process_stylex_rules` is the assembly; `width_sort_key` holds the
//! order it sorts by: the media-query width key a rule's at-rule chain earns,
//! the comparator over such keys, and the declared property name the
//! comparator reads first.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod collation;
pub mod process;
pub mod width_sort_key;
