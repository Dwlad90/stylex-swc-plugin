# 34 — Two consumers act on a bare hash hit

Status: `resolved`
Blocked by: None

**What was found.** While measuring 29. The `stylex-utils` glossary said of the
structural hash that "callers use it to narrow a candidate set and then confirm
with `eq_ignore_span` — the hash never decides equality by itself". Two of the
four callers do exactly that; the other two do not:

| consumer                    | on a hit                       |
| --------------------------- | ------------------------------ |
| `jsx_spread_attr_exprs_map` | confirms with `eq_ignore_span` |
| `queued_decl_items` dedup   | confirms with `eq_ignore_span` |
| the evaluator's `seen` memo | **returns the cached fold**    |
| `InsertionSlot::BeforeDecl` | **splices the metadata**       |

`evaluate_cached_within_budget` looks up `traversal_state.seen` by
`stable_hash_unspanned(path)` and returns `evaluated_value.value.clone()` with no
comparison against the expression the entry was recorded for. `flush_pending_insertions`
matches `decl_init_hashes(&item)` against its `before_decl` map the same way. So
a 64-bit collision is a **wrong folded value** or a declaration's style metadata
spliced before a different declaration — silently, in the output, with no
diagnostic.

The glossary line is corrected as part of 29. This ticket is the behaviour.

**Why it is not urgent.** SipHash over a structural walk, and the population is
the distinct expressions in one file: at 10 000 of them the birthday bound puts a
collision near 3e-12 per file. It has never been observed, and cannot be
reproduced by writing a source file — a case for it has to construct the
collision.

**Why it is worth a ticket anyway.** It is the constraint on 33. "Make the key
cheaper" and "make the key narrower" look like the same change from the outside,
and the second one is only safe for the two consumers that confirm. Whoever
touches the key needs this table in front of them, and the cheapest way to
guarantee that is for the two unconfirmed consumers to confirm — both already
own the expression they would compare against.

- [x] Decide whether `seen` should hold the expression it memoized and confirm on
      a hit — **no**. A confirm costs a subtree compare on every hit and a stored
      deep clone of every memoized expression. The width was the cheaper answer
- [x] Same question for `InsertionSlot::BeforeDecl` — same answer, for the same
      reason, and it is covered by the width too
- [x] If either stays unconfirmed, say so at the site with the reason — done in
      `cache.rs`, and in both `CONTEXT.md` glossaries

## Answer

**The key is 128 bits wide, and it made the compiler faster.**

`stable_hash_unspanned` and `stable_hash_unspanned_call` now return `u128`, hashed
by xxh3 in a single pass. Every consumer's map key widened with them —
`seen`, `jsx_spread_attr_exprs_map`, `queued_decl_items`, `all_call_expressions`
and `InsertionSlot::BeforeDecl`. Collision exposure goes from ~`1e-12` per file
to past `1e-31`. All 27 test binaries pass with no output change, which is the
claim that matters: nothing persists a key or derives a class name from one, so
changing the algorithm is invisible outside the caches.

**It is not a trade.** Against the 64-bit SipHash it replaced, on the depth
benchmark:

| measurement                      | 64-bit SipHash | 128-bit xxh3 | change |
| -------------------------------- | -------------- | ------------ | ------ |
| one key, 30 levels               | 1.17 µs        | 0.67 µs      | −43%   |
| one key, 240 levels              | 9.15 µs        | 6.44 µs      | −30%   |
| fold, 30 levels                  | 24.8 µs        | 18.9 µs      | −24%   |
| fold, 240 levels                 | 1 164 µs       | 824 µs       | −29%   |
| fallback arm, 129 props          | 12.3 µs        | 8.5 µs       | −31%   |
| whole transform, 400-create file | 26.0 ms        | 25.9 ms      | −0.6%  |

Twice the width for two thirds of the cost. The end-to-end row is the honest one
— the key is under 1% of a transform, so nobody will notice either number; the
point is that the fix is not paid for.

**The route not taken, and its price.** Two salted `DefaultHasher` states fed by
one walk is the standard-library way to get 128 bits, and it was built and
measured first: **+49% on the key and +5.8% on a whole production transform**
(26.0 ms to 27.5, 25 runs each). Paying that forever to remove a failure that
arrives once per `1e4` years is the wrong trade, and it is why this ticket
briefly looked like it was not worth doing. xxh3 emits 128 bits from a single
pass instead of hashing the same bytes twice.

**Cost.** One direct dependency, `xxhash-rust` 0.8 with the `xxh3` feature.
BSL-1.0, already on `deny.toml`'s allow-list.

**A fifth consumer, found while counting.** The table above says four; there are
five. `all_call_expressions` keys `stable_hash_unspanned_call` to a `Callee`, and
its reads scan values with `eq_ignore_span` — so reads were always safe — but
`replace_call_expression` removes by hash alone and a collision would evict the
wrong entry. Widened along with the rest. The ADR's table now lists all five.

**What is not pinned.** A collision cannot be constructed on demand, so no test
asserts the width directly. What the tests do hold is that both arms agree on it:
the fallback cases compare `stable_hash_unspanned` against `stable_hash_wide`, so
an arm that quietly narrowed to 64 bits would fail to compile.

## Found in review — the count was wrong

**Four consumers act on a bare hit, not two, and the two that were missed fail
_visibly_.** The code-frame `span_cache` is read twice on a bare hash hit — once
keyed by `compute_cache_key`, once by `compute_key_span_cache_key` — and what it
returns becomes the `file:line` on `$$css`. A collision there annotates a style
with another style's line number, in the output, where the evaluator's memo would
at least fail silently. Both were still 64-bit `DefaultHasher`.

Widened along with the rest, which is why `stable_hash_wide` is now public rather
than private to `hash.rs`. They key a _positional_ hash rather than the structural
one — the whole point is caching "where was this written", so two identical
expressions at different positions must not share an entry — so they have their
own derivation and did not come along for free.
`compute_key_span_cache_key` is now one tuple hash rather than a dozen sequential
`hash` calls, so a field added to the function cannot be forgotten in the digest.

**And `Hasher::finish` is not the low half of the wide digest.** xxh3's 64-bit and
128-bit digests are separate constructions over the same stream, so the two are
unrelated numbers. The doc comment claimed otherwise; corrected, and pinned in
`wide_hasher_tests` — a caller reaching for the familiar `finish` gets a narrower
key _and_ a different one.

Also from review: `Xxh3Default` replaces `Xxh3`, which copies a 192-byte custom
secret into every instance. One instance is built per key, so that copy was the
largest fixed cost of a small key — worth another 2-5%.
