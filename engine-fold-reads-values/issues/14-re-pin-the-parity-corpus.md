# 14 — Re-pin the parity corpus

**What to build:** The corpus states what the two compilers now actually do,
and cannot quietly acquire a new divergence.

Every entry in the corpus was measured against a guard that read syntax. Most
of them are about to change verdict: entries that recorded the reference
compiler compiling where this one refused should now record agreement, and
each of those flips is a claim that needs its own row rather than a silent
disappearance. The measured gap alone is thirty-five methods across two
receiver shapes, plus three static surfaces.

One category is expected to survive, and only one: the locale-sensitive
methods. Their reasons were measured and belong in the record — the engine's
internationalisation support reintroduces the exact dependency conflict that
forced the engine to be vendored, it carries no locale data and the provider
is not vendored either, it would not fix the number-formatting method
regardless, and with no locale argument the reference compiler takes the
host's default so its own answer is machine-dependent.

The harness also gains a rule it does not have: an entry recording that the
reference compiler compiles where this one refuses must carry a written
reason, and a row without one fails. That is what stops the exception list
reopening later.

**Blocked by:** 07, 08, 09, 10, 11, 12, 13.

**Status:** resolved

- [x] Every existing entry is re-measured and its verdict reflects current
      behaviour
- [x] Each entry that flips to agreement carries a row saying so
- [x] Rows are added covering the thirty-five methods on both receiver shapes
      and the three static surfaces
- [x] The surviving "reference compiles, we refuse" categories are **four**, not
      one, and the locale-sensitive methods carry all four measured reasons
      -- the four reasons are written, and the premise did not survive the
      measurement. Ruled on in ADR 0008, which amends the count and gives each
      category its kind: the locale methods, the value bridge, an unreadable
      amplification length, and an escaping property read.
- [x] The two configurable ceilings are recorded as configuration rather than
      as divergence
- [x] The harness fails an entry of that category that carries no written
      reason
- [x] The corpus staleness check passes

## Comments

### From 07

Two rows for this ticket to carry.

**One verdict already moved and is re-pinned.**
`modules-1266-object-own-keys-of-a-nullish-receiver` went from `both-reject` to
`both-reject-divergent`: `Object.keys(null)` is the engine's own throw now, and
Boa words it `cannot convert 'null' or 'undefined' to object` where the reference
runtime says `Cannot convert undefined or null to object`. Both name the receiver
and neither builds the module. The `expected` and the reason are updated in
`modules.json`; nothing further is owed unless the corpus wants the wording
itself pinned.

**One divergence has no row and no owner.** `Object.getPrototypeOf({a: 1})` folds
to `[object Object]` upstream and refuses here, because a prototype is an object
carrying functions and a function has no compile-time form the fold will carry
back — the same boundary that refuses `['a'].concat(String)`. It is the outward
bridge's rule rather than the static surface's, and it is deliberate, so it wants
a corpus row recording agreement-not-wanted rather than a fix. Pinned meanwhile
as `math_and_object_statics::a_static_answering_a_prototype_refuses_on_the_way_back`.

## Answer

The corpus did not need re-measuring. It needed the rule that keeps it honest,
and thirty-eight rows it never had.

### The re-measurement was already done, and that is the finding

The ticket expects most entries to flip verdict, because every one of them was
pinned against a guard that read syntax. Measured on this branch, **nothing
flipped**: 1168 subjects, 0 changed, 0 unexpected. Tickets 05-13 each re-pinned
the rows their own change moved, and left the row that says the divergence was
closed rather than forgotten -- `modules-06-mutating-array-method` and
`modules-06-unwritten-amplification-count` are the two that carry that sentence
in full. So the first two criteria were met by the work that unblocked this
ticket, and what re-measuring bought was the evidence for saying so rather than
a diff.

That reading is worth writing down because the opposite conclusion was available
and wrong: a corpus that reports no change after a change this size looks like a
corpus nobody re-ran. It was re-run, under `@stylexjs/babel-plugin` 0.19.0, and
the harvest staleness check reports 826 declarations up to date.

### Thirty-eight rows, and why each has two halves

One row per method in the measured gap -- thirteen on an array binding,
twenty-two on a string binding -- plus one each for `Math`, `Object` and
`Number.prototype`. Every one reads `identical` with non-empty declarations on
both sides.

Each row asks the method on a **named** receiver beside the same call on a
receiver **written out**, and the two halves answer different declarations on
purpose. Written to answer the same one, they would not measure what the row is
for: the half that was never broken emits the rule by itself, StyleX gives the
two halves one class, and the row goes on reading `identical` after the half
under test has stopped folding. The first draft of these rows had exactly that
defect and it does not show in a green run, which is the argument for the note
in the README next to it.

`Number.prototype` is the one surface with no written-out half. A method call on
a number written into the source is refused by both compilers -- pinned as
`modules-06-numeric-literal-receiver` -- so that half would measure the refusal
instead of the surface.

A curated row per method is not a claim to have covered the surface. The method
nobody listed is what ticket 15 is for. What a row here is is the place a
*reason* gets written: `filter` records the one shape that was wrong rather than
refused, `map` the chain that died at its second link, `substr` that nothing
enumerates method names any more, `repeat` which ceiling bounds its result,
`toString` that a method reached through the prototype chain folds for the same
reason a declared one does.

### The rule the harness gained

A row where the reference compiler compiled and this one refused must carry a
written reason -- a `note` on the entry, or a refusal family that claims it.
Rows with neither are listed under **Refusals with no reason written down** and
the run exits non-zero.

It is the weakest of the four gates and the one the other three cannot reach. A
recorded `expected` satisfies all of them while saying nothing about why the
refusal is wanted, so a refusal added for a reason nobody wrote down outlives
the argument for it and the corpus reads as though someone had checked. What is
required is only that a reason *exist*; whether it is a good one is a person's
judgement and not a thing a harness can hold.

