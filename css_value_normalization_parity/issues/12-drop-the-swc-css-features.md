# 12 — Drop the swc CSS features

**What to build:** Remove CSS parsing, code generation, AST and visitor support
from the compiler's dependency configuration. After tickets 07 through 09,
nothing in the workspace uses any of them — the whole CSS half of that
dependency is compiled and linked for no reason.

This is the last ticket for a reason beyond tidiness: it is the **proof the
migration is complete**. If the workspace builds and every suite passes with CSS
support switched off, then nothing is quietly still on the old path. There is no
other check that gives that guarantee — a stray call site would otherwise sit
there compiling happily and reading as intentional.

Splitting this to a follow-up was considered and rejected. A dependency that is
declared but unused is indistinguishable, to the next reader, from one that is
load-bearing.

The dependency itself stays — the compiler still needs it for JavaScript
parsing. Only the CSS feature set comes off.

**Blocked by:** 08 — Move custom-property validation onto the value AST; 09 —
Delete the dead normalizer modules.

**Status:** resolved

- [x] The CSS parsing, code generation, AST and visitor features are removed
      from the crate's dependency configuration
- [x] Any other crate in the workspace that enabled them for this pipeline has
      them removed too
- [x] The workspace builds clean with them gone — this is the acceptance signal
      for the whole effort
- [x] The full test suite passes, including the JavaScript suite against a
      rebuilt native artifact
- [x] The JavaScript-parsing capability of the same dependency is untouched
- [x] Build time and artifact size are compared before and after, and recorded
      on this ticket alongside the ticket 11 numbers
- [x] The harness reports no divergence across the full corpus

## Answer

`css_ast`, `css_codegen`, `css_parser` and `css_visit` are off the three crates
that still declared them — `stylex-rs-compiler`, `stylex-transform` and
`stylex-css-parser`. `stylex-css` had already lost its whole swc dependency in
ticket 09, so the sweep is complete: no crate in the workspace asks swc for CSS
any more. The `ecma_*` features every one of them uses are untouched, and the
lockfile drops the six `swc_css_*` crates along with the transitive
dependencies they alone pulled in.

`stylex-css-parser` was in the list on paper only. It is parser combinators over
CSS *values* and never used swc's CSS support; it reads `swc_core::ecma` for the
object-literal types its media-query transform returns. The four features were
copied into it and had no callers, which is exactly the shape this ticket exists
to remove — a declared dependency reads as load-bearing to the next person.

**Nothing broke, which is the point.** The workspace builds with CSS support
switched off, so no call site was quietly still on the old path.

- `cargo test --workspace`: 5925 passed, 0 failed, 54 ignored, across 27 suites.
- `pnpm test` against a freshly built `dist/*.node`: 64 of 64 tasks successful.
- `pnpm run --filter=@stylexswc/rs-compiler parity`: **0 divergent** over 744
  declarations — 671 identical, 15 structurally divergent, 31 acceptance
  divergent, 27 both-reject, all unchanged from ticket 09's run.

## Build time and artifact size

Clean `cargo build --release -p stylex_compiler_rs` into an empty target
directory, same machine (M-series, macOS), two runs each, alternating.

| Measure               | Before               | After                | Delta          |
| --------------------- | -------------------- | -------------------- | -------------- |
| Crates compiled       | 340                  | 326                  | −14            |
| Wall clock            | 101.7 s / 110.7 s    | 119.6 s / 109.2 s    | within noise   |
| User CPU              | 369.2 s / 370.7 s    | 378.9 s / 357.0 s    | within noise   |
| `libstylex_compiler_rs.dylib` | 9 711 168 B  | 9 711 168 B          | 0 B            |
| `libstylex_compiler_rs.rlib`  | 12 122 448 B | 12 121 432 B         | −1 016 B       |
| `dist/rs-compiler.darwin-arm64.node` | 9 711 184 B | 9 711 184 B | 0 B            |

**The honest reading is that this buys no measurable build time and no artifact
size at all**, and the ticket is worth doing anyway for the reason it states —
the proof, not the bytes. Two things explain the flat numbers. The fourteen
crates that stop being compiled are small and were compiling in parallel with
the long poles, so they never sat on the critical path; the run-to-run spread on
a clean build of this workspace (±10 %) is wider than the difference they make.
And the release profile already dead-stripped every CSS symbol, since nothing
called them — the linked artifact is byte-identical, which is itself
confirmation that the code was unreachable rather than merely unused.

The measurement to expect a real gain from is the one in ticket 11, over
normalization itself, and it is measured there rather than inferred here.
