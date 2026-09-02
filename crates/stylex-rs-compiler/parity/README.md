# CSS value parity harness

Runs a corpus of CSS declarations through `@stylexswc/rs-compiler` and through a
pinned `@stylexjs/babel-plugin`, and reports — per declaration — whether the two
produce the same class name, the same rule text, and the same style-object
shape.

A StyleX class name is a hash of the canonical declaration text, so that text is
a compatibility contract. Any setup that mixes the two compilers (SSR built by
one and client bundles by the other, cached HTML, an incremental migration,
snapshot tests written against either) breaks silently when they disagree: the
markup names a class the stylesheet does not define, and nothing errors.

It lives outside the Rust test suite so `cargo test` never needs a Node
toolchain, and the harvester's own unit tests run under this package's `vitest`
suite with `parity:harvest:check` ahead of them, so a corpus that has fallen
behind the Rust tests fails rather than waiting for someone to think of it.

## Where it runs

Nothing ran the comparison itself for a long time — the staleness check above is
a scan of Rust sources, and the part that actually consults the reference
compiler ran only when a person remembered. That is how the corpus came to be 41
declarations behind without anyone noticing.

| Harness                | Runs                                   | Cost  |
| ---------------------- | -------------------------------------- | ----- |
| `parity`               | `checks` matrix, every pull request    | ~2.5s |
| `parity:positions`     | the same step                          | ~2s   |
| `fuzz:pseudo-order`    | the same step                          | ~2s   |
| `fuzz:shorthand`       | `parity-sweep`, the nightly schedule   | ~97s  |
| `fuzz:prototypes`      | the same job                           | ~2s   |
| `parity:harvest:check` | ahead of this package's `vitest` suite | <1s   |

Timings are wall clock on an Apple Silicon laptop with a warm build.

**Why CI and not a hook.** Both comparison harnesses need a built `dist/` and a
Node toolchain, which is a real cost to put in front of every commit — that
argues against a pre-commit hook, not against running them at all. The `checks`
matrix already has both: it restores the `build-public-packages` Turbo cache and
replays the build from it, so the marginal cost of the step is the ~10s the two
harnesses take. The staleness check is the one that _is_ cheap enough to sit in
front of a test script, and it does, because it needs neither.

`fuzz:pseudo-order` is generated rather than curated and still runs per pull
request, unlike the shorthand sweep, because of what it guards: a condition-key
ordering divergence costs a class name, which is the failure this whole harness
exists for, and it costs ~2s. The shorthand sweep guards where a value is cut,
which shows up in a rule rather than in a hash.

**Why the sweeps are nightly.** The shorthand harness crosses an alphabet with
itself — ~154k subjects, two compiler runs each — which is roughly forty times
the curated one. That buys nothing per commit: a value-splitter defect shows up
when a value pass or the alphabet changes, and a sweep once a night reports it
as surely as one per pull request would.

The prototype sweep is nightly for a different reason, since it is cheap: what
it guards moves when the **surface** moves. It reads every method off the
language at run time, so a new row appears under an engine upgrade, a widened
guard or an added receiver shape — none of which arrive one commit at a time.
Both are listed in the `pr-validation` gate, through the one job that carries
them, so a failing sweep fails rather than sitting green beside it.

**What a failing run says.** Each harness prints the versions it resolved before
anything else, because the reference plugin is held by the lockfile rather than
by an exact range in the catalog — it moves under a `pnpm update` without
anything in this directory changing, so a report that does not name it cannot be
compared with an older one, and it is the first thing to check when a run starts
failing on a corpus nobody touched. Each also prints, on the way out, what to do
about the thing it failed on: which file holds the expectation, and which of the
two possibilities — the compiler moved, or the expectation stopped measuring
anything — the reader has to tell apart. The person who reads a red run first
will not be the person who wrote the corpus.

## Running it

```sh
pnpm run --filter=@stylexswc/rs-compiler build     # the harness reads dist/
pnpm run --filter=@stylexswc/rs-compiler parity
```

The harness loads the compiler from `dist/`, not from the Rust sources. **A
report is only about the last build.** Rebuild after touching a crate or the
verdicts are stale.

It exits non-zero for four reasons, and each one is a report that has stopped
being read: an entry whose recorded `expected` verdict no longer holds, a
refusal family no row in the corpus reaches, a divergence nothing accounts for,
and a build the reference compiler completes that this one refuses with no
reason written down.

A divergence that should not fail a run has two ways to say so, and both are
statements a later reader can check rather than suppressions: record its verdict
as `expected` on the corpus entry, or write the refusal family that accounts for
it.

The fourth is the weakest of the four and the one the other three cannot reach.
A refusal the reference compiler does not make is the one an author feels — the
build stops where the other compiler's completes — and a recorded `expected`
satisfies every other gate while saying nothing about why that is wanted, so a
refusal added for a reason nobody wrote down outlives the argument for it and
the corpus reads as though someone had checked. What is required is only that a
reason exist, in one of the two forms the corpus already has: a `note` on the
row, or a refusal family that claims it. Whether the reason is a good one is a
person's judgement and not a thing a harness can hold. Rows that carry neither
are listed under **Refusals with no reason written down**. The direction is
asymmetric on purpose: this compiler refusing where the reference compiles costs
an author a build, and the reverse costs nobody anything, so only the first
carries the obligation.

