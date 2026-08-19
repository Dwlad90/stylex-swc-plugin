# 01 — Pin why an unused import spares the shadowed parameter

Status: `resolved`
Blocked by: None — can start immediately

**What to build:** A written answer to why the reported failure needs a *second*
reference to the imported binding, so the fix in 02 is known to be one defect
rather than two.

A dynamic style whose parameter shadows a named theme import compiles correctly
when nothing else in the module references that import, and fails as soon as
anything does. The trigger is not the same `create()` call and not prop order:

| module shape | verdict |
| --- | --- |
| the dynamic style alone | agrees with the reference implementation |
| plus an unrelated static prop | agrees |
| import also used in a *second* `create()` | fails |
| import also used *outside any* `create()` | fails |
| import also used in the same `create()`, any key | fails |
| dynamic style written *before* the static use | fails |

Two theories are already ruled out and should not be re-run. The `seen` cache in
`evaluate/cache.rs` cannot explain it, because prop order is irrelevant and a
cache theory needs a priming order. `disable_imports` cannot explain it either:
the `create` function map passes `false`.

Pinning this needs one instrumented build — temporary tracing through the
identifier path is the expected route, not more black-box probing. Record the
mechanism, then decide which of two things it is: a shadow of the scope-blind
match that 02 removes, in which case say so and close; or an independent defect,
in which case file it as its own ticket rather than folding it into 02.

- [x] The mechanism is named, with the code path that produces it
- [x] Both ruled-out theories are recorded as ruled out, so the next reader does
      not repeat them
- [x] A verdict: shadow of 02, or an independent defect with its own ticket
- [x] Any instrumentation added for the investigation is removed

## Answer

**Verdict: a shadow of 02. No independent defect, no new ticket.**

There is one defect — the scope-blind `Named` arm of `get_import_from`
(`crates/stylex-transform/src/shared/utils/common.rs:138`, `local.sym ==
ident.sym`). What varies between the passing and failing shapes is not that
match; it is whether the import declaration is still in the module by the time
the match runs.

### The mechanism

`typescript_strip` runs between the resolver and the StyleX pass
(`crates/stylex-rs-compiler/src/lib.rs:305-308`). It elides an import
declaration whose binding has no *value* reference. The shadowing arrow
parameter is a different binding, so an import referenced nowhere else looks
unused, and the declaration is dropped before the StyleX pass ever sees it.
`visit_mut_import_decl_impl` therefore never records it in `top_imports`
(`transform/visit_mut/visit_mut_import_decl.rs:24`, which pushes every
non-type-only import unconditionally), the name match has nothing to match, the
parameter deopts, and the dynamic style compiles — by accident.

Any other value reference to the binding keeps the declaration alive, restores it
to `top_imports`, and the name match fires: `evaluate` answers a confident
`ThemeRef` for the *parameter*, and `evaluate_stylex_create_arg.rs:430` aborts
because a `ThemeRef` has no expression form.

Traced through the built compiler, one instrumented build, same two modules:

| | `top_imports` | import lookup for the parameter | verdict |
| --- | --- | --- | --- |
| dynamic style alone | `@stylexjs/stylex` only | `None` | compiles |
| plus `export const x = zIndex._10` | also `zIndex.stylex.js` | `Some(zIndex.stylex.js)` → confident `ThemeRef` | aborts |

The parameter carries ctxt `#3`; the import's local binding carries `#2`. The
scope-blind arm matches them anyway; its `Default` and `Namespace` siblings,
which compare the binding, would not.

### Ruled out, with the evidence — do not re-run these

- **The `seen` cache (`evaluate/cache.rs`).** Not involved. Every probe of the
  parameter reference reported a cache *miss*, and `hash_ident_unspanned`
  (`crates/stylex-utils/src/hash.rs`) hashes `ident.ctxt` alongside the symbol,
  so a reference at ctxt `#3` and one at `#2` cannot share an entry however the
  evaluation is ordered. The original ruling-out (prop order is irrelevant, so no
  priming order explains it) holds, and this is the stronger reason.
- **`disable_imports`.** `false` in every probe, passing and failing alike.

### Consequence for 02 — the suite already reproduces it

`cargo test` runs the resolver (swc's `test!` harness does) but **not**
`typescript_strip`, so the sparing does not happen there: under `stylex_test!`
the dynamic style *alone* aborts too. That makes the reported defect reachable
from the Rust suite without the napi build, and it is the reason 02 can pin the
fix with a snapshot rather than only through the parity harness. It also means a
cargo-level case that passes today because the import was elided does not exist
— there is nothing for the fix to preserve on that axis.
