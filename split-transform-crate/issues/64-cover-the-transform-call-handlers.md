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

**Status:** resolved

- [x] `scripts/coverage-missing.sh -p stylex_transform` reports no unexercised
  region under `src/transform/stylex/`, from 191. The last 24 are read in the
  final comment below. What was left after this
  batch is listed below with the reason each one stays; the two largest groups
  were closed by [71](./71-close-the-guards-over-a-compiler-built-object.md),
  which this ticket does not open, and
  [69](./69-inject-the-rules-an-array-bound-create-declares.md) and
  [70](./70-read-first-that-works-inside-a-keyframes-step.md) closed two more
  of the rows. What was left when that batch stopped -- 7 regions in
  `create_call/helpers.rs` and 3 in `create_call/mod.rs` -- is closed by the
  last batch, which the final comment reads.
- [x] A refusal arm is covered by a case that asserts the diagnostic the arm
      gives. The `stylex.when` report is asked as a unit test rather than a
      compiled module, because a report's words are built only where a logger
      admits the level.
- [x] Tests cover regular and irregular inputs, and the edge cases each handler
      states.
- [x] The full workspace suite stays green. `cargo nextest run -p
      stylex_transform --all-features` runs 3409 tests, from the 3119 it ran
      when this ticket opened.

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

### The count after the variable-reference key work

`scripts/coverage-missing.sh -p stylex_transform` on 2026-09-17, after
[73](./73-read-a-variable-reference-key-the-way-the-reference-does.md):
**28 regions over eight files**, from 36 over fourteen. The crate reads 99.54%
of regions, 100.00% of functions and 99.67% of lines over 3399 tests.

Five of the eight that closed were the same step written five times.

**Four `export_variable_not_found` guards went.** `defineVars`,
`defineVarsNested`, `defineConsts` and `defineConstsNested` each matched the
name their result is bound to and reported the same sentence for a name that is
not there. A top-level expression carries no name only where it is a default
export, and each of the four refuses a result that is not bound to a named
export first -- so the guard was total.

It was first written as one step carrying the exclusion the ranked list allows.
A review read that as the third answer where the second one was open, and it
was right: the export check itself proves the name, so
`is_variable_named_exported` now answers the name (`named_export_name`) rather
than a yes, and the four producers take it from the check. The guard, its
exclusion, and the sentence it reported are all deleted --
`export_variable_not_found` had no caller left, and it is a sentence with no
counterpart upstream.

Three things came with it. `find_and_validate_stylex_define_vars` and
`find_and_validate_stylex_define_consts` answer the name instead of the
top-level expression, so a compiling call no longer deep-copies the whole
variable group the author wrote. `validate_define_call` lost its
`require_export` flag to a second entry point, `validate_exported_define_call`,
and asks the state whether the call is bound rather than copying what it is
bound to. Every refusal is reported in the order it was before, and the
workspace coverage gate stays at 100.00%.

**The bare `stylex(...)` handler answered nothing three times.** A callee that
is no expression, an expression that is no name, and a name that is no `stylex`
import all mean the same thing to the dispatcher above. The first of the three
is unreachable -- the declaration reader admits only a name or a member read --
and the three answers are one let-chain now. Two compiled modules were added
for the shapes that produced the question, a dynamic import and a call to the
base class of a constructor, and both agree with the Babel plugin.

| Where                                                  | Regions | Held by |
| ------------------------------------------------------ | ------- | ------- |
| `src/transform/stylex/create_call/helpers.rs`          | 8       | 64      |
| `src/transform/stylex/transform_stylex_atoms.rs`       | 5       | 64      |
| `src/shared/utils/core/evaluate_stylex_create_arg.rs`  | 4       | 63      |
| `src/transform/stylex/create_call/mod.rs`              | 3       | 64      |
| `src/transform/stylex/create_theme_nested_call.rs`     | 3       | 64      |
| `src/transform/stylex/transform_define_marker_call.rs` | 3       | 64      |
| `src/transform/stylex/create_theme_call.rs`            | 1       | 64      |
| `src/transform/stylex/visitor_utils.rs`                | 1       | 64      |

