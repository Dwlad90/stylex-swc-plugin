# 47 — Read every style shape through its parentheses

**What to build:** Parentheses are a node in this compiler's tree and none in
the reference implementation's, so a reader that matches a bare node sees a
different expression from the one the author wrote. Ticket 15 found two such
readers and
[fixed](../../../crates/stylex-evaluator/src/evaluate/nodes/object_expression.rs)
them; the review that followed found the same reading in many more places. Close
the class, and put a guard under it so a new reader cannot join it.

The defect is not the same everywhere, and the second kind is worse:

1. **A hard abort.** The build stops on an expression the same author could have
   written without parentheses and compiled.
2. **A silent hand-back.** Nothing is reported, the declaration falls to the
   runtime, and the author has no way to know a pair of brackets cost them the
   fold.

**Where it is.** Confirmed by running the shape:

- `crates/stylex-transform/src/shared/utils/validators.rs:219` and
  `.../core/evaluate_stylex_create_arg.rs:168` — the create argument in
  parentheses.
- `crates/stylex-transform/src/shared/utils/validators.rs:66` — the keyframes
  argument in parentheses.

Read statically and not yet reproduced, so each wants confirming before it is
changed:

- **Likely aborts.** `validators.rs:86` and `:315` (`init_expr.as_call()`, so
  `const vars = (stylex.defineVars({…}))` reads as non-static).
  `crates/stylex-evaluator/src/convertors.rs:24`, `:69`, `:99` and `:143` — a
  parenthesized binding initializer or unary operand reaches a `_ => panic` arm,
  and the message names `ParenExpr`.
- **Silent hand-backs.** `stylex_merge.rs:95`, `:106`, `:147`, `:169` and its
  receiver `parse_nullable_style.rs:78`, `:106` — `stylex.props((styles.root))`,
  `cond ? (styles.a) : (styles.b)` and `stylex.props(([a, b]))` all bail out and
  punt the merge to the runtime. `validators.rs:570` (`is_target_call`) —
  `(stylex.create)({…})` is not read as a StyleX call at all. `validators.rs:103`
  and the keyframes, positionTry and viewTransitionClass call transforms —
  `const fade = (stylex.keyframes({…}))` is left untransformed.
  `validators.rs:345` — `stylex.createTheme(vars, ({…}))`.
  `crates/stylex-evaluator/src/evaluate/nodes/member_expression.rs:31` —
  `(styles).root`.
- **Unusual spellings, lower priority.** `nodes/call_expression.rs:49`, `:89` and
  `:342`, `engine_stylex_functions.rs:126`,
  `nodes/member_expression.rs:721` and `:744`, `nodes/optional_chain.rs:22`,
  `flatten_raw_style_object.rs:105` and `:387`.

Struck, with the case that decided it:

- `transform_stylex_define_vars_call/helpers.rs:74` and `:250` — read
  statically as an abort, and it is not one. Both readers run on the object the
  **evaluator rebuilt**, not on what the author wrote, and that object carries no
  `Expr::Paren`: a probe on the value each reader sees answers `is_paren=false`
  for every property. `a_function_value_written_inside_parentheses` in
  `transform_stylex_define_vars_test/function_values_tests.rs` compiles to a
  snapshot byte-identical to `a_function_value`, with and without a
  `normalize_expr` at either site.
