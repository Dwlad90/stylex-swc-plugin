# 28 — The stages around the fold still abort on a deep expression

Status: `needs-triage`
Blocked by: None

**What was measured.** 20 gave the evaluator a ceiling and a stack of its own, so
the fold no longer aborts: past `MAX_EVALUATION_DEPTH` it refuses with a message
naming the file and the key path. The abort is not gone from the compiler,
though. It moved to the stages that recurse over the same expression and answer
to no ceiling — parsing, the visitor, printing, and the deep `Expr` clone a
refusal records on the evaluation state.

Debug build, 2 MiB thread — the tightest configuration, and the one the test
binaries use. Depth is levels of `(x + 1)`:

| input | 512 | 576 | 608 | 768 | 1024 |
| --- | --- | --- | --- | --- | --- |
| deep expression, no `stylex` call | ok | ok | ok | ok | **aborts** |
| deep expression inside `stylex.create()` | refuses | refuses | **aborts** | aborts | aborts |

Two readings of the same limit. The first row is the pipeline alone: parse, visit
and print a 1024-level expression and the thread's stack runs out with the fold
never involved. The second is the pipeline plus what a refusal costs — the fold
stops at its ceiling, but `deopt` clones the expression it stopped on, the code
frame formats it, and the clone's drop glue recurses over it — so the abort
arrives ~400 levels earlier.

Upstream folds every depth in this table and throws
`RangeError: Maximum call stack size exceeded` at 4096, so the whole table is a
divergence. It is a *lower ceiling* divergence rather than the one 20 fixed: a
refusal at 576 is a diagnostic, an abort at 608 is not.

**Why it is not part of 20.** 20's ceiling deliberately sits well under this, so
the fold is not the reason a build reaches a stage that cannot report. Raising it
means fixing these stages, and each has a different owner and a different remedy:
the parser is SWC's and already grows its stack, the visitor and printer are the
transform's, and the deep clone in `deopt` is arguably avoidable rather than
growable — a refusal does not obviously need to own a copy of the subtree it
refused.

**Worth checking first** whether the recursive drop is the cheapest of the four
to remove, since it is the one that costs ~400 levels of headroom on an input
that was otherwise going to be reported cleanly.

- [ ] Attribute the abort in the `create()` row to a stage — the clone, its
      drop, the code frame, or the printer
- [ ] Decide whether a refusal needs to own the expression it refused
- [ ] Re-measure both rows in a release build, since the frames are smaller and
      the numbers in this table are the floor rather than the limit
