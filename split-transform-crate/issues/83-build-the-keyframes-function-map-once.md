# 83 — Build the function map and the stable options once, not once per call

**What to fix:** three allocations on the `keyframes`, `positionTry` and
`viewTransitionClass` paths that are rebuilt for every call although nothing
about them changes inside a module. Found by a performance review of
[70](./70-read-first-that-works-inside-a-keyframes-step.md); the one that was
cheap to settle -- a deep copy of the options inside `stylex_keyframes` -- is
already done.

**1. The `firstThatWorks` function map, per call.**
`transform_stylex_keyframes_call.rs` builds two `FxHashMap`s and one boxed
`FunctionConfig` per local name, on every call, even in a module that never
writes `firstThatWorks`. The same twenty-five lines stand in
`transform_stylex_position_try_call.rs` and
`transform_stylex_view_transition_class_call.rs`, so this is one shape written
three times. The map is a function of the import sets alone.

**What has to be checked first:** whether the import sets are complete before
the first call of the three is handled. A map built too early holds fewer
names than the source spells, and a name that is not in the map does not make
one declaration drop -- it stops the whole call folding, with
`Only static values are allowed inside of a keyframes() call.` That was
measured, by deleting the registration, so the failure mode is known.

**2. `StyleXStateOptions::default()`, per call.** Built in
`stylex_keyframes.rs` and in `stylex_position_try.rs` to hash a
direction-stable string, and in `stylex-css/src/utils/when.rs` to name the
default marker. It builds the default import-source set and its strings every
time. It is a pure default, so one shared value serves all three.

**3. Each argument of a fallback list is read to text twice.**
`stylex_first_that_works.rs` calls `text()` once while it plans the fallbacks
and again while it folds them. For a string literal that is one extra `String`;
for an identifier it is a second binding lookup through
`get_var_decl_by_ident`. Linear, not quadratic, and the function is
deterministic in the text, so the texts can be kept from the first pass.

**Status:** in-review

- [x] The function map is built once per module, or the check above says why
      it cannot be, and the three call sites stop repeating the same block.
      `rule_call_eval_config` in
      `crates/stylex-transform/src/transform/stylex/visitor_utils.rs` builds it
      and `CacheState` keeps it. The import sets are complete: see the answer
      below.
- [x] The two conditions the shared map rests on are written down beside it:
      every import is recorded before the first call folds, and `env` is one
      object for the file. `the_env_survives_into_every_map_the_module_asks_for`
      measures the second, and the `shared_function_map` suite the first.
- [x] `StyleXStateOptions::default()` is built once, shared by the three
      readers. `with_default_options` in
      `crates/stylex-structures/src/stylex_state_options.rs`.
- [x] A fallback-list argument is read to text once. `ArgTexts` in
      `crates/stylex-evaluator/src/stylex_first_that_works.rs`, guarded by
      `an_argument_is_read_to_text_once`, which counts the reads through a
      mapper.
- [x] Output is unchanged: the full workspace suite, the fixtures, and
      `parity` are green, and a paired `bench:revisions` says the change is
      not a regression. The figures are in the answer below.

## Answer

### The import sets are complete before the first of the three calls

Every writer of the import sets runs in the `Discover` cycle, and all three
calls run in `TransformProducers`, which is a later pass:

- `visit_mut_import_decl.rs` -- the whole body is gated on `Discover`.
- `visit_mut_var_declarator.rs` -- the CommonJS `require` reader is called only
  from the `Discover` arm.
- `visit_mut_jsx_opening_element.rs` -- returns unless the cycle is `Discover`,
  so the namespace import an `sx` attribute adds is written inside that pass.
- `transform_stylex_calls.rs` dispatches all three handlers under
  `TransformProducers`, and `visit_mut_module.rs` runs each cycle once, in
  order.

Nothing in the change enforces this; it is a property of the pass order. A
future writer outside `Discover` would shrink the cached map, and a name the map
does not hold stops the whole call folding with
`Only static values are allowed inside of a keyframes() call.` That is the
failure the ticket names, and it is why this paragraph is here rather than in a
comment alone.

