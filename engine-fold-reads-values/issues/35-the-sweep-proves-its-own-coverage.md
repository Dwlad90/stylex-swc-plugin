# 35 — The sweep proves its own coverage

**What to build:** The generated prototype sweep fails when it stops sweeping,
so the harness that exists to prove coverage cannot report a green run over
nothing.

**It is the one new seam and the only one that proves the central claim** — that
the method nobody wrote down folds anyway. It genuinely crosses both receiver
shapes against both compilers, reads `getOwnPropertyNames` rather than a name
table, and reports unexercised methods rather than dropping them. Three things
stop it from being self-guarding.

*No floor.* The run fails only when `exercised.length === 0`. If `renderingFor`
regresses and ninety per cent of methods become `unexercised`, the sweep is
green. A recorded floor per surface is what turns the report into a gate.

*An unchecked allowlist.* `SURFACES` claims in its doc comment that "the
namespaces are exactly the compiler's `VALID_CALLEES`". They match today, and
nothing checks it — so a sixth callee added to the compiler is swept by nobody,
which is the precise failure mode the sweep exists to prevent. `lib/rust-source.ts`
already reads Rust sources, so the assertion has a mechanism waiting for it.

*Unstable output.* `byVerdict` and `byAccount` are serialised in Map-insertion
order while the console walks `ACCOUNTS` in declaration order — deliberately, for
stability. The JSON should sort too, or its diffs are noise.

**Blocked by:** none — can start immediately.

**Status:** resolved

- [x] A per-surface exercised floor is recorded and the sweep fails below it
- [x] A test parses `VALID_CALLEES` from the Rust source and asserts set equality
      with the sweep's namespace surfaces
- [x] `byVerdict` and `byAccount` are sorted on the way into JSON
- [x] The nightly wiring is unchanged — this stays a surface-change gate, not a
      per-push one

## Answer

**Each surface carries the count it was last seen to reach.** `Surface` splits
into what a call on it looks like and a `floor`, recorded from a real run:
`String.prototype` 50, `Array.prototype` 35, `Object.prototype` 7,
`Number.prototype` 7, `Boolean.prototype` 3, `Math` 35, `Object` 18, `Number` 6,
`String` 2, `Array` 3 — 166 of the 183 methods read off the language.
`shortfalls` answers which of the surfaces a pass covered fell below its own,
and the run exits 1 naming each with both numbers. It replaces the
`exercised.length === 0` check, which only fired when *every* surface fell
silent at once. Read per surface rather than over the total, because a total
hides the case worth catching: one prototype falling silent while the rest carry
the sum. The summary prints their sum beside the count on every run, so the gate
is visible without waiting for it to fail. Only swept surfaces are judged, so
`--surface Math` stays a narrower run rather than a failing one.

**The claim about `VALID_CALLEES` is now read rather than repeated.**
`phfSetMembers` reads a `phf_set!` out of Rust source — masked literals, matched
braces, the same approach the harvester's extractors use — and
`prototype-surface.test.ts` asserts set equality with the namespace surfaces
against `crates/stylex-constants/src/constants/common.rs` itself. A sixth callee
added there and not here now fails. What it reads is the *declaration* and only
that: a `use`, a call site or a comment naming the set, followed by somebody
else's `phf_set!`, would otherwise hand back the wrong list — an assertion that
loads, compares and passes while measuring another constant. The keyword in
front settles it, and it settles the short-name-inside-a-longer-one case with
it.

**The JSON is keyed in declaration order.** `byVerdict` follows `VERDICTS`,
`byAccount` and `accounted` follow `ACCOUNTS` — the order the summary already
walked, ordered once and read by both, so the two cannot disagree and neither
depends on which surfaces a run selected. A report now diffs against an earlier
one over what moved rather than over what was encountered first.

**The wiring did not move.** The sweep is still the one `pr-validation` step it
was; nothing under `.github/` changed.
