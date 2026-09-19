# 63 — Cover the transform's shared utilities

**What to build:** The tests that exercise the 334 regions under
`crates/stylex-transform/src/shared/utils/` that no test in the workspace
reaches. This is the largest of the four groups
[ticket 62](./62-record-the-transform-coverage-baseline.md) measures, and it is
the layer the call handlers above it all pass through, so it is taken first.

**Where the gap is:**

| File                                   | Uncovered regions | Regions        | Lines  |
| -------------------------------------- | ----------------- | -------------- | ------ |
| `validators.rs`                        | 64                | 87.88%         | 89.82% |
| `core/flatten_raw_style_object.rs`     | 58                | 75.11%         | 79.82% |
| `core/add_source_map_data.rs`          | 58                | 74.46%         | 77.06% |
| `core/dev_class_name.rs`               | 29                | 72.38%         | 68.22% |
| `core/evaluate_stylex_create_arg.rs`   | 27                | 91.26%         | 93.95% |
| `core/flat_map_expanded_shorthands.rs` | 16                | 68.00%         | 71.74% |
| `core/stylex_merge.rs`                 | 15                | 91.76%         | 92.11% |
| `core/parse_nullable_style.rs`         | 13                | 89.08%         | 90.16% |
| `ast/helpers.rs`                       | 8                 | 84.91%         | 84.62% |
| `core/define_vars_utils.rs`            | 7                 | 95.30%         | 95.36% |
| `core/js_to_ast.rs`                    | 7                 | 83.33%         | 80.56% |
| `core/make_string_expression.rs`       | 5                 | 94.38%         | 94.32% |
| `core/styleq.rs`                       | 5                 | 84.85%         | 88.10% |
| `core/attrs.rs`                        | 5                 | 76.92%         | 91.30% |
| the remaining six files                | 17                | 91.11%--99.08% |        |

**Four of them also hold a per-instantiation gap**, which the gate counts even
where every line is run by some test: `add_source_map_data.rs` (the function at
line 254), `attrs.rs` (line 13), `props.rs` (line 20) and `object.rs` (line 22,
where the three producer instantiations — `stylex_keyframes`,
`stylex_define_vars`, `stylex_position_try` — each miss the same five regions).
Close such a group by driving one instantiation through every line of the
function, not by adding a case per instantiation. Run
`scripts/coverage-missing.sh -p stylex_transform --show-phantoms` to see them.

**Two of the largest are one concern each**, and are worth taking that way:
`add_source_map_data.rs` is the reading of an incoming source map, whose
failure arms no fixture produces; `dev_class_name.rs` is the development class
name, which only a `dev` build writes.

**Write the tests where the crate already puts them** — a unit test beside the
code under `src/shared/utils/**/tests/`, an end-to-end one under `tests/`. A
refusal path with a message is worth an assertion on the message, not only on
the refusal.

**Blocked by:** [62](./62-record-the-transform-coverage-baseline.md) — the
baseline and the repeatable suite.

**Status:** resolved

- [x] `scripts/coverage-missing.sh -p stylex_transform` reports no unexercised
  region under `src/shared/utils/`, down from 334. The four that were left are
  read in the last comment below. Both of the
  files that held the rest are closed by a ticket of their own --
  `flatten_raw_style_object.rs` by
  [73](./73-read-a-variable-reference-key-the-way-the-reference-does.md) and
  `stylex_merge.rs` by
  [74](./74-write-the-inline-style-a-props-call-merges.md). The four that
  remain are `evaluate_stylex_create_arg.rs` lines 229 and 370, which are read
  below. The line numbers moved with the file; they were 244 and 385.
- [x] The four per-instantiation groups named above are closed, and so are the
      three the earlier batch left. No group under `src/shared/utils/` counts
      against the gate.
- [x] Tests cover regular and irregular inputs, and the edge cases each function
      states, rather than only the path that makes the number go up.
- [x] Each new test asserts what the code answers, not merely that it answered.
- [x] The full workspace suite stays green.

## Comments

### What the batch closed

`scripts/coverage-missing.sh -p stylex_transform`, `rustc 1.100.0-nightly
(4b6d04e70 2026-09-13)`, on the tip of this branch:

| Figure                                    | Baseline | Now    |
| ----------------------------------------- | -------- | ------ |
| Regions, whole crate                      | 89.03%   | 93.26% |
| Functions                                 | 92.78%   | 93.88% |
| Lines                                     | 90.09%   | 93.65% |
| Regions unexercised under `shared/utils/` | 334      | 36     |
| Tests                                     | 2865     | 3113   |

