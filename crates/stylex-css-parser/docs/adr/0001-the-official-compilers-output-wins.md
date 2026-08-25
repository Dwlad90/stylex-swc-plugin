# The official compiler's media query output wins, defects included

**Status:** accepted

A StyleX class name is a hash of the canonical declaration text, so that text is
a compatibility contract rather than a formatting preference. Where this
compiler's media query output differs from `@stylexjs/babel-plugin`'s, this
compiler is wrong — including in the two places where its output was better.

Both are reproduced deliberately. Neither is a bug to fix here.

**Contradictory branches are kept.** A ladder of exclusive breakpoints
distributes into branches that all contradict. Those branches reach the bottom
of the distribution and print as `not all`, wrapped in the disjunction nesting
that produced them, so the six queries an author wrote become two wrapped ones
and four plain ones. Emitting the bare authored query instead is semantically
identical and much shorter — and costs two of the seven class names that input
produces.

**A key collision drops a declaration.** Once contradictions are retained, two
entries of one conditional value map can canonicalize to the same query text.
The rewritten keys go into an insertion-ordered map, so the second assignment
replaces the first entry's value and keeps its position, and one authored
declaration is absent from the output. No warning accompanies it, because the
official compiler prints none — and a warning it does not print would itself be
a divergence in what a build produces.

Four things in the tree contradict that on first reading, and each is
deliberate.

**lightningcss will not parse what we emit.** Measured on 1.33.0, both wrapped
forms are refused, not only the doubly parenthesised one:

```text
@media ((not all) or (not all)) or ((not all) or ((min-width: 1440px)))
  → Unexpected token ParenthesisBlock
@media (not all) or ((min-width: 1200px) and (max-width: 1439px))
  → Unexpected token Ident("all")
```

So a project running the wrapped output through it gets a rejected stylesheet. This is
accepted knowingly, and the reason is the failure mode rather than the
frequency: a rejected stylesheet fails loudly, at build time, with the offending
rule in the message. A class-name divergence fails silently — the markup names a
class the stylesheet never defines, nothing errors, and the page renders wrong
in production. Matching the official compiler is the point of this compiler; a
minifier that cannot read valid-enough CSS is that minifier's problem to fix, or
ours to work around downstream, but not a reason to hash differently.

**An `or` nested inside an `and` loses its parentheses, and browsers reject the
result.** Serialization joins an `and` list with `" and "` and does not
parenthesize an `Or` child, where the `or` arm does the mirror for an `And`
child. Two shapes reach it: a parenthesized disjunction beside an `and`, and a
comma segment holding an `or`, which the negation chain then wraps in an `and`.

```text
@media ((min-width: 100px) or (max-width: 50px)) and (min-height: 10px)
  → @media (min-width: 100px) or (max-width: 50px) and (min-height: 10px)
```

Per Media Queries Level 4 an unparenthesized `and`/`or` mix is a syntax error,
so this is worse than the lightningcss case above: a _browser_ drops the whole
at-rule, and the author's declaration is silently lost at runtime rather than
loudly at build time.

It is emitted anyway, for the same reason everything else here is: the official
compiler emits the identical text, so the class name matches. The expectations
in `media_query_transform_test.rs` are quoted from a run of it. Adding the
parentheses would be semantically correct and would diverge the hash, which is
the trade this record exists to refuse.

Two consequences worth stating rather than rediscovering. This compiler now
emits media keys that its own `validate_media_query` refuses, because the
grammar work taught the parser that the mix is undefined while the serializer
still produces it — a round-trip inconsistency, not a bug to close. And this is
a _third_ upstream defect, alongside the two above; it is the most serious of
them, and belongs in a cross-report of its own.

**The dropped declaration is a ported defect, not a design.** It is recorded
that way on purpose, so it is revisited rather than defended: redundant CSS is
ugly, a missing style is lost work, and this is the more serious of the two.
When the upstream report is resolved, this compiler follows. Both defects are
cross-reported to facebook/stylex separately, one each, so either can be fixed
there without waiting on the other; the report numbers belong in this paragraph
once they exist.

