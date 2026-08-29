# 15 — The generated prototype sweep

**What to build:** Coverage of the prototype surface that is proved rather
than curated.

This whole effort rests on one claim: that deleting the method tables removes
a class of bug rather than moving it. A curated corpus cannot prove that,
because a curated corpus is itself a table — the method nobody listed is the
next bug report, which is the argument for deleting the tables in the first
place.

A generated harness crossing the reachable prototypes with both receiver
shapes against both compilers is what closes it. Prior art is the existing
generated fuzz harness, which crosses an alphabet with itself and runs nightly
rather than per commit for exactly this reason: a surface defect appears when
the surface changes, not on every push. A curated row is where a *reason* gets
written down; the sweep is what finds the method nobody thought of.

**Blocked by:** 14.

**Status:** resolved

- [x] A generated harness crosses the reachable prototypes and statics with
      both receiver shapes and compares both compilers on class name, rule
      text and style-object shape
- [x] It prints the reference compiler version it resolved before anything
      else, as the existing harnesses do, since that version moves under a
      dependency update without anything here changing
- [x] A divergence the corpus already records with a reason is expected rather
      than reported as new
- [x] It runs on the nightly schedule beside the existing sweep and is listed
      in the gate, so a failing run fails rather than sitting green beside it
- [x] A failing run names the method, the receiver shape and both outputs

## Answer

`pnpm fuzz:prototypes` — `parity/fuzz-prototype-sweep.ts`, with the generation
half in `parity/lib/prototype-surface.ts` and the accounting half in
`parity/lib/prototype-accounts.ts`, both under unit test. 183 methods read off
the language with `Object.getOwnPropertyNames`, 166 of them exercised, 332
subjects, ~2s. Nothing in it enumerates a method name. Which surfaces belong is
not a choice either: the namespaces are exactly `VALID_CALLEES`, and the
prototypes are the ones a value crossing the bridge can have — which is why
`Boolean.prototype` is among them and `Date` is not.

**Arguments are measured rather than tabulated**, which is what keeps the
curation from moving one level down: a pool of thirteen argument texts is crossed
up to arity two, and the first vector real JavaScript accepts — evaluated in the
harness's own process, from the same text the compilers are handed — is the one
that runs. Three exclusions then fall out of the measurement instead of being
listed as names: an answer that differs between two evaluations (`Math.random`,
which is the compiler's own refused set reached from the other side), an answer
no declaration carries, and a method no vector satisfies. All three print under
**Methods not exercised**, with what each one answered, because a sweep that
quietly dropped a third of a prototype would report the same clean number as one
that crossed all of it.

**One variable moves per row**, so a prototype method's arguments are written
out in both of its shapes — a row varying receiver *and* argument could not say
which refused it. The cost is that a callback reaches the named shape only
through a namespace method, which is stated in the source and the README rather
than left for a reader to infer.

**A reason is never copied.** An account names the curated corpus row that argues
its divergence, and the run checks the link: a row deleted, a row that lost its
note, or a row whose recorded verdict is no longer one the account claims fails
the run. That is the unreached-family check read from the other end, and it is
why the accounts are not entries in `REFUSAL_FAMILIES` — a family may not claim a
row carrying its own `expected` verdict, every curated row here carries one, and a
family added for them would go unreached across the curated corpus and fail
`pnpm parity`.

**What it found.** 45 of the 332 rows diverge, over six reasons. Five were
already deliberate, and two of those had no corpus row until now:
`modules-15-a-read-that-escapes-onto-the-function-graph` (`constructor` is an own
property of every prototype crossed, and nothing curated had named it) and
`modules-15-an-impure-static` (`Object.freeze` and the rest of `INVALID_METHODS`,
which both compilers refuse).

The sixth is a **gap**, and it is the one the sweep was built to find: a callback
reached through a name refuses here and folds upstream. It arrives through
`Object.groupBy` on named arguments — the only place the sweep names a function,
since a prototype subject names only its receiver — and it generalises to every
callback-taking method. Filed as issue 19 and recorded as
`modules-15-a-callback-reached-through-a-name`, expected rather than deliberate,
so the day it closes is a changed verdict rather than silence.
