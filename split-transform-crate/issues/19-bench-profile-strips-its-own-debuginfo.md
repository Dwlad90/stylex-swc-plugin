# 19 — The `bench` profile strips the debuginfo it asks for

**What to build:** `[profile.bench]` in the root `Cargo.toml` sets
`debug = true`. Cargo's `bench` profile inherits from `release`, and this
workspace customises `release` with `strip = "symbols"`. So a bench target
generates full debuginfo and then has it stripped: `debuginfo=2` and
`strip=symbols` reach the same unit. The `debug = true` buys nothing -- a
flamegraph over a bench binary gets no symbols from it -- and it costs build
time and disk on every bench build.

Decide which one is wrong and remove the contradiction:

- If bench profiling is meant to work, add `strip = "none"` to
  `[profile.bench]`. `pnpm run test:flamegraph` and `test:profile` exist in
  several crates, which suggests it is.
- If it is not, drop `debug = true` and say why in a comment.

**Where this came from.** Found by the performance review on
[ticket 16](./16-measure-the-crate-type-change.md), while checking whether the
`rlib`/LTO trap that ticket found in `profile.release` also applied to
`profile.bench`. **It does not** -- a criterion bench binary is a final
artifact, so it gets `-C lto` and the rlib-only crates under it get
`-C linker-plugin-lto`. Every criterion bench in this repo has been built with
fat LTO, so no bench number in this effort needs re-reading. The `strip`
contradiction is the separate thing the same check turned up.

**Blocked by:** None.

**Status:** ready-for-human

- [x] `[profile.bench]` no longer both generates and strips debuginfo.
      `strip = "none"` cancels what the profile inherits from `release`.
- [x] A comment records which behaviour is intended.
- [x] Symbols are confirmed present, by counting them rather than by reading the
      flags: a `stylex_styleq` bench binary carries **0** symbols matching
      `stylex` with the inherited strip and **95** without it. The binary grows
      from 2,110,928 to 2,723,248 bytes, which costs disk on a target nobody
      ships.

## Outcome

`strip = "none"` was the answer, because `debug = true` states the intent and
the strip was inherited by accident: cargo derives `bench` from `release`, and
`release` strips symbols so the published `.node` stays small.

Two notes for whoever profiles a bench next. The `test:flamegraph` and
`test:profile` scripts run `cargo flamegraph --test`, so they build **test**
targets, not bench targets; this fix does not change them. And a bench target
built under `profile.bench` lands in `target/release/deps/`, not
`target/bench/`, because cargo names the directory after the profile it
inherits from.