**Parity stops at a depth no author reaches.** Every negated neighbour doubles
the query text, so a twenty-rung ladder is 983 051 characters and about three
seconds, and the official compiler goes on doubling — 252 MB and 435 seconds at
twenty-eight rungs, then a heap abort. It has no give-up of its own to copy: its
`try`/`catch` guards a stack that never overflows, while the heap exhaustion is
a fatal abort no `catch` sees and the string-length error is raised outside the
one that would have caught it.

So `MAX_DISTRIBUTION_NODES = 2^18` in `src/at_queries/media_query.rs` is this
compiler's own number. Past it the merge hands the author's rules back unmerged
— the inner recovery, not the invalid-media-query-syntax refusal.

It is stated in branch **nodes** rather than in levels, and that correction
matters more than the value. A depth bound models the cost as `2^d` characters,
which is what the table below was measured against — but every one of those
`2^d` branches carries a copy of the whole `and` list, and every surviving
branch prints it, so the real figure is `2^d · rules.len()`. The second factor is
not bounded by the first: `negation_depth` charges nothing for a clause it cannot
split, so a negated non-range condition — `not (orientation: portrait)`,
`not (min-resolution: 200dpi)` — buys width for free.

That hole was reachable, and not by anything exotic, because the transform builds
the wide list itself: each key is rewritten against a negation of every later
sibling, so one `and` list is as wide as the conditional value map is long.
Measured on one property of one `stylex.create`, every row _under_ the old depth
bound:

| Non-range keys | Rungs | Depth | Input  | Wall clock | Emitted CSS |
| -------------- | ----- | ----- | ------ | ---------- | ----------- |
| 0              | 14    | 12    | 917 B  | 60 ms      | 62 KB       |
| 10             | 14    | 12    | 1.3 KB | 2 696 ms   | 98.9 MB     |
| 20             | 14    | 12    | 1.7 KB | 6 732 ms   | 255.0 MB    |
| 40             | 14    | 12    | 2.4 KB | 20 244 ms  | 739.3 MB    |
| 20             | 16    | 14    | 1.8 KB | 30 478 ms  | 1 098.6 MB  |

Under the node budget those became 15.3 MB in 477 ms, 15.3 MB in 479 ms, 15.3 MB
in 480 ms, and 150 KB in 122 ms. Do not delete the bound as arbitrary — the
curve it was chosen from is below, and the pure-ladder shape it was originally
read off is the _narrow_ edge of a two-factor cost.

Pure ladder, before (depth bound) and after (node budget). The last column is
the longest single query emitted, over all rungs rather than the first one:

| Rungs | Before, wall | Before, longest | After, wall | After, longest |
| ----- | ------------ | --------------- | ----------- | -------------- |
| 14    | 31 ms        | 15 371 chars    | 92 ms       | 30 786 chars   |
| 15    | —            | —               | 660 ms      | 61 506 chars   |
| 16    | 131 ms       | 61 451 chars    | 138 ms      | 61 506 chars   |
| 18    | 599 ms       | 245 771 chars   | 159 ms      | 61 506 chars   |
| 20    | 2 634 ms     | 983 051 chars   | 129 ms      | 61 504 chars   |
| 100   | 2 690 ms     | 5 613 chars     | —           | —              |

Apple M1 Max, release build, one ladder per process. Under the depth bound twenty
rungs was the last length that expanded, and the ladder after it collapsed from a
megabyte to a kilobyte. Under the node budget fifteen is the last length that
expands in full, and there is no collapse after it — the cost plateaus instead,
because each key's own `and` list is capped on its own rather than the whole
ladder being handed back.

## Considered options

**Keep the shortcut that skipped the expansion.** This is what the code did
before: a predicate recognized a ladder whose every branch contradicts and
returned an empty result. It was fast, it produced shorter CSS, and it was
wrong — the class names it produced named rules no official-compiler build
emits. Rejected on the contract, not on the aesthetics.

