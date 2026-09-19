# 66 — Cover the transformers, structures and enums

**What to build:** The tests that exercise the 86 regions under
`crates/stylex-transform/src/shared/transformers/`,
`src/shared/structures/` and `src/shared/enums/` that no test in the workspace
reaches. A [transformer](../../../crates/stylex-transform/CONTEXT.md) is the
fold behind one producer API, so this is the smallest and the most evenly spread
of the four groups: eleven transformers holding between one and fourteen regions
each.

**Where the gap is:**

| File                                        | Uncovered regions | Regions | Lines  |
| ------------------------------------------- | ----------------- | ------- | ------ |
| `transformers/stylex_create_theme.rs`       | 14                | 88.64%  | 87.39% |
| `transformers/stylex_types.rs`              | 13                | 92.43%  | 92.37% |
| `transformers/stylex_position_try.rs`       | 13                | 88.29%  | 88.57% |
| `transformers/stylex_define_consts.rs`      | 11                | 83.82%  | 85.48% |
| `transformers/stylex_view_transition_class.rs` | 7              | 92.93%  | 90.91% |
| `transformers/stylex_define_vars.rs`        | 7                 | 92.13%  | 91.67% |
| `transformers/stylex_keyframes.rs`          | 6                 | 95.20%  | 95.16% |
| `structures/pre_rule_set.rs`                | 4                 | 76.47%  | 76.47% |
| `enums/data_structures/fn_result.rs`        | 3                 | 80.00%  | 80.00% |
| `transformers/stylex_create_theme_nested.rs` | 3                | 85.00%  | 88.89% |
| `transformers/stylex_create.rs`             | 2                 | 97.62%  | 97.53% |
| `transformers/stylex_define_vars_nested.rs` | 2                 | 87.50%  | 91.67% |
| `transformers/stylex_define_consts_nested.rs` | 1               | 91.67%  | 94.12% |

**Two per-instantiation gaps sit here as well**:
`stylex_create_theme.rs` (the function at line 34, where the best instantiation
runs 91 of 103 regions) and `stylex_types.rs` (line 217,
`convert_number_to_string_us`). Close each by driving a single instantiation
through every line of the function.

**The transformers already have unit tests beside them** under
`src/shared/transformers/tests/`, so the gaps are additions to an established
pattern rather than a new harness. `stylex_types.rs` is the exception to the
one-API rule — it is the `stylex.types.*` value helpers — and its gap is the
value kinds no fixture spells.

`structures/pre_rule_set.rs` is the composition of
[pre-rules](../../../crates/stylex-transform/CONTEXT.md), and its four regions
are the empty and single-entry cases of a set that fixtures always build full.

**Blocked by:** [62](./62-record-the-transform-coverage-baseline.md) — the
baseline and the repeatable suite.

**Status:** in-review

- [x] `scripts/coverage-missing.sh -p stylex_transform` reports no unexercised
      region under `src/shared/transformers/`, `src/shared/structures/` or
      `src/shared/enums/`.
- [x] The two per-instantiation groups named above are closed. Every
      per-instantiation group in the crate is closed with them: the report
      prints no "Regions the coverage gate counts" section at all.
- [x] Tests cover regular and irregular inputs, and the edge cases each function
      states, rather than only the path that makes the number go up.
- [x] The full workspace suite stays green.

## Comments

### What the batch closed

`scripts/coverage-missing.sh -p stylex_transform` on the tip of this branch:

| Figure                                             | Baseline | Now    |
| -------------------------------------------------- | -------- | ------ |
| Regions, whole crate                                | 89.03%   | 98.69% |
| Functions                                           | 92.78%   | 99.69% |
| Lines                                               | 90.09%   | 98.76% |
| Regions unexercised in this ticket's three directories | 86    | 0      |
| Regions counted against the gate, whole crate       | 31       | 0      |
| Tests                                               | 2865     | 3266   |

The 83 regions that remain in the crate are all under `shared/utils/` and
`transform/stylex/`, which tickets 63, 64 and 71 hold.

### Half the eighty-six were guards over an object the producer built

The ticket reads the group as eleven transformers wanting tests. Measured, the
regions fall into two kinds, and only one of them is a test:

