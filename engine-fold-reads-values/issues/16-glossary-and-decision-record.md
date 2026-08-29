# 16 — Glossary and decision record

**What to build:** The written record says what the compiler now does, and why
the engine is here at all.

Two glossary entries assert that the fold guard reads syntax and record the
named-receiver gap as known and unintended. Both are wrong once this effort
lands. The refusals each entry lists have changed too: one is deleted, one is
solved rather than bounded, and two have become configuration. A third entry,
covering what a refused fold means for an author, needs checking against the
new diagnostics.

Separately, and more importantly: the commit that introduced the engine
describes itself as a throwaway not for merging, and it is on the default
branch. No throwaway code survives — the temporary comparison script was
deleted by the commit that shipped the fold — but that sentence is currently
the only written statement about why a large embedded JavaScript engine is in
the default branch's dependency graph, and it says the opposite of what is
true. A decision record replaces it.

The glossary is a glossary. It carries terms, not implementation, and the
decision belongs in the record rather than in a definition.

**Blocked by:** 14, 17, 18, 20, 21, 23, 24, 25.

**Status:** resolved

- [x] The entries asserting the guard reads syntax are rewritten, and the
      paragraph recording the named-receiver gap is gone because the gap is
      gone
- [x] The list of refusals in the glossary matches the refusals that exist
- [x] The entry describing what a refused fold means is checked against the
      diagnostics from 02
- [x] A decision record states why the guard now reads values rather than
      syntax
- [x] It states why an embedded JavaScript engine is a permanent dependency:
      what it costs in artifact size and lock file entries, why it is vendored
      rather than taken from the registry, and why its instance is leaked per
      thread rather than dropped
- [x] It records the locale exception with the four measured reasons, so the
      next person does not re-derive them
- [x] Issue 12 of the effort that shipped the fold — the named-receiver gap,
      currently deferred and untriaged — is cross-referenced to this effort as
      the work that closes it, without altering that ticket

## Answer

**The three entries this ticket owned needed less than it expected**, because
the tickets between 05 and 23 corrected each entry they would otherwise have
made wrong. `Fold guard` already reads *values, not only syntax* and the
named-receiver paragraph is gone; the refusal list matches what exists; and
`Refused fold` still describes what 02's diagnostics do, since every refusal in
the area is a deopt carrying the rule that refused it. What this ticket added to
the glossary is the two sentences 24 and 25 made owed: the dispatch below the
fold *applies* a call through a name rather than handing it back, and the number
bridge keeps less of the text it reads than the string bridge does.

**The decision record is
`crates/stylex-transform/docs/adr/0008-the-fold-guard-reads-values-and-the-engine-is-permanent.md`.**
It states the rule and its price, the engine's cost, and the taxonomy the
harness cannot hold.

The engine's cost is the measurement the throwaway commit already carried, and
it is confirmed rather than re-derived: the artifact grows 5.6–6.1 MiB, 58–60%,
and the lock file by 49 packages. Counted independently from `cargo metadata`,
exactly 49 of the 425 crates the compiler resolves are reachable only through
`boa_engine`. The vendoring reason is the `icu` bound conflict; the leak reason
is the undefined drop order between the engine and its collector's own
thread-local, where dropping late underflows a reference count inside a
destructor and aborts.

**The counts the accounts above asked for, settled.** Four categories survive,
across three kinds — wanted, configurable, held open. The spec's "one category"
claim is amended to four rather than argued back, and the ruling 09 deferred is
made: the token group under `String()` is the one row *held open*, because
answering both `String(group)` and `group.token` from one carriage needs a name
for a subexpression the author never wrote, which is a transport this effort
chose against. The two categories 15 and 19 counted are gone, closed by 19 and
22; the two 22 and 23 filed are gone, closed by 24 and 25.

Measured on the `modules` corpus after this work: 311 subjects, 0 changed,
0 unexpected.

### From 14

**One decision to rule on, and it is a spec sentence rather than a defect.**

The spec says locale-sensitive methods are "the one remaining category where the
reference compiler compiles and this compiler does not", and 14's criteria
repeat it. Measured on this branch there are three, all within the effort and
each carrying a written reason:

1. **Locale-sensitive methods** (2 rows) — the sanctioned one.
   `modules-06-locale-sensitive-method` carries all four measured reasons.
2. **The value bridge** (4 rows, from 09) —
   `modules-09-string-of-the-environment-object`,
   `modules-09-string-of-a-token-group`,
   `modules-09-string-of-a-spread-holding-a-function` and
   `modules-09-number-of-a-function`. A value the bridge cannot carry inward has
   no fold, and upstream folds each. 09 already asked this ticket to rule on the
   token group specifically, since the spec says a resolved theme reference
   crosses inward and it cannot under one carriage of the name.
3. **An unreadable amplification length** (1 row, from 12) —
   `modules-12-amplification-across-a-chain`. Not a configured ceiling: no value
   of `maxFoldedCharacters` folds it, because the receiver is itself a call and
   has no length the guard can read.

