# 60 — Give the styleq cache a life longer than one merge

**What to build:** The merger this compiler builds has its cache off, because a
merger built inside the merge cannot hit one. Decide whether a merger that
lives longer is worth building, and if it is, answer the two caveats its keys
carry before it caches anything.

**How it got here.** `crates/stylex-transform/src/shared/utils/core/styleq.rs`
builds a `Styleq` inside the per-merge `styleq(arguments)` function and drops it
when the call returns. Instrumenting the lookup over the whole
`stylex-transform` suite and the fixture corpus read **1,021 lookups and 0
hits** -- every lookup paid for an `Arc<CacheEntry>`, three `Arc<str>`, a boxed
slice and a fresh `Arc<CacheNode>`, and collected nothing. The merge now sets
`disable_cache: true` (commit `1c491f333`), which is output-identical by
construction: the cache only answers what the walk would compute.

So the cache is correct, tested and unused. This ticket is about the other half
-- a caller that can keep one.

**Two caveats a long-lived merger must answer.** Both are written at
`crates/stylex-styleq/src/styleq.rs:26-43`:

1. **A reference key is an address.** `CacheKey::Identity(usize)` is the address
   of the style array, and it names that array only while the array is alive.
   Nothing evicts, so a `Styleq` that outlives the styles it cached can read a
   freed address back as a hit for whatever the allocator put there next --
   another style's class names, silently.
2. **Nothing evicts.** The chain holds one entry per distinct walked suffix, so
   a merger kept across many merges keeps every path it ever walked. The
   reference implementation is safe here only because a `WeakMap` dies with its
   keys; an `FxHashMap` does not.

A `WeakMap` has no direct equivalent, so an answer is a design choice rather
than a port: hold the style alive with the entry, key on something that is not
an address, or bound the cache.

**What would say it is worth it.** A hit rate over a real corpus, taken the way
`guidelines/PERFORMANCE.md` asks -- attributed against the parent commit, on an
idle machine, with a check on what the bench measured. The
`default_styleq` cache-hit rows in `performance_bench.rs` measure the cache, not
the compiler, so they cannot answer this on their own.

**Found by:** the review of `fix_benchmarks`, which measured the hit rate at
zero.

**Blocked by:** None.

**Status:** resolved

- [x] A hit rate over a real corpus says whether a longer-lived merger would
      hit at all -- 24% for the file, 64% for the process, 0% per merge
- [x] If it would: the address key is answered, so no freed address can read
      back as a hit -- answered by not caching. No merger outlives a merge, so
      no key outlives its style
- [x] If it would: the cache is bounded, or the entries die with the styles --
      the same answer: the merger is dropped when the merge returns, so the
      chain it built goes with it and nothing accumulates
- [x] The module comment at `styleq.rs:26-43` says which answer was taken

## Answer

**A merger that lives longer hits, and still does not pay. The cache stays
off.**

The hit rate was read by logging the key chain of every merge the transform
suite and the fixture corpus make, then replaying the chains against a trie for
each scope:

| Merger lives for  | Lookups | Hits | Rate  |
| ----------------- | ------- | ---- | ----- |
| one merge (today) |   1,076 |    0 |  0.0% |
| the file          |   1,076 |  255 | 23.7% |
| the process       |   1,076 |  692 | 64.3% |

Over the 817 merges of the run that belong to a file. The other 76 are unit
tests that call the merger directly, with no file to scope one to; pooling them
into a bucket of their own read the file rate as 14.0%, which is an artifact of
the bucket and not a measurement.

So the answer to "would it hit at all" is yes. What decides it is the key.

A merger that outlives the merge cannot key on the address of the style the
merge holds, because that address dies with the merge. Two keys were measured
end to end, against a generated module of 100 components that reads each of its
styles from nine `stylex.props` sites -- a shape that repeats far more than the
corpus does, where the file-scoped cache hits 88.9% of its lookups:

| Key      | Nine props per component | One props per component |
| -------- | ------------------------ | ----------------------- |
| hash     | 2.5% slower              | 1.4% slower             |
| address  | 0.7% faster              | 0.9% slower             |

The hash key loses because it walks every property of the style, which is the
work a hit saves. Read the address row against the corpus rather than against
the module it was taken on: 89% of lookups buy 0.7%, and a file of this corpus
answers about a quarter, which is nearer the leg that repeats nothing and lost
0.9%. The address key only became possible while this ticket was
open: the reader now shares the style the state holds instead of copying it, so
the address is the state's and lives for the file. It gains 0.7% where the file
repeats and loses 0.9% where it does not, and it still has to prove the style
alive -- an inline object argument owns its own style and dies with the merge.
A cache that needs a proof for that little is not worth building.

### How the numbers were taken

[`tools/styleq_scope.md`](../tools/styleq_scope.md) spells out both patches and
both commands, and [`tools/styleq_scope.py`](../tools/styleq_scope.py) is the
reader that turns the probe log into the table above. In short:

The hit rates come from a temporary probe, not from a bench: the walk was made
to key by hash, every lookup printed its key, each program and module visit
printed a file marker, and the whole `stylex_transform` suite ran
single-threaded. `styleq_scope.py` then walked the key chains against a
trie for each scope, and left out the merges that belong to no file. The probe is not in the tree -- it is four `eprintln!`
lines, one changed key and one option.

The two key timings come from a temporary criterion bench in
`crates/stylex-transform/benches`, linking `swc_malloc` as
`guidelines/PERFORMANCE.md` asks, with both legs in one process: the same parsed
module, transformed once with the merger built per merge and once with a merger
reset per file. Both legs printed the same program before timing, so the cache
was transparent. The machine read 89.5% idle before each run. The bench is not
in the tree either: with the long-lived merger gone it would measure one leg
twice, which is what tickets 57 to 59 removed from this repository.

A synthetic module was used for the key timings on purpose. The corpus says how
often a longer-lived merger would hit; the generated module says what the best
case is worth, because 8 of its 9 merges repeat where the corpus answers about
one lookup in four. A best case that gains under a percent settles the question
for the rest.

The marker log of the run is kept at
[`baseline/styleq-scope-probe.log`](../baseline/styleq-scope-probe.log), so the
reader can be checked against a known answer.

The copy the reader dropped on the way is worth keeping: one `stylex.props`
argument used to copy every name and value of the style it names. Removing it
gained 3.0% on the nine-props module, reproduced in two runs against a saved
criterion baseline. The one-props module read 0.7% faster, which that run calls
no change: it holds 200 arguments where the other holds 1,800, so the gain
tracks the arguments as expected. The atoms pass copied one compiled namespace
the same way, and now borrows it.

The style sharing is its own commit, because it is a change of behaviour where
this ticket's own outcome is a comment:

- `2ea7bf7b4` perf(stylex_transform): share a declared style instead of copying
  it
- `0ad00cb2e` docs(stylex_styleq): record what a longer-lived merger is worth
