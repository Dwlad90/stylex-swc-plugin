# 64 — Cover the transform's call handlers

**What to build:** The tests that exercise the 198 regions under
`crates/stylex-transform/src/transform/stylex/` that no test in the workspace
reaches — one handler per `stylex.*` call, plus the shared visitor helpers
beside them.

**Where the gap is:**

| File                                                      | Uncovered regions | Regions        | Lines  |
| --------------------------------------------------------- | ----------------- | -------------- | ------ |
| `transform_stylex_define_vars_call/helpers.rs`            | 30                | 82.76%         | 84.27% |
| `transform_stylex_create_call/dynamic_style_functions.rs` | 19                | 90.40%         | 89.59% |
| `transform_stylex_create_theme_nested_call.rs`            | 19                | 81.90%         | 86.73% |
| `transform_stylex_view_transition_class_call.rs`          | 15                | 81.48%         | 81.32% |
| `transform_stylex_create_call/mod.rs`                     | 14                | 91.30%         | 92.21% |
| `transform_stylex_create_theme_call.rs`                   | 14                | 87.39%         | 89.09% |
| `visitor_utils.rs`                                        | 13                | 86.32%         | 86.99% |
| `transform_stylex_create_call/helpers.rs`                 | 11                | 91.34%         | 94.40% |
| `transform_stylex_define_vars_call/mod.rs`                | 11                | 90.76%         | 90.24% |
| `transform_stylex_atoms.rs`                               | 10                | 88.24%         | 88.37% |
| `transform_stylex_keyframes_call.rs`                      | 10                | 84.62%         | 83.82% |
| `transform_stylex_position_try_call.rs`                   | 7                 | 89.86%         | 90.54% |
| the remaining six files                                   | 25                | 88.00%--95.89% |        |

**Each handler is one API, so each gap is one question** — which authored call
the handler declines, and with which diagnostic. The uncovered arms are mostly
the refusals: a producer given the wrong argument count, a nested call whose
first argument is not an object, a theme whose base is not a `defineVars`
result. Every one of those shapes is reachable from a source file, so the
existing integration harness under `tests/` writes them with no new
scaffolding, and a `validation_*_test` file already exists for most of the
handlers.

`visitor_utils.rs` is the exception: it is the helper the handlers share, and
its gaps are best reached through whichever handler exercises the branch, not
through a case built for the helper alone.

**Blocked by:** [62](./62-record-the-transform-coverage-baseline.md) — the
baseline and the repeatable suite.

**Status:** in-review

- [~] `scripts/coverage-missing.sh -p stylex_transform` reports 33 unexercised
  regions under `src/transform/stylex/`, from 191. What was left after this
  batch is listed below with the reason each one stays; the two largest groups
  were closed by [71](./71-close-the-guards-over-a-compiler-built-object.md),
  which this ticket does not open, and
  [69](./69-inject-the-rules-an-array-bound-create-declares.md) and
  [70](./70-read-first-that-works-inside-a-keyframes-step.md) closed two more
  of the rows. `create_call/helpers.rs` holds 7 and
  `create_call/mod.rs` 3, which is what remains of this ticket.
- [x] A refusal arm is covered by a case that asserts the diagnostic the arm
      gives. The `stylex.when` report is asked as a unit test rather than a
      compiled module, because a report's words are built only where a logger
      admits the level.
- [x] Tests cover regular and irregular inputs, and the edge cases each handler
      states.
- [x] The full workspace suite stays green. 3162 tests, from 3119.

## Comments

### What the batch closed

`scripts/coverage-missing.sh -p stylex_transform`, on the tip of this branch:

| Figure                                        | Baseline | Now    |
| --------------------------------------------- | -------- | ------ |
| Regions, whole crate                          | 93.29%   | 94.94% |
| Functions                                     | 93.98%   | 97.09% |
| Lines                                         | 93.69%   | 95.15% |
| Regions unexercised under `transform/stylex/` | 191      | 75     |
| Tests                                         | 3119     | 3162   |

No region under `src/transform/stylex/` counts against the gate, at the
baseline or now: every gap here is unexercised rather than
per-instantiation.

### Two defects the handlers were hiding

**A view transition class read by its imported name was never compiled.** A
call reaches a producer handler only when the producer cycle's import-kind list
names the API, and `ViewTransitionClass` was not on it. So
`viewTransitionClass({...})` written against a named import came out of the
compiler exactly as the author wrote it and the CSS it declares was never
injected. The namespace spelling worked all along, which is why no case caught
it -- a namespace import answers for every API at once. The reference
implementation compiles both spellings, and the recorded snapshot for the named
one held the untransformed module.