What is left is the two groups this ticket named a pass of its own for: the
guards over an object the compiler itself built, and the three empty-answer
refusals the shared reader does not cover. The empty answers were probed --
`stylex.createTheme((() => 1) + 1, {…})` and the two nested spellings all report
through the *deopt* arm, not the empty one -- so no source is known to reach
them yet.

### The last regions under `transform/stylex/`

Seven groups, and only two of them wanted a mark.

**A `when` selector that is not a string has a source.** `when.ancestor(['…'])`
folds to a list, which has no string form, and the handler refuses it.
`rejects_a_selector_that_is_not_a_string` measures the sentence.

**A dynamic shorthand under a condition has a source.** `marginInline` written
as a dynamic style with a conditional value expands to two declarations that
carry a value and two that unset a property, and its path runs one step deeper
than the property it names.
`legacy_expanded_dynamic_shorthand_under_a_condition` measures both.

**The legacy expansion parsed back what the loop already held.** Each dynamic
style was given a marker value, `p0`, `p1`, and the expanded declarations were
matched to their style by parsing the index back out of the expanded value. The
loop enumerates the styles, so the style is carried beside its declarations
instead: one string parse and one list look-up per expanded declaration are
gone, and with them the two answers neither could give. The third arm of the
path rewrite went with them -- a path is the chain its key was cut from, so a
path longer than its key always continues with a `_`, and the arm for the
others was unreachable.

**The nullish-fallback read is a search.** `extract_expr_from_rule` walked the
captures with two guards inside the walk. The capture group the pattern names is
not optional, and a variable the fallback map does not hold is the next one to
look at, so the walk is a `find_map` over the names the rule holds.

**Three producers asked twice what the fold answered.** `create`, `createTheme`
and `unstable_createThemeNested` each checked the fold's confidence and then
read its value, and each wrote a refusal in both places. They read the value
once now, the way `folded_style_object_lit` reads it for the seven producers
that share it, so a fold that refused and a fold that answered nothing report
one sentence at one position. The nested theme's second argument reads through
that shared reader itself -- its three answers and its two sentences were
already the reader's -- which took 45 lines out of the handler.

**A group hash that names no variable is refused by one reader.** The nested
theme wrote the check out again, with the same sentence as
`or_refuse_nameless_group` beside `createTheme`'s. It asks that reader now.

**The dynamic-namespace list cannot be empty.** The argument reader answers a
map of dynamic functions only where it holds one, so the emptiness check over
it had no second side. It is gone.

**Two marks, each with its producer named, and the crate holds 17 where it held
15.** `defineMarker` reads the declarator its call initialises; the check above
it proves the call is a top-level expression bound to a named export, and
`stylex_state::state_writers::record_top_level_declarator` writes a named
declarator into both lists or into neither, which
`a_top_level_declarator_is_recorded_in_both_lists` measures. The other reads the
compiled namespace an atom was written into, under the name it was written with,
which `answers_a_namespace_under_the_name_it_was_given` measures.

A third step left the measurement without a mark of its own. The nested theme's
group-hash check was one of the three uncovered regions this ticket listed for
that file, and it closed by asking `or_refuse_nameless_group`, which is marked
already. The invariant holds -- a theme reference answers something other than a
variable for two keys, and the group hash is neither -- but the region closed by
routing rather than by a case, and the record should say so.

The same check also answers `defineMarker`'s export name, as the two `define*`
checks beside it do, so the handler no longer reads the declarator to find a
name the check has already proven.

### The review of the last batch

Two passes ran over these commits, one on the standards and one on performance.
The performance pass found no regression and one win: the legacy shorthand
expansion now makes one `String` fewer per expanded declaration, and reads no
value back out of a CSS text. The standards pass found five things, and each is
settled.

