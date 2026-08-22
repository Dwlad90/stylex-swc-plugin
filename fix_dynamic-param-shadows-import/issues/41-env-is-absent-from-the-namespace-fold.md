# 41 — `env` is absent from the namespace fold

Status: `resolved`
Blocked by: None

**What was found:** The folded namespace map carries `when` and not `env`, so
every reader of the fold's own keys answers a one-key list where upstream
answers two.

```js
import * as stylex from '@stylexjs/stylex';
export const styles = stylex.create({
  keys:    { fontFamily: `x${Object.keys(stylex)}y` },
  counted: { width: Object.keys(stylex).length },
  spread:  { fontWeight: `x${Object.keys({ ...stylex })}y` },
});
```

| | Babel 0.19.0 | here |
| --- | --- | --- |
| `Object.keys(stylex)` | `xwhen,envy` | `xwheny` |
| `Object.keys(stylex).length` | `2px` | `1px` |
| spread, then keys | `xwhen,envy` | `xwheny` |

Same for `Object.values` and `Object.entries`, and for the namespace reached by
an alias or shadowed by a dynamic style's parameter — every route reads the same
map.

## Why it is not [40](./40-object-keys-of-a-fold-answers-an-empty-list.md)

40 was the *classification*: the receiver normalizer had no arm for a fold, so
`Object.keys(stylex)` answered `[]`. All three readers go through
`function_fold_to_object` now and agree with each other. This is what the map
they agree on **holds**, which is a different question and one the spread reader
has answered the same way since before 40 — the row was simply invisible while
the normalizer answered the empty list.

`stylex.env` works as a call (`transform_stylex_create_test::env`), so `env` is
reachable; it is registered where a member read finds it rather than as an entry
of the identifier map the fold is built from. Whether that is worth changing
depends on what else reads that map — `nodes/identifier.rs` resolves a bare
`env` through it, and adding an entry changes what a bare reference to `env`
folds to as well as what the keys list says.

Recorded as `modules-1266-object-own-keys-of-a-fold`, which expects `divergent`
so the day the fold gains `env` reports as a changed verdict. Pinned in the
suite as `transform_stylex_create_test::object_own_keys`, whose module comment
names this ticket.

- [x] Whether `env` belongs in the identifier map is decided, not assumed —
      including what a bare `env` reference would then fold to
- [x] The corpus row keeps `divergent`, so the day this moves it reports as a
      changed verdict

## What it cost, measured rather than estimated

Filed `wontfix` first, on the estimate that widening the map's value from
`FunctionConfig` to `FunctionConfigType` would cost more than one key in a
reflection result is worth. Built on request, and the estimate was wrong in both
directions.

**Cheaper than estimated.** The type change is one line and the compiler found
every reader: three sites needed a match arm and four insertion sites needed a
wrapper. Nothing in the 78 sites that name `FunctionConfigType` moved, because
they already held the wider type. The two maps describing the same API surface
now hold the same type, which is a unification rather than a widening — the
member-expression map beside the fold has always held it.

**More expensive than estimated, in a place the estimate did not look.** The
first attempt registered `env` in `apply_stylex_env`, the hook every call that
builds a function map shares. That made a *bare namespace reference* resolve in
four places it previously did not — `keyframes`, `positionTry`,
`viewTransitionClass`, `defineConsts` — because those maps register the namespace
name nowhere else. A bare `stylex` written where a static value belongs stopped
refusing and started materializing into an object whose declaration is then
dropped, which is upstream's behaviour and the one this compiler had four tests
deliberately diverging from. Four loud refusals became silent drops.

So the entry is registered in the `create` path only, by
`register_env_in_namespace_fold`, and the four refusals are intact. The pin
commit before this one is what caught it: without the `when` and `env` pins the
symptom would have been three snapshots moving in the right direction and four
in the wrong one, in the same run.

## What moved

`Object.keys`, `Object.values` and `Object.entries` of the namespace, its
`length`, an index off it and a spread of it are all identical to
`@stylexjs/babel-plugin@0.19.0` now. Two corpus rows moved from `divergent` to
`identical`; the set's `divergent` count went from three to one.

- [x] Whether `env` belongs in the identifier map is decided, not assumed —
      including what a bare `env` reference would then fold to
- [x] The three readers are re-measured together
- [x] The corpus rows' `expected` becomes `identical`

## The estimate that was wrong, kept



The fold's map is `FxIndexMap<Atom, FunctionConfig>`. `env` is a
`FunctionConfigType::EnvObject`, and `FunctionConfig` has no variant that can
hold one — `FunctionType` has `EnvFunction` for a single env *function* and
nothing for the env object. So `env` cannot be put in the fold as it stands.

Putting it there means widening the map's value from `FunctionConfig` to
`FunctionConfigType`, which 78 sites name and which the call-resolution path
reads to find the `when` entry and call it. That is a core type of the evaluator,
widened so that reflection over the namespace object answers a second key.

What the second key buys: `Object.keys`/`values`/`entries` of the namespace, its
`length`, and a spread of it. Nothing an author writes to declare a style, and
nothing that is broken today — `stylex.env.x` resolves through
`member_expressions` and works, and a bare `env` import resolves through
`identifiers` and works.

The missing key is the whole of the divergence, and that was measured rather than
assumed: on the namespace map upstream answers `[object Object]` per value
exactly as this does, so `Object.values` differs only in the count and
`Object.entries` only in the pair. The function-source answer upstream gives is
the *single config* receiver, which is a different row and refused on purpose —
ticket 40 records it.

The alternative considered and rejected outright: insert an `env` key carrying
some harmless `FunctionConfig` so the list matches. That invents a value the
object does not have, so `Object.values` would then answer the wrong thing —
trading a short list for a wrong one, which is the mistake ticket 40 exists to
undo.

Kept as written, because the shape of the error is worth more than the
conclusion was: the count of sites naming a type said nothing about the cost,
and the cost that mattered — a shared registration hook changing what a bare
name resolves to — is not visible from any grep. The rejected shortcut is still
rejected: `Object.values` of the fold answers real values, not invented ones.
