# The injection walk over a large array is measured and kept

**Status:** accepted, and its two rejected narrowings are superseded --
see [What changed the trade](#what-changed-the-trade) at the end.

`flush_pending_insertions` places each queued `_inject2(...)` statement in front
of the declaration that holds the styles it belongs to, and it finds that
declaration by hashing candidates it reads off the module. As this was written,
what it read was every top-level initializer: a declarator bound to one compiled
object was hashed once, and a declarator bound to an array was looked inside,
because a `create` call written in an array declares its rules through the
statement rather than through the object -- so every element was hashed, then
every object an element held, then every literal below that. The section at the
end says what it reads now.

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
empty before the hashing that empties it. (Taken with the widening all the
same, because the widened walk reads statements this one never reached. It is
worth nothing on the corpus shape and everything on a module whose styles come
first and whose other statements are many. `0003` is what made the map able to
empty at all.)

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

## What changed the trade

The walk was widened, because the shape it read was the wrong one. A statement
can hold the registered object behind a wrapping call, a conditional, a member
access, a class field or an assignment, and the walk read arrays, objects and
parentheses alone -- so each of those shipped a class name that no stylesheet
defined. It now reads every expression the item holds and does not enter a
function body. `RegisteredObjects` in `src/state_manager.rs` is that walk, and
it replaces the four `push_*_hashes` functions this record names above. Both
readings are in
`crates/stylex-transform/tests/transform_stylex_create_test/program_level_positions.rs`,
each against measured `@stylexjs/babel-plugin` 0.19.0 output.

That widening changed what the two narrowings above are worth, so both are
taken:

**Stop at the object that matched.** Rejected above as buying 1.7% with a rule
a new producer could break silently. With the walk reading every expression it
is no longer 1.7%: it is what keeps a declarator bound to one compiled object
at the one hash it always cost, because the walk would otherwise descend into
the largest object in the module. The rule it rests on is that nothing a
registered object holds is registered itself -- a rule a nested call does not
break, because the rules a `keyframes` inside a `create` argument declares are
keyed to the object it was folded into rather than to the name left in it. That
is pinned in two places rather than reasoned about:
`a_keyframes_inside_the_create_argument_injects_both_rules` measures it through
the whole compiler against upstream, and
`a_name_below_the_object_that_matched_is_not_looked_for` states the boundary on
the walk itself.

**Hash only string literals.** Taken with it, for the reason recorded above:
every key a producer registers is an object or a name, never a boolean or a
number. It is not an unpriced special case now -- it is one arm of the walk,
and the figures below are with it.

### The measurement after both

Criterion medians on the same machine and the same afternoon as the table
above, and the old code was re-measured first: it read within 1% of that table
(123.9 us, 503.6 us, 2.011 ms, 22.29 ms), so the two columns are comparable.

| `create` calls | array, before | array, after | object, before | object, after |
| -------------- | ------------- | ------------ | -------------- | ------------- |
| 128            | 123.9 us      | 43.5 us      | 39.3 us        | 39.5 us       |
| 512            | 503.6 us      | 168.0 us     | 576.1 us       | 581.6 us      |
| 2048           | 2.011 ms      | 706 us       | 2.396 ms       | 2.481 ms      |
| 21,733         | 22.29 ms      | 8.31 ms      | --             | --            |

The array shape costs **63% less** at every size, corpus size included, and it
is still linear in the calls. The object legs are unchanged: they differ by 0.3%
to 3.5%, which is inside what this machine resolves, and the work on that path
is the same one hash it was. Injection off is unchanged, as it must be -- that
leg never reaches the walk.

So the widened walk is cheaper than the narrow one it replaces, and the shape
that pays the most pays 14 ms less per compile of `lotsOfStyles.js`.

### What the widening costs, where it costs anything

One guarantee narrowed, and it is the one the old cost note rested on. An
object initializer used to be hashed once and never looked inside, match or no
match. It is now looked inside when it does **not** match, because an object
around the styles is a shape an author writes and the rules still belong to the
statement. So an authored object that is not the styles -- a configuration
table, a message catalogue -- is hashed once per level rather than once, and a
hash reads the whole subtree under it. Past 128 properties the hash gives up
and copies the subtree instead, so at that width the bytes copied grow with the
depth as well.

`array-with-data/N` is that shape: the array export behind a top-level authored
object, eight levels deep and 200 names per level, so every level takes the
copying arm. Same machine, same afternoon, the old code measured first.

| `create` calls | before   | after    |
| -------------- | -------- | -------- |
| 128            | 252.4 us | 709.5 us |
| 512            | 650.7 us | 841.9 us |
| 2048           | 2.232 ms | 1.381 ms |

Read it as one curve rather than three numbers. The authored object costs the
new walk about 0.45 ms more than it cost the old one, whatever the styles
beside it are; the styles cost 63% less. So the module is slower while the data
outweighs the styles, the two cross somewhere past 512 calls, and a module of
2,048 calls with the same data in front of it is 38% faster.

The 0.45 ms is kept rather than guarded, for two reasons. There is no cheap way
to know a queued key cannot be below an object -- the queue holds hashes, and a
hash answers about a node, not about what a node contains. And the shape has to
be extreme before it is measurable: 1,800 authored properties, nested eight
deep, in the same module as the styles, with runtime injection on. What the leg
buys is that the next change to this walk is read against it.
