# 09 — Guard the cases the reorder must not break

Status: `resolved`
Blocked by: 02, 03, 05, 06, 07, 08

**What to build:** The agreeing half of the audit, recorded as corpus guards, so
the next change to the identifier chain reports a regression as a *changed
verdict* rather than as silence.

Every ticket before this one carries the corpus entry for the case it fixes.
What none of them owns is the set of inputs that already agree with the
reference implementation and only needed to keep agreeing across a reordered
chain:

- a dynamic style parameter shadowing a module-level `const`
- a parameter named `firstThatWorks`
- a reference bound to a function declaration, and to a class declaration
- member mutation of a `const` read in a style value
- both passing shapes from 01: the dynamic style alone, and with an unrelated
  static prop

Each records the verdict it is known to read, which is what turns a future
regression into a reported change rather than a quiet one. These are guards, not
demonstrations — none of them is expected to change, and any that does is the
finding.

- [x] Every case above is a corpus entry with the verdict it is known to read
- [x] The parity run is clean end to end, no unexplained verdict
- [x] Any case that turns out *not* to agree is recorded here rather than
      quietly given an expected-divergence verdict

## Answer

**Eighteen rows in `corpus/modules.json`, every listed case guarded, and one
divergence found that has nothing to do with this chain.**

Two of the listed cases were already carrying their verdict from earlier tickets
and needed nothing: a parameter shadowing a module-level `const`
(`modules-1266-param-shadows-a-const`) and the first passing shape from 01, the
dynamic style alone (`modules-1266-import-unreferenced-elsewhere`). The rest are
new.

### The guards, and which chain step each holds

| entry | verdict | the step it holds |
| --- | --- | --- |
| `…-shadowed-param-beside-an-unrelated-static-prop` | identical | 01's second passing shape, now held by the binding comparison rather than by an elided import |
| `…-param-named-first-that-works` | identical | the name-keyed function map, consulted before the chain, holds a name only where the module imports it |
| `…-reference-bound-to-a-function-declaration` | both reject | step 8's declaration-kind refusal, `Unsupported expression: FunctionDeclaration` on both |
| `…-reference-bound-to-a-class-declaration` | both reject | the same step's other declaration list, `Unsupported expression: ClassDeclaration` on both |
| `…-member-mutation-of-a-const-in-a-style-value` | both reject | step 4, mutation through an assignment |
| `…-mutating-method-receiver-in-a-style-value` | both reject | step 4, mutation through a method receiver |
| `…-object-assign-target-in-a-style-value` | both reject | step 4, mutation through an argument position |
| `…-a-reassigned-binding-in-a-style-value` | both reject | step 3, the other half of the split write set |
| `…-an-unwritten-const-object-in-a-style-value` | identical | the control steps 3 and 4 need: the same module with the write removed has to fold |
| `…-an-unclosed-function-read-through-a-binding` | both reject | a malformed value reached through a binding is refused where the same text written inline is |
| `…-an-unclosed-string-read-through-a-binding` | both reject | the same, for the refusal that names a string rather than a function |
| `…-an-unclosed-bracket-read-through-a-binding` | identical | the malformed value neither compiler refuses, emitted verbatim by both |
| `…-a-custom-property-through-a-shadowed-param` | identical | the custom-property path, where a key is neither expanded nor validated |
| `…-a-prefixed-property-through-a-shadowed-param` | identical | an authored `-webkit-` property taking two different folded values, since neither compiler adds a prefix of its own |
| `…-a-shadowed-param-at-extreme-condition-depth` | identical | the recursive value walk at eight levels, at-rules interleaved |
| `…-many-shadowed-params-in-one-create` | identical | twenty-four reads of twelve names that are each both an import and a parameter, sharing one evaluation cache |
| `…-a-shadowed-param-spread-into-a-style-object` | both reject | the one position that asks the resolved value for a shape rather than for a value |

The split write set is the reason steps 3 and 4 get four rows between them. It
was meant to change no outcome, and the only way to see that it still fills both
halves is a row per producer refusing on its own.

Nine rows are refusals. One holds the outcome only and says so: the spread
refuses with `Referenced constant is not defined.` here against
`Only static values are allowed inside of a create() call.` upstream. The other
eight refuse with the same *sentence* on both sides -- never the same message,
since this compiler prefixes the evaluator's key path and upstream prefixes an
absolute file path, which is exactly why the harness compares acceptance and not
text. That is the gap [17](./17-the-corpus-cannot-report-a-changed-refusal.md)
owns; each note names the sentence so a wording change is at least readable
against the row.

Four rows go beyond the bullet list above, which asked only for member mutation
of a `const`: the two other producers the same walk records, the reassignment
half of the split write set, and the control both halves need. They are here
because the split was the part of the reorder most likely to change an outcome
silently, and one row per producer is the only way a half that stopped being
filled would report.

Three rows replace what began as one. A row naming an unclosed paren, bracket
and quote together measured only the first: the compile ends at the first
refusal. Split apart, they also disagree -- the paren and the quote are refused
with two different sentences, and the unclosed bracket is not refused at all but
emitted verbatim by both compilers.

### The case that did not agree

The depth guard, written at eight levels with the pseudo-classes nested
`:hover` > `:focus` > `:active` > …, read **divergent** — and not on the
resolution. Declarations matched leaf for leaf, one inline variable each; the
leaf *selector* did not. Four probes narrowed it: two nested pseudo-classes agree
in either order, three agree only when the nesting order is already alphabetical.
Upstream sorts the whole accumulated list; this compiler sorts each pair as it
nests and appends the next.

Filed as [19](./19-three-nested-pseudo-classes-hash-differently.md), with
its own corpus row
(`modules-three-nested-pseudo-classes-hash-differently`, `expected: divergent`)
holding the three-pseudo-class shape it is smallest in. It predates
this effort — the ordering is the same before and after the chain reorder — so it
is not a regression of the reorder, and it is not given an expected-divergence
verdict on a row that is supposed to be measuring resolution: the depth guard now
nests alphabetically and reads identical, and the divergence is measured once, in
the row written for it. That has a cost worth naming: non-alphabetical nesting
*at depth* is now covered by nothing, and putting it back is a checkbox on 19.

### The run

`pnpm run --filter=@stylexswc/rs-compiler parity`, on a build of this branch:
912 subjects, **0 changed verdicts**, exit 0. The `modules` set alone is 109
subjects, 97 of them carrying an expectation.
