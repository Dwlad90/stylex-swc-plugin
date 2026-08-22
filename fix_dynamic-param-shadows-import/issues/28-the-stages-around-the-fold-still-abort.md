# 28 — The stages around the fold still abort on a deep expression

Status: `resolved`
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

- [x] Attribute the abort in the `create()` row to a stage — the clone, its
      drop, the code frame, or the printer
- [x] Decide whether a refusal needs to own the expression it refused
- [x] Re-measure both rows in a release build, since the frames are smaller and
      the numbers in this table are the floor rather than the limit

## Answer

Attributed, decided, and re-measured. No code changed: the stage that aborts is
SWC's parser in release and our printer in debug, and the clone the ticket
suspected turned out to cost nothing.

### How it was measured

A throwaway `depth_probe` example in `stylex-transform`, run once per depth as
its own process, because a stack overflow is an abort rather than a catchable
panic. It spawns a 2 MiB thread, writes the module to a real file so the
code-frame path reads source from disk exactly as a build does, then parses,
applies the StyleX pass and prints -- with `eprintln!` markers between the
stages, which reach the terminal before the abort because stderr is unbuffered.
Deleted after the measurement; it is fifty lines and reproducible from this
description.

The debug table reproduced exactly as filed: `bare` answers at 768 and aborts
at 1024, `create` refuses at 576 and aborts at 608.

### The stage, in debug

Not the clone, and not any of the other three candidates. The markers place the
abort inside `get_source_code`, at
`print_module(code_frame, module.clone(), ...)` -- **the printer**, reached
because `state.get_seen_module_source_code()` answers `Some((module, None))`, a
memoized module with no source text beside it, so the code frame prints the
module rather than reading the file. The clone in front of it *completes*: a
marker between the clone and the print prints, and the abort follows.

The order the four candidates fell in:

| stage | reached | overflows |
| --- | --- | --- |
| `deopt`'s `path.clone()` and its drop | yes | no |
| `Expr::Call(call.clone())` at the assert, and its drop | yes | no |
| `compute_cache_key`, the derived `Hash` over the subtree | yes | no |
| `module.clone()` in `get_source_code` | yes | no |
| `print_module` over that clone | yes | **yes** |
| `find_expression_span`, `emit_error` | never reached at 608 | — |

### Whether a refusal needs to own the expression it refused

It does not need to *for headroom*, which is the only reason this ticket asked.
`deopt` was changed to record `None` instead of `path.clone()` and the whole
table was re-measured: **576 still refuses and 608 still aborts, unmoved.** The
recursive drop the ticket flagged as "the one that costs ~400 levels" costs
none of them. So the ~400-level gap between the two rows is not the refusal
paying for a copy of what it refused; it is the code frame printing a module.

The clone stays. It is what every diagnostic call site points at -- the fallback
`unwrap_or_else(|| *first_arg.to_owned())` is a whole `create()` argument where
the deopt path is the operand that actually failed -- and removing it buys
nothing this ticket was opened to buy.

### Release, and the surprise in it

Re-measured in release, same 2 MiB thread, both rows:

| input | 512 | 576 | 640 | 704 | 768 | 1024+ |
| --- | --- | --- | --- | --- | --- | --- |
| deep expression, no `stylex` call | ok | ok | ok | ok | **aborts** | aborts |
| deep expression inside `stylex.create()` | refuses | refuses | refuses | refuses | **aborts** | aborts |

**The two rows converge, and the stage is the parser in both** -- the abort
lands before the `parsed` marker. The gap the ticket describes, where a refusal
at 576 is a diagnostic and an abort at 608 is not, **does not exist in a release
build**: everything the transform does after parsing fits inside what the parser
already needed.

Release is not uniformly roomier, which is the part worth recording against the
ticket's expectation that "the frames are smaller and the numbers in this table
are the floor". The `create` row gains 128 levels (576 to 704) and the `bare`
row **loses** 256 (1024 to 768). Inlining does not only shrink frames; in a
recursive descent parser it merges callee locals into the caller, and SWC's
expression parser is where that lands.

### What is left, and who owns it

One stage, and it is not this crate's: **SWC's parser is the floor at 768 levels
in release.** Raising it means growing the stack around the *parse*, and the
parse happens in the host -- `stylex-rs-compiler`'s napi entry, the SWC plugin,
each test harness -- rather than anywhere `stylex-transform` can reach. That is
a change to every call site for an input no project writes, against a ceiling
`maxEvaluationDepth` already refuses at 32, so it is named here rather than
filed: a build that reaches 768 levels of nesting has a generator loose in it.

Our printer stays unbounded and stays second in line. It is only the binding
constraint in a debug build, where it costs the `create` row 160 levels against
the parser's own limit, and it is reached only when a diagnostic has to print a
memoized module because no source text was stored beside it.
