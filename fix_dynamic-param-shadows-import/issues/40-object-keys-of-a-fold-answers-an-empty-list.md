# 40 — `Object.keys` of a fold answers an empty list

Status: `resolved`
Blocked by: None

**What was found:** `Object.keys` of the folded namespace map answers `[]` where
the reference implementation answers its keys, and the same compiler spreads
those keys correctly one function away.

```js
export const styles = stylex.create({
  a: { fontFamily: `x${Object.keys(stylex)}y` },
});
```

| | |
| --- | --- |
| Babel 0.19.0 | `font-family:xwhen,envy` |
| here | `font-family:xy` |

Found while measuring ticket 23's second bridge. Not caused by it: the arm has
answered this way as long as the receiver normalizer has existed, and the row
only became *visible* when the template evaluator stopped dropping every
interpolation it could not spell — before that both halves were swallowed.

## The mechanism

Three places read "what own enumerable properties does this value have", and the
folded map is classified differently in each:

| reader | answer for a folded map |
| --- | --- |
| `nodes::object_expression`'s spread arm | its keys, via `function_fold_to_object` |
| `helpers::evaluate_result_to_js_object` | an object, since ticket 23 |
| `helpers::normalize_js_object_method_args` | falls to `_ => None`, then to `NoOwnKeys` |

The third has no arm for `FunctionConfigMap` at all, so the receiver reads as
"not an object" and `Object.keys` answers the empty list — which is the one
answer that is neither a refusal nor the truth. `Object.values` and
`Object.entries` share the normalizer and read the same way.

## Why it is filed rather than fixed

`function_fold_to_object` already produces the object the first reader uses, so
routing the normalizer through it is a small change. What is not small is
deciding what the *values* of those keys should be — a spread of the fold
refuses at the first value that is not a style value, and `Object.values` of it
would have to answer something for a function. That is the same question ticket
15 left open for the rest of the fold family, and it wants measuring across all
three methods rather than patching the one row that showed.

- [x] `Object.keys`, `Object.values` and `Object.entries` of a fold are each
      measured against upstream, and against a single config as well as a map
- [x] The three readers above answer through one classification, or each says
      why its question differs
- [x] Corpus rows for whichever verdict is decided

## What was done

`normalize_object_method_receiver` reads a fold through
`function_fold_to_object`, the same object form the spread arm and the member
read already read it through. Three readers, one classification.

The question the ticket left open -- what the *values* of those keys should be --
answered itself once the receiver was classified: the object form already carries
a value per key, so `Object.values` and `Object.entries` of a fold need nothing
of their own. A single config answers `fn`, which is this compiler's internal
shape, and upstream answers `fn` too; they happen to spell it alike.

Measured across every receiver the evaluator can hand the three methods -- a
map, a single config, an alias, a spread, a shadowed namespace, a theme
reference, each primitive, an object, an array, an array with a hole, and
non-ASCII keys. Pinned in `transform_stylex_create_test::object_own_keys`.

### The three divergences the measurement turned up

- **The fold's contents.** Upstream's namespace carried `env` beside `when` and
  this one carried `when` alone, so the keys list was one short. Not this
  classification -- the spread reader had always answered the same way. Filed as
  [41](./41-env-is-absent-from-the-namespace-fold.md) and since fixed there, so
  the row is `identical`.
- **A nullish receiver.** `Object.keys(null)` folded to `[]`, where the language
  throws -- quietly, in every position a key list can be read from. It refuses
  now, with the `TypeError` the language raises rather than a sentence about
  arrays, so both compilers refuse the same input with the same words. Fixed
  here rather than filed: it is the same classification, and the sentence is the
  whole of what a refused build hands an author.

  Worth recording how nearly it was filed wrong. The first measurement said the
  refusal was swallowed downstream -- a template answering `xy`, a `length` read
  answering `0` -- and a ticket was written against that. It was a stale `dist`:
  the parity harness loads the built binary, not the Rust sources, and the build
  predated the arm. Rebuilt, every position refuses. The README says so in as
  many words, and it cost a ticket to relearn.
- **`Object.values` and `Object.entries` of a single config.** `Object.keys` of
  that receiver folds and agrees -- both answer `fn` -- and the value at that key
  is the function the config wraps, whose `ToString` is source text this
  evaluator does not keep. Upstream answers the source text of its *own*
  implementation, hashing a class name off text no author wrote and no two
  versions spell alike, so this refuses instead. Pinned as
  `invalid_values::object_values_of_a_single_config_is_refused` and its `entries`
  twin, and recorded as `modules-1266-object-values-of-a-single-config` -- the
  verdict was asserted here before anything held it, which a review caught.

`Object.keys` of an array with a hole is divergent in this compiler's favour --
`['1']` is what the language says, and upstream aborts the module.