Fifteen of the twenty files the ticket names now hold no unexercised region:
`validators.rs`, `object.rs`, `live_declarations.rs`, and under `core/`
`dev_class_name.rs`, `flat_map_expanded_shorthands.rs`, `js_to_ast.rs`,
`attrs.rs`, `props.rs`, `styleq.rs`, `stylex_merge.rs`,
`make_string_expression.rs`, `member_expression.rs`,
`stylex_nested_utils.rs`, `convert_style_to_class_name.rs`,
`define_vars_utils.rs`, `parse_nullable_style.rs`, plus `ast/helpers.rs`.

### Three of the four per-instantiation groups are closed

`object.rs` (line 22), `attrs.rs` (line 13) and `props.rs` (line 20) no longer
count against the gate. `obj_map` closed the way the ticket asked -- one
function item passed at one call site, given both an object and a map, so one
instantiation runs the whole function. The other two closed by the guard going:
`attrs` took the properties apart again through three `?` operators over a
shape `props` is the only producer of, and reads the map directly now.

`add_source_map_data.rs` line 254 closed too, with one case for a state that
carries no input source file.

### A test that covers half a function makes a new group

The gate scores a function on its best-covered instantiation, and a unit test
beside the code is a second instantiation of everything it touches. Where the
integration suite covers the other half, neither instantiation is complete and
the gate counts the difference -- even though every region is run by some test.
Three groups stand for that reason:

| Function                         | Best instantiation | Misses |
| -------------------------------- | ------------------ | ------ |
| `add_source_map_data` (line 44)  | 64 of 86           | 11     |
| `flatten_raw_style_object_logic` | 151 of 158         | 3      |
| `validators::contains_call`      | 9 of 11            | 1      |

A fourth, `validate_theme_variables`, closed: the case the integration side was
short of is `createTheme(1, {})`, which is a source an author can write.

Closing one means one binary running the whole function. For `contains_call`
and the flattener that is a handful of cases on the side that is short. For
`add_source_map_data` it is the reporting arms, which only the lib binary can
reach -- it needs the capturing logger to open the level -- against the span
path, which needs a state carrying the module the annotation quotes. The
harness for the second half is written
(`maps_key_position_through_input_source_map`, named here as
`through_an_input_source_map`, which is in no file) and does not yet make a
`file:line` come out: the key spans and the frame the line is
read off are laid out in two position spaces.

### What is left, and why

36 regions over six files.

| File                                 | Regions | What they are                                                                                                                             |
| ------------------------------------ | ------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `core/add_source_map_data.rs`        | 18      | the hydration note, the "found a span, and it is empty" report, two `?` paths of the input-map reader, and the working-directory fallback |
| `core/evaluate_stylex_create_arg.rs` | 11      | refusals after a fold that answered, and the two property kinds a namespace cannot hold                                                   |
| `core/flatten_raw_style_object.rs`   | 4       | the `var()` key branch, and one fall-through                                                                                              |
| `core/parse_nullable_style.rs`       | 1       | a style variable holding no namespace of that name                                                                                        |
| `core/stylex_merge.rs`               | 1       | one fall-through                                                                                                                          |
| `validators.rs`                      | 1       | the optional-call base of a member chain                                                                                                  |

### What the compiler stopped doing

Five copies a compiling call used to pay for, none of them read on that path:

- A theme cloned its whole second argument before it decided whether it had
  anything to report. The four calls beside it already reported lazily.
- The default marker was built twice, once for each loop that registers it.
- A property key was copied for every declaration, and again for every
  condition under it. It is borrowed now.
- A condition of a merge was copied twice: once into the set passed to the key
  builder, once into the expression built from it.
- The conditions of one property copied the whole object they were written in,
  and then copied each condition out of that copy.

And one comparison: the key path asked whether it held a key by making a string
of the key, only to drop it.

The shorthand expansion now borrows the key it is given. A shorthand is read by
name and then expanded into keys of its own, so the name is copied on the one
path that keeps it -- a property that stands for itself. The fallback values of
one property each named it, and every name was a string.

### One result kind for each thing the compiler makes

`FnResult` named three answers -- `Props`, `Attrs` and `Stylex` -- where the
compiler makes two. Nothing in production told the first two apart: the one
reader handled them in a single arm, the three accessors that separate them are
all test-only, and the `NestedStringObject` each wrapped was always the same
variant of it. So `props` read as a wrapper over two types that distinguish
nothing, which is what a reviewer called a middle man.

