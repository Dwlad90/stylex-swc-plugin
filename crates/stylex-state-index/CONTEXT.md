# stylex-state-index

The lookup structures the
[state manager](../stylex-state/CONTEXT.md#state-manager) composes to answer
"which declarator, which call, which span" with one hash probe instead of a scan
of the module. Pure lookup machinery: nothing here decides what a style means,
and nothing here holds the entries it points at.

## Language

**Candidate index**:
Where the entries holding a given thing live, bucketed by a key that narrows to
them, held beside the collection it indexes on the
[state manager](../stylex-state/CONTEXT.md#state-manager). The answer to
"which recorded entry holds _this_?" -- which declarator a call initialises,
which style variable it is bound to, which top-level expression it is, which
top-level expression a name binds, which import specifier binds a reference,
whether a declarator at this position is already stored. Each of those was a
walk of the whole collection comparing whole subtrees with `eq_ignore_span`,
once per call the transform meets, which made the consumer phase quadratic in
the number of calls a module makes. The key only narrows: equality still decides
between the candidates it hands back, so the answer is the one the walk gave.
What the key is depends on the question -- a structural hash for an expression,
a source position for where something was written, a name for what a declarator
binds.
_Avoid_: lookup table, call map, bucket map

**Key span index**:
Where every style namespace key of the module's _own parsed source_ is written,
collected in one walk and held beside that memoized source on the
[state manager](../stylex-state/CONTEXT.md#state-manager). What the
`file:line` annotation on `$$css` is resolved from: the annotation asks for the
authored position of every namespace of every `stylex.create` call, and
answering each by walking the source made a `dev` build quadratic in the size of
a file that is one long list of styles. A key two namespaces spell is several
candidates, ranked by how much of the compiled call each reproduces; a tie
resolves to nothing, because a wrong `file:line` is worse than none. Distinct
from the state manager's span cache, which memoizes the _answers_ this index is
asked for, keyed by the lookup rather than by the key.
_Avoid_: namespace key index, key map, position table

**File offset**:
How far into its own file a position sits, and the only thing the
[key span index](#key-span-index)'s proximity tie-break may compare. Two
`BytePos` in this compiler can name the same character and hold different
numbers: the index is built from a module re-parsed into the code frame's
shared, process-global source map, while the call it places is read out of the
per-transform one, and a source map gives each file a start position after the
previous file's end. So the two agree only for the first file a process
compiles. A file offset can only be built from a position and the
[module base](#module-base) it belongs to, and exposes no way to read the number
back out, so the subtraction cannot be skipped at a new call site.
_Avoid_: byte position, column, index

**Module base**:
Where the module being transformed starts, in the source map it was parsed into
— the thing a position is measured against to become a
[file offset](#file-offset). Its own type for two reasons: both arguments would
otherwise be `BytePos`, so transposing them compiles and answers zero for every
candidate; and it must have no default, because a base nobody recorded would be
byte zero, which turns every offset straight back into the raw position. Where a
base may be genuinely unavailable it is spelled as absent rather than defaulted,
so a lookup that never got one loses the proximity tie-break instead of silently
ranking by "earliest in the file". _Avoid_: module start, origin, offset base

**Call lookup**:
The half of a key-span lookup that belongs to the `stylex.create` _call_ rather
than to one of its namespaces: the sibling keys every namespace of that call
ranks against, the proximity anchor, the span cache key's call-side digest, and
the call wrapped as an expression for the value-matching fallback. Built once
per call, because building any of it per namespace makes the call quadratic in
its own namespace count — the same shape the [key span index](#key-span-index)
removed one level up. One type rather than four arguments so that they cannot
describe different calls: a digest paired with another call's keys is a wrong
span cached under a key that looks right. The wrapper inside it is a deep clone,
so it is built on the first namespace that needs one and never for a call whose
namespaces all hit the span cache. _Avoid_: call keys, sibling context, lookup
context

**Namespace key query**:
One namespace of one _compiled_ `stylex.create` call, described the way the
[key span index](#key-span-index) ranks candidates against it: the key to find,
the sibling keys of its call, the keys of its own value object, the proximity
anchor and the callee. Read off the compiled call rather than the source, so
nothing in it can be taken for granted — shorthand expansion has already
rewritten the values, and a synthesized call carries no position at all.
_Avoid_: lookup request, span query, key request

**Candidate rank**:
How well one candidate matches the namespace being placed, higher being better.
The field order _is_ the precedence: whether the candidate belongs to the call
being placed, then how much of the namespace's own value it reproduces, then how
many of the call's other namespace keys it spells, then how close it is written
to the call. Compared rather than scored, so no weight has to be chosen and a
tie stays a tie — which the lookup refuses instead of guessing.
_Avoid_: score, weight, match quality
