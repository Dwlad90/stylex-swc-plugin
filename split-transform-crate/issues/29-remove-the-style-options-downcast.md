# 29 — Remove the style-options downcast escape hatch

**What to build:** Two separate traits stand in for the state manager to avoid
a dependency cycle, and one of them is routinely defeated. A public helper
downcasts a style-options handle straight back to the concrete state manager,
and ten call sites across two crates use it. The trait it defeats has exactly
one implementer and is immediately downcast, with a comment observing that
every handle the compiler builds is that one type — so the abstraction buys
nothing and the helper is a public item whose only purpose is to cross a
boundary and cancel it.

Pick one inversion and commit to it: either widen the options trait so it
answers what those ten call sites actually need, or let them take the state
manager directly and drop the trait from that path. This is a design-level
change to the crate graph — write the approach down and get it agreed before
editing, and keep the hottest path free of new indirection.

**Approach (agreed):** Drop the trait from that path.

The three function-pointer types that spell `&mut dyn StyleOptions` --
`StylexExprFn`, `StylexWhenFn` and `FunctionType::ArrayArgs` -- live in
`stylex-state/src/functions.rs`, the same crate as `StateManager`, and every
consumer (`stylex-evaluator`, `stylex-transform`) depends on that crate. So the
trait crosses no boundary that still exists; it is a leftover from before the
split. Of its six methods only `css_property_seen`/`_mut` had a caller:
`options` and `other_injected_css_rules`/`_mut` had none, because the field is
public and read directly, and `as_any_mut` existed only to feed the downcast.

The three signatures now take `&mut StateManager`. The downcast helper, the
`impl StyleOptions for StateManager` block and the `StyleOptions` trait are
gone; `css_property_seen`/`_mut` stay as inherent methods, because the map is a
private field of the cache. `WhenMarkerValue` stays -- it crosses a real
boundary, since `stylex-css` sits below the evaluator and cannot name the
evaluated-value types.

**Blocked by:** 21

**Status:** resolved

- [x] Either the options trait answers what the call sites need, or they take
      compilation state directly and the trait leaves that path
- [x] The downcast helper is gone, and nothing re-introduces an equivalent
- [x] No new dynamic dispatch lands on the evaluation path
- [x] Benches confirm the fold is unchanged
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
- [x] The addon is rebuilt and the JavaScript suite re-run — it exercises the
      built artifact rather than the Rust sources, so a green Rust run is not
      evidence on its own

**Result:** The three signatures take `&mut StateManager`. `StyleOptions`, its
impl and `downcast_style_options_to_state_manager` are deleted, and a grep of
`crates/` finds no reference to any of them. The fold path lost three vtable
calls and three downcasts and gained nothing, so no dynamic dispatch landed on
it. `engine_fold_bench` moves between -1.5% and +1.3% across ten measurements,
which is the band a control leg produces --
[bench/ticket-29.md](../bench/ticket-29.md).

`rustc-hash` left `stylex-types`, whose last use was the removed trait.
