# 70 — Read `firstThatWorks` inside a keyframes step

**What to settle:** A `firstThatWorks` call written as a step value in
`stylex.keyframes` drops the declaration. The step survives, empty, and nothing
says why.

```js
import * as stylex from "@stylexjs/stylex";
export const fade = stylex.keyframes({
  from: { display: stylex.firstThatWorks("grid", "flex") },
  to: { display: "block" },
});
```

```js
_inject2({
  ltr: "@keyframes x1t391ty-B{from{}to{display:block;}}",
  priority: 0,
});
export const fade = "x1t391ty-B";
```

`from{}` — the `display` declaration is gone. The named spelling
(`import { keyframes, firstThatWorks }`) answers the same, so this is not an
import question.

**Why it looks unintended.** `transform_stylex_keyframes_call` builds a function
map for exactly this: it registers `firstThatWorks` under every local name a
named import gave it, and under `stylex.firstThatWorks` as a member of each
namespace import. A registration nothing reads is either a step the compiler
stopped taking or a map entry that was never needed.

The same call inside `stylex.create` folds and produces the fallback pair --
`create_within_first_that_works_and_rtl` records `float:left;float:left` with an
`rtl` beside it -- so the helper itself works. It is the keyframes position that
drops it.

**The reference was run, and it answers the same.** Its
`transform-stylex-keyframes-test.js` has no case for a fallback list in a step
value, so the corpus said nothing either way, and `parity:probe` was used
instead. `@stylexjs/babel-plugin` 0.19.0 writes the same empty `from{}` and the
same `x1t391ty-B` for the module above, and for the named, aliased, shorthand
and single-value spellings beside it. So the drop is what both compilers do,
and it cannot be changed here alone: the animation name is a hash of the rule
text, so a step that declared something would answer a name upstream never
writes.

**The registration is read, and a step key shows it.** A fallback list in a
step **key** is not dropped: a key is a selector list, so
`[firstThatWorks('0%', '50%', '100%')]` comes out as `100%,50%,0%` -- the
helper reorders what it is handed -- and the step declares what it holds.
Upstream answers the same. That position is what shows the folded list itself,
which the value position cannot: there the list is dropped, so the output is
the same whatever it held.

**Status:** resolved

- [x] The module above agrees with the reference implementation. Nine spellings
      were measured with `parity:probe`, and every one of them answers the same
      animation name and the same rule text as `@stylexjs/babel-plugin` 0.19.0.
- [x] A fallback list in a keyframes step declares nothing, and that is the
      answer both compilers give. It is now recorded by
      `tests/transform_stylex_keyframes_test/fallback_lists.rs`, beside the
      step-key position, which does declare.
- [x] The registration is read. Deleting the named-import arm was tried, and
      every case that spells the helper by a local name then fails with
      `Only static values are allowed inside of a keyframes() call.`, because
      an argument the map does not know stops the whole call from folding
      rather than dropping one declaration. The two step-key cases add what the
      step-value cases cannot: they show what the registration folds *to*,
      since a key is printed rather than dropped.

## Comments

Found while closing [64](./64-cover-the-transform-call-handlers.md).

### What was measured

`parity:probe` on 2026-09-16, `@stylexjs/babel-plugin` 0.19.0,
`runtimeInjection: true`. Every row below is the same in both compilers.

| Source                                     | Answer                          |
| ------------------------------------------ | ------------------------------- |
| `display:` a list                          | `from{}`, `x1t391ty-B`          |
| the named, aliased and twice-bound of it    | the same name                   |
| `display: ['grid', 'flex']`                | the same name -- it is the array |
| a list beside `opacity: 0.5`               | `from{opacity:.5;}` -- only the list drops |
| a list of one value                        | the same name                   |
| `margin:` a list                           | `from{}` -- every longhand drops |
| `[firstThatWorks('from','to')]`            | `to,from{display:grid;}`        |
| `[firstThatWorks('0%','50%','100%')]`      | `100%,50%,0%{opacity:.5;}`      |
| a twenty-value list as a key               | all twenty, joined              |

### The region 64 could not close is already closed

`scripts/coverage-missing.sh -p stylex_transform` names no region in
`transform_stylex_keyframes_call.rs`. The `ImportKind::FirstThatWorks` arm is
reached by `keyframes_folds_a_first_that_works_call_under_its_imported_name`,
which arrived with the view-transition-class fix after this ticket was filed.
What this ticket adds is the answer that arm produces, which nothing recorded.
