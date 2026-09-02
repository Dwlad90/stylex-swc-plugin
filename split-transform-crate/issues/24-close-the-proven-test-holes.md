# 24 — Close the proven test holes in the new crates

**What to build:** Several of the new crates sit at 100% region coverage and
are still wrong, because coverage measures lines rather than behaviour. The
review proved four such holes by reading the tests.

The candidate index is generic and production instantiates it six ways, but
its tests instantiate three — and one of those carries a comment claiming to
stand for indices that are really keyed by something else. The coverage tool
keeps only the best-covered instantiation, so the generic reads as fully
covered while three real ones never run.

The file-offset helper has a saturating path that release ships, but every
test builds offsets through a test-only constructor that bypasses both it and
its debug assertion.

The diagnostics code-frame entry points are all generic over an injected
state trait, and the only implementation the crate can see is its own test
double — so the gate measures the double exclusively while the real
implementation lives in an excluded crate.

Three panic tests assert that a panic happened but not which message, while
the code under test routes three different inputs to three distinct messages;
one non-export code path is never asserted; and two evaluator error arms have
no direct test.

Finally, the corpus holds roughly 169 normalization-shaped CSS values that are
asserted only through whole-transform integration tests plus a generated
fixture, with no direct unit test anywhere.

**Blocked by:** 21

If this ticket overflows a single context, split the corpus normalization
criterion out first — it is the largest and the least coupled to the rest.

**Status:** resolved

- [x] The candidate index's untested production instantiations are exercised,
      including that dummy spans collide and that one name under two syntax
      contexts stays apart — five cases in `candidate_index_test`, and the
      misleading comment on `narrows_on_a_key_that_is_not_a_hash` is corrected
- [~] The file-offset saturating path is asserted, with its debug assertion
      covered too — one case per profile, since the two cannot both be
      reachable, and `FileOffset::of` is now `pub(crate)`, which still gates
      construction behind a `ModuleBase`. What remains: no command in this
      repository builds the Rust suites in release, so the clamp case is one a
      person has to reach for. Its doc says so and names the command. Wiring a
      release leg into the gate is a decision about the gate, not about this
      type — issue 19 is where the bench profile is already being settled, and
      it is the right home for it
- [x] The three message-blind panic tests assert their distinct messages
- [x] The non-export pattern-bound-call path is asserted
- [x] The evaluator's refused result and its two distinct declaration-check
      messages have direct tests — two new modules, `check_declaration_test`
      and `evaluate_result_test`
- [x] The nine diagnostics trait methods are exercised through a real state
      manager, including the seen-module source round-trip and the one rule
      the test double encodes: the key-span index is dropped when the module
      is replaced — `stylex-state/src/tests/diagnostic_state_test.rs`, every
      case through the trait rather than the inherent method
- [x] The corpus's normalization values gain direct unit tests, with expected
      values taken from a harness run rather than from the review document —
      85 corpus values asserted across three seams; see the Comments for how
      the set was derived and for five corpus entries that are not CSS
- [x] The review's own unclosed check is closed: the three rewritten test
      files are byte-diffed against the base branch to confirm no test *input*
      changed when its cases were renamed. The review matched them by scenario
      and assertion count only, so a subtle input change inside a rewritten
      case would not have been caught — and these tests are the refactor's
      central invariant — done over all 6,220 base cases rather than three
      files; see the Comments
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code

## Comments

### What closed the corpus criterion

The harness was run against the built addon and its report read for the
reference spelling of every entry:

```sh
pnpm run --filter=@stylexswc/rs-compiler build
pnpm run --filter=@stylexswc/rs-compiler parity -- --json parity/results/default.json
```

Of the 794 single-declaration entries the reference compiler completes, **193**
are ones a value pass rewrites — the rest come back as written, which the
existing `unchanged` tables already state as a shape. 104 of the 193 had no
assertion at the seam, and 90 of those 104 agree between the two compilers; the
other 14 are `acceptance-divergent`, where this compiler refuses a rule-breaking
token the reference compiler emits, which the `rejects` tables cover.

Of the 90: 68 are asserted in the new
`crates/stylex-css/src/css/tests/corpus_normalization_test.rs`, grouped by what
the rewrite does. 6 are content quoting and 10 are the logical-value polyfill,
which are different passes and are asserted at those passes — `transform_value`
in the compiler crate and `generate_ltr` beside its own suite. 5 are not CSS at
all (below). 1 is a ten-layer `boxShadow` whose subject
`normalizes_every_entry_of_a_long_list` already states over 500 layers.

Of the 10 logical-value values, 8 already had a case beside `generate_ltr`, so
4 were added — two of them a keyword in the *second* position of a two-keyword
value, which nothing covered. All 6 content values went into one case table in
`transform_value_test`. So **85** corpus values gained a direct assertion:
68 + 10 + 6, less the 1 dropped and counting the 8 that were already there.

