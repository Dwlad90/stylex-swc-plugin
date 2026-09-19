# 77 — Restore the workspace coverage gate

**What went wrong:** The coverage batch of tickets 63 to 66 left
`pnpm run test:coverage:workspace` red. Seventeen regions over three crates
that are **on** the gate go unexercised, so the `tests-coverage` job in
`.github/workflows/pr-validation.yml` fails on this branch.

**Why it was not seen.** Each of the four tickets measured with
`scripts/coverage-missing.sh -p stylex_transform`, which reads one crate. The
gate reads the workspace with `stylex_transform` excluded, and that is a
different question: a helper the transform alone calls is covered in the first
reading and uncovered in the second.

**Where the seventeen are:**

| File                                   | Regions | What is uncovered                                                                                                                   |
| -------------------------------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `stylex-state/src/state_manager.rs`    | 12      | `cached_default_marker_values` and `insert_cached_default_marker_values`, the public pair and the two `CacheState` steps under them |
| `stylex-structures/src/pair.rs`        | 4       | `PairCow::as_css_text`                                                                                                              |
| `stylex-diagnostics/src/code_frame.rs` | 1       | the `<no message>` answer of `locate_span_with_panic_boundary`, for a panic that carries no text                                    |

The first two are new API that only the transform calls. The third is a
fallback no test reached at all.

**Blocked by:** None.

**Status:** resolved

- [x] Each of the three crates covers its own API with a unit test, rather than
      being covered from the crate above it.
- [x] `pnpm run test:coverage:workspace` reports zero uncovered lines, regions
      and functions.
- [x] The full workspace suite stays green.

## Comments

### What the gate reads now

`pnpm run test:coverage:workspace`, `rustc 1.100.0-nightly`:

| Figure    | Before | After   |
| --------- | ------ | ------- |
| Regions   | 99.92% | 100.00% |
| Functions | 99.79% | 100.00% |
| Lines     | 99.93% | 100.00% |
| Tests     | 6185   | 6191    |

### The tests, and what each one asserts

- **The two per-file memos.** The default marker memo asks the three answers a
  memo owes: nothing before it is written, the value it was given after, and
  the second value when it is written twice. It went beside the short-filename
  memo, which is the same shape -- and both then moved out of
  `binding_queries_test.rs` into `file_caches_test.rs`, because that module
  says every question in it is asked of a binding reference and a memo is not
  one.
- **`PairCow::as_css_text`.** Two cases. The first asserts the text **and** the
  borrow: `PairCow::borrowed` exists to spell a declaration without owning
  either half, and a pair that copied its halves would still give the right
  text, so the halves are compared by address. The second builds a `PairCow`
  that owns its halves, because the kind admits one and the text must not
  depend on where the halves are kept.
- **The panic boundary.** `locate_span_with_panic_boundary` reads a panic
  payload as a `String`, then as a `&str`, then gives up. There is a case for
  each: a panic with static words, a panic with words built where it is raised,
  and `panic_any(7u32)`, which carries no words at all. The three share one
  helper that checks the sentence around the detail and answers the detail, so
  the sentence is written once here and cannot drift from the one in
  `code_frame.rs`.

### The advice this leaves

Measure a coverage change with the workspace gate, not with
`coverage-missing.sh -p <crate>` alone. The per-crate reading counts a caller
in another crate, and the gate does not.

### What the review added

Three review passes ran over the change -- standards, spec and performance.
None found a defect in the code. What they found is above: the panic prefix was
written twice, the memo tests sat in a module whose own header excludes them,
and the `PairCow` case asserted only that the two kinds agree, which they
cannot fail to do while both call one reader. The performance pass reports one
cost it does not ask for: `stylex_merge.rs` clones the cached marker map on
every hit and again per import name. That reading was too narrow -- the `create`
path did not read the memo at all -- and the whole of it is
[78](./78-build-the-default-marker-once-for-the-file.md).