**The namespace fold guard has a source, so it needed no mark.** A module that
binds one name twice -- `import { keyframes as stylex }` beside
`import * as stylex` -- registers that name for the API first, so the namespace
registration beside it finds no fold to write into and leaves the API in place.
The language forbids the module and `@babel/parser` stops on it, so the
reference implementation never reads it and there is nothing to compare with;
the SWC parser reads it, so this compiler answers something, and
`a_name_bound_to_an_api_and_to_the_namespace_keeps_the_api` records what. The
case writes a `keyframes` call inside the style, which is what shows which
registration won: the rule is injected, where a lost API binding would fold to
nothing.

**The merged readings say what they are.** The three producers that now read the
fold once each carried a comment saying a refusal and a confident answer with no
value are the same fault. They are not: the evaluator's memo answers nothing
while a walk stays confident, which
`stylex-evaluator`'s `UNRESOLVED_MEMO_WARM` cases measure. What is true is
narrower and is what the comments say now -- the reachable answer reads the
sentence and the position it always did, and the other one is a state no source
through these producers reaches. Every source tried reaches the refusal instead:
the same subtree written twice inside a `create` call folds twice rather than
answering from the memo.

**Two invariants name the test that holds them.** A key is a prefix of its path,
which the shorthand expansion rewrites by, and
`a_key_is_a_prefix_of_its_path_that_ends_at_a_property` measures it beside the
walk that writes one. The atoms marks name
`a_namespace_of_two_string_literals_folds` and
`answers_a_namespace_under_the_name_it_was_given`.

**The atoms trait said something that is no longer true.** Its two readers
promised to leave the author's expression for the runtime where a style is not
statically evaluable. The compiler in this repository never answers that: it
writes the object it compiles from two string literals. The trait says what
`None` means and who answers it.

**The marker's declarator position is measured, not only its presence.**
`the_position_names_the_declarator_the_call_initialises` reads two markers in
one module and asks each call for its own name.

### The second review round, and the defect it found

**The atoms pass refused a style it is allowed to leave alone.** The pass reads
the object it writes itself, and the first batch read that as a state no input
can reach, so the graceful answer -- leave the member expression for the runtime
-- became a refusal behind a mark. It is reachable: the object is two levels
deep and `maxEvaluationDepth` is a project setting clamped to one at the low
end, so a project that sets it to one stops the fold above the atom. Measured:
six of this file's cases abort at that ceiling.

The answer is back, and the ceiling is the source that measures it --
`inline_static_the_fold_cannot_reach_is_left_for_the_runtime`. The mark
is gone with it, which is the first of the four ranked answers rather than the
third.

**Four smaller things.** The case for a path holding no property was named for a
claim it could not carry, and says what it measures now. The twice-bound name
compiled a module that could not show which registration won, and now writes a
`keyframes` call that can. A doc comment still named the marker check by its old
name. And the two producer cases spelled the atoms namespace name out instead of
reading the constant the pass writes it with.

### The round after that

Six things, all in what the code says rather than in what it does, except the
last.

**A rule was written as though it held for every input.** A key is a prefix of
its path, and a key names a property, so it is never empty. The expansion needs
the second half and the first one does not give it: the rewrite replaces the key
at the head of a path, and an empty key would match at every step. The comment
states both halves now, and says what an empty key would do to the line that
reads them.

**A case was named for the wrong side of the ceiling.** A style *below* a
ceiling folds; this one is the style the fold cannot reach, and the case says
so.

**The shorthand marker is one constant.** It carried an index from the days when
a declaration found its style by parsing that index back out. Nothing reads it,
and no expansion tells one marker from another, so the index, the arithmetic
that sized it and the string built for each style are gone. What the value still
has to be is written down: one token holding a digit, which is the shape
`listStyle` sorts into the slot a keyword would not take.

Three smaller ones: a bail-out comment that read as though the ceiling were the
only way a fold answers nothing, a test doc that pointed at a walk in another
file, and a constant spelled out where the file next door imports it.
