# 15 — The function map read where it is not a map

Status: `resolved`
Blocked by: None

**What was found:** Ticket 08 made a folded function map reach namespace
validation from a dynamic style's value position. Four neighbouring positions
read the same fold and still diverged, each for its own reason -- one of which is
owned by [16](./16-a-shadowed-function-import-emits-css-upstream-refuses.md),
and a second of which 16 closed outright (case 3 below). Measured against
`@stylexjs/babel-plugin` 0.19.0 under the parity harness's configuration.

All four share one root: the reference implementation's `identifiers` is a plain
JS object, so *every* entry has a key set and a string coercion. This compiler's
`FunctionConfigType` has four shapes, only one of which (`Map`) carries keys, and
`FunctionConfig` carries none at all.

### 1. A static namespace value reaches a different consumer — **fixed**

```js
export const styles = stylex.create({ a: { height: stylex } });
```

| | |
| --- | --- |
| Babel | `Invalid pseudo or at-rule.` |
| here, before | `a > A style value can only contain an array, string or number.` |
| here, now | `Invalid pseudo or at-rule.` |

The materialization moved out of the create call's dynamic consumer into
`js/evaluate/mod.rs::function_fold_to_object_expr`, which the static object
evaluator now asks the same question. Materializing there was the wider decision
this issue said it was -- `nodes/object_expression.rs` is read by `defineVars`
and `createTheme` too -- so every consumer that reads it was measured, and the
answers are in **What the neighbours read** below. Nothing regressed; two of them
converged with it.

### A fifth position, not among the four

The **spread** is not one of this ticket's four cases, and it is where the one
wrong-output divergence was: `{ ...keyframes, color: 'red' }` compiled
`{color:red}` where upstream refuses. It has the ticket's root exactly -- an
entry with no key set where upstream's has one -- and it was found by measuring
case 1, so it landed here rather than waiting for a ticket of its own. Following
[16](./16-a-shadowed-function-import-emits-css-upstream-refuses.md)'s rule it
gets its own commit, so a bisect lands on the wrong-output fix and not on the
sentence changes around it.

Two things had to be right for the object to carry the answer:

- **One level down.** A map stands for one key per entry, each carrying that
  entry's own object, because the reference implementation registers
  `identifiers[stylex] = { when: stylexWhen }` and `stylexWhen` is an object of
  functions. Flat, the namespace's spread read a function where upstream reads an
  object and the two sentences parted.
- **A function, not `null`, as the placeholder.** A value position refuses on the
  key and never reads the placeholder, so `null` was invisible there. A spread
  copies the entry onto the style object, where `null` is an *absent value* that
  declares nothing -- so `{ ...keyframes, color: 'red' }` compiled
  `{color:red}`, a style object the author did not write, where upstream refuses
  the function it copied. That was the one wrong-output divergence this issue
  turned up, and it is fixed.

### 2. A named import of a function-map entry that is not a `Map`

Split out into [16](./16-a-shadowed-function-import-emits-css-upstream-refuses.md):
it is the one case of the four that emits CSS the reference implementation
refuses, rather than refusing with different words, so it is owned on its own and
does not wait on the rest of this issue.

### 3. A `FunctionConfig` read off the map — **closed**

```js
export const styles = stylex.create({ dyn: (stylex) => ({ height: stylex.when }) });
```

| | |
| --- | --- |
| Babel | `Invalid pseudo or at-rule.` -- `stylexWhen` is an object of the when functions |
| here | `Invalid pseudo or at-rule.` |

Closed as a side effect of
[16](./16-a-shadowed-function-import-emits-css-upstream-refuses.md), and the
reasoning recorded here was wrong. This said reaching upstream's message "means
the when surface carrying its names, not a change at the consumer". It did not:
the marker map behind the config already carries the names, so once the consumer
materialized a single function config the keys were there to refuse.

Pinned in
`validation_stylex_create_test::invalid_values::when_read_off_a_shadowed_namespace_is_refused_as_a_namespace`.

### 4. The fold coerced to a string — **a decided divergence**

```js
export const styles = stylex.create({ dyn: (stylex) => ({ height: `${stylex}px` }) });
export const other  = stylex.create({ dyn: (stylex) => ({ [stylex]: '1px' }) });
```

| input | Babel | here |
| --- | --- | --- |
| template | `height:[object Object]px` | `height:px` |
| computed key | `.x…[object Object]{[object object]:1px}` | `dyn > A style value can only contain an array, string or number.` |
| concatenation | `height:[object Object]` | `Unsupported expression: BinaryExpression` |
| ternary condition | `height:1px` | `A style value can only contain an array, string or number.` |

Decided as a divergence, not fixed: both compilers write or refuse nonsense, and
agreeing means reproducing a coercion neither compiler intends. Measuring the
row turned up two more readings of it -- a concatenation, which is the template
coercion in an operand, and the fold's *truthiness*, which upstream reads as
`true` -- and both are recorded with the other two rather than chased.