| Flag                     | Effect                                            |
| ------------------------ | ------------------------------------------------- |
| `--only-mismatches`      | print only the entries that disagree              |
| `--set <name>`           | limit to one corpus set; repeatable               |
| `--filter <substring>`   | limit to entries whose subject text contains it   |
| `--json <path>`          | also write the full machine-readable report       |
| `--font-size-px-to-rem`  | enable the font-size conversion in both compilers |
| `--style-resolution <n>` | which resolution both compilers run under         |

## Reading a verdict

| Verdict                  | Meaning                                                                                                                                                                                                                                                                                              |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `identical`              | Both accepted and agreed byte for byte.                                                                                                                                                                                                                                                              |
| `identical (empty)`      | Both accepted and emitted **nothing**. Agreement about no declaration at all, which is not evidence of parity — a property both compilers drop, or a fixture that failed to carry the value in, agrees just as loudly. Counted apart so a corpus that stops measuring anything shows up as a number. |
| `divergent`              | Both emitted the same properties but spelled a value differently — and therefore hashed a different class name. This is what the harness is for.                                                                                                                                                     |
| `structurally divergent` | Both accepted but emitted different properties, or a different number of them. The divergence is in shorthand expansion or property validation, not in value normalization.                                                                                                                          |
| `acceptance divergent`   | One accepted and the other rejected.                                                                                                                                                                                                                                                                 |
| `both reject`            | Both rejected, and complained about the same thing. What is compared is the sentence each wrote, with the text saying _where_ it happened removed — see **Comparing two refusals** below.                                                                                                            |
| `both reject (diverged)` | Both rejected, for reasons they word differently. A refused build hands the author nothing but the message, so two compilers stopping one for opposite reasons have diverged in the only behaviour a refused input has.                                                                              |

Both compilers receive the same module text and the same option object,
constructed once in `lib/compare.ts`. Option drift would surface as a
normalization divergence and send the reader chasing the wrong thing, so options
are never spelled out per subject.

`--style-resolution` is the one option a run is expected to vary, because what
differs between the three is which longhands a shorthand becomes and what order
they land in — property expansion rather than value spelling, and a class name
and a style object's key order both depend on it. Absent the flag a run uses
`property-specificity`, which is what both compilers fall back to on their own
and therefore what every recorded verdict was taken under; the subject block
prints it either way, since a report that does not say which resolution it
measured cannot be compared with another one. A name that is not one of the
three is refused rather than defaulted.

All three have been run over the whole corpus. `application-order` was the one
that had never been: it reported seven rows where both compilers emitted the
same CSS and the same set of style-object keys in a different order, from five
shorthands that spread another shorthand's expansion into the middle of their
own list and appended it instead. That is fixed, and the position of every one
of those spreads is now pinned in `application_order.rs` — the corpus found it,
but a unit test is where it belongs, since nothing runs the corpus under a
non-default resolution.

Right-to-left rule text is compared but never printed. An RTL-only difference is
a value difference all the same, and without comparing it a verdict would call
such a pair identical; the left-to-right spelling shows what changed more
plainly, so that is what the report displays.

The **style-object shape** is the other half of an answer the rule text cannot
carry. A property whose value is `null` emits no CSS, so two compilers that
disagree about whether the property exists at all agree on every rule — and a
`null` is how an absent value is spelled, which is what unsets an earlier
declaration of the same property when two styles merge. `lib/style-object.ts`
reads each `$$css`-marked object literal out of the emitted module and records
which keys exist and, per key, whether it carries a class name or an absence.
Class names are replaced by a placeholder there, so a hash divergence is
reported once, by the half that already reports it. The shape is printed only
when it is what differs.

This is deliberately not a comparison of emitted JavaScript. The two compilers
print code differently — which consumed declarations they leave standing, how an
injection is wrapped, JSX spacing — so comparing the text would report a
divergence on every entry and say nothing about StyleX.

### Comparing two refusals

A verdict that asked only _whether_ both compilers rejected read two refusals
for opposite reasons as agreement, and that is most of what a corpus of
degenerate values holds. What blocked comparing the messages was never the
comparison but the decoration each compiler wraps its complaint in:

```text
[StyleX] a > color > Invalid pseudo or at-rule.
/abs/path/to/value.js: Invalid pseudo or at-rule.
```

Neither wrapper can be hard-coded away — this compiler's carries the evaluator's
key path, which is the authored object's own keys, and upstream's carries an
absolute file path — so `lib/refusal.ts` derives it, from the marker this
compiler brands every diagnostic with and from the filename the harness itself
handed both compilers. It strips only what says _where_: the marker and the
breadcrumbs, the `-->` location line, the repaired rule text a CSS refusal
carries, and upstream's code frame. What survives is the complaint, newlines
included — several diagnostics are two lines in both compilers, and the second
carries the advice.

Every rule is pinned in `__tests__/refusal.test.ts`, including a refusal with no
prefix at all and one whose complaint contains a colon. The reduced sentence is
carried on the outcome as `sentence`, beside the message as it was thrown: the
verdict compares the sentence, the human report prints it, and `--json` keeps
the raw message as the evidence for both.

The upstream version is held by the lockfile, not by an exact range in the
catalog. The subject block prints the version actually resolved, so a report is
attributable either way — but after a `pnpm update`, read that block before
comparing against an older report.

### Comparing where a refusal points

Stripping the text that says _where_ is what makes two messages comparable, and
it leaves the position unmeasured — so a diagnostic naming a line that is
correct as written reads as agreement here. A refused build hands an author two
things, and that is the second one.

