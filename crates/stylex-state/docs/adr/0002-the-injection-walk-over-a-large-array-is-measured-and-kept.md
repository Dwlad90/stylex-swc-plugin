# The injection walk over a large array is measured and kept

**Status:** accepted

`flush_pending_insertions` places each queued `_inject2(...)` statement in front
of the declaration that holds the styles it belongs to, and it finds that
declaration by hashing the candidates every top-level initializer reaches. A
declarator bound to one compiled object is hashed once. A declarator bound to an
array is looked inside, because a `create` call written in an array declares its
rules through the statement rather than through the object, so every element is
hashed, then every object an element holds, then every literal below that.

Nothing measured the second shape. The one entry in
`crates/stylex-rs-compiler/benchmark/fixtures.v1.json` that turns runtime
injection on binds its `create` call to a name, and the two large-array modules
in the corpus run with injection off -- which drops every `BeforeDecl` item
before the walk starts and skips it entirely. The descent was therefore argued
about on node counts rather than read off a clock.

`crates/stylex-state/benches/injection_walk_bench.rs` is that clock. It puts the
paths side by side at the same style count, on the corpus shape: 3 namespaces
per compiled object and 3 class-name declarations per namespace, which is what
`lotsOfStyles.js` compiles to over its 21,733 `create` calls.

## The measurement

Criterion medians, `aarch64-apple-darwin`, 2026-09-16. Three runs agreed within
3%. The machine was a working desktop, not an idle one -- load averages 14.6 to
17.8 across the runs, 69-72% idle -- so read these as figures good to a few
percent, which is all the decision below needs.

| `create` calls | array, injection on | object, injection on | array, injection off |
| -------------- | ------------------- | -------------------- | -------------------- |
| 128            | 124.1 us            | 39.4 us              | 1.51 us              |
| 512            | 503.7 us            | 582.2 us             | 5.29 us              |
| 2048           | 2.025 ms            | 2.422 ms             | 20.6 us              |
| 21,733         | 22.44 ms            | --                   | 216.2 us             |

21,733 is the corpus size, so the largest figure is read rather than
extrapolated. The object control is not run there: at that size it prices the
deep-clone hash arm over half a million nodes, which answers no question this
file asks.

Four things the table says.

**The array shape is linear in the number of calls, not square.** Four times the
calls costs 4.06 and 4.02 times the time at the two curve steps, and the jump
from 2,048 to the corpus size -- 10.6 times the calls -- costs 11.1 times the
time, 4% above linear. The square in the cost note on `push_held_hashes` is in
the _depth_ of the containers, and a compiled style object is three levels deep
whatever it holds.

**The array shape costs about three times the object shape at the same style
count.** That is the 128-call pair, which is the only clean one: above 128
properties `stable_hash_unspanned` gives up and falls back to a span-stripped
deep clone, so the object legs at 512 and 2048 price that arm rather than the
in-place walk. The benchmark asserts which arm each object leg took, so the pair
can never be read as the descent when it is not.

That three is the whole difference between the two shapes, not the price of the
descent alone. The array leg also fills a map of 128 entries, removes 128 of
them and splices 128 items into the body, where the object leg does one of each.
The descent is the larger part of the gap, but this pair does not separate them.

**Injection off is about 1% of injection on** -- 216.2 us against 22.44 ms at
the corpus size. It is not a flat floor either: it grows with the number of
calls, because it drops every queued item in the drain loop before the walk
starts. That is the part of the subject the corpus already pays.

## The decision

Keep the walk as it is, and keep it out of the fixture corpus.

The walk over `lotsOfStyles.js` costs **22.44 ms**. The comparison that settles
its weight is against the same module compiled with injection **on**, because
that is the only build that pays for it. On the same machine, with the addon in
`crates/stylex-rs-compiler/dist`, five runs each:

| `lotsOfStyles.js`   | median   |
| ------------------- | -------- |
| injection off       | 1,100 ms |
| injection on        | 1,333 ms |
| what injection adds | +233 ms  |