The type now names what is made: the class name a `stylex(...)` call becomes,
and the values a `props(...)` or `attrs(...)` call becomes. `props` is one line,
the same shape as the two beside it. The last uncovered region in
`shared/enums/data_structures/fn_result.rs` closed with it -- a test asks each
result for the kind it does not hold.

### One argument kind, asked once

`stylex_merge` asked whether an argument was an object, a name, a member or a
call, read it if so, and then asked the same question again to decide what to do
with the answer. `StyleObject::Unreachable` existed only to fill the gap between
the two asks -- a value the reader never answers, carried so the second ask had
something to match on.

Asking once removes it. The type now holds exactly the three answers the reader
gives, the compiler confirmed the variant had no producer left, and the arm in
`styleq` that answered for it went with it. This is the same duplicated question
as the six validators above, in the shape a type rather than a call.

### The `var()` key branch reads a different question than its name

`flatten_raw_style_object_logic` asks `CSS_VALUE_SPLIT_REGEX` whether a key is a
`var()` reference and, on a match, names the key by what it wraps. That
expression is `(\))(\S)|(\")(\")` -- a bracket followed by a non-space, or two
quotes -- which `var(--x)` does not match and `a""b` does. So the branch is
unreachable for every key a call can carry, and reachable only by one that
would be named wrongly. The comment above it now says what the code asks rather
than what the name suggests. Worth a ticket of its own; not settled here,
because the fix changes what the compiler answers.

### Guards that went, and what holds the invariant now

Each is the second or fourth answer in `guidelines/stack/RUST.md`, with the
producer named in its place:

- Six validators asked their own `is_*_call` predicate, which every caller had
  already asked. Two of the six answered `Option` for the same reason and never
  answered `None`.
- `props`, `attrs` and `stylex` answered `Option<FnResult>` and never `None`;
  `make_string_expression` answered `Option<Expr>` the same way.
- `member_expression` refused a spread, a key that is not a name, and a
  property with no value. The evaluator rebuilds every object it folds under
  plain names, so none can arrive.
- `NestedStringObject::_as_styles` had no caller.
- `is_target_call` asked the receiver whether it had a name and then read it.
- The conditions of one property are collected through one `entry` look-up.
- A string literal in a compiled style is read as the text it is.

Where the language needs a case the input cannot reach, the step left out of
the measurement is a function that computes nothing: it is handed the answer the
caller has already worked out and chooses between it and the refusal. The read
that produces the answer stays measured at the call site. There are eight:
`or_refuse_initializer`, `or_refuse_theme`, `or_refuse_nameless_group`,
`or_refuse_unfolded_key`, `or_refuse_nameless_key`,
`or_refuse_handled_template` and `report_unmatched_key`. One is not of that shape -- `object_under` projects a
`&mut` out of an enum the line above has just set, which cannot be handed in.

A first pass wrapped the read and the refusal together. The review read that as
hiding live code with the dead case, which it was: the `is_match` call, the
parenthesis-normalising read and the key-name read all stopped being measured
along with the guard. Splitting them is what `guidelines/stack/RUST.md` means by
marking _the step_.

### The test binary opens the log level

A `log` macro builds its message only when a logger admits the level, so every
warning argument in the crate was unexercised code. The lib test binary now
installs the capturing logger the four other crates use, wrapping the printing
one rather than replacing it. What prints is unchanged; what a case reads back
is per thread.

### What the second batch closed

`scripts/coverage-missing.sh -p stylex_transform`, on the tip of this branch:

| Figure                                    | Baseline | Batch 1 | Now    |
| ----------------------------------------- | -------- | ------- | ------ |
| Regions, whole crate                      | 89.03%   | 93.26%  | 95.60% |
| Functions                                 | 92.78%   | 93.88%  | 97.39% |
| Lines                                     | 90.09%   | 93.65%  | 95.80% |
| Regions unexercised under `shared/utils/` | 334      | 36      | 8      |
| Gate groups under `shared/utils/`         | 4        | 3       | 0      |

`add_source_map_data.rs`, `validators.rs` and `parse_nullable_style.rs` now
hold no unexercised region, and no function under `shared/utils/` counts
against the gate.

### The suite could not reach the annotation path at all

Every span read the `$$css` annotation makes needs `GLOBALS`, and a unit test
does not hold it. The reader stopped inside its own panic boundary and reported
instead, so the whole of "the namespace was located in the authored text" was
measured by no case -- which is why the earlier batch read the gap as two
position spaces. The boundary answered one sentence for every cause, and it
now carries what the panic said; the first run with that in place named the
missing globals directly.