`pnpm parity:positions` compares it, over `corpus/positions.json`: one subject
per branch of the reference-resolution chain, plus five CSS refusals, each
refused by both compilers with the same sentence so that the position is all
that is left to disagree about. Verdicts are `identical`, `divergent`,
`no-position` — one side stopped without saying where — `neither-position` —
neither did — and `not-refused`. An entry may pin a known divergence with
`expected`, and the run exits non-zero when an entry's verdict is not what it
expects, in either direction.

The CSS subjects are what `neither-position` was added for, and they are the
whole of that verdict's population. **No CSS refusal in either compiler says
where it happened.** This one throws the sentence with the repaired rule text
appended and writes no frame to stderr, unlike every evaluator refusal beside
it; the reference compiler prefixes the filename and attaches no
`@babel/code-frame` excerpt. That is agreement about a hole rather than a
disagreement about a line, which is why it is not folded into `no-position` — a
`no-position` row is one to act on, and five permanent ones in front of the
reader is how a report stops being read. All five carry `expected`, so the day
either compiler starts framing a value refusal is a changed verdict rather than
silence.

Three of the five value guards are deliberately not asked here: the
declaration-terminating token, the unclosed comment and the nesting budget are
refusals the reference compiler does not make at all — it emits the value — so
there is no second position to compare and the subject could only ever read
`not-refused`. Those three are pinned by refusal family in the value harness
instead. What is asked is the two the reference compiler does refuse, the
two-fault value where the two compilers reach the same sentence from _different_
passes because the guard order here was changed to buy that agreement, and one
condition key, which arrives at the refusal from the style walk rather than from
a value pass. The last is the one that makes the finding general: the silence is
a property of every CSS refusal, not of the value passes.

```sh
pnpm run --filter=@stylexswc/rs-compiler build     # the harness reads dist/
pnpm run --filter=@stylexswc/rs-compiler parity:positions
```

Each subject runs in a **child process**, because the two positions arrive in
different channels: upstream throws its `@babel/code-frame` excerpt inside the
message, while this compiler writes a code frame to stderr and throws the
sentence alone. Node cannot redirect its own file descriptor 2 and a native
write goes straight to it, so capturing that frame means being the parent of the
process that wrote it. `lib/position.ts` parses both, and every shape either
compiler produces is pinned in `__tests__/position.test.ts` — a parser that
silently misreads a frame would turn the whole set green.

Each subject is written to `parity/__fixture__/positions-<id>.js` while the run
lasts, because this compiler locates a refusal in the file it names rather than
in the string it was handed. Those paths are git-ignored.

A path per subject, and that is what lets the children overlap: while they
shared one fixture, each was overwriting the file the last was compiling, so the
run had to be serial and its wall clock was 18 process spawns end to end — each
paying for the `tsx` loader, the addon and `@babel/core` again. The value
harness pins a single filename for the opposite reason, and that reason does not
reach here: it hashes class names, which read the path, while this compares a
line and a column that neither `rustPosition` nor `babelPosition` takes from it.

## Ordering, over random pairs: `pnpm fuzz:pseudo-order`

A condition key's sorted path is hashed into the class name, so the order two
keys sort in is a compatibility contract in the same way a value's spelling is.
The curated corpus pins the shapes someone thought of. This asks the same
question over pairs nobody chose, and it is what the collation dependency in
`crates/stylex-css/src/utils/pre_rule.rs` is held to.

It checks two things, and they are not the same check:

- **Against the reference compiler.** Both compilers are handed the same two keys
  nested, and the class names and rule text have to match. That is the contract,
  and it is the only oracle covering the whole path from key to hash rather than
  the comparator alone.
- **Against the ordering algorithm.** The order this compiler chose, read out of
  the emitted selector, has to match `Intl.Collator` at the **root** locale. This
  is what tells a failure in the first check apart: a comparator that moved,
  against a reference whose own answer moved under it.

Keys are attribute selectors, and not for convenience. Every pseudo-class and
pseudo-element name CSS defines is ASCII, so an attribute selector is the only
way a non-ASCII key reaches the comparator at all — [the collation ADR][adr]
argues that at length. It also keeps a generated key clear of the selector
syntax a random character would otherwise spell, which would make the
subject about parsing rather than about order.

The run also prints how many of its pairs the **build machine's default locale**
and root collation order differently. That is not a defect count: upstream calls
`localeCompare` bare, so its answer follows the machine, and a Swedish or Danish
one sorts `ö` after `z`. It is the remainder `pre_rule.rs` names, reported as a
number so nobody has to remember to re-derive it.

```sh
pnpm run --filter=@stylexswc/rs-compiler fuzz:pseudo-order
pnpm run --filter=@stylexswc/rs-compiler fuzz:pseudo-order -- --pairs 20000
```

The seed is fixed and printed, so a failing pair is reproducible from the report
rather than from the clock.

## The corpus

Four checked-in JSON files under `corpus/`, loaded in this order, with duplicate
subjects collapsed onto the first entry seen. `positions.json` is a fifth, read
only by `parity:positions` — its subjects ask where a refusal points rather than
what either compiler emitted, so running them through the value comparison would
report nine refusals nobody wrote them to measure:

- **`reported.json`** — the six divergences reported in issue #1256, one entry
  per illustrating value. Hand-written.