The cases for it compare the two spellings of one module rather than record what
the named one printed. A snapshot of the named spelling alone says nothing: a
call the compiler never reached prints as an ordinary module.

**`stylex.create` was the one producer that did not refuse a spread.** It read
`args.first()?.expr` and never looked at `spread`, so
`create(...{ root: { color: 'red' } })` compiled and injected a rule. The nine
producers beside it refuse it, and so does the reference implementation.

### Four questions, asked once each

The gaps were mostly the same question written out per handler, and each copy
left the other's answer unmeasured.

- **The argument, and the spread in it.** Ten handlers read the argument at an
  index, refused a spread there, and answered `None` from the handler when the
  list was too short. That last answer tells the dispatcher "not my call", so a
  producer given no argument fell through every other handler and left the call
  untransformed -- for a state the count check rules out. `argument_at` reads it
  once, and `or_refuse_missing_argument` beside it is the step left out of the
  measurement.
- **The position a refusal is reported at.** Twenty sites spelled
  `deopt.unwrap_or_else(|| *argument.to_owned())`. The two answers are split
  across the producers -- `evaluate` always records the position it refused on,
  and the create argument reader has one refusal that records none -- so no site
  met both. `refusal_site` reads it once, where the create path covers the
  fallback and the nine others cover the position.
- **The object the argument folded to.** Seven handlers wrote out the same three
  answers: the fold refused, it answered a non-object, or it answered nothing.
  The copies had drifted on how the third reads -- four reported it with no code
  frame -- and on the prefix, because four asked through `assert!`, which panics
  with the string it formats rather than with this compiler's own error.
  `folded_style_object` reads all three once; 289 lines went, 137 came.
- **An arrow function value's body.** Two readers of a `defineVars` object each
  asked whether the body was an expression and each wrote its own refusal for an
  answer neither can get, measured against every spelling that could carry one.
  `arrow_body_expr` answers it once.

### What is left, and why

75 regions over sixteen files.

| File                                     | Regions | What they are                                                                                                                                                         |
| ---------------------------------------- | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `create_call/dynamic_style_functions.rs` | 19      | guards over the compiled object the compiler itself built -- a computed key, a prop that is not a key-value, a prop that is not a prop                                |
| `define_vars_call/helpers.rs`            | 15      | the same shape over the evaluated object, plus the cycle walk's two unreachable fall-throughs and a private name as a key                                             |
| `create_call/helpers.rs`                 | 8       | the legacy shorthand expansion of a dynamic style: three `?` paths and the third arm of the path rewrite                                                              |
| `create_call/mod.rs`                     | 8       | the `stylex.when` non-string refusal, the empty-answer refusal, and the array-bound registration -- see [69](./69-inject-the-rules-an-array-bound-create-declares.md) |
| `transform_stylex_atoms.rs`              | 5       | the bail-out, the namespace read and the flat-value fall-through, over an object built from two strings                                                               |
| `create_call/runtime_function_map.rs`    | 3       | the default marker having no values, and a namespace entry that was just created                                                                                      |
| `transform_define_marker_call.rs`        | 3       | three `?` paths the validator beside them rules out                                                                                                                   |
| `transform_stylex_keyframes_call.rs`     | 3       | the `firstThatWorks` registration nothing reads -- see [70](./70-read-first-that-works-inside-a-keyframes-step.md)                                                    |
| `create_theme_nested_call.rs`            | 3       | two empty-answer refusals and a theme reference whose group hash is not a variable                                                                                    |
| the remaining six files                  | 8       | one empty-answer refusal each, and the non-map identifier in `visitor_utils.rs`                                                                                       |

Two groups of that are one question again -- a guard over an object the
compiler built a line earlier -- and closing them means giving those objects a
type that cannot hold the refused shape, inside a 335-line function nested nine
levels deep. That is worth a ticket of its own rather than a pass here.

The empty-answer refusals that remain are the ones the shared reader does not
cover: `create`, which reads its argument through a different folder, and the
two theme handlers, whose two arguments read different sentences.

### Three findings filed rather than fixed

- [69](./69-inject-the-rules-an-array-bound-create-declares.md) -- a `create`
  bound inside a top-level array compiles its class names and injects none of
  its rules, leaving half a runtime-injection prelude behind.
