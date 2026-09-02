# Review — ticket 27 (performance nits)

Three reviewers over the working tree before the commit: the two axes the
`code-review` skill runs, plus a performance-only reviewer.

## What changed because of the review

**The `SortedKeys` design was withdrawn.** The first attempt held a namespace's
value keys as a sorted, deduplicated `Vec` and answered membership from it with
`binary_search`, which removed the second collection the cache key used to
build. The performance reviewer pointed out that `Atom::Hash` writes one
precomputed word while `Atom::Ord` compares the strings, so membership would get
dearer on a path that runs once per candidate. Measured, it did -- 3x to 23x --
so the hash set stayed and only the ordering moved, to an inline `SmallVec`
sorted unstably. That is the ticket's own first suggestion, "a stack-allocated
small vector", and it removes the allocation with nothing traded for it. Numbers
in [bench/ticket-27.md](../bench/ticket-27.md).

**`builds_the_name_in_one_exactly_sized_allocation` was deleted.** Both the
standards and the spec reviewer read `assert_eq!(result.capacity(),
result.len())` as pinning a `String` implementation detail rather than
behaviour: `with_capacity` promises *at least* the capacity asked for, so a
standard-library change would redden a correct function. The one-allocation
shape is stated in the function's own documentation instead.

**The measured numbers were re-labelled.** The performance reviewer held the
record and the `CandidateIndex::record` documentation against
`guidelines/PERFORMANCE.md`: a best-of-N on an ad-hoc harness is not the
bootstrapped verdict a blocking comparison uses, and a difference of two
unpaired multi-millisecond runs is not a penalty curve. Both now say
"directional" and say what they are not.

**The span cache key gained a test and a note.** Nothing in the suite guarded
the ordering the key depends on; `code_frame_test.rs` now checks it with the
hasher the cache actually uses, including the case that outgrows the inline
buffer. A comment beside the `v5` version string records that the byte stream
did not move, since `SmallVec` hashes as a slice exactly as the `Vec` did.

## What was left as it is, and why

**The allocator item belongs to ticket 17.** "The moved benches link the
allocator the addon ships" is ticket 17, parked in `backlog` because it closes
every bench series in this effort and needs one clean re-baseline. Folding that
cost into this ticket would spend it without the re-baseline. The criterion
stays open here and names 17.

**`call_digest` still collects and sorts per call.** The performance reviewer is
right that the sibling keys are the larger set. They are also ordered once per
*call* rather than once per namespace, which is the cost this ticket names, and
the same measurement that withdrew `SortedKeys` says a sorted list is the wrong
shape for the membership test `rank` runs against those keys. Out of scope, and
not obviously a win.

**The very large test inputs stay.** The spec reviewer read the 100 000-segment
path case as padding. The request this work was done under asks for extremely
large inputs explicitly, and the case costs microseconds.

## Standing findings, not actioned

- `resolve` interns an `Atom` per namespace, and `rank` compares the callee with
  `eq_ignore_span` for every candidate. Both are plausibly dearer than either
  overlap and neither is this ticket's subject.
- No optimization in this change is registered under `bench:revisions`, so none
  of it is guarded by the blocking gate. That is true of the debug path as a
  whole and wants its own ticket rather than a line here.
