# 40 — Give the published musl target an allocator

**What to build:** `swc_malloc` chooses an allocator for six of the seven
targets `napi.targets` publishes. It declines every musl target at once, so
`x86_64-unknown-linux-musl` shipped on musl's `mallocng` with nobody deciding
that. The paired release gate priced it: the musl artifact read 1.13-1.53x
**slower** than the previous release while every glibc artifact read 0.33-0.82x
faster.

This is a performance change, which the spec puts out of scope — *"Any
behaviour change, bug fix, performance optimisation or idiomatic cleanup. If a
defect is noticed during a move, it is recorded and fixed separately."* The
maintainer's call was to take it here rather than defer it, because the split
is what surfaced the reading and the release that would ship it is the next
one. It is recorded here so the exception is visible rather than implied, which
is what the spec asks for.

mimalloc segfaults on ARM64 musl (microsoft/mimalloc#556), so exactly one musl
target is named and a second one is a decision to make rather than a line to
copy.

Landed as `68edbe2b0`, with no ticket at the time.

**Blocked by:** ticket 16, which dropped the `rlib` and let fat LTO run.

**Status:** resolved

- [x] `x86_64-unknown-linux-musl` links mimalloc; no other musl target does
- [x] The six targets `swc_malloc` answers for still link it
- [x] A check fails when the targets `lib.rs` declares and the targets
      `napi.targets` publishes drift apart
- [x] The workspace manifest and `CONTEXT.md` carry the measurement and the
      ARM64 reason
- [x] `npm.yml` loads and exercises the Alpine artifact on two Node versions
      before anything is published

## Comments

**The spec exception is deliberate and recorded.** Every earlier performance
exception carried a spec amendment; this one carries a ticket instead, which
answers the same question — where is the decision written down.

**The manifest check was rewritten in ticket 41.** It scanned `package.json`
for the first `"targets"` key, which answered about the way the manifest is
written rather than about the manifest, and used `.expect()` where
`guidelines/stack/RUST.md` asks for a `match`. It parses with `serde_json` now.