- **Refusals a caller can reach.** A producer given a value that is no object,
  a theme override naming a variable no group declares, a variable group bound
  to no export. These are tests, and they are in
  `tests/transformer_refusals_test.rs`, one module per question rather than one
  file per producer, because it is the same question asked of eleven producers.
- **Guards over an object the producer itself wrote a few lines earlier.** Five
  producers built a map of `Rc<FlatCompiledStylesValue>` through `obj_map`, then
  read it back and asked what kind of value each entry held. No source reaches
  any of those arms. This is the same class ticket 71 records for
  `src/transform/stylex/`, and it took the same answer -- `RUST.md` answer 2,
  give the step a type with exactly the states its producer can build:

| Producer              | Now holds                            | Guards gone |
| --------------------- | ------------------------------------ | ----------- |
| `keyframes`           | `IndexMap<String, Vec<Pair>>`        | 4           |
| `positionTry`         | `Vec<Pair>`                          | 7           |
| `viewTransitionClass` | `IndexMap<String, String>`           | 6           |
| `defineConsts`        | one pass, no intermediate map        | 3           |
| `defineVars`          | one pass, `@property` written at source | 5        |
| `createTheme`         | `ThemeVars`, settled once by the validator | 9      |

`obj_map`, `ObjMapType`, `obj_map_keys_key_value` and `Pipe` have no caller
left and are deleted with them, and so is `FlatCompiledStylesValue::CSSType`,
which nothing writes now that the `@property` rule is built where the type is
read.

### Three decisions to judge

1. **The single-entry arm of `PreRuleSet::create` was deleted, not tested.**
   The ticket names "the empty and single-entry cases". The empty case is a
   test. The single-entry case was `match flat_rules.first() { Some(rule) =>
   rule.to_owned(), None => panic }`, and the `None` cannot be entered when
   `len()` is 1, so it is `RUST.md` answer 4: the arm goes and
   `swap_remove(0)` moves the one rule out instead of copying it. The
   single-entry behaviour itself is still asserted.
2. **`convert_number_to_string_using` was reshaped.** `RUST.md` says never to
   reshape a signature to move a region, and names "a parameter that every
   shipped caller fills with the same constant" as one of the two shapes that
   does. The function had exactly that parameter -- `default_str`, which all
   three callers filled and nothing read -- so removing it, and the
   `Rc<dyn Fn>` the function allocated per call, follows the rule rather than
   breaking it. The per-instantiation group closed as well.
3. **Three crates outside the three directories changed.** Two message
   constants in `stylex_constants` and one value in `stylex_state` lost their
   last reader here, and `as_identifier` moved to `stylex_utils` because
   `defineVars` and `defineConsts` wrote the same seven lines. None of this is
   new behaviour; it is what the deletions left behind.

### Parity was measured, not argued

`pnpm run parity:probe` from `crates/stylex-rs-compiler`, every rewritten
producer, against the reference plugin:

| Shape                                                        | Result |
| ------------------------------------------------------------ | ------ |
| `keyframes` with a logical value, and with a repeated step    | identical |
| `viewTransitionClass`, three parts, one declaring nothing     | identical |
| `positionTry` `{ insetInlineStart, top }`                     | identical, doubled form kept |
| `positionTry` `{ float }`, `{ marginStart }`, `{ start }`     | both refuse, same message |
| `defineVars` with an at-rule, a CSS type, a `--` name, a digit-leading name | identical |
| `defineConsts` with a template, a `--` name, a digit-leading name | identical |
| `createTheme` with `default` and two at-rules                 | identical, same rule order |
| `create` with a repeated property and a null value            | identical |

### The review found two defects the tests did not

- `keyframes` lost the step dedupe the old `obj_map` gave it, because the
  rewrite held the steps in a `Vec`. No source reaches it -- the evaluator
  folds a repeated step before the producer sees it, measured with
  `parity:probe` -- but the producer's own contract had changed. It holds an
  `IndexMap` again.
- `doubled_css_text` asked `unwrap_or_default()` for the repeated property
  name, which is the substitution `RUST.md` forbids where the value reaches a
  folded answer. The repeat is written from the resolver's key now, and what
  goes unchecked is recorded in place.
