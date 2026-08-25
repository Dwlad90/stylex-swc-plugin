# 05 — Spike: a pure-Rust JS engine instead of hand-written method folds

Status: `resolved`
Phase: Deferred

**What to answer:** Whether the prototype-surface gap in 06 should be closed by
embedding a JS engine rather than by hand-writing the methods.

The reference implementation does not enumerate methods. It reflects
(`evaluate-path.js:1007-1010`):

```js
const val = object.node.value;
func = val[property.node.name];
```

so it inherits all of `String.prototype`, `Array.prototype` and
`Object.prototype` for free. Any enumeration this compiler writes is finite by
construction, and method N+1 is the next issue — which is the shape of
[#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265) itself.

`boa_engine` is pure Rust, so no C toolchain is added and all seven NAPI
targets — including `x86_64-unknown-linux-musl` and `aarch64-pc-windows-msvc` —
keep cross-compiling. Where receiver and every argument are already static, the
values go to the engine and the result comes back.

**It does not solve surrogates.** Boa's strings are UTF-16 internally, but the
result must land in `Lit::Str`'s `Atom`, which is UTF-8. The unpaired surrogate
dies at this compiler's boundary, not the engine's. The engine buys method
coverage, not exactness — 06's U+FFFD rule still applies.

Answer with numbers, on a throwaway branch, before any of 06 is written:

- Binary-size delta on each of the seven published targets.
- Cold-start and per-call cost against the release gate. `bench:verdict` fails
  at a reproduced 1.20x; `bench:budget` is seeded on
  `x86_64-unknown-linux-gnu`. See `guidelines/PERFORMANCE.md`.
- Whether the engine's `String.prototype` / `Array.prototype` coverage actually
  matches the divergence table in 06.
- Behaviour of a `stylex.create` call folding ~50 values with the engine
  initialised lazily and reused.

A JS engine inside a compiler whose pitch is that it is faster than the
reference implementation has to be defended with this repo's benchmark harness,
not with an argument.

## Answer

Measured on `feat_boa-engine`, a throwaway branch. Host: Apple M1 Max, Node
24.11.0, `x86_64-apple-darwin` cross-built from the same machine. The vehicle is
`boa_fold.rs`: it prints a self-contained method call back to source, evaluates
it in one lazily created engine reused for the process, and converts the result
back to a literal.

**The verdict: the coverage is real, the throughput is free, and the 6 MiB is a
price worth paying — what grows is a build-time artifact, not anything a
consumer ships. The one thing in the way is dependency resolution — and it is
solvable: upstream boa resolves against this workspace once `icu_collator` is
pinned to `=2.0.0`, at the cost of one documented decision in `pre_rule.rs`.
Relaxing boa's bound upstream costs nothing at all.** Pick one of §1's three
options before starting 06 on the engine. The leaked engine per thread (§5) is a
constraint to write down, not an objection.

### 1. Upstream `boa_engine` does not resolve against this workspace

`boa_engine` 0.21.1 requires `icu_normalizer ~2.0.0`. `stylex_css` requires
`icu_collator 2.3.1`, which requires `icu_normalizer ~2.3.0`. Both are 2.x, so
Cargo must pick one version and cannot:

```
error: failed to select a version for `icu_normalizer`.
    ... required by package `boa_engine v0.21.1`
  previously selected package `icu_normalizer v2.3.0`
```

Every number below comes from `superui_boa_engine` 0.3.3, a third-party fork
whose only change is relaxing that requirement to `>=2.0.0, <3`. The numbers
carry over to upstream unchanged — the fork differs in a version bound and
nothing else — but a published compiler depending on one person's fork of an
engine is not a trade this spike can make on its own.

**There is a way to use upstream boa, and it costs one documented decision.**
`icu_normalizer` cannot be removed from either side. Boa needs it for
`String.prototype.normalize` (`builtins/string/mod.rs` imports
`ComposingNormalizer` and `DecomposingNormalizer`), it is not behind a feature,
and swapping it for `unicode-normalization` would mean patching the engine's
internals rather than a version bound — strictly worse than the fork. On this
side it is not a direct dependency at all: it arrives through `icu_collator` and
through SWC's `url` → `idna_adapter`. So nothing can be substituted; the graph
just has to agree on one version, and every consumer can accept `2.0.1`:

| `icu_collator` | pulls `icu_normalizer` | resolves with upstream boa |
| -------------- | ---------------------- | -------------------------- |
| `2.0.0`        | `2.0.1`                | **yes**                    |
| `2.1.0`–`2.1.2`| `2.1.1`                | no                         |
| `2.2.0`–`2.2.1`| `2.2.0`                | no                         |
| `2.3.0`–`2.3.1`| `2.3.0`                | no                         |

Pinning `icu_collator = "=2.0.0"` with `compiled_data` instead of `unstable`
resolves the whole workspace against upstream `boa_engine` 0.21.1 —
`idna_adapter` follows from 1.2.2 down to 1.2.1 on its own — and
`cargo check --workspace --all-features` then reports **exactly one** error:

```
error[E0599]: no associated function or constant named `new_root` found for
              struct `CollatorBorrowed<'a>` in the current scope
   --> crates/stylex-css/src/utils/pre_rule.rs:178:38
```

`CollatorBorrowed::new_root` does not exist before 2.1, so `PSEUDO_COLLATOR`
goes back to `CollatorBorrowed::try_new` and its `Err` arm. That arm is
unreachable — the compiled CLDR data is a build-time dependency — which is
precisely why it was replaced: `pre_rule.rs` and the root `Cargo.toml` both
record that an unreachable arm is a region no test can exercise, and it was the
last thing keeping `scripts/coverage-missing.sh` from a clean run.

So the choice is between three things, and it is a person's to make:

1. **Pin `icu_collator` to `=2.0.0`** and take the uncoverable arm back, plus
   three years of ICU collation fixes not applied. Cheapest to do, undoes a
   decision that was argued in writing.
2. **Relax the bound upstream in boa.** Measured, not assumed — see below. Two
   lines, both decisions intact, `icu_collator` stays at 2.3.1. This is the one
   to try first.
3. **Depend on the fork.** Fastest, and the worst supply-chain position of the
   three for a package this many builds pull.

### 2. Binary size

Measured `.node`/`dylib`, release profile as published (fat LTO, one codegen
unit, symbols stripped):

| target                 | base       | with engine | delta                |
| ---------------------- | ---------- | ----------- | -------------------- |
| `aarch64-apple-darwin` | 9.58 MiB   | 15.15 MiB   | **+5.57 MiB, +58.2%** |
| `x86_64-apple-darwin`  | 10.10 MiB  | 16.17 MiB   | **+6.07 MiB, +60.1%** |

The other five targets each build on their own host or container
(`.github/workflows/npm.yml`), and no cross toolchain for them exists on a
darwin machine — Docker is installed but has no running daemon. They need a
`workflow_dispatch` run. The two measured targets differ by 2 percentage points
and the added code is target-independent pure Rust plus ICU data tables, so
expect the same ~6 MiB on each; the musl binding is the one worth confirming,
because it is the smallest baseline.

`Cargo.lock` grows by **49 packages**, and the clean release build compiles
**83 more crates**.

### 3. Cost against the release gate

Paired same-process `bench:revisions` (10 rounds, seed 1, all three fixture
categories) followed by `bench:verdict`:

```
Paired verdict: base vs candidate
  thresholds: warn>=1.10, fail>=1.20, improvement<=0.50
  ... 60 fixtures, every one status=pass
Suite passed
```

Every ratio lands in **0.989–1.017**, inside noise. Nothing in the corpus writes
a method call on a literal, so the engine is never constructed and the transform
pays nothing. `bench:budget` reports `unseeded` plus three
environment-mismatch problems on this machine, exactly as
`guidelines/PERFORMANCE.md` describes; it says nothing about this change either
way and has to run on `x86_64-unknown-linux-gnu`.

Where the engine does run:

- **Cold start** — first fold in a fresh process: **~240 µs**
  (one-fold file 1.51 ms versus the same file with the value already folded
  1.27 ms, three runs each).
- **Warm per fold** — **~3.4 µs**
  (fifty-value `create`: 0.533 ms all literals, 0.701 ms all folds, p50 of 60
  iterations).

A file with fifty folds costs under a fifth of a millisecond more than the same
file with fifty literals. Lazy construction and reuse are what make this
uninteresting, and both are one `thread_local`.

### 4. Coverage against 06's divergence table

All 70 expressions from the table, run through the engine and through Node, both
printing the same tagged form:

- **69 of 70 identical**, with the `annex-b` feature on (which is what supplies
  `substr`; without it, 68). The single difference is `Math.random()`, which is
  nondeterministic and which the reference implementation refuses anyway.
- Adversarial battery — 21 of 21 identical: unpaired surrogates, `1e308`
  indices, negative pad lengths, `[].reduce` throwing `TypeError`,
  `flat(Infinity)`, `toWellFormed`, `codePointAt` past the end.
- **Locale-sensitive methods diverge and must stay excluded.** Without ICU
  locale data the engine answers `'ä'.localeCompare('z', 'de')` as `1` where
  Node says `-1`, `'i'.toLocaleUpperCase('tr')` as `I` where Node says `İ`, and
  `(1234.5).toLocaleString('de-DE')` as `1234.5` where Node says `1.234,5`.
  These are exactly the four 06 excluded by hand, so its boundary was right.
  `normalize('NFC')` and `normalize('NFKC')` do agree — `icu_normalizer` is not
  optional in boa — so `normalize` could move into scope.

Against the reference implementation itself, byte for byte on class name and
declaration text: **48 of 48** rules on the fifty-value fixture and **32 of 32**
on a wider fixture covering 21 string methods, 13 array methods and chained
calls. `NaN`, `Infinity` and `-0` results agree too — the engine emits
`z-index:NaN` for `'abc'.charCodeAt(10)`, and so does the reference
implementation.

**Chaining comes free.** `['1px','dashed','blue'].concat([]).join(' ')` and
`[4,8].map(size => size + 'px').join(' ')` fold, because the receiver of the
outer call is printed with the rest of the chain and evaluated once. That is the
gap 06 names as the shape of the original bug, and one dispatch through an
engine cannot reproduce it.

### 5. Dropping the engine aborts the process

`cargo test --workspace --all-features` died on **SIGTRAP**:

```
boa_gc-0.21.1/src/internals/gc_header.rs:69: attempt to subtract with overflow
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
```

The collector keeps its state in a thread-local of its own, and the order two
thread-locals are destroyed in is not defined. When the `Context` is dropped
after the collector, a reference count underflows — and because that panic runs
inside a destructor, it aborts rather than unwinding, so this compiler's panic
boundary cannot turn it into a diagnostic. In a `.node` that is the consumer's
bundler dying with no message.

Holding the engine in `ManuallyDrop` fixes it: 5 073 tests then run to
completion. In release the subtraction wraps silently instead of trapping, which
is why the built `.node` never showed it — a latent corruption is worse than the
crash that found it. So an engine here must be leaked, one per thread, by
design and with the reason written down.

### 6. What the existing suite says about the behaviour change

With the hook in, 7 of 5 080 tests fail, and each one is a refusal that became a
fold:

```
member_length_tests::a_receiver_that_did_not_fold_refuses_rather_than_being_counted
unfoldable_operand_tests::every_unfoldable_shape_survives_every_logical_operand_position
unsupported_shape_tests::a_deeply_nested_refusal_stays_a_refusal
unsupported_shape_tests::a_method_call_on_a_receiver_kind_with_no_folds_refuses
unsupported_shape_tests::an_unfoldable_method_on_a_unicode_receiver_refuses_rather_than_aborting
unsupported_shape_tests::char_code_at_past_the_end_refuses_rather_than_aborting
unsupported_shape_tests::names_the_value_a_refusal_arrived_with
```

Most are 06's stated goal arriving. Two need a decision rather than a rewrite:
`"abc".charCodeAt(10)` now folds to `NaN` and emits `z-index:NaN`, and
`[1,2].reduce((total) => total / 0, 1)` emits `order:Infinity`. Both are
byte-identical to the reference implementation, and both are invalid CSS that
today's refusal keeps out of the stylesheet. Parity and a useful error point in
opposite directions there; 06 has to say which one wins.

### 7. Surrogates, confirmed unsolved

`'\u{1F600}a'.slice(1)` compiles under both compilers and the class names
diverge exactly as 06 predicted — reference `xi08yer`, this compiler `xn5tvdn`,
declaration text rendering to a replacement character either way. The engine
carries the lone surrogate correctly; it dies converting to `Lit::Str`'s UTF-8
atom. 06's U+FFFD rule stands unchanged.

### 8. One risk 05 did not list

A single argument makes the compiler allocate without bound:
`'x'.repeat(200000000)` folds — agreeing with Node — at a peak RSS of
**5.37 GB**. `RuntimeLimits` caps loop iterations, recursion and stack, but not
allocation, because the growth happens inside a native builtin rather than a
counted loop. Hand-written folds have the same exposure in principle, but they
do not implement `repeat` at all, so today the shape is refused. An engine turns
a typo into a build that exhausts memory, and a size guard on the arguments
would be a precondition of shipping one.

### Reproducing

`crates/stylex-transform/src/shared/utils/js/evaluate/boa_fold.rs` and the
`boa_fold::try_fold` call at the top of `call_expression::evaluate` are the whole
vehicle. `crates/stylex-rs-compiler/parity/spike05-parity.ts` compares one
module against `@stylexjs/babel-plugin` rule by rule. Both are throwaway and
this branch is not for merging.

### 9. Option 2, measured

Against a clone of `boa-dev/boa` at `v0.21.1`, patched into this workspace
through `[patch.crates-io]`:

```diff
--- i/Cargo.toml
+++ w/Cargo.toml
@@ -154,8 +154,8 @@
-icu_properties = { version = "~2.0.0", default-features = true }
-icu_normalizer = { version = "~2.0.0", default-features = false }
+icu_properties = { version = ">=2.0.0, <3", default-features = true }
+icu_normalizer = { version = ">=2.0.0, <3", default-features = false }
```

**Two lines, not one.** `icu_normalizer` alone is not enough: `boa_parser` also
requires `icu_properties ~2.0.0` for its lexer, and `icu_collator 2.3.1` needs
`~2.3.0`, so the second bound surfaces as the next resolution error. Relaxing
both is the whole change.

With that patch in place:

- The workspace resolves with **`icu_collator` untouched at 2.3.1**, one version
  of every ICU crate (`icu_normalizer 2.3.0`, `icu_properties 2.3.0`,
  `icu_collections 2.3.0`), `new_root` intact, `unstable` intact, and
  `idna_adapter` at its current 1.2.2. Option 1's cost disappears entirely.
- `cargo test --workspace --all-features`: **5 073 passed**, and the only 7
  failures are the same seven §6 lists — the spike hook's intended behaviour
  change, identical under the fork and under the patch. So boa compiles and runs
  correctly against ICU 2.3: `normalize('NFC')` and `normalize('NFKC')` answer as
  Node does on that build, along with the rest of §4's corpus.
- boa's own suite with the relaxed bound: `cargo test -p boa_engine -p boa_parser
  -p boa_string --features annex-b` → **1 292 passed, 0 failed**.

**The relaxation is permissive, not forcing**, which is the argument to make
upstream: boa's own build still resolves `icu_normalizer` to 2.0.0, because its
`intl` feature pins `icu_casemap`, `icu_collator` and `icu_calendar` at `~2.0`
and those hold the family down. Nothing about boa's own builds or CI changes. A
consumer that carries a newer ICU elsewhere in its graph — and does not enable
`intl`, as this workspace does not — becomes able to depend on boa at all.

Moving boa's whole `intl` stack forward is a larger change (17 bounds, and it
lands on icu 2.1 rather than 2.3) and is not needed for this. The patch is saved
alongside this spike's other artifacts.

## Comments

### Code review, 2026-08-25 — `b0bb7c5dd...HEAD`

Two axes, run as independent sub-agents over commits `46ff6b789` and
`1899d2076`, reported separately and deliberately not merged or reranked.
`vendor/boa/core/**` and `vendor/boa/utils/**` were excluded as unmodified
upstream source; our own integration files were in scope.

#### Standards

**Documented-standard breaches**

1. **`vendor/boa/` placement contradicts `guidelines/STRUCTURE.md` (hard).**
   STRUCTURE.md, Rust Crates: third-party code is a crate *because* the boundary
   is visible — "The workspace `members` glob only matches `crates/stylex-*`, so
   it is listed explicitly in the root `Cargo.toml`. **Anything else vendored
   belongs beside it on the same terms.**" This change instead adds
   `exclude = ["vendor/boa"]` plus `[patch.crates-io]`. Whatever the merits (and
   `vendor/boa/README.md` argues them well), STRUCTURE.md now states a rule the
   repo no longer follows and is not updated in the diff. Either place it on
   `postcss-value-parser`'s terms or amend STRUCTURE.md.
2. **`vendor/boa/README.md` — our own markdown now exempt from lint.** CLAUDE.md
   Quick Reference: "80 for markdown". Lines 30 and 60 exceed 80. The new
   `lefthook.yml` `vendor/**` excludes are right for upstream source but also
   silence the three files we authored there. Consider `vendor/boa/*.md` staying
   in the markdown glob, or the README living at `docs/`.
3. **`spike05-parity.ts` — broad object assertions.**
   `guidelines/stack/TYPESCRIPT.md`, Coding Standards: "Do not use
   double-casting … or broad object assertions … utilize type guards, type
   predicates, or schemas". Two sites: `entry as [string, { ltr?: string } |
   undefined]` and `(babelResult?.metadata as { stylex?: unknown[] })?.stylex`.
   Soft for a throwaway harness, but `parity/lib/compilers.js` is shared and a
   typed helper there would remove both.
4. **`boa_fold.rs` has no tests.** `guidelines/stack/RUST.md`, Coverage: "100%
   line coverage is enforced". Zero `#[cfg(test)]`, and `call_expression.rs` now
   routes every method call through it. Acceptable for a measurement vehicle; a
   hard blocker if kept.

Clean on the rules most often missed: no `.unwrap()`/`.expect()`, no std
`HashMap`, workspace dep used in `crates/stylex-transform/Cargo.toml`, all lines
<= 100, commit types (`chore`/`build`) match `guidelines/git/CONVENTIONS.md`.

**Baseline smells (judgement calls)**

- **Duplicated Code** — `is_self_contained` and `free_identifiers_are_within`
  are the same recursive `Expr` walk with overlapping arms
  (`Expr::Lit(lit) => !matches!(lit, Lit::Regex(_))`, `Paren`, `Unary` all
  repeated verbatim). One walk parameterised by how a bare `Ident` is judged
  would collapse them.
- **Mysterious Name** — `boa_fold`/`try_fold` name the vendor and the mechanism,
  not the concept. `engine_fold` or `fold_by_evaluation` would survive a swap of
  engine.
- **Primitive Obsession** — `params: &[String]` built with
  `ident.sym.to_string()` and compared by `contains`; `Atom` is the domain type
  and avoids an allocation per identifier per compare.
- **Divergent Change (mild)** — `crates/stylex-rs-compiler/parity/` gains a
  harness that is not in `parity/README.md`'s "Where it runs" table nor any
  script, so the directory's documented inventory is now incomplete.

#### Spec

**(a) Missing / partial**

- *"Binary-size delta on each of the seven published targets."* Only 2 of 7
  measured (§2). The shortfall **is** stated honestly ("They need a
  `workflow_dispatch` run") with a reasoned extrapolation. But the ticket's
  premise — *"all seven NAPI targets… keep cross-compiling"* — is left
  unverified for musl and `aarch64-pc-windows-msvc`; §2 addresses size, never
  buildability, and the answer doesn't flag that half as open.
- *"`bench:budget` is seeded on `x86_64-unknown-linux-gnu`"* — not run there
  (§3: "unseeded… has to run on x86_64-unknown-linux-gnu"). Honest, but the
  release-gate half of requirement 2 is unanswered. `bench:verdict` passing is
  also vacuous by the answer's own admission ("the engine is never
  constructed"), which it says plainly.
- Requirements 3 and 4 are answered with numbers and provenance.

**(b) Scope creep**

The ticket says *"Answer with numbers, on a throwaway branch"* and §Reproducing
repeats *"Both are throwaway and this branch is not for merging."* Commit
`1899d2076` is not throwaway: it vendors nine boa crates with
`[patch.crates-io]`, and edits `NOTICE.md`, `deny.toml` (a RUSTSEC ignore),
`lefthook.yml` + its snapshot, `.oxlintrc.jsonc`, `.oxfmtrc.json`, and the
workspace `exclude`. Its Cargo.toml comment argues policy ("Patching is sound
here rather than a workaround a published crate would have to apologise for"),
which is an adoption decision, not a measurement. `boa_fold.rs` and
`parity/spike05-parity.ts` are within a measurement vehicle; the repo-wide
config and licence surface is not.

06's *"Do not start it before 05 answers how"* is respected — no work on 06's
boundaries appears — but 06's body was edited to carry 05's answer as a bespoke
`## 05's answer` section rather than `## Comments` (tracker: "Comments and
conversation history append to the bottom of the file under a `## Comments`
heading").

**(c) Implemented but wrong**

1. **The Answer contradicts the diff.** §1: *"the choice is between three
   things, and it is a person's to make"* — pin `=2.0.0`, relax upstream, or
   depend on the fork — and *"Pick one of §1's three options before starting
   06."* The branch picks an unlisted fourth (vendoring), and neither 05 nor 06
   mentions the word "vendor" anywhere. The tracker now misdirects the human
   decision it defers to; only `vendor/boa/README.md` records the actual choice.
2. **Numbers' provenance is stale.** §1: *"Every number below comes from
   `superui_boa_engine` 0.3.3"* (ICU 2.0 line). The vendored build resolves ICU
   2.3 throughout, and §2's size and §3's timings were not re-measured on it.
   §9 only re-ran the test suites, not the size or bench numbers.
3. **A deliberate divergence is silently broken.** 06: *"Mutating methods keep
   deopting… `is_mutating_array_method` already refuses them."* The hook in
   `call_expression.rs:150` runs `boa_fold::try_fold` **before** everything, so
   `["a","b"].sort().join("-")` now folds. §4/§6 never mention it — §6 accounts
   for 7 failures as "06's stated goal arriving", so a boundary 06 holds
   deliberately is reported as progress.
4. Minor: the Answer's preamble says *"one lazily created engine reused for the
   process"*; it is one per thread (§5 corrects it, the preamble doesn't).
   `NOTICE.md` says *"Only two dependency bounds… are changed"*;
   `vendor/boa/README.md` also narrows `members`.

#### Summary

Standards: 4 findings plus 4 judgement calls; worst is the STRUCTURE.md rule the
vendoring contradicts without amending it. Spec: 7 findings; worst is (c)3 — the
hook folds mutating array methods, which 06 refuses deliberately. Reproduced:
`['opacity','color'].sort().join(', ')` emits `transition-property:color,opacity`
and `['a','b'].push('c')` emits `z-index:3`.