### Three questions the annotation reader asked twice

Each second ask kept a case no input could enter, and each is gone:

- The line of a namespace. `try_get_span_line_number` already answers nothing
  for a span that names no place, so asking the span whether it was empty said
  the same thing first and left the other reason -- a frame that cannot read a
  position off the span -- reported by nothing.
- The bounds of a key position in the input source map. The line read and the
  text read bound it between them.
- The name of a style variable. `is_style_var_ident` answers from the style
  map, and `parse_nullable_style` looked the same name up in the same map
  again. `style_var_namespaces` answers both together.

The working directory is now read where the naming starts and passed on, so
the naming of a file is decided by its arguments. A test cannot move the
process into a directory of its own without moving every other test with it,
which is why a file under a working directory that is in no package was
unmeasured.

### What a getter in a dynamic style did

It was dropped without a word: the style came out empty and the declaration was
in no output. The reference plugin reads a method, a getter and a setter as one
shape and refuses all three, which this reader now does -- measured with
`parity:probe`, which stopped the build there and compiled `(c) => ({})` here.

### The four regions that are left

| File                            | Line | What it is                                                                                       |
| ------------------------------- | ---- | ------------------------------------------------------------------------------------------------ |
| `flatten_raw_style_object.rs`   | 134  | the `var()` key branch -- [73](./73-read-a-variable-reference-key-the-way-the-reference-does.md) |
| `stylex_merge.rs`               | 258  | the JSX spread fall-through -- [74](./74-write-the-inline-style-a-props-call-merges.md)          |
| `evaluate_stylex_create_arg.rs` | 244  | `materialize_style_value`'s last refusal                                                         |
| `evaluate_stylex_create_arg.rs` | 385  | a value the evaluator answered nothing for while confident                                       |

The two in `evaluate_stylex_create_arg.rs` are live refusals rather than dead
guards, and each needs a source rather than a mark:

- Line 244 refuses every folded shape with no expression form. The comment over
  `materialize_style_value` says the set is not audited, so the work is to
  audit it and write one source per shape that reaches it.
- Line 385 is `evaluate` answering nothing while confident. That is reachable
  in principle -- the evaluator's memo outlives the refusal that wrote it, as
  `UNRESOLVED_MEMO_WARM` in `stylex-evaluator` shows -- but every warming
  source tried through a `create` call deopts first, with
  `Unsupported expression: BinaryExpression`. So the route into a `create`
  namespace is not yet known, and the guard must stay until one is.

### What the review of this batch changed

Four findings, all settled in `fix(stylex_transform): settle the review of the
shared-utility coverage`:

- A position at the very end of the input file names no character, and the text
  read that was to bound it from above still slices for it. `src.get(a..len)`
  is a valid range. The end check is back; the start is not, because the line
  read refuses a position below the file and one case says so.
- Naming the reason of a refused value after its key was written twice, in two
  shapes, with two answers: one arm treated a key with no name as a case, the
  other stopped the build over it. One reader answers both now. A key with no
  name is refused where it is read -- `evaluate_obj_key` says
  `The key has no name at compile time.` before any value under it is read --
  and `a_namespace_key_that_spells_no_name_is_refused` measures that with a
  string literal holding an unpaired surrogate.
- The namespaces a member read is answered with are borrowed rather than
  counted.
- The working-directory fixture writes nothing, so the suite leaves no
  directory behind.

A performance pass over the same commits reported no production regression:
the member read loses one hash look-up per argument, the annotation loop loses
one allocation per namespace that writes no entry, and `lookup_line` reads a
`OnceCell` the first in-range namespace already paid for.

### The working directory was read from two sources

`PluginPass` has carried the directory of the compilation since the compiler
was written, and the compiler sets it on every compile. Nothing read it. The
naming of a file read the process instead -- a second source for an answer the
compilation already held, asked once for each file named.

It reads the state now. The value is the same one the compiler puts there, so
what a build writes does not move: none of the 29 snapshots that carry a
`$$css: "file:line"` comes from a test that states a directory.

Nothing stands in for a directory that cannot be read any more. An empty path
is not a directory every file lies outside -- it is one the naming measures
against, which is the substitution `guidelines/stack/RUST.md` refuses because
the value folds into the answer. The state answers with no directory instead,
which every rule in the naming already has a case for. That answer also covers
a directory no text can spell, so the refusal over it goes: an unreadable
environment is not an invariant this code established, which is what
`stylex-evaluator/docs/adr/0002` reserves an abort for.