So the walk is **1.7%** of the compile that pays for it, and **9.6%** of what
turning injection on costs. The rest of that 233 ms is building and placing
153,437 `_inject2(...)` statements, which is the work the walk exists to
position.

The 3,067 ms `Rollup plugin - lotsOfStyles.js` row in `budget.json` is **not**
the denominator, for two reasons: it is an upper bound over CI runs on
`x86_64-unknown-linux-gnu` rather than a median on this machine, and it compiles
with injection off, so it does not contain the 22.44 ms at all. It corroborates
the order of magnitude and nothing finer.

## Considered options

**Add a fixture pairing a large array with runtime injection.** Rejected, but
not for the reason it first looks like. Such a fixture would not price the walk
-- it would price the whole injection path, which is the +233 ms above, 21% of
the module. A walk regression inside it is 1.7% of the row, well inside the 1.25
headroom the ceilings carry, so the row would report the rest of the injection
path under the name of the walk. A new fixture also costs what the release gate
charges: it compares against the last published version, which has no reading
for a row that did not exist.

**Stop descending into an element once its own hash matched.** Rejected. It
would cut the gap sharply, because a compiled object that matched is the object
a producer registered and holds no second one. But "holds no second one" is a
fact about what the producers emit today, not about the shape, and the descent
exists precisely because an object here cannot be told from a wrapper around one
without asking. Buying 1.7% with a rule that a new producer can break silently
is the wrong trade.

**Hash only string literals, not every literal.** Rejected on size, though it is
provably correct today: every key a producer registers is an object, or the
name `create_string_expr` leaves for `keyframes`, a view-transition class or a
position-try -- never a boolean or a number. Narrowing the `Expr::Lit` arms in
`push_init_hashes` and `push_held_hashes` to `Lit::Str` would drop the `$$css:
true` hash and its lookup, 3 of the 16 hashes an element costs: about 8-10% of
the walk, 0.15% of the compile. It is recorded here so the next reader does not
re-derive it, not left in the code as an unpriced special case.

**Exit the walk early once the bucket map is drained.** Rejected as worth
nothing on this shape. `lotsOfStyles.js` has three top-level items, and all
21,733 keys are consumed inside the one export declaration, so the map cannot
empty before the hashing that empties it.

**Compose the hash from its children's hashes.** Rejected as out of proportion.
It would collapse the repeated subtree walks, but `stable_hash_unspanned` is
also the evaluator's memo key, and
`../../stylex-evaluator/docs/adr/0006-an-incremental-memo-key-was-built-and-measured-slower.md`
records that shape being built and measured slower there.

**Leave the walk unmeasured.** Rejected: that is the state this record replaces.
The descent was added on reasoning about node counts, and the next change to it
would have been argued the same way.

## Consequences

- A change to the walk is read against `InjectionWalk` in this crate, not
  against the fixture corpus, which cannot see it.
- `InjectionWalkDisabled` stays, because it is what the corpus measures. The gap
  between the two groups is the part of the subject no release gate watches, and
  a reader who forgets that reaches for the corpus numbers.
- The object legs above 128 calls price the deep-clone hash arm. They are kept
  because that arm is what a real module of that shape takes, and the assertion
  in the benchmark is what stops them being read as the descent.
- **The injection path is now priced at scale, and this walk still is not.**
  Measuring the walk turned up a larger gap beside it: no module in the corpus
  was priced with injection on over an array of `create` calls, so the 21% that
  turning it on costs was invisible to the release gate, and the walk is only
  the smallest tenth of that 21%. `Feature - runtime injection at scale` closes
  that gap -- `lotsOfStyles100.js`, 100 `create` calls in a top-level array,
  with `runtimeInjection: true`. It prices the whole injection path, which is
  the right subject for a corpus row and the wrong one for this walk, so it
  does not change the decision above. Its ceiling needs seeding before the next
  release; the steps are in `guidelines/PERFORMANCE.md` under "Seeding a new
  ceiling".