- `nodes/call_expression.rs:342` (`member_callee`'s receiver name) — a real
  asymmetry with `engine_fold/guard.rs`, and unobservable. The fold guard reads
  the same position through its parentheses and answers first, so
  `(Math).max(1, 2)`, `(stylex.types).angle('0deg')` and `(a).push(1)` each give
  exactly what the bare spelling gives, in this compiler and in the reference
  one. Left as found rather than changed on an argument no source can test.

Confirmed **safe** and not to be touched: `evaluate/mod.rs:386` normalizes at the
dispatch, `evaluate_stylex_create_arg.rs:206`, `:218` and `:468` already
normalize, and the post-fold namespace validators (`validators.rs:683`, `:761`)
read evaluated values rather than syntax.

**How to take it.** Reproduce first, fix second: a case that compiles the
parenthesized spelling to the same output as the bare one is what says a site is
really broken, and is the same shape ticket 15's transform case takes. A site
whose parenthesized spelling already compiles alike wants no change and should be
struck from the list above.

**Blocked by:** None.

**Status:** resolved

- [x] Every site above is either fixed or struck, with the case that decided it.
- [x] A guard sits under the class:
      `every_shape_in_the_class_compiles_alike_in_both_spellings` compiles
      twenty-three shapes both ways and compares what each printed, and three
      more tables do the same for the compiled JSX call, `createTheme` and
      `defineVars`, so a reader added later that matches a bare node fails for
      the shape it reads.
- [x] A parenthesised spelling compiles to the same output as the bare one, for
      each shape a case covers. Measured against
      `@stylexjs/babel-plugin@0.19.0` through `parity:probe`, and pinned by
      `parenthesised_spellings.rs`, `create_theme.rs` and
      `callee_shape_dispatch_tests.rs`.
- [x] The full workspace suite stays green, with `pnpm typecheck`,
      `format:check`, `lint:all`, `pnpm test` and
      `cargo test --workspace --all-features`.

## Comments

**2026-09-09, ticket 15.** `validators.rs:570` (`is_target_call`) is now
**confirmed** rather than read statically: `(stylex).create({ base: { color:
'red' } })` is left untransformed here and compiles to a rule upstream, so the
whole call reaches the runtime with no styles at all. It is the largest of the
silent hand-backs on the list.

Two sites were struck in the same pass, and the reasoning is in the list above:
the two `defineVars` readers, which see the evaluator's rebuilt object rather
than the author's syntax, and `member_callee`'s receiver name, which the fold
guard already reads through its parentheses.

**2026-09-07, ticket 15.** Raised out of the review of
`fix(evaluator): read a style function through its parentheses`. That commit
closed the two sites a dynamic style goes through; the list above is what the
same reading turns up elsewhere and is recorded rather than fixed, because each
site is a change to shipped behaviour and wants its own case.


**2026-09-13, part done.** Every site on the list above was **reproduced rather
than read**, by compiling the bare spelling and the parenthesised one and
comparing the output. That is the evidence the ticket asked for, and it moved
four entries off the list and five into a fix.

**Reproduction, one table.** Each row compiles both spellings through the
transform. `differs` means the parenthesised one either stopped the build or
compiled to something else.

| Shape | Reproduced | Now |
| --- | --- | --- |
| `stylex.create(({…}))` | differs, build stops | fixed |
| `stylex.keyframes(({…}))` | differs, build stops | fixed |
| `const fade = (stylex.keyframes({…}))` | differs, silent hand-back | fixed |
| `const pt = (stylex.positionTry({…}))` | differs, silent hand-back | fixed |
| `const vtc = (stylex.viewTransitionClass({…}))` | differs, silent hand-back | fixed |
| `export const s = (stylex.create({…}))` | differs, build stops | fixed |
| `const m = (stylex.defineMarker())` | differs, wrong refusal | fixed |
| `(stylex.create)({…})` | differs, silent hand-back | **open** |
| `(stylex).create({…})` | differs, silent hand-back | **open** |
| `stylex.props((styles.root))` | differs, silent hand-back | **open** |
| `stylex.props(([a, b]))` | differs, silent hand-back | **open** |
| `cond ? (styles.a) : (styles.b)` | **agrees** | struck |
| `(styles).root` | **agrees** | struck |
| `-((1))` as a style value | **agrees** | struck |
| `const w = (1)` read as a value | **agrees** | struck |
| `stylex.createTheme(vars, ({…}))` | needs a `.stylex.js` host | **unmeasured** |
| `const vars = (stylex.defineVars({…}))` | needs a `.stylex.js` host | **unmeasured** |

Upstream compiles every parenthesised spelling above to exactly what it compiles
the bare one to, measured against `@stylexjs/babel-plugin@0.19.0`.

**What the fix was, for the five.** The argument readers unwrap the parenthesis
where they read it. The initializer group was not a validator at all: the
structural-hash indexes in `stylex-state` record a declarator's initializing
call under a key computed from the initializer, and a parenthesized one was
recorded under **no key**, so the call was never found and reached the runtime
untransformed. `call_key_of`, `push_declaration`, `push_top_level_expression`
and the three lookups now read the initializer through its parentheses, so one
key answers both spellings.

**`is_target_call` was left reading the callee bare, on purpose.** Unwrapping
it there was tried and reverted: the dispatch above it -- `process_declaration`
in `transform/mod.rs` -- still matches an `Expr::Ident` or an `Expr::Member`
bare, so unwrapping only in the predicate transforms nothing. What it did
instead was class `(stylex.props)(styles.b)` as a consumer call that is then
never transformed, which drops `b` from the namespace the pruner keeps and
leaves the element silently unstyled. **The dispatch has to move first**, and
the two callee rows go with it.

**Both span lookups now read through a parenthesis, not one.**
`find_call_declaration_by_span` and `find_top_level_expr_by_span` are asked
together -- `validate_stylex_define_marker_indent` reads both in one branch --
so a paren that blinded one of them decided which of two refusals an author
read. Normalising the pair moved `const m = (stylex.defineMarker())` onto
upstream's own sentence, word for word.

**`stylex.props` is a separate reader** in `stylex_merge.rs` and
`parse_nullable_style.rs`, untouched here.

**Two shapes in this class are recorded and not fixed**, found in the review
rather than by the probe: `state_writers.rs` records
`pattern_bound_top_level_calls` from a bare `Expr::Call`, so
`const { a } = (stylex.create({…}))` stays invisible; and
`holds_call_in_top_level_array` records array spans bare while
`is_bound_create_expr` strips parentheses.

**The unusual spellings on the list below were not reached**, and neither were
the two `defineVars`/`createTheme` rows, which the transform test harness
cannot host without a `.stylex.js` filename.


**2026-09-14, the rest of the class.** Every remaining row is closed. The list
above and the table from the previous pass both read the same way now: no site
is left open and no shape is unmeasured.

**The four open rows.** Two were one defect and not two. The visitor's dispatch
-- `process_declaration` in `transform/mod.rs` -- and the predicate beside it,
`is_target_call`, each read the callee bare, so `(stylex.create)({…})` and
`(stylex).create({…})` were not read as StyleX calls at all and the module
reached the runtime with no styles. The previous pass recorded that the dispatch
has to move first, and it did: both now read the callee and its receiver through
their parentheses, so a producer call and a consumer call are classed alike and
the namespace the pruner keeps is not dropped. The other two are the merge
arguments -- `stylex_merge.rs` reads each argument through its parentheses, at
both levels, so `stylex.props((styles.root))` reaches an arm of the resolver and
`stylex.props(([a, b]))` is flattened into its elements; `parse_nullable_style`
reads its own argument, a member's receiver and a computed key the same way.

**The two unmeasured rows both agreed.** The probe could not host a
`.stylex.js` filename, so it takes one: `pnpm parity:probe '<json>' vars.stylex.js`
names the file both compilers are told the source came from, defaulting to
`probe.js`. Measured there, `const vars = (stylex.defineVars({…}))` already
compiled alike -- struck. `stylex.createTheme(vars, ({…}))` did not: the second
argument was read bare and stopped the build. It is read through its parentheses
now.

**The two shapes the review recorded are fixed.** `state_writers.rs` reads a
pattern-bound declarator's initializer through its parentheses, so
`const { a } = (stylex.create({…}))` is marked program level and its result is
not hoisted into a variable the author did not write. `state_manager.rs` records
a top-level array's span through its parentheses, through one `array_span_of`
shared by the writer, the replacement and the debug assertion -- the same
reading `is_bound_create_expr` already used.

**The unusual spellings were reached, and three of nine were real.** Measured
one by one: `(pick)()`, `(palette.pick)()` and `(stylex.firstThatWorks)(…)` each
refused with `Unsupported expression: CallExpression` where the bare spelling
folds. The evaluator's call dispatch, the engine fold's guard and
`engine_callable` now read the callee through its parentheses. The receiver
inside it deliberately does **not**: the fold prints the source back for the
engine to parse, and `{ a: 1 }.valueOf()` opens a block there rather than naming
an object, so the receiver is carried with its brackets. Only the receiver's
*name* is read through them, which is what `(palette).pick()` and
`(stylex).firstThatWorks(…)` need. The rest -- `optional_chain.rs`,
`flatten_raw_style_object.rs`, the template and spread positions -- already
compiled alike and are struck.

**One test changed its claim rather than its subject.**
`parentheses_around_a_method_take_it_out_of_the_arm_that_reads_receivers`
recorded the old asymmetry as deliberate. The asymmetry is gone, so it is now
`parentheses_around_a_method_reach_the_arm_that_reads_receivers` and asserts
the fold. `parentheses_do_not_hide_a_shadowed_callee` reads upstream's own
sentence now -- `Unsupported expression: FunctionDeclaration` -- rather than the
catch-all, because the dispatch reads the name.

**Three gates were failing on `develop` before this work and are fixed here.**
The `lone surrogate in a name` refusal family claimed one verdict and ticket 51
added rows under the other, so the parity harness reported two unexpected rows;
the family reads both verdicts now, and its test reads the same. The elision
suite still expected the refusal ticket 49 replaced. The harvested corpus and
the generated value-parser cases were both stale.

**The guard the ticket asked for.** The snapshots say what one spelling prints,
which cannot catch a new reader: a reader that matches bare changes only the
parenthesised spelling, and a snapshot of that spelling records the change as
correct. `every_shape_in_the_class_compiles_alike_in_both_spellings` compares
the two spellings of one module instead -- fourteen shapes, each pair differing
only by the brackets -- through a new
`assert_spellings_agree` in the test utilities. Reverting `process_declaration`
to read its callee bare makes it fail, which is how it was checked.

**Two rows the review raised and the probe struck.** `theme_ref_base`
(`member_expression.rs:717`) reads a member chain's base bare; `(vars).c` off a
theme import compiles to exactly what `vars.c` compiles to, here and upstream,
so it is struck rather than changed. The dangling `_inject` import in the
top-level-array snapshot is the runtime-injection harness and not the
parenthesis: the bare spelling shows it identically, which the case now proves
by holding both spellings in one module.


**2026-09-14, three sites the list never held.** The review of `fix_benchmarks`
found the class was not closed: three readers were in it and on no list, so they
were neither fixed nor struck. Each is the silent shape -- nothing is reported,
the declaration falls to the runtime, and the author gets no styles and no
error. Reproduced first, as the ticket asks.

| Shape | Reproduced | Now |
| --- | --- | --- |
| `const stylex = (require('@stylexjs/stylex'))` | differs, silent hand-back | fixed |
| `const css = (require('@stylexjs/atoms'))` | differs, silent hand-back | fixed |
| `(require)('@stylexjs/stylex')` | differs, silent hand-back | fixed |
| `require(('@stylexjs/stylex'))` | differs, silent hand-back | fixed |
| `(React).createElement("div", { sx })` | differs, silent hand-back | fixed |
| `(_$setAttribute)(e, "sx", v)` | differs, silent hand-back | fixed |
| `_jsx(("div"), { sx })` | differs, silent hand-back | fixed |
| `_jsx("div", ({ sx }))` | differs, silent hand-back | fixed |
| `_$setAttribute(e, ("sx"), v)` | differs, silent hand-back | fixed |
| `{ [('a')]: {...} }` as a namespace key | differs, silent hand-back | fixed |
| `const x = ({...})` in `decl_init_hashes` | **agrees** | struck |
| `const s = (stylex.create({...}))` through `find_top_level_expr_named` | **agrees** | struck |

**The two requires.** `discover_commonjs_stylex_require` and
`discover_commonjs_atoms_require` each read the initializer with
`init.as_deref().and_then(Expr::as_call)`. A parenthesised `require` answers
`None` there, so the import is never registered and the **whole module** reaches
the runtime unstyled. Both read it through `init_call` now, which is the reader
made for this question, so the rustdoc claim that the readers "ask it in one
place" is true.

**Inside the two requires, three more.** The review of this work found that the
call the two functions then read was itself read bare: the callee, so
`(require)('@stylexjs/stylex')` was not a `require` at all, and the argument, so
`require(('@stylexjs/stylex'))` named no module. The two functions also held a
byte-identical copy of the predicate. One `required_module` now answers what
module a `require` call names, reading both positions through their
parentheses, and the source test and the atoms test both ask it -- the same
"one rule rather than one spelling per site" `init_call` is built on.

**The compiled JSX call.** `is_jsx_runtime_call` read the callee and the
receiver inside it bare, so `(React).createElement("div", { sx })` was not read
as a JSX runtime call and the `sx` prop was handed back. The Solid.js setter
beside it read its own callee the same way. Every other position the `sx` scan
reads was bare too -- the element name, the props object, the Solid attribute
name, and the computed key `find_sx_prop` matches. All of them read through
their parentheses now.

**The computed namespace key.** `namespace_name_from_prop_key` and
`try_namespace_name_from_prop_key` in `stylex-ast` read a computed key bare,
while `convert_member_prop_to_string` beside them already normalised one. So
`{ [('a')]: {...} }` named no namespace where `{ ['a']: {...} }` did. The two
readers agree now.

**One dead field went with them.** `props_declaration` was written from a bare
`Expr::Call` read of the initializer and read nowhere in the workspace. It is
deleted rather than repaired: a reader whose answer nothing asks for is not a
reader.

**`decl_init_hashes` is struck.** It hashes only an init that
`is_object() || is_lit()`, so `const x = ({...})` is hashed under no key. The
queue side keys on the object the transform **produced**, not on what the author
wrote: `visit_mut_expr` assigns `*expr = value` to the outer expression, so a
parenthesised initializer is a bare `Expr::Object` by the time the flush reads
it. Both sides skip parentheses, the keys agree, and
`a_parenthesised_create_initializer` compiles to the bare spelling's output --
which is what says so.

**The guard grew, and three one-sided snapshots joined it.**
`every_shape_in_the_class_compiles_alike_in_both_spellings` holds twenty-three
shapes now, and `a_compiled_jsx_call_is_read_through_its_parentheses` holds the
seven positions the `sx` scan reads. The `createTheme` and `defineVars` shapes
recorded only one spelling each, which a reader that matches bare cannot fail --
it changes only the parenthesised spelling, and the snapshot of that spelling
records the change as correct. Both are guard tables now, in their own files,
because each needs a harness the default transform does not give:
`assert_spellings_agree_with` takes the compile step so the one assertion serves
all three tables.

Upstream compiles every shape above to what it compiles the bare one to,
measured against `@stylexjs/babel-plugin@0.19.0` through `parity:probe`.


**`find_top_level_expr_named` is struck.** It compares the recorded expression
with the one it is asked about raw, where every sibling lookup normalises, so it
read as the last open site of the class. It is not one. The lookup runs in the
finalize cycle, and by then `visit_mut_expr` has assigned the produced object to
the **outer** expression, so both sides hold a bare `Expr::Object` -- the same
reason `decl_init_hashes` was struck.

Reproduced rather than read, two ways. A guard row compiles a non-exported style
variable with one namespace the pruner keeps and one it drops, in both
spellings, and the two agree: `unused` is dropped from each. And normalising
only the recorded side -- the change that would break a raw-passing caller --
leaves the whole workspace suite green, which says no case reaches the lookup
with a parenthesis on either side.