One coverage exclusion went with it. `add_source_map_data.rs` holds at 100%
with one step fewer left out of the measurement, and `stylex_state` at 100%.

### The performance review of the shared utilities

A performance pass ran over this ticket's commits. It read every allocation the
batch added against the one it replaced and found the batch to be a net win,
which is recorded above. Three costs survived it; two are fixed and one is
read and left.

**The default marker was built once per call, for a value fixed for the file.**
It reads only the class name prefix, so every `stylex.props`-family call in a
module built the same two strings, the same index map and the same two counted
pointers. The batch had already halved this by building it once for both loops
rather than twice; the state keeps it now, beside the short file name it
already caches, so only the first call in a file builds it.

**A member argument made two strings to ask two maps that take a slice.**
`styles.root` read the object name and the property name into owned strings to
index `dynamic_style_namespaces` and the namespace map. Both take a string
slice. The object name is borrowed now, and the property name is borrowed
unless the author wrote a computed key, which is the one spelling that has no
slice to lend.

**The key path's copy is left as it is.** `normalize_key_path` takes the path
by value and each caller hands it a copy, which reads as a copy per emitted
declaration. Taking a slice does not remove it: the caller keeps the path, so
every step of it is copied either way, and only the vector itself is saved.
That is one allocation against a measurement floor of about 16%
([Performance Policy](../../../guidelines/PERFORMANCE.md)), so the change would
be churn that no gate can show.

### The standards and spec review, and three corrections to this record

Two more passes ran over this ticket. They found one defect in the code and
three statements here that the tree does not support.

**A build that collected no inline style was refused.** The reader of the
create argument stopped asking whether the recursion had collected inline
styles and refused the build where it had not, which the step before it did not
do. The option is read as what it is again -- nothing to add, or a map to add
-- so the refusal, and the coverage exclusion that carried it, are both gone.

**The attribute reader's invariant is asserted at its producer.** The reader
over a merge table states that no table holds a spread or a computed key, and
cited no assertion for it. `files_every_answer_as_a_key_value_under_a_name` is
that assertion, which is the answer
[the repository ranks fourth](../../../guidelines/stack/RUST.md) and the shape
the finalize sweep already uses.

**Three statements here were wrong.** `obj_map` did not close the way this
record says; it was deleted, and the group closed with it. The list of refusal
readers named `or_refuse_unspellable_path`, which is in no file. And the count
of what is left out of the measurement is read off the refusal readers alone,
where the tree holds more exclusions than that under `shared/utils/`.

**The exclusions themselves stand.** A reviewer read them as a breach of the
rule against reshaping a signature to move a region. That rule names two
shapes -- a parameter every caller fills with the same constant, and a
substitution such as `unwrap_or` that charges the region to `library/core` --
and these are neither. They are the third answer the guidelines rank, which is
allowed where the step says why it is total, and each one does. What the review
is right about is the ranking: the third answer is for a step that the first
two do not fit, and the inline-style reader above was one that the second did.

### What this ticket still holds

`scripts/coverage-missing.sh -p stylex_transform` on 2026-09-17, after
[73](./73-read-a-variable-reference-key-the-way-the-reference-does.md) closed
`flatten_raw_style_object.rs`: **four regions under `src/shared/utils/core/`,
all in `evaluate_stylex_create_arg.rs`**, at lines 229 and 370. Nothing else
under `src/shared/utils/` is unexercised.

Line 229 is the refusal for a create argument that folded to a value which is
neither a list nor an object the function map stands for. Line 370 is the `?`
over a key's folded value inside the argument walk. Both are guards over a
value the walk above them has already read, so each one wants the ranked list
asked of it rather than a case written for it.

### The last four regions under `shared/utils/`

Both were in `evaluate_stylex_create_arg.rs`, and neither needed a mark.

**A style value the compiler has no expression for has a source after all.**
`materialize_style_value`'s last refusal is reached by `stylex.env` written
whole where a value belongs -- `root: (c) => ({ color: stylex.env })`. The env
object is a fold with no expression form and no object form, so it falls
through every named arm.
`stylex_env_written_whole_as_a_dynamic_style_value_is_refused` measures it,
against the sentence it reports.

**The namespace walk asked twice what the fold answered.** It refused a
namespace that folded to nothing with a sentence of its own, and then refused a
namespace that folded to a value with no object form with another. Both mean the
same thing to an author -- the value under this key is not a namespace -- so the
first read is gone and the absent value falls into the second sentence. This is
the reading `materialize_style_value` beside it already takes of a style value,
which is why the two now agree.
