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

So `MAX_DISTRIBUTION_DEPTH = 18` in `src/at_queries/media_query.rs` is this
compiler's own number. Past it the merge hands the author's rules back unmerged
— the inner recovery, not the invalid-media-query-syntax refusal. Eighteen is
chosen against output size and not against stack depth, because depth is not
what runs out: a bound generous enough for twenty-six rungs would permit a 63 MB
single query. Do not delete it as arbitrary — the curve it was chosen from is
below.

| Rungs | Wall clock | First rung's query |
| ----- | ---------- | ------------------ |
| 14    | 31 ms      | 15 371 chars       |
| 16    | 131 ms     | 61 451 chars       |
| 18    | 599 ms     | 245 771 chars      |
| 20    | 2 634 ms   | 983 051 chars      |
| 21    | 2 664 ms   | 1 078 chars        |
| 40    | 2 673 ms   | 2 135 chars        |
| 100   | 2 690 ms   | 5 613 chars        |

Apple M1 Max, release build, one ladder per process. Twenty rungs is the last
length that expands; the ladder that follows it collapses from a megabyte to a
kilobyte, which is the authored query with one printed negation per later rung.

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

## Consequences

**A ladder past twenty rungs compiles differently from upstream, on purpose.**
This is the one place byte parity is knowingly abandoned, and it is above any
ladder a person writes. The parity harness carries the reported input at six
rungs, not at twenty-one, so it measures agreement rather than this boundary.

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

**Two grammar differences are kept on purpose.** Parentheses nested around a
single condition — `@media ((min-width: 1px))` — and one bare `not` straight
after a media type's `and` — `@media screen and not (orientation: portrait)` —
are accepted here and refused upstream. The language defines both, and
upstream's `oneOf` chain simply has an alternative for neither.

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
`MAX_DISTRIBUTION_DEPTH` gives up and hands the author's rules back — the inner
recovery — because a query too deep to _merge_ is still a query, and emitting it
unmerged is what the official compiler does. `MAX_QUERY_NESTING_DEPTH` refuses
with the invalid-media-query-syntax error — the outer refusal — because a query
too deep to _parse_ yields nothing to emit; there is no unmerged form to fall
back to. The pass's two failure modes are kept visibly apart precisely so this
choice reads as a choice. Note that the second refuses input that is valid CSS,
which is the price of not aborting the process on it.

**A query may nest sixty-four levels of parentheses.** Walking each level once
is linear in time and still recursive in stack, and five thousand levels aborted
the process — a stack overflow is not unwindable, so nothing downstream could
have turned it into a diagnostic. The depth is counted before the parse, by the
same walk over the raw text that checks the balance, because the parse is what
would abort. Sixty-four is the same number and the same reasoning as
`MAX_VALUE_NESTING_DEPTH` in `stylex-css`, which guards the identical exposure
for values.
