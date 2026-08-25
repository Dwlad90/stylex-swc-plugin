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

**The verdict: the coverage is real and the throughput is free, but the engine
cannot be added to this workspace today, and the two costs it does carry —
6 MiB and a leaked engine per thread — are decisions for a person, not
measurements.** Do not start 06 on the engine until the first of those is fixed
upstream.

### 1. Upstream `boa_engine` does not resolve against this workspace

`boa_engine` 0.21.1 requires `icu_normalizer ~2.0.0`. `stylex_css` requires
`icu_collator 2.3.1`, which requires `icu_normalizer ~2.3.0`. Both are 2.x, so
Cargo must pick one version and cannot:

```
error: failed to select a version for `icu_normalizer`.
    ... required by package `boa_engine v0.21.1`
  previously selected package `icu_normalizer v2.3.0`
```

Downgrading is not available either: `icu_collator` 2.0.0 has no `unstable`
feature, and `pre_rule.rs` needs it for `CollatorBorrowed::new_root`.

Every number below therefore comes from `superui_boa_engine` 0.3.3, a
third-party fork whose only change is relaxing that requirement to
`>=2.0.0, <3`. A published compiler taking a dependency on one person's fork of
an engine is not a trade this spike can make on its own. The fix is a one-line
relaxation in boa; until it lands upstream, this option is closed.

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