**Reshape the shortcut to return the surviving shape.** The tempting middle
path: keep the fast path, make it produce the wrapped text directly. Rejected
because its correctness claim — that it fires exactly when the full expansion
would contradict — has no upstream line to check against, and parity is the
whole purpose of the change it would sit inside. A fast path may come back, but
as its own change, benchmarked against the numbers above.

**Warn when a declaration is dropped.** Rejected: the official compiler emits no
diagnostic, and a build that prints something theirs does not is a difference in
observable behaviour, which is the class of thing this ADR exists to eliminate.

**Bound the recursion by depth alone, generously.** Rejected once the cost was
measured. Depth is the logarithm of the real quantity; a bound set where a stack
would notice is a bound set tens of megabytes too late.

**Bound the recursion by depth alone, tightly.** Also rejected, and this is the
option the first version of this record actually took. Depth is the logarithm of
only _one_ factor of the cost, and the other one — the length of the list every
branch carries — is free of it. There is no depth that both keeps the twenty-rung
ladder and refuses the 255 MB shape, because that shape sits at depth 12. So the
budget had to change units, not value.

**Budget the whole declaration rather than one `and` list.** Not taken, and still
open. It would cap what a comma query multiplies, which the node budget does not.
Left out because it is a question about resource limits rather than about parity,
and because a per-list cap was enough to close the reachable blow-up.

## Consequences

**A ladder past fifteen rungs compiles differently from upstream, on purpose.**
This is the one place byte parity is knowingly abandoned, and it is above any
ladder a person writes. The parity harness carries the reported input at six
rungs, not at sixteen, so it measures agreement rather than this boundary.

The boundary used to sit at twenty rungs, and moving it in was the price of
seeing the width factor. Nothing pinned moved with it: the ladder expansion test
still measures 15 393 characters and every canonicalization fixture is
byte-identical, because those shapes are far inside either bound.

**The bound caps one `and` list, not one compile.** The boundary is crossed once
per `and` node, so lengthening a ladder stops costing more — a hundred-rung one
takes the same 2.7 seconds as a twenty-four-rung one, because the deepest thing
either contains is a twenty-rung sub-ladder — but _widening_ still does. A comma
query is several `and` lists, and each pays the ceiling separately: measured on
the same ladder, one disjunct is 2.6 s and 182 MB, two are 4.0 s and 240 MB,
four are 6.6 s and 354 MB, eight are 12.1 s and 577 MB. That is linear rather
than exponential, and it is what the official compiler does too, without a bound
at all. A global budget across the whole declaration would cap it; it is not
built here because it is a different question from parity, and because nothing
has asked for it.

**The expansion is the performance baseline, not a regression to chase.** Any
future fast path has to beat the numbers above and justify its own correctness
separately.

**Two parse paths became one.** The transform validates rather than parses, so
the balanced-parenthesis check that the tokenizer's synthesized closing
parenthesis would otherwise hide is on the only path there is. It skips a
parenthesis written as an escape or sitting inside a string, where upstream's
own counter would not — but upstream never runs that counter on this path, so
what is matched is what its parser actually accepts.

**Four divergences were found by re-reading upstream, and all four are fixed.**
They were pre-existing rather than introduced by the canonicalization work, and
each is the kind this record exists to eliminate. Every expectation below was
obtained by executing 0.19.0, not by reading it.

- **Each range bound takes its key from its own operator.** `doubleInequalityRuleParser`
  reads `lowerKey = op === '>' ? max- : min-` and `upperKey = op2 === '>' ? min- : max-`,
  so a mixed-direction range puts two bounds on one side and the merge collapses
  them. Deriving a fixed `min`/`max` pair and only choosing which value went
  where could not express that: `(500px > width < 1000px)` is
  `(max-width: 499.99px)` upstream and came out here as `not all` — a satisfiable
  query compiled to a rule that can never match.
- **`not only <type>` is accepted.** `mediaKeywordParser` takes both modifiers as
  independently optional and its serializer reads `not` first, so
  `@media not only screen` prints `@media not screen`. Refusing it failed a build
  the official compiler completes.
