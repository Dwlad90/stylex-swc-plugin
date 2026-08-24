# 22 — The media-query transform is measured by the corpus

**What to build:** a regression in the computed media-query bound reports as a
changed verdict rather than as silence.

The corpus this effort is measured by carries **no** subject that exercises the
media-query transform. The eight `@media` rows in the module set use a media
query as a condition *key* — a place to hang a value — and never ask the
transform to derive a bound, merge an interval, or rewrite a range. So the fix
this whole spec exists for is pinned by the Rust suites alone, and the harness
whose job is to say what the reference compiler produces cannot see it move.

Four shapes were run through both compilers to check this is coverage rather
than a divergence, and all four read `identical`: the four-breakpoint
fractional-`rem` chain from the reported issue, a strict range query
(`width > 400.5px`), a container query at a fractional bound, and a `@supports`
condition. So the work is to record what already agrees, with `expected` set,
which is what makes a later disagreement loud.

A declaration entry cannot ask this — a derived bound needs several breakpoints
on one property — so these are module subjects.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] The reported four-breakpoint shape is a module subject recording
      `identical`, so the digits of every derived bound are pinned against the
      reference compiler rather than against this compiler's own output
- [x] Range syntax, a container query and a `@supports` condition each carry a
      subject, since each reaches the transform by a different path
- [x] At least one subject sits on a bound whose arithmetic is not exact in
      single precision, so a narrowing regression cannot pass
- [x] Every added subject's verdict was read from a run, never reasoned about
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Closing note

Delivered. Four module subjects reach the media-query transform, and all four
were read from a run rather than reasoned about: `modules-1267-fractional-rem-
breakpoint-chain` (the reported four-breakpoint shape),
`modules-1267-strict-range-query-nudged-bounds`,
`modules-1267-container-query-fractional-bound` and
`modules-1267-supports-condition-beside-a-derived-bound`. Every one records
`identical`.

The first carries the criterion about single precision on its own: its derived
bounds are `28.799999999999997rem` and `32.870000000000005rem`, neither of which
an `f32` can hold, so a narrowing regression cannot pass it.