- **`modules.json`** — whole modules rather than declarations, for the
  questions a declaration cannot ask. Hand-written; see below.
- **`edge.json`** — non-ASCII content, escape sequences, URL bodies containing
  CSS-looking syntax, comments, importance annotations, empty values, and
  unclosed constructs. Hand-written.
- **`harvested.json`** — the CSS declarations the Rust test suites carry, as
  far as the scan below recognizes them. **Generated — do not edit.**

Adding a case means editing one of the three hand-written files. Entries take an
optional `note`, which the report prints next to a mismatch, an optional
`expected` naming the verdict the entry is known to read, and an optional
`configuration` naming the option whose value decides a refusal.

An `expected` verdict is how a divergence someone has already looked at is told
apart from a new one. While it holds, the report marks the entry `(expected)`
and `--only-mismatches` leaves it out. When it stops holding — in either
direction — the entry is listed under **Verdicts that changed**, counted, and
the run exits non-zero, so a divergence that quietly goes away is as loud as a
new one: an entry recording a divergence that no longer happens has stopped
measuring what it was written for. `note` says _why_; `expected` is what the
harness checks.

A `configuration` says the refusal is not a disagreement at all. The rows that
carry one name the two allocation ceilings or the evaluation depth, and what they
have in common is that the same source folds to the same value on both compilers
once the option passes the number the input needs, so the reference compiler
compiling is the absence of the setting rather than a divergence from it. Those
rows print `(configured: <option>)`, are counted on their own summary line, and
are listed under **Configured ceilings** with the option each names. They are
still expectations first: a configured row whose verdict moves is `changed` like
any other recorded one, because a ceiling that has stopped refusing — the guard
moved, or the default rose past the input — is exactly what a row read as
accounted for would otherwise go quiet about. The loader refuses a row that
names an option and carries no `expected` verdict for that reason, and one that
carries no `note` for the other: a row naming a knob and not a reason still
records a build the reference compiler completes. The option name is checked
against the compiler's own options, so a row cannot point at a setting nobody
can set.

Not every ceiling is one. `modules-12-amplification-across-a-chain` is refused
because the length the call would build cannot be read at all — its receiver is
itself a call — so no value of the option folds it, and that row is a divergence
with a written reason rather than a configuration.
`modules-21-an-unmeasured-callback-receiver` is the same distinction inside a
callback: the length written into the body is readable and the ceiling is
compared against it times the receiver's element count, so what refuses is a
reading that count cannot come from — a declared length arriving through a
parameter, and a comparator, which the language runs more often than its
receiver is long. Neither is a number an option can move.

### Refusal families

Half the permanent divergences cannot carry an `expected` verdict. They live in
`harvested.json`, which is regenerated wholesale from the Rust sources, so a
value written there is lost on the next harvest — and the generated corpus next
door has no entries at all to write on. They are pinned by **family** instead:
`lib/refusal-families.ts` holds one entry per reason this compiler diverges on
purpose, each with the verdict a member reads and the test for whether a row is
one.

Both harnesses read that list, so neither can come to disagree with the other
about which refusal is deliberate. A row a family accounts for is labelled
`(pinned: <name>)` and left out of `--only-mismatches`; a row no family claims
is **unexpected**, and that is the number a reader acts on. The report prints
each family's reason under the summary, stated as what agreement would cost.

Between them they account for every divergent row in both corpora. The table is
pinned against the code by `__tests__/refusal-families.test.ts`, which is also
why no count is written here — a number in this file went stale twice before:

| Family                            | Why agreement is not wanted                                              |
| --------------------------------- | ------------------------------------------------------------------------ |
| declaration-terminating token     | Emitting a `;`, `{` or `}` closes the declaration being generated.       |
| reference TypeError               | The reference compiler crashes; agreement means reproducing the crash.   |
| unclosed comment                  | Emitting `/*` comments out every rule injected after it.                 |
| unprefixed custom property        | The `--` prefix is a StyleX rule, so there is no CSS behaviour to match. |
| lone surrogate in a name          | No Rust string holds one, so there is nothing to refuse it later with.   |
| nesting past the recursion budget | Agreement means recursing until the stack aborts the process.            |
| style key off `Object.prototype`  | The reference compiler reaches an inherited method, not a CSS property.  |

A family reads a verdict, or a set of them where the reason survives this
compiler's own behaviour changing around it: a reference crash is a reference
crash whether this side accepted the value or refused it for a fault of its own,
and those read different verdicts. The declaration order is also precedence —
the first family to claim a row keeps it — and no pair overlaps as the table
stands. It is still stated because a family added below an existing one inherits
that precedence silently, so where the next one goes is a decision rather than a
detail.

The list shrinks as well as grows. A family that named which of two true
complaints an author was handed is gone: the declaration-terminating token guard
now runs _after_ the two rejections the reference compiler also makes — an
unclosed function and an unclosed string — so a value carrying both faults is
refused with the same sentence on both sides and reads `both reject`. Nothing
was traded for it: a value whose only fault is the token still reads that
complaint, and the token still outranks the unprefixed custom property, which
the reference compiler has no opinion about.

It grows the same way, out of rows that were sitting as expectations. Two
subjects where both compilers refuse a lone surrogate — in a theme export name
and in a condition key — read `both reject (diverged)` and always will: this
compiler refuses at the point the name is decoded, because no Rust string holds
a lone surrogate at all, which is before the pass the reference compiler refuses
it in. That is not an ordering that could be swapped to buy the same sentence,
so it belongs here, stated as what agreement would cost, rather than in two
`expected` rows that read like unfinished work. Their neighbour did not: a
dynamic parameter spread into a style object was refused here for the binding
and upstream for the call it sits in, both true, and the compiler now names the
call as well — so that row reads `both reject`.

