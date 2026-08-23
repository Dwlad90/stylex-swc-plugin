# Code review and remediation plan

Branch `fix_dynamic-style-parameter-shadowing-an-imported-binding` against
`develop` — 149 commits, 401 files, +35,123 / −2,015 (21,280 insertions of Rust
source).

Reviewed in five parallel passes over disjoint slices: reference resolution,
diagnostics/spans, state+options+FFI, CSS-in-JS transform core, and
tests/benches. Every finding survived an explicit refutation attempt; findings
that did not survive are in [Checked and cleared](#checked-and-cleared) so they
are not re-litigated.

Every fix below is given as an exact old → new replacement. Nothing here is
advice.

**Baseline facts, verified before review:**

- `cargo clippy --workspace --all-targets` — clean, zero warnings.
- `cargo fmt --all --check` — clean.
- No `.unwrap()` / `.expect()` added outside test files.
- The headline fix is correct — see [Checked and cleared](#checked-and-cleared).

---

## Contents

| | |
|---|---|
| [Verdict](#verdict) | what the branch got right, and where the defects actually are |
| [1. 🔴 Critical](#1--critical--logic--ffi) | C1–C7 |
| [2. 🟡 Optimization](#2--optimization--performance--complexity) | O1–O13 |
| [3. 🟢 Nitpick](#3--nitpick--readability--testing) | N1–N25 |
| [4. 🛠️ Execution order](#4-️-execution-order) | 11 landable steps |
| [Checked and cleared](#checked-and-cleared) | refuted, do not re-review |

---

## Verdict

The branch's actual subject — preferring the module's own binding when a name is
shadowed — is **sound**. `Ident::eq_ignore_span` compares `sym` *and* `ctxt`
(`swc_ecma_ast-27.0.0/src/ident.rs:198-206`), the resolver runs ahead of the
pass (`transform/mod.rs:402`), and the 128-bit memo key hashes `ctxt` before
`sym` (`stylex-utils/src/hash.rs:487-492`), so a dynamic-style parameter carries
its own `SyntaxContext` and cannot resolve to the import it shadows, nor alias
it in the cache. The eight-step chain is well documented and unusually well
tested — `resolution_order.rs` alone is 1,517 lines and pins every ordering
pair, all three specifier kinds, and the `Id`-collision case the reorder's
"inert" claim depends on.

The defects are in the machinery built *around* that fix:

- a span index that compares byte offsets from two different `SourceMap`s (C1),
- an options path where a negative number removes the ceiling it configures (C2),
- a numeric key spelled with Rust's `Display` rather than JavaScript's
  `ToString` (C3),
- four new benchmarks whose published numbers do not measure what their ADRs
  claim (C4, C5),
- and three test-integrity holes that would let real regressions land silently
  (C6, C7).

---

## 1. 🔴 Critical / Logic / FFI

### C1 — `KeySpanIndex` ranks candidates by a distance between two unrelated coordinate systems

`crates/stylex-transform/src/shared/structures/key_span_index.rs:200`

`IndexedCandidate::candidate_lo` is a `BytePos` in the **process-global**
`SOURCE_MAP` (`build_code_frame_error.rs:48`). `NamespaceKeyQuery::target_lo` is
a `BytePos` in the **per-transform** `SourceMap` built in
`rs-compiler/src/lib.rs:239`. `rank` subtracts one from the other.

Traced and confirmed:

1. `visit_mut_module_impl:702` memoizes the *input* module with
   `source_code: None`.
2. `get_key_span_from_source_code_impl:402` calls `memoize_module`, whose
   `Some(source_code)` fast path therefore cannot hit — so it falls to the else
   branch, re-parses the file into the global map, and **overwrites** the memo
   (`build_code_frame_error.rs:551-570`).
3. `KeySpanIndex::build` consequently indexes the re-parsed module.

`SourceMap::new_source_file` gives each file a `start_pos` after the previous
file's end, and the global map is never cleared. So only the **first** file a
process compiles has the two systems aligned — which is exactly the
configuration every unit test uses (`key_span_index_test.rs:87-97` builds each
fixture in a source map of its own whose first byte is `BytePos(1)`). From the
second file onward `target_lo` is small while every `candidate_lo` sits at the
accumulated global offset, so `argmin |c − t|` degenerates to `argmin c`: **the
first object literal in the file spelling the key always wins.**

Compounded by [O1](#o1--the-process-global-source_map-retains-a-full-copy-of-every-module-per-compile):
each compile appends another copy of the file to the global map, pushing the
offsets further apart on every save in watch mode.

Failure input — second or later module in the process, `dev` build:

```js
export const a = stylex.create({ root: { color: 'red' } });
export const b = stylex.create({ root: { color: 'blue' } });
```

Both candidates tie on `namespace_value_overlap` and `sibling_overlap`, so
distance decides — and `b`'s `root` is annotated with `a`'s line in `$$css`.
A silently wrong `file:line`, which `resolve`'s own doc says the ranking exists
to prevent.

#### Fix (preferred): make both sides file-relative

```rust
// key_span_index.rs:36 — old
#[derive(Clone, Debug, Default)]
pub(crate) struct KeySpanIndex {
  by_key: FxHashMap<Atom, Vec<IndexedCandidate>>,
}

impl KeySpanIndex {
  pub(crate) fn build(module: &Module) -> Self {
    let mut index = Self::default();

    module.visit_with(&mut index);

    index
  }
```

```rust
// key_span_index.rs:36 — new
#[derive(Clone, Debug, Default)]
pub(crate) struct KeySpanIndex {
  by_key: FxHashMap<Atom, Vec<IndexedCandidate>>,
  /// Where the indexed module starts in the code frame's source map, so a
  /// candidate's position can be expressed as an offset into its own file
  /// rather than into a map the query side does not share.
  base: BytePos,
}

impl KeySpanIndex {
  pub(crate) fn build(module: &Module) -> Self {
    let mut index = Self {
      base: module.span.lo,
      ..Self::default()
    };

    module.visit_with(&mut index);

    index
  }
```

```rust
// key_span_index.rs:51 — old
  /// Where the candidate's call is written, for the distance tie-break.
  candidate_lo: BytePos,
```

```rust
// key_span_index.rs:51 — new
  /// Where the candidate's call is written, as an offset into its own file, for
  /// the distance tie-break.
  candidate_offset: u32,
```

```rust
// key_span_index.rs:129 — old
    let candidate_lo = object_lo(object).unwrap_or(call.span.lo);
```

```rust
// key_span_index.rs:129 — new
    let candidate_lo = object_lo(object).unwrap_or(call.span.lo);
    let candidate_offset = candidate_lo.0.saturating_sub(self.base.0);
```

```rust
// key_span_index.rs:142 — old
            sibling_keys: Rc::clone(&sibling_keys),
            candidate_lo,
```

```rust
// key_span_index.rs:142 — new
            sibling_keys: Rc::clone(&sibling_keys),
            candidate_offset,
```

```rust
// key_span_index.rs:194 — old
  fn rank(&self, query: &NamespaceKeyQuery) -> CandidateRank {
    CandidateRank {
      namespace_value_overlap: overlap(&self.namespace_value_keys, &query.namespace_value_keys),
      sibling_overlap: overlap(&self.sibling_keys, &query.sibling_keys),
      distance_from_target: Reverse(
        query
          .target_lo
          .map(|target_lo| self.candidate_lo.0.abs_diff(target_lo.0)),
      ),
    }
  }
```

```rust
// key_span_index.rs:194 — new
  fn rank(&self, query: &NamespaceKeyQuery) -> CandidateRank {
    CandidateRank {
      namespace_value_overlap: overlap(&self.namespace_value_keys, &query.namespace_value_keys),
      sibling_overlap: overlap(&self.sibling_keys, &query.sibling_keys),
      // Both sides are offsets into their own file, never raw `BytePos`: this
      // index is built from a module re-parsed into the code frame's shared
      // source map, and the query is read off the compiled call in the
      // compiler's per-transform one. The absolute numbers live in different
      // coordinate systems and only the offsets compare.
      distance_from_target: Reverse(
        query
          .target_offset
          .map(|target| self.candidate_offset.abs_diff(target)),
      ),
    }
  }
```

Query side — `key_span_index.rs:231`, `:256`, `:281`, `:311`:

```rust
// old
  /// Where the call's object argument starts, for the proximity tie-break.
  pub(crate) target_lo: Option<BytePos>,
```

```rust
// new
  /// Where the call's object argument starts, as an offset into its own file,
  /// for the proximity tie-break.
  pub(crate) target_offset: Option<u32>,
```

```rust
// CallLookup::new — old
    Self {
      call_expr,
      object_arg,
      digest: call_digest(call_expr, object_arg, &sibling_keys),
      sibling_keys,
      target_lo: object_arg
        .and_then(object_lo)
        .or_else(|| (!call_expr.span.is_dummy()).then_some(call_expr.span.lo)),
      wrapped: OnceCell::new(),
    }
```

```rust
// CallLookup::new — new (takes the module base the compiled call is positioned in)
  pub(crate) fn new(call_expr: &'a CallExpr, module_base: BytePos) -> Self {
    // ... sibling_keys / object_arg unchanged ...
    Self {
      call_expr,
      object_arg,
      digest: call_digest(call_expr, object_arg, &sibling_keys),
      sibling_keys,
      target_offset: object_arg
        .and_then(object_lo)
        .or_else(|| (!call_expr.span.is_dummy()).then_some(call_expr.span.lo))
        .map(|lo| lo.0.saturating_sub(module_base.0)),
      wrapped: OnceCell::new(),
    }
  }
```

```rust
// CallLookup::query — old
      target_lo: self.target_lo,
// new
      target_offset: self.target_offset,
```

Plumbing — record the base once and pass it:

```rust
// visit_mut_module.rs:702 — old
    if cfg!(debug_assertions) || !self.state.options.use_real_file_for_source {
      self.state.set_seen_module_source_code(module, None);
    }
```

```rust
// visit_mut_module.rs:702 — new
    if cfg!(debug_assertions) || !self.state.options.use_real_file_for_source {
      // Recorded beside the memo because the key-span lookup compares a
      // compiled call's position against candidates indexed in a *different*
      // source map. Only offsets into the file compare; this is the base they
      // are taken from.
      self.state.set_input_module_base(module.span.lo);
      self.state.set_seen_module_source_code(module, None);
    }
```

```rust
// add_source_map_data.rs:81 — old
  let lookup = CallLookup::new(call_expr);
// new
  let lookup = CallLookup::new(call_expr, state.input_module_base());
```

Cache key — `build_code_frame_error.rs:425`:

```rust
// old
  stable_hash_wide(&(
    "stylex-key-span:v4",
    siblings_digest,
    query.namespace_key,
    sorted_value_keys,
    query.target_lo.map(|lo| lo.0),
  ))
```

```rust
// new
  stable_hash_wide(&(
    "stylex-key-span:v5",
    siblings_digest,
    query.namespace_key,
    sorted_value_keys,
    query.target_offset,
  ))
```

#### Fix (minimal alternative), if the plumbing is deferred

Delete `distance_from_target` from `CandidateRank` entirely and let a genuine
tie return `DUMMY_SP`. That is already `resolve`'s documented policy — *"a tie is
refused rather than guessed, because a wrong `file:line` is worse than none"* —
and falls back to the value matcher. Strictly better than guessing with a
meaningless number, at the cost of more `DUMMY_SP` answers.

#### Test that would have caught it

```rust
  /// Every other case here parses its fixture into a source map of its own, so
  /// the index and the query share an origin and the distance tie-break is
  /// meaningful by accident. Production does not: the index is built from a
  /// module re-parsed into the process-global map, and the query is read off the
  /// compiled call in the per-transform one. This is the second file in a map,
  /// which is the first one where the two disagree.
  #[test]
  fn a_second_file_in_one_source_map_still_resolves_its_own_call() {
    let source_map: Lrc<SourceMap> = Default::default();

    let first = "const a = stylex.create({ root: { color: 'red' } });\n";
    let second = "const b = stylex.create({ root: { display: 'flex', flexGrow: 1 } });\n";

    let _ = parse_into(&source_map, "first.ts", first);
    let module = parse_into(&source_map, "second.ts", second);

    let index = KeySpanIndex::build(&module);
    let span = index.resolve(&query("root", &["root"], &["display", "flexGrow"], None));

    assert!(!span.is_dummy(), "the lookup resolved no position");
    assert_eq!(
      source_map.lookup_char_pos(span.lo).file.name,
      FileName::Custom("second.ts".to_owned()),
      "a call in the second file must not resolve to a candidate in the first"
    );
  }
```

---

### C2 — A negative `maxEvaluationDepth` silently removes the ceiling

`crates/stylex-rs-compiler/src/structs/mod.rs:76,185`

`Option<u32>` is read through `napi_get_value_uint32`, which per the Node-API
specification *"truncates to the equivalent of the bottom 32 bits"* rather than
erroring. The generated `.d.ts` says `number`, so `maxEvaluationDepth: -1`
typechecks, arrives as `4294967295`, passes `resolve_from`'s `depth > 0` guard
(`evaluation_depth.rs:66`) and is honoured.

Verified: `normalizeRsOptions` performs no range validation (only presence and
strip-undefined, `normalizeRsOptions.spec.ts:514-536`), and no test covers a
negative value.

Failure scenario: a user writes `maxEvaluationDepth: -1` in `stylex.config.js`
intending "no limit", then compiles a generated token file with a deep
expression. The fold descends while `stacker::maybe_grow` allocates 16 MiB
segments (`evaluate/cache.rs:21`) and the Node process is OOM-killed with no
file, no message and no diagnostic — precisely the abort ADR 0004 says the
ceiling exists to convert into a message. `0` is explicitly guarded; negatives,
the easier typo, are not. `2**32` maps to `0` and silently falls back to the
default, which is a second surprise in the other direction.

```rust
// crates/stylex-rs-compiler/src/structs/mod.rs:76 — old
  pub max_evaluation_depth: Option<u32>,
```

```rust
// new
  #[napi(ts_type = "number")]
  pub max_evaluation_depth: Option<i64>,
```

```rust
// crates/stylex-rs-compiler/src/structs/mod.rs:185 — old
      max_evaluation_depth: val.max_evaluation_depth.map(|depth| depth as usize),
```

```rust
// new
      // `napi_get_value_uint32` applies `ToUint32`, so a negative or
      // out-of-range `maxEvaluationDepth` used to arrive as a number near
      // `u32::MAX` -- a ceiling high enough that the fold exhausts memory before
      // it reaches one, which is no ceiling at all. Read as a signed integer and
      // refuse anything that is not a usable depth, which falls back to the
      // environment and then to the default exactly as an absent value does.
      max_evaluation_depth: val
        .max_evaluation_depth
        .and_then(|depth| usize::try_from(depth).ok()),
```

Close the same hole for every other caller —
`crates/stylex-structures/src/evaluation_depth.rs`, beside
`DEFAULT_MAX_EVALUATION_DEPTH`:

```rust
/// The highest ceiling a caller can ask for.
///
/// The ceiling exists to turn a stack overflow into a diagnostic, so a number
/// large enough that the fold exhausts memory before it reaches the number is
/// not a ceiling. `stacker` grows the fold in 16 MiB segments, so a million
/// levels is already far past any input a diagnostic could still describe.
pub const MAX_EVALUATION_DEPTH_LIMIT: usize = 1 << 20;
```

```rust
// resolve_from:66 — old
    Some(depth) if depth > 0 => depth,
// new
    Some(depth) if depth > 0 => depth.min(MAX_EVALUATION_DEPTH_LIMIT),
```

Tests:

```rust
  #[test]
  fn a_ceiling_past_the_limit_is_clamped_rather_than_honoured() {
    assert_eq!(resolve_from(Some(usize::MAX), None), MAX_EVALUATION_DEPTH_LIMIT);
    assert_eq!(resolve_from(Some(MAX_EVALUATION_DEPTH_LIMIT + 1), None), MAX_EVALUATION_DEPTH_LIMIT);
    assert_eq!(resolve_from(Some(MAX_EVALUATION_DEPTH_LIMIT), None), MAX_EVALUATION_DEPTH_LIMIT);
  }
```

```ts
// crates/stylex-rs-compiler/__test__/index.spec.ts
// A negative ceiling reached the evaluator as ~4.29 billion, because
// `napi_get_value_uint32` truncates rather than refusing. Read as a signed
// integer it is not a usable depth, so it falls back exactly as an absent value
// does -- which is observable as the default's refusal still happening.
test('maxEvaluationDepth: a negative ceiling falls back to the default', () => {
  expect(() => compileAtDepth(deepFixture(100), -1)).toThrow(
    /At most 32 levels of nested evaluation are supported/
  );
});

test('maxEvaluationDepth: a ceiling past the 32-bit range falls back to the default', () => {
  expect(() => compileAtDepth(deepFixture(100), 2 ** 32)).toThrow(
    /At most 32 levels of nested evaluation are supported/
  );
});
```

---

### C3 — Numeric computed keys are spelled with Rust's `Display`, not JS `ToString`

`crates/stylex-transform/src/shared/enums/data_structures/evaluate_result_value.rs:250`
(consumed by `member_expression.rs:52,64-72`)

`as_string_key` returns `n.value.to_string()`. Measured — `rustc` on one side,
Node 24 on the other:

| value | Rust `to_string()` | JS `String()` |
|---|---|---|
| `-0.0` | `"-0"` | `"0"` |
| `1e-7` | `"0.0000001"` | `"1e-7"` |
| `1e21` | `"1000000000000000000000"` | `"1e+21"` |

`index_slot`'s doc states the rule correctly — *"the language reads an index only
where the key is the canonical spelling of the number"* — but is handed a
non-canonical spelling. So `[10, 20][-0]` classifies as `Missing` and folds to
`undefined` where JS and upstream fold `10`; `({'1e-7': 'x'})[1e-7]` misses and
folds to `undefined` instead of `'x'`.

In StyleX a style value of `undefined` means "unset", so this is a **silent
wrong-output path**, not a refusal the author would see.

The project already owns the correct routine —
`stylex_utils::number::to_js_string` (`stylex-utils/src/number.rs:103-113`),
whose own comment reads *"Covers `-0`, which JS also renders as `\"0\"`"*. It is
simply not the one being called.

```rust
// evaluate_result_value.rs:250 — old
        Expr::Lit(Lit::Num(n)) => Some(n.value.to_string()),
```

```rust
// new
        // A property key is `ToPropertyKey`, which is `ToString` -- not Rust's
        // `Display`. The two part company on `-0` (`"0"` in the language, `"-0"`
        // here) and on every magnitude that takes exponential form (`"1e-7"`
        // against `"0.0000001"`), so `list[-0]` read no element and `obj[1e-7]`
        // found no property.
        Expr::Lit(Lit::Num(n)) => Some(stylex_utils::number::to_js_string(n.value)),
```

Tests:

```rust
// crates/stylex-transform/src/shared/utils/js/evaluate/tests/array_index_tests.rs
  /// `String(-0)` is `"0"` in the language and `"-0"` in Rust's `Display`, so
  /// this used to classify as a missing property and fold to `undefined`.
  #[test]
  fn a_negative_zero_index_reads_the_first_element() {
    assert_folds_to_number("[10, 20][-0]", 10.0);
  }

  /// `String(1e-7)` keeps exponential form where Rust's `Display` expands it.
  #[test]
  fn an_exponential_index_reads_the_property_it_spells() {
    assert_folds_to_string("({ '1e-7': 'x' })[1e-7]", "x");
  }
```

---

### C4 — `evaluate_bench` times its own setup and asserts nothing; ADR 0006's table is diluted

`crates/stylex-transform/benches/evaluate_bench.rs:206-236`

Two defects in one loop. Confirmed directly: `grep -c assert
crates/stylex-transform/benches/evaluate_bench.rs` returns **0**, against 20 in
`module_path_bench.rs` and 13 in `transform_debug_bench.rs`.

**(a) Setup inside `b.iter`.** Every iteration builds
`StyleXOptions::default()`, constructs a `StateManager`, and walks the module
**twice** (`fill_top_level_expressions`, `fill_top_level_var_declarations`)
before the first `evaluate` call. This is the exact defect commit `4b199c7b3`
fixed in `module_path_bench.rs`, whose own message says
`StyleXOptions::default()` *"allocates two strings, an index set and a shared
map, and on this branch also reads the evaluation-depth environment behind a
`OnceLock` … work that does not exist on the revision being compared against"*,
and that removing it moved a published figure from +29 ns to +9.1 ns. The branch
fixed one file and left the identical pattern in the other file it edited. Every
number in ADR 0006's `evaluate_bench` table therefore includes a constant the
fold does not pay, on both legs — so a real regression inside `evaluate` reads
smaller than it is.

**(b) No guard, on fixtures that cannot resolve their imports.**
`guidelines/PERFORMANCE.md:80` states as fact: *"Every bench in
`crates/stylex-transform/benches` panics unless its subject produced the output
it exists to time."* After this branch that sentence is false.

It matters concretely here. The new `dynamic-param-shadows-import` and
`dynamic-param-shadows-import-edges` fixtures import `./vars/zIndex.stylex.js`
and friends, but the bench builds `StateManager::new(StyleXOptions::default())`
— no filename — and `CheckModuleResolution::default()` is `CommonJs { root_dir:
None }`. The repo's own helper documents what that costs
(`tests/utils/transform.rs:187-190`: *"Resolving one takes both a real filename
and `haste` resolution … otherwise the case is about the path rather than about
what it asks"*). A bench **cannot** supply that filename —
`StateManager::plugin_pass` and `set_plugin_pass` are `pub(crate)`
(`state_manager.rs:417,657`), unreachable from `benches/`. So the
theme-reference half of both new fixtures deopts, and the legs partly price a
refusal: the "fast because the work stopped happening" trap `PERFORMANCE.md`
opens with.

```rust
// evaluate_bench.rs:218-235 — old
    for fixture in fixtures {
      group.bench_function(fixture.name, |b| {
        b.iter(|| {
          let mut state = StateManager::new(StyleXOptions::default());
          fill_top_level_expressions(black_box(&fixture.module), black_box(&mut state));
          fill_top_level_var_declarations(black_box(&fixture.module), black_box(&mut state));

          for expression in &fixture.expressions {
            black_box(evaluate(
              black_box(expression),
              black_box(&mut state),
              black_box(&functions),
            ));
          }
        })
      });
    }
```

```rust
// new
    for fixture in fixtures {
      assert_folds_something(&fixture, &functions);

      // Batched because the state cannot be reused: `seen` memoizes what it
      // folded, so a second iteration against one state would time the memo.
      // Building it is setup, not work under measurement -- it allocates the
      // options, walks the module twice, and on this branch reads the
      // evaluation-depth environment behind a `OnceLock`, none of which is
      // `evaluate`.
      group.bench_function(fixture.name.clone(), |b| {
        b.iter_batched(
          || fixture_state(&fixture),
          |mut state| {
            for expression in &fixture.expressions {
              black_box(evaluate(
                black_box(expression),
                black_box(&mut state),
                black_box(&functions),
              ));
            }
          },
          BatchSize::SmallInput,
        )
      });
    }
```

New helpers, above `evaluate_benchmarks` (add `criterion::BatchSize` to the
imports):

```rust
/// The state one iteration folds against.
fn fixture_state(fixture: &EvaluateFixture) -> StateManager {
  let mut state = StateManager::new(StyleXOptions::default());

  fill_top_level_expressions(&fixture.module, &mut state);
  fill_top_level_var_declarations(&fixture.module, &mut state);

  state
}

/// Panics unless the fixture folds at least one of its expressions confidently.
///
/// A refusal is fast, and a leg that got quick because the fold stopped
/// happening is indistinguishable from a win -- `guidelines/PERFORMANCE.md`.
/// This harness cannot give the state a filename (`plugin_pass` is
/// `pub(crate)`), so a theme reference it cannot resolve deopts; the count
/// printed on failure is what the fixture actually folds rather than what it
/// holds.
fn assert_folds_something(fixture: &EvaluateFixture, functions: &FunctionMap) -> usize {
  let mut state = fixture_state(fixture);

  let confident = fixture
    .expressions
    .iter()
    .filter(|expression| evaluate(expression, &mut state, functions).confident)
    .count();

  assert!(
    confident > 0,
    "the `{}` leg folded none of its {} expressions confidently, so it is timing a \
     refusal rather than a fold",
    fixture.name,
    fixture.expressions.len()
  );

  confident
}
```

**If `assert_folds_something` fires on the two shadowing fixtures, that is the
answer to the question, not an obstacle.** The honest resolution is then to move
those legs to a harness that can set a filename, or to state in the fixture
comment that only the shadowed-parameter arm is priced — and to correct ADR
0006's table either way. See [step 6](#4-️-execution-order).

---

### C5 — `key_fallback_benchmarks` would report the same numbers if its subject were deleted

`crates/stylex-transform/benches/evaluate_depth_bench.rs:232-253`

The group's entire meaning is the gap between 128 and 129 properties, and its
doc quotes that gap as a result (`8.5 µs against 2.7`), as does ADR 0005. The
boundary is `MAX_UNSPANNED_HASH_COLLECTION_LEN = 128`
(`stylex-utils/src/hash.rs:24`), a private constant the bench never consults.
Raise it to 256 — a plausible tuning change, since ADR 0005 records the arm
being taken on a 130-colour palette — and both legs take the in-place arm, the
gap collapses to nothing, and the group reports a flat pair that looks exactly
like a win.

```rust
// evaluate_depth_bench.rs:245 — old
    let object = tower_expr(&parse(&source));

    group.bench_function(format!("object/{props}"), |b| {
      b.iter(|| black_box(stable_hash_unspanned(black_box(&object))))
    });
```

```rust
// new
    let object = tower_expr(&parse(&source));

    // The gap between the two legs is the arm's price, and only while they take
    // different arms. The fallback is `stable_hash_wide(&drop_span(clone))`, so
    // a leg whose key equals that value took it -- the same identity
    // `stylex_utils`' `hash_test.rs` pins the arms with. Without this, raising
    // the collection limit past 129 would leave both legs on the fast arm and
    // report the collapse as a flat pair.
    let took_fallback =
      stable_hash_unspanned(&object) == stable_hash_wide(&drop_span(object.clone()));

    assert_eq!(
      took_fallback,
      props > 128,
      "an object of {props} properties {} the fallback arm, so this group is no \
       longer pricing the boundary between the two arms",
      if took_fallback { "took" } else { "did not take" }
    );

    group.bench_function(format!("object/{props}"), |b| {
      b.iter(|| black_box(stable_hash_unspanned(black_box(&object))))
    });
```

Imports: `stylex_utils::hash::{stable_hash_unspanned, stable_hash_wide}` and
`swc_core::ecma::utils::drop_span`.

---

### C6 — The evaluation-depth env var is never exercised, and an ambient value breaks two suites

`crates/stylex-structures/src/evaluation_depth.rs:47-53,181-191`;
`.config/nextest.toml`

`resolve_from` / `parse_depth` are exemplary — every precedence pair, the trim,
the zero refusal, eleven unusable spellings, `+8`, the overflow spelling. But
the only test touching the public entry point,
`the_public_resolver_reads_the_process_environment`, computes
`env::var(MAX_EVALUATION_DEPTH_ENV)` on **both** sides of its `assert_eq!`, so it
holds for any value including none. Rename the constant to
`STYLEX_MAX_EVAL_DEPTH` and the entire suite passes while the escape hatch
documented in `rs-compiler/README.md:494` is dead.

Separately: `core_stylex_options.rs:99` calls `resolve_max_evaluation_depth(None)`
from `Default`, so every default-options test inherits the machine's
environment. On a box or runner that exports the variable, the new 1,077-line
`tests/transform_stylex_create_test/evaluation_depth_budget.rs` (which pins
behaviour at exactly 32) fails wholesale with messages naming CSS output, not
the environment — and the three new JS specs at `__test__/index.spec.ts:365-389`
do the same across the NAPI boundary. Confirmed: `.config/nextest.toml` has no
`[env]` block.

```rust
// evaluation_depth.rs:47 — old
pub fn resolve_max_evaluation_depth(configured: Option<usize>) -> usize {
  static FROM_ENV: OnceLock<Option<String>> = OnceLock::new();

  let from_env = FROM_ENV.get_or_init(|| env::var(MAX_EVALUATION_DEPTH_ENV).ok());

  resolve_from(configured, from_env.as_deref())
}
```

```rust
// new
pub fn resolve_max_evaluation_depth(configured: Option<usize>) -> usize {
  static FROM_ENV: OnceLock<Option<String>> = OnceLock::new();

  let from_env = FROM_ENV.get_or_init(read_env);

  resolve_from(configured, from_env.as_deref())
}

/// One read of the documented variable.
///
/// Split out from the cache so a test can prove *which* variable seeds it
/// without depending on when the cache was first filled -- a test that went
/// through the public resolver would be order-dependent under plain
/// `cargo test`, where the whole crate shares one process.
fn read_env() -> Option<String> {
  env::var(MAX_EVALUATION_DEPTH_ENV).ok()
}
```

```rust
// evaluation_depth.rs:181-191 — old
  #[test]
  fn the_public_resolver_reads_the_process_environment() {
    assert_eq!(
      resolve_max_evaluation_depth(None),
      resolve_from(None, env::var(MAX_EVALUATION_DEPTH_ENV).ok().as_deref())
    );
  }
```

```rust
// new
  // `cargo nextest run` gives each test its own process, which is what makes
  // the write below safe; it is the only environment write in the crate, and it
  // is undone before the test returns. Written against `read_env` rather than
  // the public resolver because the cache is filled once per process, so a test
  // that went through it would answer differently depending on test order.
  #[test]
  fn the_cached_read_takes_the_documented_variable() {
    // SAFETY: this test owns its process under nextest, and nothing else in
    // this binary reads the environment.
    unsafe { env::set_var(MAX_EVALUATION_DEPTH_ENV, "  256 ") };

    assert_eq!(read_env().as_deref(), Some("  256 "));
    assert_eq!(resolve_from(None, read_env().as_deref()), 256);

    // SAFETY: as above.
    unsafe { env::remove_var(MAX_EVALUATION_DEPTH_ENV) };

    assert_eq!(read_env(), None);
  }

  #[test]
  fn the_public_resolver_answers_the_default_with_nothing_configured() {
    assert_eq!(
      resolve_max_evaluation_depth(None),
      resolve_from(None, read_env().as_deref())
    );
  }
```

```toml
# .config/nextest.toml — old
[store]
dir = "target/nextest"
```

```toml
# new
[store]
dir = "target/nextest"

# The evaluator's ceiling is read from the environment when a project does not
# configure one, and the depth-budget suite pins the built-in default. An
# ambient value on a developer box or a CI runner would move every expectation
# in it, so the variable is pinned for the run rather than inherited.
[env]
STYLEX_MAX_EVALUATION_DEPTH = { value = "32", force = true }
```

---

### C7 — Two `key_span_index` regressions would pass the whole suite silently

`key_span_index.rs:96` and `:320-339`

**(a) `resolve` is never given a candidate worse than the incumbent.** Every
multi-candidate fixture presents candidates in *improving* order
(`key_span_index_test.rs:131,168,221,279`). Replace the `Some(_) => {}` arm with
`Some(_) => { best = Some((rank, candidate.span)); ambiguous = false; }` — a
last-wins slip — and all of them still pass, while a `dev` build starts pointing
every frame at the last object in the file spelling the key. Line 91's
`ambiguous = false` (a tie followed by a strict improvement) is unreached too.

**(b) `call_digest` / `CallLookup` have no test at all.** `call_digest` feeds
`compute_key_span_cache_key` (`build_code_frame_error.rs:425`), which keys
`state.cached_span` — a cache ADR 0005 lists as one of the four that **return on
a hash hit alone**, and whose failure it describes as *"directly visible in the
output as a style annotated with another style's `file:line`"*. Drop
`call_expr.span` and the object span from the tuple at lines 334-336 and every
test passes, while two `create` calls in one file with identical namespace names
share one cache entry.

```rust
// crates/stylex-transform/src/shared/structures/tests/key_span_index_test.rs
// (add `CallLookup` to the import on line 18)

  /// Every other multi-candidate case here presents its candidates in improving
  /// order, so a loop that simply took the last one would pass them all. This is
  /// the half that says the incumbent is kept.
  #[test]
  fn a_later_candidate_that_ranks_lower_does_not_displace_the_best() {
    let source = "\
const first = stylex.create({
  root: { display: 'flex', flexGrow: 1 },
});
const second = stylex.create({
  root: { color: 'red' },
});
";

    assert_eq!(
      resolved_line(source, "root", &["root"], &["display", "flexGrow"]),
      line_of(source, "root: { display")
    );
  }

  /// And that a strict improvement *clears* an earlier tie rather than only
  /// outranking it -- `resolve`'s `ambiguous = false` on the improvement arm.
  #[test]
  fn a_strict_improvement_clears_an_earlier_tie() {
    let source = "\
const first = stylex.create({ root: { color: 'red' } });
const second = stylex.create({ root: { color: 'red' } });
const third = stylex.create({
  root: { display: 'flex', flexGrow: 1 },
});
";

    assert_eq!(
      resolved_line(source, "root", &["root"], &["display", "flexGrow"]),
      line_of(source, "root: { display")
    );
  }

  /// Two calls that differ only in where they are written must not share a span
  /// cache entry, and one call must digest the same twice. Nothing else asserts
  /// this, and a collision here is a frame pointing at the wrong `create` --
  /// `cached_span` returns on the key alone, with no structural confirm.
  #[test]
  fn the_call_digest_separates_two_calls_and_is_stable_for_one() {
    let module = parse(
      "\
const first = stylex.create({ root: { color: 'red' } });
const second = stylex.create({ root: { color: 'red' } });
",
    );

    let calls = collect_create_calls(&module);
    let first = CallLookup::new(&calls[0], module.span.lo);
    let second = CallLookup::new(&calls[1], module.span.lo);

    assert_ne!(
      first.digest(),
      second.digest(),
      "two calls at different positions must key apart"
    );
    assert_eq!(
      first.digest(),
      CallLookup::new(&calls[0], module.span.lo).digest(),
      "one call must digest the same however often it is asked"
    );
  }
```

---

## 2. 🟡 Optimization / Performance / Complexity

### O1 — The process-global `SOURCE_MAP` retains a full copy of every module *per compile*

`build_code_frame_error.rs:557` — flagged independently by two reviewers.

`register_source_once` (`:113-140`) exists to stop the global map accumulating
module text, and its doc claims the list *"grows by one entry per module a
process transforms"*. `memoize_module` bypasses it, calling
`code_frame.source_map.new_source_file(...)` directly and unconditionally — and
`swc_common` documents that method as "does not ensure that only one SourceFile
exists per file name". It always appends.

A Next.js dev server with `debug: true` in one long-lived process: every save
recompiles the edited module and appends another full copy of its text to a
`OnceLock` that is never cleared. A 200 KB module saved 500 times retains
100 MB, monotonically — and `BytePos` is a `u32`, so the address space is finite
too. This is also what makes [C1](#c1--keyspanindex-ranks-candidates-by-a-distance-between-two-unrelated-coordinate-systems)
degrade over a session rather than only across files.

```rust
// build_code_frame_error.rs:555 — old
    let source_file = code_frame
      .source_map
      .new_source_file(Arc::new(file_name.clone()), source_code.clone());
```

```rust
// new
    // Re-registering an unchanged name appends another copy of the text to a map
    // that is never cleared, so a watch-mode process leaks one module per save.
    // A file already in the map is reused; a changed one is registered afresh,
    // and the content compare is what makes the reuse safe.
    let source_file = match code_frame.source_map.get_source_file(file_name) {
      Some(existing) if existing.src.as_str() == source_code => existing,
      _ => code_frame
        .source_map
        .new_source_file(Arc::new(file_name.clone()), source_code.clone()),
    };
```

The better long-term shape is to drop the `OnceLock` and give each `CodeFrame`
its own `SourceMap` scoped to the transform — nothing outside one file's
compilation reads it, and the whole class of growth (and C1 with it) goes away.
Either way, correct the doc at `:118-122`, which currently reads as a guarantee
the code does not make:

```rust
// old
  /// The guard is a linear scan of the source map's file list, which grows by one
  /// entry per module a process transforms.
// new
  /// The guard is a linear scan of the source map's file list. The list grows by
  /// one entry per *distinct content* a process registers, which for an
  /// unchanging file is one -- see `memoize_module`, which is the registration
  /// this guard has to agree with.
```

### O2 — Four immutable `FxHashSet<Id>` where there was one, in a struct deep-cloned per callback invocation

`state_manager.rs:459-495`

`binding_writes` became `binding_reassignments` + `binding_mutations` +
`binding_deep_mutations` + `declared_bindings`. All four are assigned exactly
once from the `Discover` pre-scan (`visit_mut_module.rs:744-747`, `:772-775`)
and never mutated afterwards outside tests. But `StateManager` derives `Clone`
and `arrow_function.rs:62` does `traversal_state.clone()` **once per invocation
of a dynamic style's callback** — so the diff quadrupled a per-invocation copy
of the module's entire binding population. `declared_bindings` alone is *every*
declared `Id` in the file.

```rust
// state_manager.rs:459 — old
  pub(crate) binding_reassignments: FxHashSet<Id>,
  pub(crate) binding_mutations: FxHashSet<Id>,
  pub(crate) binding_deep_mutations: FxHashSet<Id>,
  pub(crate) declared_bindings: FxHashSet<Id>,
```

```rust
// new
  // Behind an `Rc` for the reason the parsed module is: filled once by the
  // `Discover` pre-scan and read-only afterwards, in a struct a dynamic style's
  // callback clones once per invocation. `declared_bindings` alone is every
  // binding the module declares, so copying the four of them made a callback's
  // cost scale with the size of the file it sits in.
  pub(crate) binding_reassignments: Rc<FxHashSet<Id>>,
  pub(crate) binding_mutations: Rc<FxHashSet<Id>>,
  pub(crate) binding_deep_mutations: Rc<FxHashSet<Id>>,
  pub(crate) declared_bindings: Rc<FxHashSet<Id>>,
```

`Rc::new(collector.binding_reassignments)` at the four assignment sites;
`Rc::make_mut` in `resolution_order.rs`'s test seams. The four accessors
(`has_binding_reassignment`, `has_binding_mutation`, `has_deep_binding_mutation`,
`declares_binding`) need no change — they already take `&self`.

The same clone also copies `declarations: Vec<VarDeclarator>` (deep AST clones)
and `seen: FxHashMap<u128, Rc<SeenValue>>`. `declarations` is the bigger fish and
is also write-once-per-declarator, so the treatment generalises — see
[O6](#o6--statedeclarations-is-scanned-linearly-per-reference-and-built-in-on).

### O3 — The tail refusal clones both declaration lists, on the hot dynamic-style path

`evaluate/binding.rs:415-432`, `js/check_declaration.rs:31-54`

The comment says the two `Vec<Ident>` clones are *"paid for only on the refusal
path"*, which reads as rare. It is not: an unresolved-identifier refusal is
exactly how a dynamic style's parameter is detected.
`evaluate_stylex_create_arg` folds the arrow body with the parameters *not*
registered, so every parameter reference in every dynamic style reaches line
420, allocates two `Vec<Ident>` sized to the module's whole class + function
declaration list, and `check_ident_declaration` linearly re-scans both — after
lines 325-331 already scanned the same two lists one step earlier. A module with
40 functions and 30 dynamic-style parameter references does roughly 2,400
redundant `eq_ignore_span` calls and 60 vector allocations for nothing.

```rust
// binding.rs:415 — old
  // Cloned out of the state, because the refusal below writes to it: a
  // declaration-kind refusal records the binding whose declaration its frame
  // names, and the borrow checker cannot hold the declaration lists open across
  // that write. Two `Vec<Ident>` of the module's `class` and `function` names,
  // paid for only on the refusal path.
  let class_names = traversal_state.class_name_declarations().to_vec();
  let function_names = traversal_state.function_name_declarations().to_vec();

  check_ident_declaration(
    ident,
    &[
      (DeclarationType::Class, class_names.as_slice()),
      (DeclarationType::Function, function_names.as_slice()),
    ],
    state,
    traversal_state,
    normalized_path,
  )
}
```

```rust
// new
  // The declaration kind is resolved to a `Copy` verdict before the refusal
  // writes to the state, so the lists are read where they live: holding a
  // `&[Ident]` open across `deopt_at_declaration`'s `&mut StateManager` is what
  // the two clones were paying for, and a `DeclarationType` borrows nothing.
  let declared_as = declares_ident(traversal_state.class_name_declarations(), ident)
    .then_some(DeclarationType::Class)
    .or_else(|| {
      declares_ident(traversal_state.function_name_declarations(), ident)
        .then_some(DeclarationType::Function)
    });

  check_ident_declaration(ident, declared_as, state, traversal_state, normalized_path)
}

/// Whether `declarations` holds the binding `ident` names -- the same `Id`
/// comparison every other step of this chain makes.
fn declares_ident(declarations: &[Ident], ident: &Ident) -> bool {
  declarations
    .iter()
    .any(|declared| declared.eq_ignore_span(ident))
}
```

```rust
// check_declaration.rs:14 — old
pub(crate) enum DeclarationType {
  Class,
  Function,
}
```

```rust
// new
#[derive(Clone, Copy)]
pub(crate) enum DeclarationType {
  Class,
  Function,
}
```

```rust
// check_declaration.rs:31-54 — old
pub(crate) fn check_ident_declaration(
  ident: &Ident,
  declarations_map: &[(DeclarationType, &[Ident])],
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  path: &Expr,
) -> Option<EvaluateResultValue> {
  for (decl_type, declarations) in declarations_map {
    if declarations.iter().any(|item| item.eq_ignore_span(ident)) {
      return deopt_at_declaration(
        path,
        &ident.sym,
        state,
        traversal_state,
        &match decl_type {
          DeclarationType::Class => unsupported_expression("ClassDeclaration"),
          DeclarationType::Function => unsupported_expression("FunctionDeclaration"),
        },
      );
    }
  }

  deopt(path, state, UNDEFINED_CONST)
}
```

```rust
// new
pub(crate) fn check_ident_declaration(
  ident: &Ident,
  declared_as: Option<DeclarationType>,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  path: &Expr,
) -> Option<EvaluateResultValue> {
  match declared_as {
    Some(decl_type) => deopt_at_declaration(
      path,
      &ident.sym,
      state,
      traversal_state,
      &match decl_type {
        DeclarationType::Class => unsupported_expression("ClassDeclaration"),
        DeclarationType::Function => unsupported_expression("FunctionDeclaration"),
      },
    ),
    None => deopt(path, state, UNDEFINED_CONST),
  }
}
```

The rustdoc on `check_ident_declaration` (`:19-30`) stays as written — it
describes which node each arm reports against, which is unchanged.

### O4 — Every `stylex.*` member read allocates a `String` and an `Atom` where it used to allocate nothing

`member_expression.rs:535,546`

The `FunctionConfigMap` arm used to match
`Some(EvaluateResultValue::Expr(Expr::Ident(ident)))` and probe with
`&ident.sym` — zero allocations. It now routes through `as_string_key()`
(`Some(ident.sym.to_string())`) and re-interns with `Atom::from(name.as_str())`.
This runs for `stylex.create`, `stylex.props`, `stylex.keyframes`,
`stylex.when.*`, `stylex.env` and `stylex.defineVars` — every namespace member
expression in every file. The widening to computed keys (`stylex["when"]`) is
correct and worth keeping; it just need not cost the identifier path two heap
allocations.

```rust
// member_expression.rs:535 — old
          let name = property.as_ref().and_then(|prop| prop.as_string_key());

          // ...
          if let Some(name) = &name
            && let Some(entry) = fc_map.get(&Atom::from(name.as_str()))
          {
```

```rust
// new
          // The identifier spelling is the overwhelming majority of these reads
          // and already holds an interned `Atom`, so it is taken without a round
          // trip through `String`; every other spelling still goes through the
          // one reading `as_string_key` decides, so `stylex["when"]` resolves the
          // entry `stylex.when` does.
          let name: Option<Atom> = match property.as_ref() {
            Some(EvaluateResultValue::Expr(Expr::Ident(ident))) => Some(ident.sym.clone()),
            other => other
              .and_then(|prop| prop.as_string_key())
              .map(|key| Atom::from(key.as_str())),
          };

          // ...
          if let Some(name) = &name
            && let Some(entry) = fc_map.get(name)
          {
```

The fall-through at `:552` (`read_fold_member(..., property.as_ref(), ...)`) is
unchanged, and `fold_entry_value`'s `convert_key_value_to_str(key_value) == key`
comparison keeps working against `name.as_str()`.

Interacts with [C3](#c3--numeric-computed-keys-are-spelled-with-rusts-display-not-js-tostring):
once `as_string_key` uses `to_js_string`, this fast path must stay the `Atom`
clone — identifiers are unaffected by numeric spelling, so the two changes are
independent and compose.

### O5 — `depth_refused` de-memoizes siblings, not just the unwind

`evaluate/cache.rs:68-77,150`; `state_manager.rs:515-522`

The field's doc says the conservatism *"lasts exactly as long as the unwind that
earned it"*. It does not: it is cleared only when a new **top-level** fold begins
(`evaluation_depth == 0`). So once one namespace inside a `stylex.create` hits
the ceiling, every subsequent unconfident subtree in the same top-level fold —
sibling namespaces, unrelated properties, none of them ancestors of the refusal
— is barred from recording its refusal and gets re-walked. Safe (it only ever
memoizes less), but it turns one deep property into a whole-call
de-memoization, which is the opposite of what the ceiling is for.

A monotone counter answers the exact question — "did a depth refusal happen
inside *my* subtree" — at the same cost, and drops the reset entirely.

```rust
// state_manager.rs:522 — old
  /// ... Cleared when a
  /// new top-level fold begins, so the conservatism lasts exactly as long as the
  /// unwind that earned it.
  pub(crate) depth_refused: bool,
```

```rust
// new
  /// How many depth refusals this transformation has raised, ever.
  ///
  /// A counter rather than a flag, because the question each frame has to answer
  /// is whether a depth refusal happened *inside its own subtree* -- a frame
  /// snapshots this before recursing and compares afterwards. A flag could only
  /// say "somewhere since the last reset", which barred a sibling evaluated
  /// after the refusal from recording a refusal of its own, and needed a reset
  /// at `evaluation_depth == 0` to be bounded at all.
  pub(crate) depth_refusals: u64,
```

```rust
// state_manager.rs:627 — old
      depth_refused: false,
// new
      depth_refusals: 0,
```

```rust
// cache.rs:68-77 — old
  if traversal_state.evaluation_depth == 0 {
    // A new top-level fold. Whatever the previous one refused, its unwind is
    // over, so the frames of this one are free to record their own answers.
    traversal_state.depth_refused = false;
  }

  if traversal_state.evaluation_depth >= ceiling {
    traversal_state.depth_refused = true;

    return deopt(path, state, &expression_too_deep(ceiling));
  }
```

```rust
// new
  if traversal_state.evaluation_depth >= ceiling {
    traversal_state.depth_refusals = traversal_state.depth_refusals.wrapping_add(1);

    return deopt(path, state, &expression_too_deep(ceiling));
  }
```

```rust
// cache.rs:138-150 — old
    None => {
      let val = _evaluate(path, state, traversal_state, fns);

      if state.confident {
        // ... resolved: true ...
      } else if !traversal_state.depth_refused {
```

```rust
// new
    None => {
      // Snapshotted so the guard below asks about *this* subtree rather than
      // about the whole top-level fold: a sibling evaluated after a refusal is
      // not inside the unwind that earned it and may record its own answer.
      let refusals_before = traversal_state.depth_refusals;
      let val = _evaluate(path, state, traversal_state, fns);

      if state.confident {
        // ... resolved: true ...
      } else if traversal_state.depth_refusals == refusals_before {
```

Behaviour on the case the guard exists for is identical — an ancestor of the
refusal always sees the counter move. The long comment at `cache.rs:30-45`
explaining *why* ancestors must be left out stays accurate and should be kept,
with `depth_refused` renamed in its text.

### O6 — `state.declarations` is scanned linearly per reference, and built in O(n²)

`evaluate/binding.rs:296-302`; `shared/utils/common.rs:140-148,462-479`

`get_var_decl_from` is `state.declarations.iter().find(..)` over a
`Vec<VarDeclarator>`, and `get_var_decl_by_ident` runs the same scan again and
then `.clone()`s the whole declarator — name pattern, type annotation and the
entire initializer subtree — even when step 5 (`binding.rs:308`) or step 7
(`:395`) refuses and the clone is dropped unread. When `has_deep_binding_mutation`
fires, lines 297 and 302 scan the list twice back to back. The list is itself
built in `fill_state_declarations` with an O(n) `eq_ignore_span` deep AST
comparison per insert, so the structure is O(n²) in the module's declarator count
before a single style is folded.

The chain's own comments claim the guards *"cost no scan of the declaration list
at all"* — true of steps 3 and 4, not of the deep-mutation step or of steps 5
and 8.

The `Vec`'s source order is load-bearing for `find_top_level_expr` /
`queue_insertion`, so add an index beside it rather than replacing it.

```rust
// state_manager.rs, beside `declarations` — new
  /// Position in [`Self::declarations`] of the declarator binding each named
  /// `Id`, so a reference resolves with one hash probe instead of a scan.
  ///
  /// Only `Pat::Ident` declarators appear: they are the only shape
  /// `get_var_decl_from` ever matched. First writer wins, which is the same
  /// answer the linear scan's `find` gave.
  pub(crate) declaration_index: FxHashMap<Id, usize>,
```

```rust
// common.rs:462-479 — new, appended inside the existing `if !…any(..)` body
    if let Pat::Ident(binding) = &decl.name {
      state
        .declaration_index
        .entry(binding.id.to_id())
        .or_insert(state.declarations.len());
    }

    state.declarations.push(decl.clone());
```

```rust
// common.rs:140-148 — old
pub(crate) fn get_var_decl_from<'a>(
  state: &'a StateManager,
  ident: &'a Ident,
) -> Option<&'a VarDeclarator> {
  state
    .declarations
    .iter()
    .find(|var_declarator| matches_ident_with_var_decl_name(ident, var_declarator))
}
```

```rust
// new
pub(crate) fn get_var_decl_from<'a>(
  state: &'a StateManager,
  ident: &'a Ident,
) -> Option<&'a VarDeclarator> {
  // One hash probe rather than a scan of every declarator in the module. The
  // `Vec` keeps its source order, which `find_top_level_expr` and
  // `queue_insertion` both depend on; this only says where in it to look.
  let position = *state.declaration_index.get(&ident.to_id())?;

  state.declarations.get(position)
}
```

Separately, `get_var_decl_by_ident` should clone only what step 8 consumes
(`Option<Box<Expr>>`) rather than the whole declarator; step 5 needs nothing but
`declarator.span`, which is `Copy`.

### O7 — `KeySpanIndex` indexes every call in the module, not only StyleX calls

`key_span_index.rs:159-167`

`visit_call_expr` indexes any call whose first argument is an object literal. In
an application module that is `useMemo`, `createSlice`, `defineConfig`,
`test.each({...})`. Two costs: the index holds candidates that can never be the
answer, and — since the callee is not part of `IndexedCandidate` — a foreign
object spelling the same namespace key can tie with the real one and drive
`resolve` to `DUMMY_SP`, sending the lookup back to the O(module) value walk the
index exists to eliminate.

Concrete: `const cfg = createSlice({ root: { color: 1 } });` beside
`stylex.create({ root: { color: 'red' } })` — same key, same value key, tie on
both overlap fields, so the answer is decided by C1's broken distance metric or
refused outright.

```rust
// key_span_index.rs:41 — old
struct IndexedCandidate {
  span: Span,
  namespace_value_keys: Vec<Atom>,
  sibling_keys: Rc<Vec<Atom>>,
  candidate_offset: u32,
}
```

```rust
// new
struct IndexedCandidate {
  span: Span,
  namespace_value_keys: Vec<Atom>,
  sibling_keys: Rc<Vec<Atom>>,
  candidate_offset: u32,
  /// The callee the candidate's object was written as an argument to, shared
  /// with the object's other candidates.
  ///
  /// The index walks *every* call with an object first argument, because
  /// narrowing it to StyleX would mean teaching it the import bindings. That is
  /// cheap enough, but a `useMemo({ root: … })` beside a `stylex.create({ root:
  /// … })` ties on both overlap fields and refuses the lookup, so the callee is
  /// the first thing ranked on.
  callee: Rc<Callee>,
}
```

```rust
// CandidateRank:189 — old
pub(crate) struct CandidateRank {
  pub(crate) namespace_value_overlap: usize,
  pub(crate) sibling_overlap: usize,
  pub(crate) distance_from_target: Reverse<Option<u32>>,
}
```

```rust
// new — the derived `Ord` makes field order the precedence, and `true > false`
pub(crate) struct CandidateRank {
  /// Whether the candidate was written as an argument to the call being placed.
  /// First, because an object handed to some other function is not a worse
  /// answer than the right one -- it is not an answer.
  pub(crate) callee_match: bool,
  pub(crate) namespace_value_overlap: usize,
  pub(crate) sibling_overlap: usize,
  pub(crate) distance_from_target: Reverse<Option<u32>>,
}
```

```rust
// rank — new first field
      callee_match: query
        .callee
        .is_some_and(|callee| self.callee.eq_ignore_span(callee)),
```

with `index_object` taking `Rc::new(call.callee.clone())` once per object (beside
`sibling_keys`), and `NamespaceKeyQuery` gaining
`pub(crate) callee: Option<&'a Callee>` fed from `CallLookup`'s
`call_expr.callee`.

### O8 — Every debug annotation pays a full structural hash once any refusal is recorded

`build_code_frame_error.rs:253-259,324-332`

`framed_declaration_of` short-circuits on `has_framed_declarations()` — the right
guard. But `deopt_at_declaration` (`deopt.rs:33-44`) records a framed declaration
for **non-fatal** deopts too, and a non-fatal deopt is the normal outcome for a
dynamic style that falls through to an inline style. Once one has happened, every
subsequent `get_span_from_source_code` in that module pays an extra
`stable_hash_wide` over the whole target expression subtree — the cost the guard
was added to avoid, defeated by the commonest input.

```rust
// build_code_frame_error.rs:253 — old
  let framed_declaration = framed_declaration_of(target_expression, state);
  let cache_key = match framed_declaration.as_ref() {
    Some(name) => compute_declaration_cache_key(compute_cache_key(target_expression), name),
    None => compute_cache_key(target_expression),
  };
```

```rust
// new
  // Hashed once, not once per branch: the expression key is the input to both
  // the framed-declaration lookup and to whichever cache key comes out of it.
  // A build that recorded no framed declaration never hashes here at all.
  let expression_key = state
    .has_framed_declarations()
    .then(|| compute_cache_key(target_expression));
  let framed_declaration = expression_key.and_then(|key| state.framed_declaration(key).cloned());
  let cache_key = match (expression_key, framed_declaration.as_ref()) {
    (Some(key), Some(name)) => compute_declaration_cache_key(key, name),
    (Some(key), None) => key,
    (None, _) => compute_cache_key(target_expression),
  };
```

`framed_declaration_of` (`:324`) then has no caller and should be deleted; its
rustdoc's point ("the read side of `frame_declaration_of`, and the only one")
moves to the comment above.

### O9 — Three redundant allocations in the hottest CSS loop

**(a) The caller copies the pseudo list twice.**
`shared/utils/core/convert_style_to_class_name.rs:47`

`sort_pseudos` takes `&[String]` and copies internally; the caller copies first,
into a `&mut` binding it never mutates. This runs once per property per namespace
per `create` call, and the new `sort_pseudos` (which clones each key into a
`PseudoRun`) makes the redundant copy strictly more expensive than it was on
`develop`.

```rust
// old
  let unsorted_pseudos = &mut pseudos.to_vec();
  let sorted_pseudos = sort_pseudos(unsorted_pseudos);
// new
  let sorted_pseudos = sort_pseudos(pseudos);
```

`pseudos: &mut [String]` coerces to `&[String]`, and it is still passed unsorted
to `generate_css_rule` further down, so nothing else changes.

**(b) `sort_pseudos` pays the run partition even when there is nothing to
partition.** `stylex-css/src/utils/pre_rule.rs:43`

The overwhelmingly common input has no pseudo *element* — `[':hover']`,
`[':hover', ':focus']`, `['[data-x]', ':hover']`. For those the function still
allocates a `Vec<PseudoRun>`, a `Vec<String>` inside the single run, clones every
key into it, then allocates a third `Vec` and moves everything across.

```rust
// pre_rule.rs:46 — new, immediately after the `len() < 2` early return
  // With no element in the list the partition below produces exactly one
  // `Sortable` run covering every key, so this is the same answer for three
  // fewer allocations -- and it is the shape almost every key path has.
  if !pseudos.iter().any(|pseudo| is_pseudo_element(pseudo)) {
    let mut sorted = pseudos.to_owned();
    sorted.sort_unstable_by(|a, b| pseudo_comparator(a, b));

    return sorted;
  }
```

**(c) `class_names_for_prop` builds a throwaway `Vec<String>` and an unreachable
panic.** `transform_stylex_create_call/dynamic_style_functions.rs:36-50,86`

`class_strings` materializes a `String` per class, and the last entry is a pure
`cls.clone()` that `create_string_expr` then copies again into an `Atom`.

```rust
// old — delete the whole `class_strings` block (lines 36-50)
  // Pre-calculate class strings with spaces to avoid repeated allocations
  let class_strings: Vec<String> = class_list
    .iter()
    .enumerate()
    .map(|(index, cls)| {
      if index == class_list.len() - 1 {
        cls.clone()
      } else {
        let mut spaced = String::with_capacity(cls.len() + 1);
        spaced.push_str(cls);
        spaced.push(' ');
        spaced
      }
    })
    .collect();
```

```rust
// old, inside the loop
    let cls_with_space = &class_strings[index];
// new — the separator is appended lazily, and the last class borrows
    let cls_with_space: Cow<'_, str> = if index + 1 == class_list.len() {
      Cow::Borrowed(cls.as_str())
    } else {
      Cow::Owned(format!("{cls} "))
    };
```

(`create_string_expr` takes `&str`, so `&cls_with_space` works unchanged at both
call sites.)

```rust
// dynamic_style_functions.rs:80-89 — old
  let joined = if expr_list.is_empty() {
    create_string_expr("")
  } else {
    expr_list
      .into_iter()
      .reduce(|acc, curr| create_bin_expr(BinaryOp::Add, acc, curr))
      .unwrap_or_else(|| {
        stylex_panic!("Expected at least one expression to reduce in class name concatenation.")
      })
  };
```

```rust
// new — `reduce` already answers `None` for exactly the empty case the `if`
// tested, so the two branches were the same question asked twice and the
// `unwrap_or_else` arm was unreachable.
  let joined = expr_list
    .into_iter()
    .reduce(|acc, curr| create_bin_expr(BinaryOp::Add, acc, curr))
    .unwrap_or_else(|| create_string_expr(""));
```

### O10 — `FunctionConfigType::EnvObject` compares unequal to itself

`shared/structures/functions.rs:165`

`(EnvObject(_), EnvObject(_)) => false` was a defensible shrug when the variant
held a bare `IndexMap` (deep-comparing an env object per equality probe). Now
that the payload is `Rc`, the honest answer is one pointer compare — and since
the whole point of the change is that there is exactly one such object per
compile, `ptr_eq` is not an approximation but the correct answer for every value
the compiler builds.

```rust
// old
      (Self::EnvObject(_), Self::EnvObject(_)) => false,
```

```rust
// new
      // One pointer compare, and an exact answer rather than a conservative one:
      // the `env` option's object is shared from the options, so two
      // `EnvObject`s are the same object or they are not.
      (Self::EnvObject(a), Self::EnvObject(b)) => Rc::ptr_eq(a, b),
```

### O11 — Two bench-harness defects

**(a) `transform_debug_bench.rs:30-32` describes a cost the timed loop does not
pay.**

Both halves of the claim are wrong, and the file half-knows it (lines 249-252
call the source map "process-global and keyed by file name").
`register_source_once` short-circuits on the global `OnceLock`, and
`assert_annotates_every_namespace` runs a full `dev` transform before the group —
so by the time criterion starts timing, the read is already amortized to zero
across every iteration. Under the bench's default `use_real_file_for_source:
false` the source never comes off disk at all.

```rust
// old
//! The `file:line` lookup *does* re-read and re-parse the module from disk on
//! its first miss, once per transform, because that is what the compiler does
//! too -- it is part of what the annotation costs, not harness overhead.
```

```rust
// new
//! What the timed loop does **not** include is registering the module's source
//! for position lookup. `register_source_once` short-circuits on a
//! process-global `OnceLock` keyed by file name, and the guard run above the
//! group has already filled it -- so the read is paid once for the whole run,
//! not once per iteration. In a real `dev` build it is also usually not a disk
//! read: the memoized parsed source is preferred, and `use_real_file_for_source`
//! is off here as it is there. What the legs price is the per-namespace lookup
//! against an already-registered source, which is where the quadratic lived.
```

**(b) `module_path_bench.rs:583-589` takes the rest of the file down with it.**

The comment two lines above says a `const` assertion was rejected because it
*"would refuse to compile the bench in a debug tree, where every other group in
this file is still worth running"*. The runtime `assert!` unwinds out of
`module_path_benchmarks` and kills `StructuralKey`, `StateManager` and
`FullPipeline` anyway — exactly the outcome the reasoning was avoiding.

```rust
// old
  assert!(
    !debug_assertions_on,
    "this group is only meaningful in a build with debug assertions off: the \
     memoized-source clone it exists to price is forced on under \
     `cfg!(debug_assertions)`, so both settings would clone and the difference \
     against `ModuleWalk` would read as zero"
  );
```

```rust
// new
  if debug_assertions_on {
    // Skipped rather than asserted: a panic here unwinds out of
    // `module_path_benchmarks` and takes the three groups below with it, which
    // is the outcome the note above says a `const` assertion was avoided to
    // prevent. The group's absence from the report is the signal.
    eprintln!(
      "skipping `SeenModuleSource`: the memoized-source clone it exists to price is \
       forced on under `cfg!(debug_assertions)`, so both settings would clone and the \
       difference against `ModuleWalk` would read as zero"
    );

    return;
  }
```

### O12 — `parity:positions` exits 0 when it compared nothing

`parity/parity-positions.ts:291-294`

The value harness treats an empty selection as an error
(`parity-values.ts:151-154`); the position harness returns cleanly. `pnpm
parity:positions --filter improt` (a typo), or a `corpus/positions.json` whose
`entries` array was emptied by a bad merge, prints "No subjects matched." and
exits 0 — a passing position-parity run over zero subjects.

```ts
// old
  if (entries.length === 0) {
    console.log(chalk.yellow('No subjects matched.'));
    return;
  }
```

```ts
// new
  // An empty selection is a broken invocation or a broken corpus, not a pass.
  // The value harness already treats it that way; this one returned cleanly and
  // reported parity over nothing.
  if (entries.length === 0) {
    console.error(chalk.red('No position subjects match the given filter.'));
    process.exit(1);
  }
```

### O13 — FFI: the whole options object is re-marshalled per file

`rs-compiler/src/lib.rs:202-299` — **observation, not a regression on this
branch. File as a separate issue; do not land here.**

Every `transform()` call deserializes ~40 `StyleXOptions` fields out of JS
(`importSources`, `aliases`, `unstable_moduleResolution`, the `env` `JsObject`
walk), converts them through `StyleXOptionsParams` into `CoreStyleXOptions`, and
throws the result away — for a value that is constant for the entire build. This
diff adds one scalar to that and does not make it worse. A `#[napi] struct
Compiler` created once with the options and exposing `compile(filename, code)`
would remove it, and would also let `input_source_map` stop round-tripping
through a JSON `String` per file.

---

## 3. 🟢 Nitpick / Readability / Testing

### Correctness-adjacent

**N1 — the deep-mutation step sits ahead of step 5, breaking the invariant the
block states.** `binding.rs:288-300`

The comment says: *"What the extra reach is **not** allowed to do is change an
answer that already agreed."* Placed between steps 4 and 5 it does exactly that
for one shape:

```js
import * as stylex from '@stylexjs/stylex';

const styles = stylex.create({ x: { color: theme.a.b } });

const theme = { a: { b: 'red' } };
theme.a.b = 'blue';
```

Upstream 0.19.0: `constantViolations` is empty (a member write is not one),
`isMutated` is false, so the position comparison at `:664` wins →
`Referenced value is used before declaration.` Here: `has_deep_binding_mutation`
is true and `get_var_decl_from` finds the declarator →
`Referenced value is not a constant.` Both refuse and both frame the same
declaration, so no build changes — but the sentence the author reads is wrong
about why, and the file's own rule says a divergent step must not reorder an
answer the two compilers already agreed on.

```rust
// binding.rs:296-302 — old
  if traversal_state.has_deep_binding_mutation(ident)
    && get_var_decl_from(traversal_state, ident).is_some()
  {
    return deopt_at_declaration(path, &ident.sym, state, traversal_state, NON_CONSTANT);
  }

  let declarator = get_var_decl_by_ident(ident, traversal_state, &state.functions);
```

```rust
// new — the declarator lookup takes this position …
  let declarator = get_var_decl_by_ident(ident, traversal_state, &state.functions);
```

```rust
// … and the probe moves below the hoisted-declaration check that ends at :340
  // Placed behind step 5 rather than beside step 4, because it is the one step
  // upstream does not have: a reference above its own declaration is early on
  // both sides, and refusing it here for a write would answer differently from
  // upstream on an input the two already agree about. Asked of a
  // `VarDeclarator` -- the `declarator` above -- rather than of the binding,
  // for the reason the paragraph below gives.
  if declarator.is_some() && traversal_state.has_deep_binding_mutation(ident) {
    return deopt_at_declaration(path, &ident.sym, state, traversal_state, NON_CONSTANT);
  }
```

This also removes O6's duplicate `get_var_decl_from` scan. It needs a companion
test: `resolution_order.rs:280-341` has `.reassigned()` and `.mutated()` but no
`.deeply_mutated()` builder, so nothing at the chain's own level pins where the
step sits relative to steps 5 and 8.

**N2 — `reads_before_its_declaration`'s unreachability claim is reachable.**
`binding.rs:52-55`

```js
function makeStyles() {
  return stylex.create({ a: { color: makeStyles } });
}
```

The reference sits after `makeStyles`'s *name* ends but before the *function*
ends, so `:325` says "not early" and the chain falls to step 8's `Unsupported
expression: FunctionDeclaration`. Upstream compares against
`binding.path.node.end` — the whole `FunctionDeclaration` — and answers
`Referenced value is used before declaration.` Message-only.

```rust
// old
/// A hoisted declaration is compared against the end of its *name* rather than
/// the end of its body, which is what upstream's `binding.path.node.end` is. The
/// two part company only for a reference from inside the declaration's own body,
/// which no style value reaches.
```

```rust
// new
/// A hoisted declaration is compared against the end of its *name* rather than
/// the end of its body, which is what upstream's `binding.path.node.end` is. The
/// two part company for a reference from inside the declaration's own body --
/// `function f() { return create({ a: { color: f } }) }` is early upstream and
/// falls through to the `FunctionDeclaration` refusal here. Both refuse, so only
/// the sentence differs; closing it means recording the declaration's whole span
/// beside its name, which `declarations_state` does not carry today.
```

**N3 — `CoreStyleXOptions.max_evaluation_depth` is a `pub usize` accepting `0`.**
`core_stylex_options.rs:59`

The builder and every JS path guard zero, but a struct-update literal
`CoreStyleXOptions { max_evaluation_depth: 0, ..Default::default() }` compiles and
makes `evaluate_cached` refuse *every* expression at `cache.rs:74`, including the
folds the compiler runs for itself. Either type it `NonZeroUsize`, or clamp at the
read site:

```rust
// cache.rs:66 — new
  // `.max(1)` because the field is a bare `usize` a struct-update literal can
  // set to zero, and a ceiling of zero refuses the compiler's own folds.
  let ceiling = traversal_state.options.max_evaluation_depth.max(1);
```

**N4 — `any_level_needs_a_default`'s doc contradicts its body.**
`define_vars_utils.rs:182-196`

The doc closes with *"and stops descending into a CSS type, whose `value` is its
own shape and not a map of at-rules"*. The body does no such thing:
`object_needs_a_default` recognises the `syntax`+`value` pair and returns
`false`, and then the `.any(...)` recurses into **every** key of that same
object, `syntax` and `value` included. Harmless today, but the comment describes
a guard that is not there.

```rust
// old
  if object_needs_a_default(obj) {
    return true;
  }

  get_key_values_from_object(obj)
    .iter()
    .any(|key_value| any_level_needs_a_default(&key_value.value))
}
```

```rust
// new
  if object_needs_a_default(obj) {
    return true;
  }

  // A CSS type's `value` is its own shape, not a map of at-rules, so the
  // recursion stops here rather than reading `syntax` and `value` as levels.
  if css_type_keys_present(obj) {
    return false;
  }

  get_key_values_from_object(obj)
    .iter()
    .any(|key_value| any_level_needs_a_default(&key_value.value))
}
```

where `css_type_keys_present` is the `syntax && value` half of
`object_needs_a_default`, extracted so the two cannot drift.

**N5 — `namespace_value_keys` does not do what its comment says.**
`key_span_index.rs:357-372`

The doc says *"The first such property wins"*, but the `find_map` closure returns
`None` both for a property that does not name the namespace **and** for one that
names it with a non-object value — so a later same-named property with an object
value is still matched. Harmless today (the comment's own justification is that a
compiled call cannot repeat a key), but the code and the comment disagree.

```rust
// new
fn namespace_value_keys(object: &ObjectLit, namespace_key: &str) -> FxHashSet<Atom> {
  object
    .props
    .iter()
    .find(|prop| {
      prop_as_key_value(prop).is_some_and(|key_value| {
        namespace_name_from_prop_key(&key_value.key)
          .is_some_and(|name| name.as_ref() == namespace_key)
      })
    })
    .and_then(prop_as_key_value)
    .and_then(|key_value| match key_value.value.as_ref() {
      Expr::Object(value) => Some(collect_object_lit_keys(value).collect()),
      _ => None,
    })
    .unwrap_or_default()
}
```

**N6 — a `.js` file containing a TS type alias emits a dangling export.**
`lib.rs:365-368`. With `verbatim_module_syntax: true`,
`strip_module_items_with_semantic`'s `ExportNamed` arm is skipped, so
`type A = number; export { A };` in a `.js` file drops the alias and keeps
`export { A }` — a module that fails to link. Only reachable for TS syntax in a
JavaScript file, which is malformed input the pipeline happens to parse. Add a row
to the spec's "still stripped from a JavaScript module" table asserting the
non-`type` spelling, or a scope sentence on `is_javascript_input`'s doc.

**N7 — `InjectableStyleKind::rule_text()` enshrines an ltr/rtl preference upstream
lacks.** `injectable_style.rs:26-38`. Upstream (`visitors/stylex-create.js`,
`nullishVarExpressions`) takes `ltr` whenever it is a string, and
`generateCSSRule` always produces a non-empty `ltr`, so upstream never reads
`rtl` here. The Rust helper substitutes `rtl` whenever `ltr` is *empty*. Faithful
to the inlined code it replaced, so not a regression — but promoting it to a
public method invites new callers to inherit a rule with no upstream counterpart.
Add a line to the method's doc saying the empty-`ltr` arm is this compiler's
directional-rule spelling, not upstream's fallback, and is unreachable from
`generate_css_rule` output.

**N8 — documented, measured divergence: non-ASCII condition keys hash differently
from Babel.** `stylex-css/src/utils/pre_rule.rs:144-171`. `pseudo_comparator`
ranks every non-ASCII byte above every printable-ASCII character, where root
collation places an accented letter beside its base letter. So
`{'[data-état]': {…}, '[data-f]': {…}}` nested in one key path sorts one way here
and the reverse in Babel — a different class-name hash for the same source. It is
named and pinned in both test suites. The framing to add to the doc is that the
consequence is **cross-compiler class-name divergence** in mixed
Babel/rs-compiler builds, not merely an ordering curiosity — that is what decides
whether a real collation dependency is ever worth taking on.

**N9 (pre-existing, adjacent) — `const_rules` are sorted before
`generate_css_rule` sees them.** `shared/structures/pre_rule.rs:127`. Upstream's
`get constRules()` returns the filtered key path **unsorted**; only
`convertStyleToClassName` sorts, and only for the hash. `generateCSSRule`
receives the unsorted list and wraps the declaration in that order. The Rust
getter sorts, so a style nesting two `var(--…)` const keys in non-alphabetical
source order emits a different nesting order than Babel. Class-name hash
unaffected. Not introduced by this branch; flagged because `sort_at_rules` is in
the diff and the redundant sort is also wasted work.

### Docs

**N10 — `compute_key_span_cache_key`'s rustdoc is a merge artifact.**
`build_code_frame_error.rs:417-424` — two doc comments spliced together, ending
on a bare `///`.

```rust
// old
/// The same, for a namespace-key lookup. 128 bits for the same reason as
/// [`compute_cache_key`].
///
/// Hashed as one tuple rather than field by field, so the wide hasher is built
/// once and the pieces cannot drift out of the key by being added to the
/// function and forgotten in the digest.
/// The per-namespace half, mixed with the digest above.
///
```

```rust
// new
/// The per-namespace half of a key-span cache key, mixed with the call digest
/// from [`CallLookup::digest`]. 128 bits for the same reason as
/// [`compute_cache_key`].
///
/// Hashed as one tuple rather than field by field, so the wide hasher is built
/// once and the pieces cannot drift out of the key by being added to the
/// function and forgotten in the digest.
```

**N11 — `register_source_once`'s doc promises a return value it does not have.**
`build_code_frame_error.rs:113-114` — "and reports whether the file is available
afterwards" describes a `Result<bool, _>`; it returns `Result<(), Error>`.

```rust
// new
  /// Registers `source` for `file_name` unless the shared source map already
  /// holds it. Returns `Err` only when producing the source itself failed.
```

**N12 — `CandidateRank`'s `Reverse<Option<u32>>` doc states the opposite of what
the ordering does.** `key_span_index.rs:183-186` — "no measured distance outranks
every measured one". `None < Some(_)`, so `Reverse(None) > Reverse(Some(_))`: an
*unmeasured* distance outranks every measured one. Moot in practice (the `Option`
comes from the query and is uniform across one `resolve`), but it is load-bearing
prose for a `derive(Ord)`.

```rust
// new
/// call's other namespace keys it spells, then how close it is written to the
/// compiled call. The distance is `Reverse`d so a nearer candidate wins. The
/// `Option` is uniform across one `resolve` -- it comes from the *query*, not
/// from the candidate -- so `Reverse(None)` outranking every `Reverse(Some(_))`
/// never mixes measured and unmeasured candidates in one comparison: a call with
/// no position of its own leaves every candidate tied here and the overlap
/// fields decide alone.
```

**N13 — `module_path_bench.rs:692`** says the group times `StateManager`
"construction **and drop**". Criterion's `iter_batched` retains the routine's
outputs and drops the batch after the measurement, so drop is outside the timed
region. Drop "and drop" from the sentence.

**N14 — ADR 0006 cites a patch path no other clone can reach.** Its headline
consequence is *"the work is kept, not just described"*, pointing at
`.scratch/fix_dynamic-param-shadows-import/issues/33-composed-key.patch`.
Confirmed: `.scratch` is a **symlink outside the repository** (`git check-ignore`
reports "beyond a symbolic link") and `CLAUDE.md` states it is never committed.
The ADR's stated audience — *"the next person to ask this question should measure
a variant, not rebuild the base"* — is precisely someone who will not have that
worktree. Either commit the patch under
`crates/stylex-transform/docs/adr/attachments/`, or restate the consequence as
"the mechanism is described here; the base has to be rebuilt".

**N15 — markdown line width.** `CLAUDE.md` sets 80 for markdown.
`crates/stylex-structures/CONTEXT.md:65` is 160 columns;
`crates/stylex-transform/CONTEXT.md` has 18 lines over 80 where `develop` had 1.

```markdown
<!-- old -->
`STYLEX_MAX_EVALUATION_DEPTH` environment variable, then the built-in default of 32. Precedence in that order so a stray value in a CI environment cannot change

<!-- new -->
`STYLEX_MAX_EVALUATION_DEPTH` environment variable, then the built-in default
of 32. Precedence in that order so a stray value in a CI environment cannot
change
```

Also: `stylex-transform/CONTEXT.md`, "Folded function map" uses `--` where every
neighbouring entry uses an em dash.

**N16 — two fixture comments describe things that do not exist.**
`benchmark/perf_fixtures/props-and-attrs.js:14-16` points a reader at an
`.input.map.json` that was correctly never written (per `PERFORMANCE.md`);
`benchmark/perf_fixtures/logical-rtl.js:6` claims it is measured under the
polyfill option, but no manifest entry names `enableLogicalStylesPolyfill` and it
is not in `BOOLEAN_OPTION_KEYS`, because `PERFORMANCE.md` records it changes
nothing on this fixture. Correct both comments to say so.

**N17 — `binding.rs:176`'s resolved-import arm is exercised only by
`validation_stylex_create_test::theme_reference_style_values`**, not by
`resolution_order.rs` (whose `StateManager` has no filename, so every import
resolves `Unresolved`). Naming that in the module header stops a future reader
assuming this file covers it.

### Tooling

**N18 — the Stage-2 review hook is satisfied by any prior invocation.**
`.agents/hooks/require-stage2-review.sh:43-51`

The `reviewed` probe scans the whole transcript for *any* `code-review` Skill
call — before the commit, on unrelated code, or aborted mid-run. A session that
opens with a review and then commits four unreviewed changes passes forever.
Given the hook exists precisely because *"no amount of instruction has reliably
prevented"* the failure, ordering matters.

```bash
# new — replace both jq probes (lines 33-51) with one pass that keeps positions
positions=$(jq -s '
  [ .[] | .message?.content? | arrays[]?
    | select(.type == "tool_use")
    | if .name == "Bash" and ((.input.command // "")
        | test("(^|[;&|(]\\s*)git\\s+(-\\S+\\s+|\\S+=\\S+\\s+|-C\\s+\\S+\\s+)*commit"))
      then "commit"
      elif .name == "Skill" and (.input.skill // "") == "code-review"
      then "review"
      else empty end
  ]
' "$transcript" 2>/dev/null || echo '[]')

# A review before the last commit is not a review of it.
needs_review=$(jq -r '
  (map(. == "commit") | index(true)) as $any_commit
  | if $any_commit == null then false
    else ((to_entries | map(select(.value == "commit")) | last | .key)
          > ((to_entries | map(select(.value == "review")) | last | .key) // -1))
    end
' <<<"$positions" 2>/dev/null || echo false)

if [ "$needs_review" = "true" ]; then
```

**N19 — the hook fails open on a malformed transcript.** Same file, `:14`.
`set -euo pipefail` means a malformed transcript line makes `jq -s` exit
non-zero, aborting the script before either probe runs — so the hook silently
stops guarding. The `|| echo` fallbacks in N18's snippet close this.

**N20 — the commit regex misses `git -C some/dir commit`.** Same file, `:39`:
`git\s+(-\S+\s+|\S+=\S+\s+)*commit` — `some/dir` matches neither alternative.
The `-C\s+\S+\s+` alternative in N18's snippet closes it.

### Test quality

**N21 — an assertion that cannot fail.** `key_span_index_test.rs:338`:
`assert_eq!(rank(1, 3, Some(5)), rank(1, 3, Some(5)))` is `x == x` under a derived
`PartialEq`. It also does not test what its comment claims — ambiguity is tested
by `two_equally_good_candidates_resolve_to_nothing`.

```rust
// old
    // Identical signals rank equal, which a lookup reports as ambiguous.
    assert_eq!(rank(1, 3, Some(5)), rank(1, 3, Some(5)));
// new
    // Identical signals rank equal, which a lookup reports as ambiguous. That
    // the *lookup* does so is `two_equally_good_candidates_resolve_to_nothing`;
    // asserting it of the derived `PartialEq` here would be `x == x`.
```

**N22 — an ordering no input can reach.** Same file, `:335`:
`rank(1, 3, None) > rank(1, 3, Some(0))`. `distance_from_target` is `None`
exactly when `query.target_lo` is, which is shared by every candidate in one
`resolve`. Harmless, but it reads like coverage of a live branch. Fold it into
N12's comment rather than asserting it.

**N23 — a test that passes for the wrong reason.**
`declaration_span_tests.rs:202-205`. `a_member_assignment_target_is_not_a_declaration`
passes because `[holder.c] = pair;` parses as an assignment expression, not
because of the `Pat::Expr` arm — flip that arm to `true` and it still passes.
Rename to `a_name_spelled_only_as_an_assignment_target_declares_nothing`, and note
in `declaration_span.rs:292-294`'s comment that `Pat::Expr` / `Pat::Invalid` are
unreachable from a parsed declarator.

**N24 — the new NAPI depth specs assert only that something compiled.**
`__test__/index.spec.ts:370,376`. `.toContain('$$css')` is true of any successful
compile; it does not say the tower folded to the right number. The Rust boundary
tests next door do this correctly. `deepFixture(n)` is `MY_CONST = 5` plus `n`
increments, so `deepFixture(100)` folds to `105` and `deepFixture(10)` to `15`.

```ts
// new — beside `compileAtDepth`; the tuple shape mirrors line 332's destructuring
const injectedCss = (result: ReturnType<typeof compileAtDepth>) =>
  result.metadata.stylex.map(([, rule]) => rule.ltr).join('');
```

```ts
// old
  expect(compileAtDepth(source, 320).code).toContain('$$css');
// new — `$$css` is present for any successful compile; the folded value is the
// thing under test, and it is what a ceiling that silently stopped applying
// would change.
  expect(injectedCss(compileAtDepth(source, 320))).toContain('z-index:105');
```

```ts
// old
  expect(compileAtDepth(source).code).toContain('$$css');
// new
  expect(injectedCss(compileAtDepth(source))).toContain('z-index:15');
```

```ts
// old
  expect(compileAtDepth(deepFixture(29)).code).toContain('$$css');
// new
  expect(injectedCss(compileAtDepth(deepFixture(29)))).toContain('z-index:34');
```

**N25 — leaked temp directories.** `build_code_frame_error_tests.rs:29`. The
`AtomicUsize` + pid fixture directories under `std::env::temp_dir()` are
collision-free across processes and threads but never removed; a full run leaks
one directory per fixture test. Wrap in a guard type whose `Drop` calls
`std::fs::remove_dir_all` and ignores the error.

### Coverage gaps to close

| Module | Untested behaviour |
|---|---|
| `key_span_index.rs` | incumbent-kept vs last-wins (C7a); `call_digest` / `CallLookup` at all (C7b); `CallLookup::new`'s position fallback and the `create(someVar)` no-object path (`:269-274`, `:281-283`); a non-StyleX call spelling the same key (O7); **two modules in one `SourceMap`** — the configuration in which C1 is visible |
| `declaration_span.rs` | `Pat::Rest` and `Pat::Assign` (`:290-291`) — `const [first, ...c] = pair`, `const { token: c = 'red' } = theme`, `const [c = 'red'] = pair`; `export default class` (`:72-73`); a name whose declaration `strip()` synthesized |
| `evaluation_depth.rs` | which variable seeds the cache (C6); the clamp (C2) |
| `binding.rs` | the deep-mutation step's position relative to steps 5 and 8 — no `.deeply_mutated()` builder exists (N1) |
| `evaluate_result_value.rs` | numeric key spelling: `[10,20][-0]`, `({'1e-7':'x'})[1e-7]` (C3) |
| diagnostics | astral-plane column parity — Babel's `loc.column` counts UTF-16 code units, SWC's counts Unicode scalar values, and the divergence is currently neither measured nor excluded; minified single-line source through both `declaration_span` and `key_span_index` |
| benchmark manifest | `sourceMap` / `classNamePrefix` parsing (`lib/fixtures.ts:206-219`) asserted nowhere — only `styleResolution` is; nothing binds `BOOLEAN_OPTION_KEYS` to actual fixture usage, which is how four dead keys got in |

The last row's fix, in `benchmark/__tests__/fixtures.test.ts`:

```ts
  // The other half of the allowlist rule: `fixture-manifest.test.ts` proves an
  // unknown key is refused; this proves no accepted key is dead.
  test('every allowlisted option key is used by a fixture', () => {
    const used = new Set(fixtures.flatMap(fixture => Object.keys(fixture.options ?? {})));

    for (const key of [...BOOLEAN_OPTION_KEYS, 'styleResolution', 'sourceMap', 'classNamePrefix']) {
      expect(used.has(key), `${key} is allowlisted but no fixture measures it`).toBe(true);
    }
  });
```

---

## 4. 🛠️ Execution order

Each step is independently landable and independently verifiable. Gate every one
on `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`, `pnpm test`; add
`pnpm lint:type-aware` for any TypeScript change. Rebuild `dist/*.node` before
any JS suite that imports `@stylexswc/rs-compiler` means anything.

| # | Scope | Why here |
|---|---|---|
| 1 | **C2** — options ceiling + boundary specs | Smallest blast radius, highest severity-to-risk. Nothing depends on it. |
| 2 | **C3** — `to_js_string` in `as_string_key` + two fold tests | One line and two tests. Verify against the parity corpus after. |
| 3 | **C6**, **N3** — env-var test, `[env]` pin, zero clamp | Do before everything else so the depth suites stop being environment-sensitive and later steps get honest failures. |
| 4 | **C7** + the `key_span_index` / `declaration_span` coverage rows | Mutation-detection tests written **before** the refactor they guard. |
| 5 | **C1** + **O1** | Same call path, and O1 makes C1 worse. Land file-relative offsets, or take the minimal alternative if the plumbing is deferred. The two-modules-in-one-`SourceMap` test from step 4 is the gate. |
| 6 | **C4**, **C5**, **O11** — bench integrity, then **re-run and correct ADR 0005 and ADR 0006's tables** | ⚠️ Most likely to change a claim already written down. If `assert_folds_something` fires, the shadowing fixtures were pricing a refusal and the ADR text must say so. |
| 7 | **O5**, **O3**, **O4**, **O8** — evaluator hot-path | Measure each against the now-trustworthy benches rather than assuming. |
| 8 | **O2**, **O6** — `Rc` the write-once state, add `declaration_index` | Largest refactor; last, with benches in place. |
| 9 | **O9**, **O10**, **O7**, **O12** | Remaining allocation and harness fixes. |
| 10 | **N1**, **N2**, **N4**–**N25** | Docs, comments, hook, test quality. Batch into one or two commits by area. |
| 11 | **O13** | File as a separate issue against `rs-compiler`. Do **not** land on this branch. |

Suggested commit shape, following the repo's conventional-commit scope
convention:

```
fix(stylex_structures): refuse a ceiling the boundary cannot represent      # 1
fix(stylex_transform): spell a numeric key the way the language does        # 2
test(stylex_structures): name the variable the cached read takes            # 3
test(stylex_transform): pin the candidate a second file resolves to         # 4
fix(stylex_transform): compare positions within one file, not across maps   # 5
perf(stylex_transform): make the evaluate bench prove it folded something   # 6
perf(stylex_transform): stop a sibling paying for another's depth refusal   # 7
perf(stylex_transform): share the write-once binding sets                   # 8
```

---

## Checked and cleared

Recorded so they are not re-reviewed. Each was investigated and refuted.

**The headline fix is correct.** Shadowing is genuinely scope-aware:
`Ident::eq_ignore_span` compares `sym` *and* `ctxt`
(`swc_ecma_ast-27.0.0/src/ident.rs:198-206`), the resolver runs ahead
(`transform/mod.rs:402`, `rs-compiler/src/lib.rs:353`), and
`get_import_by_ident`, `get_var_decl_from`, `declares_binding`, `has_binding_*`
and `class_name_declarations` are all keyed on the full `Id`. A dynamic-style
parameter carries its own `SyntaxContext` and cannot resolve to the import it
shadows. The memo cannot alias two shadowed names: `hash_ident_unspanned`
(`stylex-utils/src/hash.rs:487-492`) hashes `ctxt` before `sym`, and the fallback
arm's `drop_span` clears only `Span`, leaving `ctxt` in the derived `Hash`.
`get_var_decl_by_ident`'s `stylex_panic!` / `stylex_unimplemented!` arms are
unreachable from this chain.

**CSS semantics.** `sort_pseudos`' run-growth rewrite is exact upstream parity —
upstream's reduce grows a run to arbitrary length and sorts the whole run, where
the old Rust capped runs at two; the class-name movement in the `buttons-demo`
fixtures is the *fix*. `ASCII_PRIMARY_ORDER` was derived independently by sorting
the 95 printable ASCII characters with `localeCompare` on Node 24 / `en-US`:
character-for-character identical. `pseudo_comparator` was reimplemented and
fuzzed against `localeCompare` over **300,000 random printable-ASCII pairs: zero
disagreements**. `default` is genuinely unreachable from both comparators.
`to_int32` is spec-correct including the `fmod`-exact large-value path (`1e21|0`,
`3e9|0`, `-1.9|0`, `~[4294967296]` all match Node). The empty-class-list guard
removal is correct — upstream keeps the key, which is what makes `{color: null}`
unset an earlier merged declaration. Namespace fold key order matches upstream,
which matters because the first key decides which refusal an author reads.
`insert_stylex_identifier_entry` preserves the old semantics exactly.
`collect_vars_by_at_rules` now names the variable rather than the at-rule, which
matches upstream.

**Hashing.** The 128-bit widening cannot move a class name: `create_hash`
(murmur) is untouched, and `stable_hash_wide` / `WideHasher` feed only in-process
caches (evaluator memo, injection slot, span cache). The native-endian
`write_usize` inside the `Hash` impls therefore has no cross-platform
consequence. `Atom`'s `Ord` is `as_str().cmp(...)`, so `sorted_sibling_keys.sort()`
in `call_digest` is content-ordered, not pointer-ordered. `compute_cache_key`'s
claim that the hash includes the span is correct — `swc_common::Span` derives
`Hash` over `lo`/`hi`.

**Panics / FFI.** No path from the diagnostics subsystem can abort the Node
process: `locate_span_with_panic_boundary` (`:238-246`) and `emit_error`
(`:151-159`) both wrap in `catch_unwind`, `try_get_span_line_number` additionally
guards `DUMMY_SP`, `panic = "abort"` is deliberately not set
(`Cargo.toml:44-49`), and `lib.rs:238` catches again. There is no byte-slicing of
source text in production code — only in a test helper, which uses `source.get(..)`
and reports rather than panicking. Everything after `parse_env_object` /
`parse_debug_file_path` is inside the NAPI `catch_unwind`, and both of those
return `Result`. The depth counter does not leak on error paths — the ceiling is
checked *before* the increment, and the only non-decrementing exit is an
unwinding panic that no path resumes from with the same `StateManager`;
`stacker`'s catch-and-resume claim is accurate (`stacker-0.1.24/src/lib.rs:160-165`).
`OnceLock` for the env var is lock-free with no `Mutex` on any hot path, and
config still wins over the environment, so per-project overrides are intact.
`Rc<SeenModuleSource>` + `OnceCell<KeySpanIndex>` under `StateManager: Clone` is
sound — no path crosses a thread, and the shared cell means clones share one
built index, as documented.

**Import elision.** SWC's `verbatim_module_syntax` skips exactly
`strip_module_items_with_semantic`; `visit_mut_import_specifiers` still drops
inline `type` specifiers unconditionally and `retain.rs:76` still drops whole
`import type` declarations. Elision only ever *removes*, and an unrecognised
extension answers "TypeScript" — strictly additive for `.js`. The spec pins all
of it.

**Parity harness is sound.** Both compilers load eagerly and hard-fail
(`parity/lib/compilers.ts:14,38-44` — a missing upstream is a module-resolution
crash, not a skip); the corpus is 4 checked-in sets / ~1,000 subjects with a
throwing loader; the only `try`/`catch` around a run records the refusal as a
comparable outcome; `refusalSentence`'s stripping rules are pinned, including a
test that two different complaints stay unequal; verdicts compare joined strings,
not counts; `identical-empty` exists so agreement-about-nothing cannot read as
parity; both entry points set `process.exitCode = 1` on a changed expectation.
Message parity was verified byte-for-byte against upstream 0.19.0 for all five
new/uncommented constants (`IMPORT_FILE_EVAL_ERROR`, `USED_BEFORE_DECLARATION`,
`UNINITIALIZED_CONST`, `missing_default_value`, `MISSING_DEFAULT_VALUE_UNNAMED`).

**Not defects.** `KeySpanIndex` is genuinely lazy and genuinely one walk — a
`OnceCell` inside `SeenModuleSource` reached only from the debug path — and
`FxHashMap<Atom, Vec<_>>` is the right choice over a sorted `Vec` there; `Rc<Vec<Atom>>`
for sibling keys and `OnceCell<Expr>` for the wrapped call both avoid
per-namespace clones correctly. `.claude` is a tracked symlink to `.agents`, so
`.agents/settings.json`'s hook path resolves on a fresh clone. Zero `#[ignore]`d
Rust tests were added (the one `+#[ignore]` in the diff is inside prose). No
skipped or conditionally-no-op JS tests; a missing `dist/*.node` fails at import
because `benchmark/lib/types.ts` value-imports from it. `GLOBALS` / mark-allocator
sharing across the new unit tests is safe — each wraps in a fresh `Globals::new()`.
`get_import_by_ident` matching on the binding is sound, and dropping the old
*imported*-name match is right (`import { spacing as sp }` leaves `spacing`
unbound). `convert_expr_to_bool_wrapper` does not bypass production code — the
duplicate truthiness table was deliberately deleted and the wrapper is
user-supplied callback code in `env.rs` tests. `deopt`'s first-refusal-wins guard
means the new `template_literal` / `unary_expression` early returns cannot have
their message overwritten. `binding.rs`'s coverage via `resolution_order.rs` is
the standard the rest of the branch should be held to.