### What the change removed beside the map build

A performance review of the first draft found that the map build was only half
the cost: `evaluate` copies the whole `FunctionMap` on every call, which rebuilt
the allocations the memo had just saved. `evaluate_with_functions` is public
now, and takes the shared map the memo holds. The four other call sites that
build a map and hand it straight to `evaluate` --
`createThemeNested`, `defineVarsNested`, `defineConsts` and
`defineConstsNested` -- wrap theirs in an `Rc` for the same reason. They are
built per call still, because each module makes one or two such calls.

Two smaller ones in `stylex_first_that_works`: the fold moves each text out of
the memo rather than copying it, and the shape with no variable in the list
moves each argument into the answer rather than deep-copying the expression.

### The paired benchmark

`bench:revisions`, base at `f305c209d` against this change, 10 rounds, seed 1,
both subjects timed in a process of their own. `bench:verdict` over the raw
stats: **suite passed**, 63 fixtures, no `warn` and no `fail`. The five that
carry one of the three calls all read at or below 1.000, which is the right
direction although each is inside the noise band:

| Fixture                                  | Point | Lower | Upper |
| ---------------------------------------- | ----- | ----- | ----- |
| `page-with-keyframes`                    | 0.995 | 0.984 | 1.001 |
| `Feature - keyframes and animations`      | 0.996 | 0.988 | 1.006 |
| `Feature - keyframes and animations (dev)`| 1.000 | 0.996 | 1.004 |
| `Feature - view transitions`              | 0.996 | 0.994 | 1.002 |
| `Feature - view transitions (dev)`        | 1.007 | 0.991 | 1.019 |

The run is a "no regression" reading rather than a measurement of the saving.
The machine was a working desktop at about 55% idle, and the corpus holds no
module with many `keyframes` calls, which is the shape the saving grows with.

### What the review of the first commit changed

Three rounds of review ran over the first commit. What they found, and what was
done:

- **The memo was not guarded.** Delete the cache lookup and every test stayed
  green, so nothing held the property the ticket is about.
  `transform/stylex/tests/rule_call_eval_config_test.rs` asks it directly, with
  `Rc::ptr_eq` over two calls. It fails when the lookup is removed, which was
  measured both ways.
- **A data clump.** `identifiers` and `member_expressions` travelled together
  through eight signatures. They are a `FunctionMap` minus one flag, so
  `apply_stylex_env`, `register_stylex_helper`, `register_stylex_identifier`,
  `apply_unstable_conditional` and `register_env_in_namespace_fold` take the
  map, and six builders write into one rather than assembling it at the end.
- **The same registration written seven times.** "Register under every name an
  import gave the helper, and as a member of every namespace" is
  `register_stylex_helper` now, and its identifier-only half is
  `register_stylex_identifier`. `defineVars` turned out to register exactly what
  `build_eval_config` does, so it calls it.
- **A hand-maintained count.** The memo store was briefly an array indexed by
  the variant, which panics if a variant is added without changing the length.
  One exhaustive match names both slots instead, so a new set of helpers does
  not compile until it has one.
- **The term clashed.** "Nested rule" already means the rule the call written
  *inside* one of these three declares. The three are a **Rule call** now, in
  the code and in both glossaries.

Two findings were read and not acted on, with the reason:

- The `rest` arguments of a fallback list are still copied rather than moved.
  Every argument that reaches there is a string or an identifier, because
  anything else stops the build, so the copy is of a cheap node.
- The wider maps are still built per call. Each module makes one or two of
  those calls, and the copy `evaluate` made is gone, so a memo would buy
  nothing that can be measured.

`bench:revisions` over the follow-up, base at the first commit: **suite passed**,
66 fixtures, no warning. The `create` fixtures, which carry the most-rewritten
builder, read 1.001 and 1.006.

### Corrections to the ticket text

`StyleXStateOptions::default()` does not build the default import-source set:
`CoreStyleXOptions::default()` leaves that set empty. What it does build is two
owned strings, an empty set, a shared empty env object and a module-resolution
default.
