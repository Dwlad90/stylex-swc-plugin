//! The nesting order of at-rule keys that are constants.
//!
//! A constant from a `defineConsts` module is read as `var(--hash)`. Used as a
//! key, it is a condition like an at-rule, and the rule text wraps the
//! declaration in it. The reference wraps the constant keys in the order of the
//! key path: the first key is the innermost wrapper and the last key is the
//! outermost wrapper. The keys are not sorted for the rule text. They are
//! sorted only for the class name hash, so the class name does not change with
//! the order.
//!
//! The processed sheet replaces each wrapper with the at-rule of the constant,
//! so a wrong order puts `@starting-style` outside `@media`, for example.
//!
//! Every rule text below is the output of `@stylexjs/babel-plugin` 0.19.2 for
//! the same module, file name and haste module resolution.

use crate::utils::{
  prelude::*,
  transform::{stringify_js, theme_import_transform, ts_syntax},
};

const CONSTANTS_IMPORT: &str = "import { c } from 'c.stylex.js';";

/// The module that `body` gives, compiled with the constants import.
fn compiled_with_constants(body: &str) -> String {
  let input = format!(
    "import * as stylex from '@stylexjs/stylex';\n{}\n{}",
    CONSTANTS_IMPORT, body
  );

  stringify_js(&input, ts_syntax(), |tr| {
    theme_import_transform(tr.comments.clone())
  })
}

/// Asserts that the compiled module injects every rule in `rules`.
#[track_caller]
fn assert_rules(body: &str, rules: &[&str]) {
  let output = compiled_with_constants(body);

  for rule in rules {
    assert!(
      output.contains(&format!("ltr: \"{}\"", rule)),
      "the output does not inject the rule {}:\n{}",
      rule,
      output
    );
  }
}

// The constants the cases read, and the variable each one is read as. The
// transform does not read `c.stylex.js`; the at-rule column shows the module
// the reference comparison used, and it is the at-rule the processed sheet
// puts in place of the variable.
//
// | key   | at-rule                       | variable          |
// | ----- | ----------------------------- | ----------------- |
// | `c.k` | `@starting-style`             | `var(--x1blk1k2)` |
// | `c.m` | `@media (min-width: 5px)`     | `var(--x19oe3oe)` |
// | `c.s` | `@supports (display: grid)`   | `var(--xxc0iyh)`  |
// | `c.q` | `@container (min-width: 7px)` | `var(--x11c7tph)` |
//
// In some cases the variable names sort in a different order from the key
// path, so a sort of the constant keys changes the rule text. In the other
// cases the two orders are the same. Both kinds are here, so that each
// direction of the order is tested.

// ── Two constant keys ───────────────────────────────────────────────

// The reported shape. The inner key `c.m` is the outer wrapper.
#[test]
fn starting_style_key_holding_a_media_key_wraps_in_the_media_key() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.k]:{default:'b',[c.m]:'g'}}}});",
    &[
      "var(--x1blk1k2){.xtcmlow.xtcmlow{color:b}}",
      "var(--x19oe3oe){var(--x1blk1k2){.xhmyawn.xhmyawn.xhmyawn{color:g}}}",
    ],
  );
}

#[test]
fn media_key_holding_a_starting_style_key_wraps_in_the_starting_style_key() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.m]:{default:'b',[c.k]:'g'}}}});",
    &[
      "var(--x19oe3oe){.xrde52e.xrde52e{color:b}}",
      "var(--x1blk1k2){var(--x19oe3oe){.xhmyawn.xhmyawn.xhmyawn{color:g}}}",
    ],
  );
}

#[test]
fn media_key_holding_a_supports_key_wraps_in_the_supports_key() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.m]:{default:'b',[c.s]:'g'}}}});",
    &[
      "var(--x19oe3oe){.xrde52e.xrde52e{color:b}}",
      "var(--xxc0iyh){var(--x19oe3oe){.xz3y41g.xz3y41g.xz3y41g{color:g}}}",
    ],
  );
}

#[test]
fn supports_key_holding_a_media_key_wraps_in_the_media_key() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.s]:{default:'b',[c.m]:'g'}}}});",
    &[
      "var(--xxc0iyh){.x1tj9391.x1tj9391{color:b}}",
      "var(--x19oe3oe){var(--xxc0iyh){.xz3y41g.xz3y41g.xz3y41g{color:g}}}",
    ],
  );
}

#[test]
fn media_key_holding_a_container_key_wraps_in_the_container_key() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.m]:{default:'b',[c.q]:'g'}}}});",
    &[
      "var(--x19oe3oe){.xrde52e.xrde52e{color:b}}",
      "var(--x11c7tph){var(--x19oe3oe){.x1mus6g4.x1mus6g4.x1mus6g4{color:g}}}",
    ],
  );
}

#[test]
fn container_key_holding_a_media_key_wraps_in_the_media_key() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.q]:{default:'b',[c.m]:'g'}}}});",
    &[
      "var(--x11c7tph){.x1x9hykm.x1x9hykm{color:b}}",
      "var(--x19oe3oe){var(--x11c7tph){.x1mus6g4.x1mus6g4.x1mus6g4{color:g}}}",
    ],
  );
}

// ── Three constant keys ─────────────────────────────────────────────