- [70](./70-read-first-that-works-inside-a-keyframes-step.md) -- a
  `firstThatWorks` call read as a keyframes step value drops the declaration,
  and the step survives empty.
- The `"Function type"` refusal beside the array-bound registration is not a
  sentence an author can act on. Recorded in 69, which is where the branch it
  sits in is settled.

### The review round after the batch

Five review passes ran over the commits above. What they found, and what came
of it.

**The import-kind list went.** The first pass called the producing cycle's list
a pure sync hazard rather than a check: it can only under-approximate, because
each handler asks its own predicate a line later, so a name let through is
refused there while a name held back never arrives. Adding
`ViewTransitionClass` to it fixed one instance and left the shape that produced
it. Both cycles now derive from `ImportKind::is_read_by_the_consuming_cycle`, a match
over every variant, so a kind added to the enum does not compile until somebody
says which cycle reads it -- measured, not argued: adding a variant gives
`E0004`. Two APIs are consumed and every other one is produced, and `Env` --
which belonged to neither list -- is inert on the producing side, because every
handler's own predicate refuses a name bound only to it.

`is_stylex_import_for_kinds` had no caller left after that and went with it.

**Two claims in new comments were wrong, and are corrected.** That the empty
answer is unreachable, which does not follow from `evaluate` clearing its value
on a refusal -- the memo can answer nothing confidently, so the comment now says
the state is unsettled rather than impossible. And that nothing new reaches a
reader when a refusal reports through the compiler's error rather than through
`assert!` -- the colour is new at stderr, which strips nothing, and the stack
trace is new at both boundaries when `log` admits `Info`.

**`refusal_site` was measured as uncovered where it now lives.** The workspace
coverage run excludes `stylex_transform`, and every production caller of that
function is in it, so moving the answer into `stylex_evaluator` moved a region
into a crate the run measures. It has two cases of its own now, and
`evaluate_result.rs` is back at 100%.

**Three smaller things.** A whole-struct unbox on the compiling path, replaced
by two field moves. A test helper that compiled one spelling twice, now reading
the agreed output the comparison already built. And a duplicate case for a
destructured marker, which said what an existing case said.

### The performance review of the call handlers

A performance pass ran over this ticket's commits, reading each of the four
de-duplications against the code it replaced. None of them traded a borrow for
a copy -- every one is allocation-neutral or better against `develop`. Two
costs the de-duplication gathered into one place are now removed there, and one
is left.

**A producer was handed a copy of its own argument.** `argument_at` answered
with a copy of the expression the author wrote, which for a `stylex.*` call is
the whole style tree. Eleven of the twelve producers only read it. Before this
ticket each of the twelve made that copy for itself, so the copy is as old as
the handlers; gathering the read into one function is what made it one line to
remove. `create` is the one producer that rewrites its argument, and it asks
for the copy where it needs it.

**A theme copied both folded arguments.** `createTheme` read each argument's
value by reference and copied it, where
`unstable_createThemeNested` beside it moves the same two values. Both files
changed `stylex_create_theme` from a mutable borrow to a shared one in this
batch, and only the nested one followed through. They agree now.

**`refusal_site` copies the refusal position, and that is left.** Taking the
position by reference means copying it where the old spelling consumed it. All
thirteen call sites are inside a panic message, so the copy is paid only by a
build that is about to stop, and it is the price of reading the position beside
a value already moved out of the same result.

### The standards and spec review

Two more passes ran over this ticket. Both defect fixes it claims are in the
tree and are covered the way it says. Three things came of the passes.

**A refusal reported without its frame.** Every refusal in the two theme
handlers draws a code frame except the nested one's empty answer, which gave
the sentence alone. An author who reached it read what was wrong and not where.
It draws a frame now.

**A comment stated an order the code does not keep.** It said the first
argument is refused before the second one is read. Both are read before that
point. The comment now states what the step does give -- this sentence rather
than the producer's own words for the same input.

**The empty-answer refusals of the two theme handlers are still their own.**
The shared reader answers `non_static_value` where a fold was confident but
gave no value; `createTheme` answers `non_style_object` there. Moving the
second argument onto the shared reader would settle that, and it would also
move the second argument's refusal ahead of the first argument's, because the
shared reader asks all three questions in one place. That is a change to which
sentence an author reads first, so it belongs with the reshape of these two
handlers rather than here.

One statement here was wrong and is corrected above: the method the two cycles
derive from is `is_read_by_the_consuming_cycle`, not `is_consumed_by_a_call`,
which is in no file.