**How the first attempt got this wrong, since it is the reusable part.** The
gap was first computed by scanning for the `same`/`unchanged` case-table
constructors alone, and three other shapes assert at the same seam: a direct
`normalize_css_property_value` call in `common_test`, `normalize_value_test`'s
own table — which takes value *before* property — and the two `rejects`
variants. That over-reported the gap by 26, and 29 of the first 94 cases
duplicated an existing assertion byte for byte, two whole groups of them
(`spells_a_zero_angle_in_degrees` and the `10ms` half of the millisecond group)
adding nothing at all. A scan for a seam's existing coverage has to know every
way the suite spells a call to it, not the one way the module being written
spells it.

### Five corpus entries the harvester read out of Rust source that is not CSS

Not asserted, because a case over one of them would state nothing about CSS:

| Entry | What it really is |
| --- | --- |
| `width: limit 64, found 65` | an assertion message in `value_normalization_parity_test` |
| `width: limit 64, found 5000` | the same, one case below |
| `width: , ` / `boxShadow: , ` | the separator argument of a `join(", ")` |
| `s: 0.25rem` | a JavaScript object key in an embedded test source |

The same class of defect as the ternary colon already recorded against the
harvester. Worth its own ticket: the harvester needs to know a Rust string
literal that reaches an `assert!` or a `join` from one that is a declaration.

### The rename check, done over the whole tree

The criterion named three files. The check was written against every `#[test]`
function in the tree instead — 6,220 at the merge base, 6,343 at HEAD — since
the same argument applies to any renamed case and the base file boundaries had
themselves moved. Each case body was read with a scanner that skips strings and
comments, because a plain brace count reads a `{` inside a string literal as a
block opening and walks the body past the end of the file; a first pass that
did that reported four false edits.

**Two cases kept their name and changed their inputs.** Both are accounted for:

- `get_canonical_file_from_root_dir` — the fixture path moved with the crate,
  `@stylexswc/transform:src/shared/structures/tests/...` to
  `@stylexswc/state:src/tests/...`. That *is* the input, and it had to change.
- `a_string_key_and_the_numeric_spelling_of_it_are_one_key` — inputs identical.
  The two extra literals are the strengthened assertion (`vec!["42"]`,
  `vec!["y"]` where the old case asserted only `result.len() == 1`).

**49 base case names are absent from HEAD.** Read one by one:

- The `remove_duplicates` and `assign_props` families moved to `stylex-ast`
  and were renamed. Every case maps one-to-one onto a HEAD case with the same
  input, and every assertion is stronger — `keys_of`/`number_value_of`/
  `inner_keys_of` where the old case asserted a length. One input differs:
  the spread payload in `an_old_spread_keeps_its_place` and
  `a_new_spread_keeps_its_place` is `1.0` where the old case wrote `0.0`,
  because the two base copies of the helper disagreed and the surviving one
  won. Immaterial — a spread names no key, so `assign_props` never reads the
  payload.
- `overlapping_non_object_value_appended` (old `x: 1`, new `x: 2`) has no
  same-input twin. `a_repeated_key_takes_the_later_value_in_the_place_it_first_took`
  covers the branch with different numbers and also pins the position the key
  keeps, which the old case did not.
- `panics_when_init_is_none` became
  `get_expr_from_var_decl_panics_without_initializer`, which renames the
  declarator from `x` to `a` and adds the message assertion. The name is not
  read on that path.
- The `evaluate_bin_expr` arithmetic cases existed twice at the base — once in
  the transform's `common_tests` and once in the evaluator's `common_test`.
  The split kept the larger set, now `stylex-js/src/tests/operators_tests.rs`
  (26 cases against 17). Every operator and scenario the dropped copy covered
  is still covered; some operand values differ where the two copies disagreed.
- The `resolve_node_package_path` cases went with their subject, which the
  split deleted as dead code.

No case was found whose input changed while its name and scenario claimed
otherwise.

### Where each criterion landed

| Criterion | Files |
| --- | --- |
| candidate index | `stylex-state-index/src/tests/candidate_index_test.rs` |
| file offset | `stylex-state-index/src/{key_span_index.rs,tests/key_span_index_test.rs}` |
| panic messages | `stylex-declarations/src/tests/lookup_tests.rs` |
| pattern-bound call | `stylex-state/src/tests/state_writers_test.rs` |
| evaluator refusals | `stylex-evaluator/src/tests/{check_declaration_test,evaluate_result_test}.rs` |
| diagnostics trait | `stylex-state/src/tests/diagnostic_state_test.rs` |
| corpus normalization | `stylex-css/src/css/tests/{corpus_normalization_test,generate_ltr_test}.rs`, `stylex-transform/src/shared/utils/css/tests/transform_value_test.rs` |