A family is pinned by reason and never by count, so a corpus that grows does not
churn expectations. What is checked instead is that every family still claims
something: a family no row reaches is listed under **Refusal families no row
reached** and exits non-zero, for the same reason a changed `expected` does —
either the refusal is gone, which is worth reading, or the corpus stopped
reaching it.

A family claims a row on its evidence and not on the diagnostic alone, where
there is evidence to read. `declaration-terminating token` asks whether the
value actually carries a `;`, `{` or `}` that is neither escaped nor inside a
string; `reference TypeError` reads both sides rather than only the reference's.
Claiming on a sentence by itself absorbs the guard's own false positives into
this column, and it did: an escaped `\;` closes nothing, the reference compiler
emits it, and the row sat here as a divergence produced on purpose until the
guard was fixed.

Family membership is decided from the complaint this compiler wrote, matched as
a prefix, so rewording a diagnostic un-pins the rows it accounted for and they
come back as unexpected. That is the intended direction — the wording is what a
refused build hands the author. Every family, and every near miss that must
_not_ be claimed, is pinned in `__tests__/refusal-families.test.ts`.

### Module subjects

A declaration entry is `{ id, property, value, origin }` and is wrapped in the
smallest module that carries it. A module entry is `{ id, label, source, origin
}` and is handed to both compilers verbatim. Which one an entry is comes from
whether it carries a `source`, so nothing has to be written down twice and the
generated `harvested.json` stays free of a field whose value never varies.