What is owed is one sentence in the decision record: either the spec's "one
category" claim is amended to three, or one of the two later categories is
argued back to agreement. Nothing needs fixing to leave them standing — 14 gave
the harness a gate that fails any such row carrying no written reason, so the
three are the whole of the category and cannot grow quietly.

### From 15

**The category the decision record has to rule on has grown by one, and the
growth is the sweep working rather than a new divergence appearing.**

The comment above counts three surviving categories where the reference compiler
compiles and this one refuses. The generated sweep adds a fourth and confirms
one of the three has a wider population than the curated rows showed:

4. **A callback reached through a name** (1 row) —
   `modules-15-a-callback-reached-through-a-name`. Not deliberate: it is a gap,
   filed as issue 19, and it generalises past the one row to every
   callback-taking method. It is recorded with a reason so the sweep reports it
   once rather than nightly, which means the decision record has to say plainly
   that a written reason is not the same thing as a wanted refusal — the corpus
   has both kinds now, and only the row's own note tells them apart.

Two further rows were added, and neither is a new category: `constructor` on
every prototype the sweep crosses
(`modules-15-a-read-that-escapes-onto-the-function-graph`) is the escaping-read
guard, deliberate and previously unrecorded; the impure statics
(`modules-15-an-impure-static`) are refused by both compilers, so nothing an
author writes is lost.

So the sentence this ticket owes is now two: whether the spec's "one category"
claim is amended, and whether the record distinguishes a refusal argued for from
a gap held open. The second is the one a later reader needs, because the harness
treats them identically and the corpus is where the difference is written.

**Blocked by 19** as well, since the fourth category disappears if 19 lands
first — which is the outcome to prefer over documenting it.

### From 18

**The category has grown by one, and the count the record has to rule on is now
five.** The comments above count four. 18 adds:

5. **A declared array length** (3 rows) —
   `modules-18-a-length-a-call-declares` and
   `modules-18-a-declared-length-that-never-crosses` are _configured_: raise
   `maxFoldedEntries` past the declared length and both fold to upstream's
   value, so they belong with `modules-06-amplified-length` rather than with the
   divergences. `modules-18-a-declared-length-inside-a-callback` does not: the
   length is readable and no value of the option bounds it, because a callback
   runs once per element of a receiver nothing measured. That row is 12's
   distinction on the other ceiling and the record should say so beside 12's own.

**And it sharpens the sentence 15 asked for.** 15 said the record has to
distinguish a refusal argued for from a gap held open, because the harness treats
them alike. 18 adds a third kind, which the harness _does_ tell apart and a
reader may not: a refusal an author can configure away. Five categories, three
kinds — wanted, configurable, and held open — and only the row's own note says
which.

**One glossary entry moved already.** The `Fold guard` paragraph on
length-amplifying calls now carries the declared-length half, and the
`Allocation ceilings` entry says `maxFoldedEntries` bounds a length a call
declares as well as one it carries. Neither was an entry this ticket listed as
wrong; they were entries 18 would have made wrong by leaving them alone. The
three entries this ticket owns are untouched.

### From 19

**The fourth category is gone, and a fifth of the same kind took its place — so
the count is still five and the record's job is unchanged.** 15 added _a
callback reached through a name_ as a gap held open;
`modules-15-a-callback-reached-through-a-name` now records agreement and its
account is deleted, which is the outcome this ticket said to prefer over
documenting it.

What replaced it is narrower and was measured while closing that one:

4. **A call reached through a name** (1 row) —
   `modules-19-a-call-reached-through-a-name`. A function _passed_ by name now
   folds; one _called_ through a name does not, because the guard admits a callee
   only where it is a member expression or an unshadowed global. Also a gap
   rather than a wanted refusal, filed as issue 22, so 15's sentence — that a
   written reason is not the same thing as a wanted refusal — is the sentence
   this row needs too, unchanged.

**Blocked by 22** now rather than by 19, on the same terms: the category
disappears if 22 lands first, which is again the outcome to prefer over
documenting it.

### From 20

**No sixth category, and the count of _kinds_ is where the change lands.** The
two rows 20 adds — `modules-20-a-string-the-evaluator-doubles` and
`modules-20-a-string-a-template-grows` — are both `configuration:
maxFoldedCharacters`, so they are 18's third kind: a refusal an author can
configure away. They join `modules-06-amplified-length` rather than the
divergences, and the sentence 18 asked for covers them unchanged.

**What the record owes is one line about which ceiling bounds what.** Before 20,
every reading of `maxFoldedCharacters` sat where a value crossed a fold, and the
glossary said so. It now also bounds a string the evaluator grows _itself_, at
the two expressions that grow one — `+` and an interpolation — because nothing
crosses a fold there and a chain that doubles its own result is innocent one line
at a time. `concat` and `repeat` were measured against the same question and
needed nothing, both being calls the fold already bounds in and out. That is the
decision, and it is why the bound is on the growth rather than on what a binding
may hold: an inline `(a + a).length` holds nothing and allocates the same.

