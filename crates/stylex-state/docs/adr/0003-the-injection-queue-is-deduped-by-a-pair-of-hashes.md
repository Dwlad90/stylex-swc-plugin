# The injection queue is deduped by a pair of hashes

**Status:** accepted

`register_styles` files the rules a producer compiled and queues one
`_inject2(...)` statement per rule, against each key the placement walk can
find. Two questions decide what that costs: which keys are queued, and how a
rule already queued against a key is recognised.

Both answers were wrong in a way that grew with the module.

## What was there

**The dedup held the statements.** `queued_decl_items` was an
`IndexMap<u128, Vec<ModuleItem>>` -- a bucket per key holding a copy of every
injecting statement queued under it -- and each new rule scanned the bucket
with `eq_ignore_span`. Its field note said the bucket stays at "typically 1-2
entries". It does not: `register_styles` passes the same key for every rule of
one call, so the bucket holds one entry per rule, and the scan is a triangle
over them. `lotsOfStyles.js` compiles 21,733 calls into 153,437 rules, so the
map also retained a second copy of all 153,437 statements for the whole
compile. The map was written in two places and read nowhere else.

**A key nothing could match was queued.** A `create` call that is not at
program level is hoisted: the object moves to a declaration of its own and the
call site becomes a reference to it. `register_styles` was handed that
reference and keyed the rules to its hash, with the object's hash as a
fallback. The walk never reads a reference, so the first key could not match,
and a bucket that cannot match is a bucket that cannot be removed -- which
kept the walk from ever knowing it was done. Every ordinary component module
has one.

## The decision

- `queued_decl_rules` is an `FxHashSet<(u128, u128)>`: the key of the statement
  and the hash of the rule's own object. The injecting statement is built from
  the rule and from the injection identifier, which is one per module, so the
  rule is what tells two statements apart. One probe per rule, no copy of any
  statement, and nothing retained but two words per rule.
- `placement_keys` reads the keys off the call, and it queues only a shape the
  walk can find: the object a producer left at the call site, or the name a
  compiled `keyframes`, `positionTry` or `viewTransitionClass` answered with,
  plus the fallback hash where the caller has one. A hoisted call therefore
  queues one key -- the object's -- rather than two.

## The measurement

`crates/stylex-state/benches/injection_queue_bench.rs`, group
`InjectionQueue`: 64 calls, each declaring `rules` rules. Criterion medians,
`aarch64-apple-darwin`, 2026-09-17, on a working desktop at about 70% idle, the
old code measured first.

| rules per call | before   | after    |
| -------------- | -------- | -------- |
| 8              | 309.4 us | 311.7 us |
| 32             | 1.763 ms | 1.261 ms |
| 128            | 15.80 ms | 5.118 ms |

Four times the rules costs 5.7 and 9.0 times the time before, and 4.05 and 4.06
times after. The triangle is gone, and what is left is linear in the rules.

Eight rules per call is the corpus average, and there the two read the same:
**+1.6%**, which is the hash of the small rule object standing in for a scan of
one or two entries. That is the whole price of the change, and it buys the
module of wide namespaces -- a token file, a `defineConsts` of a hundred
constants -- **68% less** at 128 rules per call. The copies the old map
retained are gone at every size.

## Consequences

- A change to the queueing path is read against `InjectionQueue` in this crate.
  No corpus fixture prices it: every module in the corpus is near the average,
  which is the one place the two readings agree.
- The walk can now know it is done, because every queued key names a shape it
  reads. `0002` records what that is worth to it.
- A producer that registers something that is neither an object nor a string
  literal queues no key and its rules are never injected. That is what the walk
  already did with such a key; `placement_keys` is where it is now said once,
  beside the walk's own arms.
