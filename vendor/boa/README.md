# Vendored `boa`

The JavaScript engine the evaluator hands a static method call to, kept here as
source rather than pulled from crates.io.

- **Upstream:** <https://github.com/boa-dev/boa>, tag `v0.21.1`, commit recorded
  in `VENDORED_COMMIT`.
- **Licence:** MIT or Unlicense, at your option. Both texts are beside this
  file, unmodified.

## Why it is vendored

Published `boa_engine` 0.21.1 requires `icu_normalizer ~2.0.0`, and
`boa_parser` requires `icu_properties ~2.0.0`. `stylex_css` requires
`icu_collator 2.3.1`, which requires `icu_normalizer ~2.3.0` and
`icu_properties ~2.3.0`. Those are the same major version, so Cargo has to
resolve each to a single version and cannot — the engine simply does not go into
this workspace as published.

The alternatives were measured and this one was chosen:

- Pinning `icu_collator` back to `=2.0.0` resolves, but that version has no
  `unstable` feature, so `pre_rule.rs` loses `CollatorBorrowed::new_root` and
  gets back the uncoverable `Err` arm that file exists to explain.
- A third-party fork of the engine on crates.io resolves too, and makes a
  published compiler depend on one person's release of someone else's engine.
- Vendoring costs repo size and this file. Nothing else in the graph moves:
  `icu_collator` stays at 2.3.1 and every ICU crate resolves to one 2.3 version.

`.scratch/fix_runtime-sx-conditions/issues/05-spike-a-js-engine-as-the-parity-vehicle.md`
carries the measurements behind that choice.

## What was changed

Two dependency bounds in the workspace manifest, and nothing else:

```diff
-icu_properties = { version = "~2.0.0", default-features = true }
-icu_normalizer = { version = "~2.0.0", default-features = false }
+icu_properties = { version = ">=2.0.0, <3", default-features = true }
+icu_normalizer = { version = ">=2.0.0, <3", default-features = false }
```

The relaxation is permissive rather than forcing: upstream's own builds still
resolve those crates to 2.0, because boa's `intl` feature pins `icu_casemap`,
`icu_collator` and `icu_calendar` at `~2.0` and those hold the family down. This
workspace does not enable `intl`, so it takes the 2.3 line the rest of its graph
is already on. Verified: boa's own suite passes with the change in place
(1 292 tests), and this workspace's suite passes against the vendored engine.

`members` in the workspace manifest was also narrowed to the nine crates
actually vendored, since the upstream globs name directories that are not here.
No engine source file is modified.

## Bumping it

1. Check whether upstream has relaxed the bounds itself. If it has, delete this
   directory, drop the `[patch.crates-io]` section from the root `Cargo.toml`,
   and depend on the release.
2. Otherwise: clone the new tag, copy `core/{ast,engine,gc,interner,macros,parser,string}`
   and `utils/{small_btree,tag_ptr}` over this directory along with the licences
   and the workspace manifest, then reapply the two bound edits and the narrowed
   `members` list. Record the new commit in `VENDORED_COMMIT`.
3. Run `cargo test --workspace --all-features` and the paired benchmark. An
   engine bump can move a fold's result, and a fold's result is a class name.

Nothing here is formatted or linted by this repo's hooks — `lefthook.yml`
excludes `vendor/**` so a commit cannot rewrite upstream source — and
`cargo fmt --all` skips it because it is not a member of this workspace.