Neither half is about the fold alone:

- The **template** half is the template evaluator dropping *any* interpolation
  with no literal form, so a theme reference goes the same way. Filed whole as
  [23](./23-an-interpolation-with-no-string-form-contributes-nothing.md), which
  closed that row: the template evaluator reads `ToString` now, and the fold
  coerces as the object it is upstream, so `height:[object Object]px` agrees.
  The other three readings in the table above are unchanged and still this
  ticket's.
- The **computed key** half is not the dynamic position's: the static namespace
  reads the same way, measured, so the key coercion is what diverges and not the
  body it was written in.

## What the neighbours read

Every consumer that reads `nodes/object_expression.rs`, measured before and
after. Each row is a corpus entry carrying its verdict, except `defineVars` and
`defineConsts`: both hash the file that declares them, and the corpus hands every
subject the same filename, so such a subject refuses for the filename before the
value under test is read. Those two are measured in the Rust suites, where the
filename is a parameter.

| position | Babel | here | verdict |
| --- | --- | --- | --- |
| style value, at any depth, on any property | `Invalid pseudo or at-rule.` | same | both reject |
| whole namespace (`{ a: stylex }`) | `Invalid pseudo or at-rule.` | same | both reject |
| whole namespace, one entry down (`{ a: keyframes }`) | `A style value can only contain an array, string or number.` | same | both reject |
| spread into a style object | `A style value can only contain an array, string or number.` | same | both reject |
| spread of the namespace | `Invalid pseudo or at-rule.` | same | both reject |
| fallback array element | `A style array value can only contain strings or numbers.` | same | both reject |
| `defineVars` value | `Default value is not defined for a variable.` | `Function values in defineVars() must be zero-argument …` | both reject, worded differently |
| `createTheme` override | `Default value is not defined for a variable.` | the same sentence, naming the variable | both reject, worded differently |
| `keyframes` step | emits `@keyframes …{from{}}` | refuses | acceptance divergent |
| `positionTry` fallback | emits the at-rule, declaration missing | refuses | acceptance divergent |
| `viewTransitionClass` part | emits the selector, body empty | refuses | acceptance divergent |
| `defineConsts` value | accepts, emits nothing | refuses | acceptance divergent |

The four `acceptance divergent` rows are decided divergences on the terms the
theme-reference rows in the same four positions were decided: upstream drops the
declaration and says nothing, and a silent drop is not a behaviour worth
reproducing.

The two `defineVars`/`createTheme` rows are a check-ordering difference inside
those calls -- upstream looks for a `default` key before it looks at what the
value holds -- rather than a question about the fold. That is a defence of the
divergence, not of the sentence: `defineVars` reads the placeholder rather than
the key, so the placeholder now decides what an author is told, and the sentence
it decided is further from the input than the one before it. Filed as
[31](./31-the-fold-s-placeholder-decides-a-define-vars-sentence.md) rather than
argued away here, with the review's objection recorded in it -- 16 had put this
consumer out of scope, and the placeholder reached it anyway.

## Found while measuring, not fixed

- **A member the fold has no entry for**, `keyframes.nope` and `keyframes[0]`.
  Upstream reads `undefined` off the object it folded and refuses it as a value;
  this compiler answers `Unexpected error: Could not determine the property being
  accessed.`, which names an internal shape rather than the input. Both refuse,
  so no build is wrong -- but the sentence is. Filed as
  [30](./30-a-member-read-off-a-fold-names-an-internal-shape.md).
- **`defaultMarker`**, which is an index map here and a bare function upstream,
  and reads `Referenced value is not a constant.` in the static position.
  Already owned by
  [21](./21-a-shadowed-default-marker-param-reports-an-internal-shape.md).
- **A condition key that is a lone surrogate** holding the fold. Upstream never
  writes the key down and refuses the fold; this compiler refuses the key's
  encoding first, which is the rule an object spread of a string and
  `charCodeAt` already answer the same way.

## What landed

- `js/evaluate/mod.rs` — `function_fold_to_object_expr` and the two functions
  under it, with the registration each shape mirrors. Unit tests in
  `js/evaluate/tests/function_fold_object_tests.rs`, one per shape, asserting the
  keys *and* whether each carries a function, because both halves are read.
- `nodes/object_expression.rs` — the value arm and the spread arm ask it.
- `core/evaluate_stylex_create_arg.rs` — the dynamic style's value position and
  the namespace position ask it; the local copy of the materialization is gone.
- 30-odd transform tests over the value, namespace, spread and operand
  positions, including the malformed-condition, unclosed-quote, astral,
  lone-surrogate, deep-nesting and many-folds shapes; the neighbour consumers
  pinned in their own suites.
- 17 new corpus entries and two updated verdicts. `--set modules` reports 0
  changed.

- [x] Case 3 fixed, by 16, and the reasoning recorded here corrected
- [x] Cases 1 and 4 are either fixed or recorded as a decided divergence
- [x] Corpus entries carry the verdict each is known to read