Most questions about this compiler are declaration questions, because a class
name is a hash of declaration text. A few are not: whether an expression the
evaluator cannot fold is _refused_ or _aborts the build_ is a fact about a
module, and it is what `corpus/modules.json` measures — the inputs reported in
[#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265), where a
method call inside a runtime `sx` condition failed the build.

The comparison is the same one: class names, rule text and style-object shape,
never the emitted JavaScript as text — `ModuleEntry` in `lib/types.ts` says why,
next to the type that would have to change to do otherwise. What a module
subject adds is the ability to ask whether a compiler _reached_ the rules at
all, which is the `acceptance divergent` verdict, and to ask a question a
declaration cannot spell: a `DeclarationEntry` value is a string that goes
through `JSON.stringify`, so a bare `null` or `false` can only be asked as a
module.

What a module subject **cannot** ask is anything about its own filename. Every
subject is handed the same one, deliberately — `haste` resolution and class
hashing both read it, so varying it per entry would vary the output for reasons
unrelated to the subject — and that filename is not a `*.stylex.js` one. A
`defineVars` call hashes the file that declares it, so such a subject refuses
for the filename in both compilers before the value under test is ever read: the
entry reports `both reject` and measures nothing. Ask that one where the
filename is a parameter, which is the Rust suites.

Most entries in that set carry an `expected` verdict, each with the `note` that
says why: some where upstream aborts and this compiler does not, some where both
reject, one where upstream folds an indexed read this compiler refuses, one
where upstream reads a condition key as a property name and emits a key named
after a pseudo-class (a defect this compiler is not going to reproduce), and the
shadowing shapes from
[#1266](https://github.com/Dwlad90/stylex-swc-plugin/issues/1266), which record
`identical` so a regression in binding-aware import resolution reports as a
changed verdict rather than as silence. The count is deliberately not written
here -- it went stale twice.

Not every entry in the set is a question about the evaluator. Several ask how a
condition tree is ordered, which a module is the smallest way to ask: a
condition tree is a style value, not a declaration, and the sorted key path
feeds the class hash. Those rows are where the harness's last two class-name
divergences were recorded and then closed — three nested pseudo-classes ordered
differently, found by a guard written for something else, and a key carrying an
accented letter, which took a collation dependency. Both now read `identical`,
which is what makes a regression in either a changed verdict rather than
silence.

Other entries carry a `note` without an expectation, saying why a subject that
reads `identical` still earns one -- the shorthand rejection table having once
diverged, for instance.

The largest block of them is one row per native method the evaluator folds. The
gap they were written for was measured at thirty-five methods that the reference
compiler folded and this compiler refused for no reason but that the receiver
was a binding, plus three static surfaces — `Math`, `Object` and
`Number.prototype` — refused because their receivers are identifiers. Each row
asks one method on a named receiver beside the same call on a receiver written
out, and the two halves answer **different** declarations deliberately: written
to answer the same one, the half that had never been broken would emit the rule
on its own and the row would go on reading `identical` after the half under test
stopped folding. `Number.prototype` is the exception with no written-out half,
because a method call on a number written into the source is refused by both
compilers and that half would measure the refusal instead.

A curated row per method is not a claim to have covered the surface — the method
nobody listed is the next bug report, which is what **The generated surface**
below is for. What a row here is for is the place a **reason** gets written
down: `filter` records the one shape that was wrong rather than refused, `map`
the chain that died at its second link, `substr` that nothing enumerates method
names any more, `repeat` which ceiling bounds its result.

### Regenerating the harvest

```sh
pnpm run --filter=@stylexswc/rs-compiler parity:harvest
# --check reports staleness and writes nothing:
pnpm run --filter=@stylexswc/rs-compiler parity:harvest -- --check
```

`harvest-corpus.ts` scans **every** `.rs` file under `crates/` — `src/` and
`benches/` included — for seven literal shapes that carry a CSS declaration. Each entry records the
`<path>:<line>` it came from, so an unexpected one can be traced back to the
test that motivated it. Run this after adding tests that carry CSS values.

The scan reads the crate names off the tree rather than from a list. It used to
name `stylex-css` and `stylex-transform`, and when the transform crate was split
apart and 39 test files moved into new crates, nothing widened the list and
nothing failed — the values under those crates simply stopped reaching the
corpus. Two kinds of file are still skipped, both generated: `__swc_snapshots__`
directories, and any source with `@generated` in its header comment. The second
is what keeps the chain below from closing into a loop, since `cases.rs` spells
its inputs as CSS rules and would otherwise harvest the corpus back into itself.

| Shape | Written as                                                      | What is taken                                                  |
| ----- | --------------------------------------------------------------- | -------------------------------------------------------------- |
| 1     | `normalize_css_property_value("color", "red", &opts)`           | both literals                                                  |
| 2     | a case table looped through one property                        | the first element of each row, or each element of a flat array |
| 3     | `"* {{ transitionProperty: opacity; }}"`                        | the declaration inside the rule                                |
| 4     | `"*{color:red}"`                                                | the same, minified                                             |
| 5     | a `stylex.create` object in a transform fixture                 | every declaration in it, except in a `stylex.env` argument     |
| 6     | `unchanged("color", "red")`, `same("color", "#ff0000", "#f00")` | the property and the **input** only                            |
| 6a    | `refuses_with("color", "red {", MESSAGE, OTHER)`                | the same, for a value expected to be refused                   |
| 7     | `rejects("width", &["*(", "/.5 *("], MESSAGE, &opts)`           | the property and every value in the slice                      |

**What the gate cannot see.** Every shape above takes a `property`/`value`
declaration, which is the _value_ a test carries and never the _capability_ it
was written for. A suite proving that a method call folds through a named
receiver contributes its `color: 'red'` and nothing that records the fold, and
one proving that a refusal names its rule contributes no value at all. So a run
reporting no change after such a suite lands is the gate answering truthfully,
not the corpus confirming the suite is covered. Those capabilities are pinned in
the curated corpus in `modules.json` instead, which is written by hand for
exactly that reason.

Shapes 6 and 7 are the two worth knowing about, because what they _omit_ is
deliberate. A verdict case row carries the expected output and the reference
compiler's spelling after the input; a rejection table carries the diagnostic
the value is expected to fail with. Neither is harvested. Deriving an
expectation from the reference compiler is the whole point of the harness, so an
expectation already written down must never become an input to it — that would
have the corpus confirming what the tests already assert.

Two guards keep the scan honest, and both matter because a CSS value is
arbitrary text that can spell anything:

- Call sites are found on the **masked** source, in which every string literal
  is blanked to spaces. A fixture value that spells `unchanged("color", "x")` is
  data, not a call.
- The first two arguments of a shape 1 or 6 call must be **adjacent** — nothing
  but whitespace before the first literal, nothing but a comma between them. A
  call whose value argument is an identifier would otherwise pair its property
  with whatever literal came next, which is usually the expected output.
- The object handed **directly** to a `stylex.env` function is not read. In
  `select({ primary: 'red' }, 'primary')` the key names a branch for the
  environment function to choose between; in `colors({ color: 'yellow' })` it
  names a property. What a key means there is decided by the function the test
  supplies, so the source cannot tell the two apart and the whole object is
  left alone. A branch body sits one brace deeper and is an ordinary style
  object, so its declarations are still harvested.

### The generation chain

Adding a Rust test that carries a CSS value invalidates a checked-in fixture in
a _different_ crate. The order is:

```text
Rust test sources
  -> pnpm --filter=@stylexswc/rs-compiler parity:harvest
       -> parity/corpus/harvested.json
            -> pnpm --filter=@stylexswc/postcss-value-parser generate:value-parser-cases
                 -> crates/postcss-value-parser/src/tests/cases.rs
```

`cases.rs` row order _is_ the corpus order, so anything that reorders the corpus
rewrites it wholesale. Both steps have a `:check` form that reports staleness
without writing, and both now run as part of their package's `test` script —
this package's runs `parity:harvest:check` before `vitest`, the way
`postcss-value-parser` already runs its own.

The first link went unguarded long enough for the corpus to fall 41 declarations
behind the suites, including every float-precision value the effort's own tests
were written to pin — which is the one place a stale corpus costs the most,
since the corpus was then not measuring parity on the values someone had just
gone to the trouble of pinning. The check needs no `dist/` and no compiler run:
it scans Rust sources, so it is cheap enough to sit in front of a test script
and there is no reason for it to be somewhere a contributor has to remember.

The scan is a heuristic over Rust sources, so the corpus contains some values
that are not valid CSS — degenerate inputs to the whitespace-repair unit tests,
keys that are pseudo-selectors rather than properties. They are kept: they cost
two compiler runs each, and filtering them by guesswork risks dropping a real
divergence. They land in the `structurally divergent`, `acceptance divergent`,
`both reject` and `both reject (diverged)` buckets, which is why those are
counted apart from `divergent`.

## The generated corpus: `pnpm fuzz:shorthand`

The corpus above is curated — every entry came from a test file or from someone
writing it down. `fuzz-shorthand-split.ts` asks the same question over a
generated one, and it exists because one defect class is not reachable by
curation: a shorthand value is cut into parts, and where the cut falls depends
on which separator, which spacing and which token shape happen to sit next to
each other. Hand-picked probes twice reported a small remainder that a generated
alphabet then multiplied.

It differs from `pnpm parity` in three ways worth knowing before reading a
number from it.

- **It pins the style resolution** rather than taking it as a flag. Value
  splitting runs only under `legacy-expand-shorthands`; under the other two a
  `padding` reaches the stylesheet whole. A run left on the default compares two
  compilers that both never called the code, and reports agreement.
- **It reports its alphabet, not a score.** What a run can claim is the token
  classes it crossed. The class list, the joiner list and the property list are
  printed above the counts precisely so that a reader compares the two rather
  than quoting the count alone.
- **It has no expectations of its own.** There is no corpus file to write an
  `expected` verdict on, so a permanent divergence is pinned by the same
  **refusal families** the curated report uses — by family and never by row,
  since hundreds of generated values reach each refusal and growing the alphabet
  would otherwise churn a fixture nobody can read.

```sh
pnpm fuzz:shorthand                                     # summary
pnpm fuzz:shorthand --show 40                            # print unexpected rows
pnpm fuzz:shorthand --property padding                   # one property
pnpm fuzz:shorthand --json parity/results/<name>.json    # full report
```

A row it reports is not yet a defect: the alphabet deliberately includes values
that are not valid CSS. What the report separates is pinned rows from unexpected
ones, and the unexpected count is the number to read — as it stands every
divergent row belongs to a family, so that count is zero and any other value is
news. `--show` prints the unexpected rows; `--json` carries both, the pinned
ones grouped by family as the evidence that the count above them is what it
says.

Adding a token class is judged on whether it can produce a **part shape** no
existing class produces, never on the rows it adds: a class crossed with the
rest of the alphabet costs roughly 900 subjects per property. The candidates
audited and dismissed are recorded in the `FRAGMENTS` documentation, because the
argument is the useful half of an audit.

## The generated surface: `pnpm fuzz:prototypes`

The corpus above carries one row per method someone measured, and the fold
evaluates a call rather than matching its name against a table — so the corpus
cannot be the evidence that the surface is covered. A curated list of methods is
itself a table, and the argument for deleting the tables was that the method
nobody listed is the next bug report.

This sweep reads the surface off the language instead:
`Object.getOwnPropertyNames` over `String.prototype`, `Array.prototype`,
`Object.prototype`, `Number.prototype`, `Boolean.prototype` and the `Math`,
`Object`, `Number`, `String` and `Array` namespaces — some 180 methods as it
stands, none of them written down anywhere here. Which surfaces belong is not a
choice either: the namespaces are exactly the compiler's `VALID_CALLEES` —
read out of the Rust source and asserted, so a sixth callee added there and not
here fails rather than going unswept — and the prototypes are the ones a value
crossing the fold's bridge can have. Each is
asked in **both shapes**, which is the question this whole effort turned on: a
prototype method on a receiver written out and on the same receiver held by a
name, and a namespace method on arguments written out and on the same arguments
held by names. That is ~330 subjects, two compiler runs each, and it compares
class names, rule text and style-object shape exactly as the curated harness
does.

```sh
pnpm run --filter=@stylexswc/rs-compiler build     # the harness reads dist/
pnpm fuzz:prototypes                                    # summary
pnpm fuzz:prototypes --show 60                          # unexpected rows
pnpm fuzz:prototypes --surface Math                     # one surface
pnpm fuzz:prototypes --json parity/results/<name>.json  # full report
```

**What it does not ask.** One variable moves per row, so a prototype method's
_arguments_ are written out in both of its shapes — the receiver is the
question, and a row varying two things at once could not say which one refused
it. So a callback, which is an argument, reaches the named shape only through a
namespace method, where the shape moves to the arguments. `Object.groupBy` is
the one such row today and it is how the gap in issue 19 was found; on the array
methods where callbacks actually live, this sweep asks only the written-out
form.

**Arguments are measured, not tabulated.** A generated subject needs an argument
list, and a table of which method takes what would put the curation back one
level down. So a small pool of argument texts is crossed up to arity two and the
first vector real JavaScript accepts — evaluated in the harness's own process,
from the same text the compilers are handed — is the one that runs. The declared
arity is tried first, then narrower, then wider, because a call made with fewer
arguments than it wants usually still answers something and a subject like that
measures the fold on a call nobody writes.

Three exclusions fall out of that measurement rather than being listed as names,
which matters because a name in a list is a claim nobody re-checks:

- A method whose answer **differs between two evaluations** cannot be compared
  across two compilers at all. `Math.random` is the whole of that category
  today, and it is also in the compiler's own refused set — the sweep reaches it
  from the other end.
- A method answering a value **no declaration carries** — `undefined`, a
  function, an iterator, `NaN`, the empty string — would be asked as a subject
  both compilers drop, which the harness reads as agreement about nothing.
- A method **no argument vector satisfies** at all, which is where the three
  three-argument statics land.

All three are printed under **Methods not exercised**, with what each one
answered. That block is the honest half of the coverage claim: a sweep that
quietly dropped a third of a prototype would otherwise report the same clean
number as one that crossed all of it.

**A terminal, where an answer has no declaration form.** A method answering an
array, a plain object or a boolean gets one further call — `join`,
`Object.keys`, `String` — so the row measures the method _and_ the terminal.
That is accepted deliberately, and it is the trade the curated rows already make
(`Object.keys(config).join(', ')` is how the curated static row is written): the
alternative is asking a predicate or a key list with nowhere to put the answer.
The three terminals are names the fold has to answer anyway, so a broken
terminal breaks its own row rather than hiding behind these.

**Where a reason lives.** A divergence this sweep expects is _accounted for_ by
naming the curated corpus row that argues it — `lib/prototype-accounts.ts` holds
one entry per reason, each with the complaint this compiler writes, the verdicts
its rows read, and the id of the row that carries the argument. The reason
itself is never copied here, because two statements of one reason come to
disagree; what is checked instead is the link. A row that has been deleted, has
lost its `note`, or has stopped recording a verdict the account claims fails the
run, under **Accounts whose reason is no longer recorded** — the same failure
the curated harness's unreached-family check catches, read from the other end.

An account claims by reason and never by method name, exactly as a refusal
family does, so adding a surface costs nothing and a reworded diagnostic un-pins
its rows rather than quietly outliving them. The sweep reads the refusal
families first, since a folded value can trip one of them without the fold being
what diverged. These are not themselves families for a mechanical reason: a
family may not claim a row carrying its own `expected` verdict, every curated
row for these refusals carries one, and a family added for them would go
unreached across the curated corpus and fail `pnpm parity`.

Every divergent row is accounted for as it stands, across six reasons: the
locale-sensitive methods, a read that escapes onto the language's function graph
(`constructor`), a receiver written as a number, a mutation that disqualifies
the binding, a static that answers by changing what it was handed, and one gap —
a callback reached through a name, which upstream folds and this compiler
refuses. The last is the row this sweep was built to find: nothing curated had
named it, because the only place the sweep names a _function_ is a namespace
method's argument. No count is written here, for the reason the refusal-family
table gives: a number in this file has gone stale twice.

**Three ways it exits non-zero**, and each is a report that has stopped being
read: a divergence nothing accounts for, an account whose corpus row no longer
carries its reason, and a surface that exercised fewer methods than it is on
record for.

The last is the failure mode a generated harness is most prone to, and the one
no other gate reaches: a method that stops answering leaves no row to disagree
about, so a change to the argument pool or to `renderingFor` that silenced half
a prototype would print a smaller coverage number beside a green run. So each
surface carries a **floor** — the count it was last seen to reach — and the run
fails below it. The floors are per surface rather than over the total, because a
total hides exactly the case worth catching: one prototype falling silent while
the rest carry the sum. The summary prints their sum beside the count on every
run, so a reader sees the gate without waiting for it to fail, and a failing run
names each surface with both of its numbers. Raising a floor is ordinary;
lowering one is the claim that needs an argument in the commit that makes it.

A failing row names the surface, the method, the receiver shape, what JavaScript
itself answers for the expression, and what both compilers answered — so the
first thing a reader has to decide, whether the fold is wrong or the reference
compiler moved, is on the screen rather than in a re-run.

## Checking a future upstream release

1. Bump `@stylexjs/babel-plugin` in this package's `devDependencies` and
   `pnpm install`.
2. Rebuild the compiler, then run
   `pnpm parity --json parity/results/<version>.json`.
3. The printed subject block states both versions, so a report is always
   attributable to a specific release.
4. Diff the new report's `divergent` set against the previous one. Anything new
   is an upstream normalization change this compiler has not adopted.

## Using a report to write test expectations

Never write or update an expectation by eye — a hand-edited expectation in this
area is just the bug re-encoded. Take it from `--json` output instead. Each
report entry carries, for both compilers:

- `classNames` — the generated class names, in emission order
- `rules` — the full LTR rule text
- `declarations` — the `property:value` text inside the braces, which is what
  value normalization produces and therefore what the value-normalization tests
  assert against
- `styleObjects` — the shape of each `$$css`-marked style object, which is where
  an absent value shows and the rule text does not

`entries[].babel.declarations` is the expectation; `entries[].rust` is what this
compiler produces today.

**A verdict with no declarations is not a verdict.** Both compilers expand or
drop shorthands such as `background` and `border`, so a subject written on one
produces an empty `declarations` array on both sides and the entry is reported
`identical` — agreement about nothing. Check that `entries[].rust.declarations`
and `entries[].babel.declarations` are non-empty before reading an `identical`
as evidence, and pick a longhand (`backgroundImage`, `boxShadow`) when writing
the subject. The exception is a subject whose whole point is a `null` value:
there `declarations` is empty on both sides by design, and `styleObjects` is
what the verdict rests on.

**One caveat when the expectation is for a value-normalization test.** The
harness runs the whole transform, and a few properties never reach value
normalization at all: `content`, `hyphenateCharacter` and `hyphenate-character`
are returned verbatim when their value already carries matching quotes, names a
content function, or is a content keyword. A verdict on one of those describes
the transform path, not the normalization seam, and the two disagree — `content:
"\2014 A"` survives the transform unchanged but has its escape resolved at the
seam. Pick a property that reaches normalization (`fontFamily` covers the same
string shapes) when sourcing an expectation for a normalization test.

[adr]: ../../stylex-css/docs/adr/0001-root-collation-orders-a-non-ascii-key.md