// The leaf rule has three wrappers, the reverse of the key path.
#[test]
fn three_constant_keys_wrap_with_the_innermost_key_outside() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.k]:{default:'b',[c.m]:{default:'g',[c.s]:'y'}}}}});",
    &[
      "var(--x1blk1k2){.xtcmlow.xtcmlow{color:b}}",
      "var(--x19oe3oe){var(--x1blk1k2){.xhmyawn.xhmyawn.xhmyawn{color:g}}}",
      "var(--xxc0iyh){var(--x19oe3oe){var(--x1blk1k2){.xc0vjxp.xc0vjxp.xc0vjxp.xc0vjxp{color:y}}}}",
    ],
  );
}

#[test]
fn three_constant_keys_in_reverse_wrap_with_the_innermost_key_outside() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.s]:{default:'b',[c.m]:{default:'g',[c.k]:'y'}}}}});",
    &[
      "var(--xxc0iyh){.x1tj9391.x1tj9391{color:b}}",
      "var(--x19oe3oe){var(--xxc0iyh){.xz3y41g.xz3y41g.xz3y41g{color:g}}}",
      "var(--x1blk1k2){var(--x19oe3oe){var(--xxc0iyh){.xc0vjxp.xc0vjxp.xc0vjxp.xc0vjxp{color:y}}}}",
    ],
  );
}

#[test]
fn three_constant_keys_in_mixed_order_wrap_with_the_innermost_key_outside() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.m]:{default:'b',[c.k]:{default:'g',[c.s]:'y'}}}}});",
    &[
      "var(--x19oe3oe){.xrde52e.xrde52e{color:b}}",
      "var(--x1blk1k2){var(--x19oe3oe){.xhmyawn.xhmyawn.xhmyawn{color:g}}}",
      "var(--xxc0iyh){var(--x1blk1k2){var(--x19oe3oe){.xc0vjxp.xc0vjxp.xc0vjxp.xc0vjxp{color:y}}}}",
    ],
  );
}

// ── Constant keys with plain at-rules ───────────────────────────────

// A plain at-rule is always inside every constant wrapper, wherever the key
// path has it.
#[test]
fn a_plain_at_rule_outside_constant_keys_is_the_innermost_wrapper() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red','@media (min-width: 9px)':{default:'b',[c.k]:{default:'g',[c.m]:'y'}}}}});",
    &[
      "var(--x1blk1k2){@media (min-width: 9px){.x80peqf.x80peqf.x80peqf{color:g}}}",
      "var(--x19oe3oe){var(--x1blk1k2){@media (min-width: 9px){.xwnwndm.xwnwndm.xwnwndm.xwnwndm{color:y}}}}",
    ],
  );
}

#[test]
fn a_plain_at_rule_between_constant_keys_is_the_innermost_wrapper() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.k]:{default:'b','@media (min-width: 9px)':{default:'g',[c.m]:'y'}}}}});",
    &[
      "var(--x1blk1k2){.xtcmlow.xtcmlow{color:b}}",
      "var(--x1blk1k2){@media (min-width: 9px){.x80peqf.x80peqf.x80peqf{color:g}}}",
      "var(--x19oe3oe){var(--x1blk1k2){@media (min-width: 9px){.xwnwndm.xwnwndm.xwnwndm.xwnwndm{color:y}}}}",
    ],
  );
}

#[test]
fn a_plain_at_rule_inside_constant_keys_is_the_innermost_wrapper() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.m]:{default:'b',[c.k]:{default:'g','@supports (gap: 1px)':'y'}}}}});",
    &[
      "var(--x19oe3oe){.xrde52e.xrde52e{color:b}}",
      "var(--x1blk1k2){var(--x19oe3oe){.xhmyawn.xhmyawn.xhmyawn{color:g}}}",
      "var(--x1blk1k2){var(--x19oe3oe){@supports (gap: 1px){.x1rjx5hm.x1rjx5hm.x1rjx5hm.x1rjx5hm{color:y}}}}",
    ],
  );
}

// ── Constant keys with pseudo-classes ───────────────────────────────

// A pseudo-class goes in the selector and does not change the wrapper order.
#[test]
fn a_pseudo_class_outside_constant_keys_stays_in_the_selector() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',':hover':{default:'b',[c.k]:{default:'g',[c.m]:'y'}}}}});",
    &[
      "var(--x1blk1k2){.x4o0mzc.x4o0mzc:hover{color:g}}",
      "var(--x19oe3oe){var(--x1blk1k2){.xnzkc1t.xnzkc1t.xnzkc1t:hover{color:y}}}",
    ],
  );
}

#[test]
fn a_pseudo_class_between_constant_keys_stays_in_the_selector() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.k]:{default:'b',':hover':{default:'g',[c.m]:'y'}}}}});",
    &[
      "var(--x1blk1k2){.xtcmlow.xtcmlow{color:b}}",
      "var(--x1blk1k2){.x4o0mzc.x4o0mzc:hover{color:g}}",
      "var(--x19oe3oe){var(--x1blk1k2){.xnzkc1t.xnzkc1t.xnzkc1t:hover{color:y}}}",
    ],
  );
}

#[test]
fn a_pseudo_class_inside_constant_keys_stays_in_the_selector() {
  assert_rules(
    "export const s = stylex.create({a:{color:{default:'red',[c.m]:{default:'b',[c.k]:{default:'g',':focus':'y'}}}}});",
    &[
      "var(--x19oe3oe){.xrde52e.xrde52e{color:b}}",
      "var(--x1blk1k2){var(--x19oe3oe){.xhmyawn.xhmyawn.xhmyawn{color:g}}}",
      "var(--x1blk1k2){var(--x19oe3oe){.x1punq2n.x1punq2n.x1punq2n:focus{color:y}}}",
    ],
  );
}