**Three entries were corrected with the change rather than left here**, on 18's
precedent — they were not entries this ticket listed as wrong, they were entries
20 would have made wrong by leaving them alone: the `Allocation ceilings`
glossary entry in `stylex-structures/CONTEXT.md`, the `maxFoldedCharacters`
section of the compiler README, and the doc comments on
`StateManager::character_ceiling` and `::entry_ceiling`, each of which counted the
sites that spend its number. The three entries this ticket owns are still
untouched.

### From 22

**The fourth category is gone, and nothing replaced it — the count is four.**
19 added _a call reached through a name_ as a gap held open;
`modules-19-a-call-reached-through-a-name` now records agreement and its account
is deleted, which is again the outcome this ticket said to prefer over
documenting it. So the surviving categories are 14's three plus 18's declared
length inside a callback, and the two sentences 15 and 18 asked for stand
unchanged: a written reason is not the same thing as a wanted refusal, and a
refusal an author can configure away is a third kind again.

**One sentence the record now owes, and it is short.** 19's account said the
admission was "one line of dispatch away", and it was — but not the line 22
proposed. Narrowing to a callback _scope_ would have closed one shape of three,
because a callback reached through a call in the arguments of `map` is walked in
the module's scope, not the callback's. What separates the calls the fold may own
from the calls it may not is **position**: the outermost call the caller asked
about stays the dispatch's, and a call nested inside an expression the fold has
already claimed is the fold's. That is not a new rule — `admit_a_stylex_function`
drew the same line for the same reason, and the record should say so, because a
reader who finds two rules will look for the difference between them and there is
none.

**Why it is the rule and not a hedge, measured.** The dispatch below the fold
already answers `content: inner('a')` with upstream's own class name, so
admitting the outermost call gains no fold and loses the resolution a dynamic
style's parameters and the injected function map depend on. That measurement is
pinned as a test rather than left in this file.

**Category 3 gained a population and not a row.** A length written on a
parameter of a function reached through a name — `const big = (x) =>
x.repeat(20)` — refuses for the reason 12 recorded: the parameter holds an
argument, and an argument's width is not something the guard reads. It refused
before 22 as well, with the general sentence rather than the rule's, so what
changed is the words. It is pinned as a test rather than as a corpus row, since
the row 12 already carries says the same thing.

**Nothing this ticket owns moved.** `Named callback`'s closing paragraph — which
said calling a function through a name is not answered, "recorded rather than
intended" — was made wrong by 22 and was corrected with it, on 18's and 20's
precedent. So was the `Fold guard` sentence claiming nothing the guard carries
records where in an expression it is: that is still true of what the guard
_carries_, and one rule now reads a position passed beside it. The three entries
this ticket owns are still untouched.

**One new gap, and it is not the fold's.** An outermost named call whose argument
is another named call — `inner(inner('a'))` — fails in the dispatch with a
sentence about a binary operand's type. It fails identically before 22, so it is
not a divergence 22 created and not a category the record has to count; it is
filed as issue 24. Whether it blocks this ticket is a judgement: the record's
claim is that the dispatch answers the outermost named call, and that claim is
true of every shape but this one.

### From 23

**No new category, and the count of kinds is unchanged — but the third kind now
has a population the record should not describe as small.** The two rows 23 adds
— `modules-23-an-array-the-interpolation-joins` and
`modules-23-an-array-the-concatenation-joins` — are both `configuration:
maxFoldedCharacters`, so they are 18's third kind, a refusal an author can
configure away. They join `modules-06-amplified-length` and 20's two rather than
the divergences, and the sentence 18 asked for covers them unchanged.

**What the record owes is one line, and it sharpens 20's.** 20 asked the record to
say which ceiling bounds what, and answered it for the two expressions that grow a
string. A third site now spends the same number and is neither of them: the join
an array's `ToString` performs, which happens _inside_ an interpolation or a `+`
rather than beside it. The bound went there because the alternative was bounding
the elements before they render, and rendering is where the cost is — each
element's `ToString` copies a string the value already holds. So the line is that
`maxFoldedCharacters` bounds a string wherever the evaluator _writes_ one, not
only where an expression grows one, and the refusal still names the expression the
author wrote rather than the join inside it.

**Three documents were corrected with the change rather than left here**, on 18's
and 20's precedent — the `Allocation ceilings` glossary entry in
`stylex-structures/CONTEXT.md`, the `maxFoldedCharacters` section of the compiler
README, and the doc comment on `StateManager::character_ceiling`, each of which
counts the sites that spend its number. The three entries this ticket owns are
still untouched.

**One new gap, and it is not the fold's.** `+a` over the same array folds to
upstream's own `NaN` after 10.2 seconds, because `ToNumber` reaches its number
through the same join and the number bridge collects it. Refusing there would
diverge from a fold upstream completes, so it needs a ruling as well as a sink —
filed as issue 24's sibling, issue 25. Whether it blocks this ticket is a
judgement on the same terms 24 was: the record's claim is about what the ceiling
bounds, and that claim is true of every site the evaluator grows a string at.
