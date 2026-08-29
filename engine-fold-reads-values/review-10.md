# Code review — ticket 10

Reviewed: the working tree against `09f2d0186` (`git diff HEAD` plus one
untracked test file). Two axes, run as independent sub-agents so neither could
rerank the other: **Standards** (this repo's documented standards, plus the
Fowler smell baseline) and **Spec** (does it match what ticket 10 asked for).

Status: reviewed, findings triaged below. Every row marked **fixed** was applied
before the change was committed.

## Standards

One hard violation, three baseline smells, one gap.

| #   | Finding                                                                                                                                                            | Verdict                                                                                                                                             |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| S1  | **Hard** — `CLAUDE.md` "80 for markdown": one added `CONTEXT.md` line is 108 chars.                                                                                 | **fixed** — both added paragraphs rewrapped; every added line is now ≤80                                                                            |
| S2  | Mysterious Name: three senses of "bound" in one module — the nesting limit, `Scope::Bound`, and `struct Bound`.                                                     | **fixed** — `Scope::Bound` → `Scope::Names`, `struct Bound` → `Bindings`; the limit sense is the only one left                                       |
| S3  | Duplicated Code: the bind-then-walk shape appears verbatim in `admit_arrow` and `admit_block`, so the ordering invariant is relied on twice and stated in neither.  | **fixed** — extracted as `Bindings::enter`, which carries the invariant                                                                              |
| S4  | Mysterious Name: `a_callback_that_escapes_its_parameters_refuses` opened with two fold assertions, so its name stated the opposite of half its body.                | **fixed** — split, with the folds under `a_callback_body_and_parameters_are_the_language_s_to_read`                                                  |
| S5  | **Gap** — `Bindings::pattern` recursed through nested patterns without spending `Guard::descend`, so a deeply nested destructuring parameter reached the engine's parser having paid nothing from the nesting budget. | **fixed** — the walk descends like every other on this bridge; pinned by `a_pattern_nested_past_the_budget_refuses`. The best find of the review |

## Spec

| #   | Finding                                                                                                                                                                                     | Verdict                                                                                                                                                                                                                             |
| --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| P1  | Checkbox 5 half met: the dynamic-parameter half is delivered and tested, the StyleX-function half is not, and no closure path remains below the fold.                                        | **kept, split** — issue 17, with a written reason and a pinned refusal test. The divergence predates this ticket, and closing it means the arrow-to-Rust-closure path, not a re-added method table                                    |
| P2  | Testing Decisions: a divergence where the reference compiler compiles and this one refuses needs a parity-corpus row.                                                                        | **deferred** — issue 17's fourth box, and ticket 14 owns re-pinning the corpus. A row's `expected` verdict has to be measured against a built compiler, not written from the source, which is the work ticket 14 is scoped for       |
| P3  | The three `evaluating_function_expressions.rs` answers changed because an arrow is now admitted as a plain array element too — outside this ticket's subject.                                | **kept** — each replaces a silently wrong answer. `Object.keys` was `["0","2"]`, dropping the element the arrow sat in; the language says `["0","1","2"]`. `values`/`entries` now leave the call standing rather than dropping it     |
| P4  | Performance: `values`/`entries` now run a full engine evaluation before the outward bridge refuses, against "the guard refuses before it resolves".                                          | **kept** — that rule is about resolving bindings, which is the expensive step the guard defers. Whether a *result* holds a function is not answerable before evaluating it, and refusing an arrow element by syntax is the table shape this work deletes |
| P5  | `if`/`else`, nested blocks, empty statements and block scoping go beyond "a callback with a block body".                                                                                     | **kept** — a block body without a branch or a nested scope is half a block body, and each is one arm of the same walk                                                                                                                |
| P6  | **The reason given for excluding loops was false.** The module sets `MAX_LOOP_ITERATIONS` and documents it as the bound that holds whatever the guard admits, so "no bound here measured" contradicts it. | **fixed** — read the vendored engine: the count lives on the *call frame* (`frame.loop_iteration_count`), so a callback invoked once per element starts a fresh count and the bound is multiplied by an element count the source never states. That is the true reason, and it is the same arithmetic `admit_amplification` already refuses |
| P7  | The assignment exclusion was broader than its own rationale.                                                                                                                                | **fixed** — the rationale was wrong about where the rule lives. An assignment is an expression, so it is answered by the value walk, not by the statement set; the docs now say so and a test pins it                                 |
| P8  | **Story 27 violation** — a statement outside the set answered `NotACandidate`, so an author read `Unsupported expression: ArrowFunctionExpression`: the vague sentence the story forbids, and KISS's third state. | **fixed** — the exclusion is now a refusal naming the statement kind (`unfoldable_statement`), with `get_stmt_node_kind` added beside `get_expr_node_kind` so the word is spelled the way the ecosystem spells it                     |

## Not from either axis

The release-profile `fixtures` target fails 15 of 19. Measured at `09f2d0186`
with this change set aside: identical failure, so it is pre-existing and
unrelated. Recorded because the debug suite is green and the two disagree.