- **An overflowed bound is kept.** The merge asks `!== -Infinity`, not
  `isFinite`. Dropping the bound was not a spelling difference: upstream's
  `(min-width: Infinitypx) and (min-height: 10px)` is invalid CSS a browser
  discards whole, where the dropped form left a rule that _applies_.
- **A fraction is spelled the way JavaScript spells a number**, so `1e30/1` is
  `1e+30 / 1` and `-0/1` is `0 / 1`.

The first two are the serious ones, because they are the direction this record's
argument does not cover. Matching upstream's _worse output_ protects a hash;
diverging from its _accepted input_ either changes which viewports a declaration
matches, or fails a build that upstream completes.

**Two grammar differences are kept on purpose.** Parentheses nested around a
single condition — `@media ((min-width: 1px))` — and one bare `not` straight
after a media type's `and` — `@media screen and not (orientation: portrait)` —
are accepted here and refused upstream. The language defines both, and
upstream's `oneOf` chain simply has an alternative for neither.

Two more acceptances are neither valid CSS nor upstream-accepted, and are
recorded rather than defended: `@media (screen)` and `@media (not screen)` parse
here and are `No parser matched` upstream, because a bare media type is not a
`<media-in-parens>`. Nobody can act on the difference — upstream refuses to build
the input — so nothing is at stake, but the enumeration is only useful if it is
complete.

Refusing valid input to match buys nothing: nobody gets a divergent class name
from a query the other compiler will not compile at all. That is the boundary of
this record's own argument. Matching upstream's _worse output_ protects a hash;
matching its _refusals_ would only cost an author a query they are entitled to
write, and protects nothing.

Every other combinator shape was matched — see the glossary's
[media query grammar](../../CONTEXT.md) entry. One caution if this is ever
revisited: upstream's parser backtracks exponentially in parenthesis nesting
depth — eight levels take it 1.12 s, twelve take 19.8 s, sixteen do not finish
in forty seconds, where this compiler answers every depth in about a
millisecond. Matching its answers must not mean matching that.

**The two bounds deliberately pick opposite failure modes.**
`MAX_DISTRIBUTION_NODES` gives up and hands the author's rules back — the inner
recovery — because a query too deep to _merge_ is still a query, and emitting it
unmerged is what the official compiler does. `MAX_QUERY_NESTING_DEPTH` refuses
with the invalid-media-query-syntax error — the outer refusal — because a query
too deep to _parse_ yields nothing to emit; there is no unmerged form to fall
back to. The pass's two failure modes are kept visibly apart precisely so this
choice reads as a choice. Note that the second refuses input that is valid CSS,
which is the price of not aborting the process on it.

**A query may nest sixty-four levels, and two guards enforce that.** Walking
each level once is linear in time and still recursive in stack, and five thousand
levels aborted the process — a stack overflow is not unwindable, so nothing
downstream could have turned it into a diagnostic. Sixty-four is
`stylex_utils::nesting::MAX_NESTING_DEPTH`, shared with the value guard in
`stylex-css`, which faces the identical exposure; the two scans differ -- one
steps over comments and `url()` bodies -- but the budget is one decision about
the stack, so it is stated once.

Two guards rather than one, because there are two recursions and neither guard
can see the other's. **Tokenizing** descends once per nested block while it
builds the token list, so a walk over the raw text has to refuse deep
parentheses _before_ tokenizing — measured directly: five thousand parentheses
abort inside `TokenList::new`, before a single token exists, where no counter
inside the parser is reached in time. **Parsing** then descends again, and for
frames no parenthesis pays for: the operand of a bare `not` is a whole rule, so
`not not not …` recurses once per keyword while a scan for parentheses sees
depth one. That gap was live — twenty thousand `not` keywords in one `@media`
key segfaulted the process with no output at all — and is closed by
`TokenList::with_depth`, which charges a frame where a frame is entered and so
needs to know nothing about spelling. A text scan would have had to
over-approximate every escape, since `n\6ft` decodes to an ident spelling `not`
and recurses identically.

A wide query is not a deep one, and the frame counter keeps that distinction
where a keyword count could not: `(not (a)) and (not (b)) and …` is parsed by a
loop, so each negation releases its frame before the next is read.