Two decisions inside it are worth stating. The direction is asymmetric -- this
compiler refusing where the reference compiles costs an author a build, and the
reverse costs nobody anything -- so only the first carries the obligation. And a
whitespace-only note fails the same way an absent one does, since otherwise the
cheapest way past the gate is a space.

Every one of the 28 rows in scope already carried a note, so the gate is quiet
today. It was verified by removing one: the run names
`modules-06-locale-string-on-an-object` and exits 1.

### Configuration is not divergence

A corpus entry may now name the option whose value decides its refusal. Three
rows carry one -- `maxFoldedCharacters`, `maxFoldedEntries`,
`maxEvaluationDepth` -- and what they have in common is that the same source
folds to the same value on both compilers once the option passes the number the
input needs. The reference compiler compiling is the absence of the setting
rather than a divergence from it. Those rows print `(configured: <option>)`, are
counted on their own summary line, and are listed under **Configured ceilings**.

They are still expectations first: a configured row whose verdict moves is
`changed` like any other recorded one, because a ceiling that has stopped
refusing -- the guard moved, or the default rose past the input -- is exactly
what a row read as accounted for would otherwise go quiet about. The loader
refuses a row that names an option and records no verdict for the same reason,
and one that carries no note for the other: a row naming a knob and not a reason
still records a build the reference compiler completes. The option name is a
union checked against `StyleXOptions` rather than a free string, so a typo fails
to load instead of loading, printing and counting while naming nothing a reader
can raise.

The ticket names two ceilings; three rows carry the field. The evaluation depth
is a configured ceiling by the same argument and its row already made it in
prose, so leaving it out would have been the inconsistency.

**One row that looks like a ceiling is not one.**
`modules-12-amplification-across-a-chain` is refused because the length the call
would build cannot be read at all -- its receiver is itself a call -- so no
value of the option folds it. It stays a divergence with a written reason.

### The premise that did not survive: four categories, not one

The ticket expects the locale-sensitive methods to be the only surviving
"reference compiles, we refuse" category. They are not, and the reason is
chronology: this ticket was written before 09 and 12 were measured. The count
settled at **four**, and ADR 0008 is where it is ruled on and where each
category's *kind* -- wanted, configurable, held open -- is written down:

1. **Locale-sensitive methods** (2 rows) -- the sanctioned one, and
   `modules-06-locale-sensitive-method` now carries all four measured reasons in
   full: the dependency conflict that forced the engine to be vendored, the
   absent locale data and unvendored provider, the number-formatting method it
   would not fix regardless, and the host default that makes the reference
   compiler's own answer machine-dependent.
2. **The value bridge** (2 rows, ticket 09) -- `String()` of the environment
   object with no `env` option set, and `Number()` of a function. Ticket 09
   landed four and left the theme reference for 16 to rule on against the spec;
   the ruling was that it crosses, as the string its own `toString` answers, so
   that row and the spread holding a function are `identical` now.
3. **An unreadable amplification length** (tickets 12 and 21) -- a length
   arriving through a parameter, and a count off a receiver that is itself a
   call.
4. **An escaping property read** (ticket 15) -- `constructor`, `call`, `apply`
   and `bind` walk off the value they were written on.

Nothing here is a defect to fix in this ticket: each row is deliberate and each
says why, which is precisely what the new gate requires. What was owed was a
decision about whether the spec's "one category" sentence or the branch's count
stands; ADR 0008 amends the sentence, and the spec's bridge section was amended
with it under ticket 42.

The 42 remaining rows in this category across the whole corpus are outside the
effort: CSS value guards claimed by refusal families, and theme-object shapes
from #1266.

### What review changed

Three findings were worth the edit and are in the diff.

The loader's requirements were the substantive one. The comment beside the gate
claimed the loader already required a note on a configured row; it required only
the verdict, so the claim was false and a note-less configured row would have
failed on the run's gate with a message about a rule the loader had not applied.
Requiring the note at load is the honest resolution — a configured row owes the
same reason every other row in that category owes, and saying so where the row
is written beats saying it on the next run.

`configuration` was a free string for a closed set of three compiler options,
which `guidelines/stack/TYPESCRIPT.md` argues against and which would have let a
typo load silently. It is a union now, checked against `StyleXOptions` so that
renaming an option in the compiler fails to compile here.

The locale row's note overreached. Written to carry the four reasons, it also
repeated the spec's claim that its category is the only surviving one — which
this ticket's own finding contradicts. The corpus note is the durable record, so
it now names the other two categories and leaves the count to the decision
record.

Two smells were raised and declined, both on the surgical-changes rule. Adding a
`Stance` kind takes six coordinated edits and a `Record<Stance['kind'], …>`
shared by the counter and the label would collapse most of them — but the five
sibling `if` lines and the ternary chain predate this ticket, and rewriting them
for readability is a change no criterion here asks for. A doc comment per module
restating what `fails` lists is the file's own established style; the one copy
this ticket introduced, in the test header, is gone and points at `fails`
instead.

### Verified

`cargo check/clippy/test --workspace --all-features` (clippy under
`-D warnings`), `pnpm typecheck`, `pnpm lint:check`, `pnpm lint:type-aware`,
`pnpm format:check`, `pnpm test` (86 tasks). On the harness itself: `pnpm
parity` (1168 subjects, 0 changed, 0 unexpected, 3 configured),
`pnpm parity:positions` (18 subjects, 0 unexpected), `pnpm parity:harvest:check`
(826 declarations, up to date), `pnpm fuzz:pseudo-order` (1000 pairs, 0
disagreements) and the nightly `pnpm fuzz:shorthand` (153624 subjects, 0
unexpected).
