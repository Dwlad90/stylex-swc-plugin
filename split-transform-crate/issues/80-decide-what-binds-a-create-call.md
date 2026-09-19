# 80 — Decide what binds a `create` call

**What to settle:** this compiler refuses a `stylex.create` call that an object
initialises, and upstream compiles it.

```js
import * as stylex from "@stylexjs/stylex";
export const all = { s: stylex.create({ a: { color: "red" } }) };
```

Here: `[StyleX] create() calls must be bound to a bare variable.` Upstream:
the module compiles, and the rules are injected before the statement.

The message is upstream's own, but the rule behind it is not the same rule.
`validateStyleXCreate` upstream refuses only a call with no parent, or one
whose parent is an expression statement. `validate_stylex_create` here asks
`is_bound_create_expr`, which accepts a declarator, a top-level array by span
containment, and a member access on the call, and refuses everything else.

The two rules disagree on more than the object. Every shape upstream accepts
and this compiler refuses is worth listing before either is changed, because
each is a module that builds there and stops the build here.

**Why it is not urgent.** A refusal is safe. The shape this compiler used to
accept _and_ compile wrongly -- a call inside an object inside a top-level
array -- is closed by [79](./79-inject-the-rules-an-object-in-an-array-declares.md),
and the walk that places the injection would find the object if the validator
let the call through. So the decision is about how strict to be, not about
correctness of the output.

**What to settle:**

- whether the array allowance and the object refusal can stand together. An
  array is accepted on purpose, and an object inside that array is accepted by
  the same span containment, so the rule already admits an object one level in
  while refusing it at the top.
- whether to widen `is_bound_create_expr` to upstream's rule, or to record the
  divergence and say why a stricter rule is wanted.

**Blocked by:** None.

**Status:** resolved

- [x] The shapes upstream accepts and this compiler refuses are listed, each
      with what it prints on both sides. The table is under `## Answer`.
- [x] The rule is widened to upstream's. Both compilers now refuse the same one
      shape, and print the same module for the other fifteen.
- [x] The message a refused shape carries is this ticket's, and it landed
      elsewhere: `07efea9fa docs(stylex_constants): drop eight sentences nothing
      reads` also renamed `export_variable_not_found` to
      `type_asserted_call_value` and rewrote what it says
      (`crates/stylex-constants/src/constants/messages.rs`). That is this
      ticket's work, carried inside a constants cleanup, and it is recorded
      here rather than moved.

## Comments

Found while closing [79](./79-inject-the-rules-an-object-in-an-array-declares.md).
Reading upstream's `validateStyleXCreate` to settle whether that ticket should
inject or refuse showed the two rules are unrelated, which is a larger question
than the one ticket asked.

## Answer

The rule is widened. Both halves of it were unrelated to upstream, so both are
replaced by the question upstream asks: **where is the call written**.

### What was measured

`pnpm run parity:probe` from `crates/stylex-rs-compiler` against
`@stylexjs/babel-plugin` 0.19.0, one `stylex.create({ a: { color: 'red' } })`
in each position a module can hold it. "Before" is the rule this ticket found.

| Position                             | Before               | Upstream, and now |
| ------------------------------------ | -------------------- | ----------------- |
| `stylex.create({…});`                | refused              | refused           |
| `const s = stylex.create({…})`       | in place             | in place          |
| `export default stylex.create({…})`  | in place             | in place          |
| `const { a } = stylex.create({…})`   | in place             | in place          |
| `const all = [stylex.create({…})]`   | in place             | in place          |
| `const all = [[stylex.create({…})]]` | in place             | in place          |
| `const all = [{ s: create({…}) }]`   | in place             | in place          |
| `const all = { s: create({…}) }`     | **refused**          | in place          |
| `Object.freeze(stylex.create({…}))`  | **refused**          | in place          |
| `cond ? stylex.create({…}) : null`   | **refused**          | in place          |
| `s = stylex.create({…})`             | **refused**          | in place          |
| `(0, stylex.create({…}))`            | **refused**          | in place          |
| `() => stylex.create({…})`           | **refused**          | hoisted           |
| `return stylex.create({…})`          | **refused**          | hoisted           |
| `const a = stylex.create({…}).a`     | **hoisted**          | in place          |
| `function f() { const s = create({…}) }` | hoisted          | hoisted           |
| `class C { p = create({…}) }`        | in place             | in place          |

Seven shapes stopped a build that upstream compiles, and one compiled to a
module upstream does not print.

A class field is the one row that is neither. Both compilers leave the object
in the initializer, so it is built once per instance rather than once for the
module. That is what upstream does, so it is kept.

### What replaced it

Upstream reads two things off the parent of the call: whether the parent is an
expression statement, which is the whole of `validateStyleXCreate`, and whether
a statement of the module holds it with no function between, which is
`isProgramLevel`. An SWC visitor carries no parent, so `fill_call_positions`
reads both off the module once and records the spans:

- `is_bare_call_statement` — the one shape upstream refuses.
- `is_program_level_call` — whether the compiled object stays where the call
  was written or is hoisted to a declaration above.
- `is_type_asserted_call` — the printer's own refusal, above.

A namespace and the declaration in a `for` head each count as one level of
their own, because upstream's walk stops at the statement they hold and this
one counts statements.

Three things went with the old rule, because the position answers all of them:
`is_bound_create_expr` and the wrappers it read through,
`pattern_bound_top_level_calls`, and `top_level_array_spans` with the
bookkeeping that kept it in step with the list of top-level expressions.

### The two questions the ticket asked

- The array allowance and the object refusal cannot stand together, and neither
  stands now. Both are a call at program level and both keep their place.
- One refusal is wanted, and it is kept: a call inside a type assertion. The
  printer drops the brackets an assertion needs, so `(x as any).root` comes
  back as `x as any.root`, which reads as an assertion to `any.root`. It is
  recorded as a position of its own rather than as a rule about what binds a
  call, and it costs no build: the shipped compiler strips every type before
  the StyleX pass, which
  `crates/stylex-rs-compiler/__test__/createCallPositions.spec.ts` measures
  through the binding by parsing what it printed.

### Review

Three reviewers read the change before it landed. Four findings were taken:

- A namespace `export` read as program level, because an export declaration is
  a module item rather than a statement and the count never rose. A namespace
  now puts everything in it below program level, which is what the reference
  walk answers for both spellings.
- The declaration in a `for` head was read as program level. It is a statement
  of its own below the loop, and counts as one.
- The two span sets sat on the state manager unshared, where every other index
  is behind an `Rc` because a dynamic style's callback clones the whole state
  per invocation. They are one `Rc<CallPositions>` now.
- The guard over the type assertion was deleted with the rule it belonged to.
  It is back, as a position of its own with a message that says what is wrong,
  and the boundary test measures that no build reaches it.

Landed as `df3474e`.
