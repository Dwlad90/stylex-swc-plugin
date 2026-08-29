# 30 — A speculated read leaves no import behind

**What to build:** The guard's walk through a branch that never runs leaves no
trace in the module it walked, so a value the stylesheet never uses does not put
an import into the output.

```js
import { colors } from './tokens.stylex.js';
const enabled = false;
boxShadow: [enabled && colors.glow, '0 0 1px'].filter(Boolean).join(' ')
```

With treeshake compensation on, `develop` emitted no extra import. This branch
emits `import './tokens.stylex.js';` for `colors.glow` — a value the fold reads
only to decide whether it *could* fold, and which the short circuit then throws
away.

**Two halves, one cause.** `admit_value` walks both sides of `Expr::Bin` and all
three arms of `Expr::Cond`, calling `Reader::resolve` on every leaf — which is
correct for the guard, since it must know whether every leaf carries. But
`speculate` restores only `confident`, `deopt_path`, `deopt_reason` and
`speculating`; `queue_theme_import_if_absent` is unconditional and is never
unwound. `speculating` today suppresses a *diagnostic*, not a side effect.

The second half is worse and shares the cause: a dead-branch leaf naming a
module function answers `Decline::rule(unfoldable_function)`, and the call site
turns a `Rule` into a hard deopt of the whole call with no fallback. So a branch
that cannot run can fail a build that used to compile.

**This is the invariant the lazy evaluator documents as its reason to exist.**
The reference implementation evaluates each side of a logical expression in a
cloned state precisely so a short-circuited dead branch may fail while the whole
expression stays confident. The fold guard is not forking that state; it is
walking through it.

Two answers to price: unwind the queue inside `speculate` — which makes the
guard's speculation genuinely free of effects — or stop resolving the arms of
short-circuiting forms at all and let the operands reach the engine
individually, which is what the spec says already happens.

**Blocked by:** none — can start immediately.

**Status:** resolved

- [x] The module above emits the same imports on this branch as on the merge
      base, with treeshake compensation both on and off — measured on the same
      module with a plain `join`, because `filter(Boolean)` does not fold here
      at all ([45](./45-a-global-as-a-callback-argument.md))
- [x] A dead branch naming a module function no longer deopts the live one
- [x] Whatever `speculate` is made to restore, it restores it on the refusal path
      as well as the success path
- [x] A test pins the import list, not just the CSS — this is a shape where the
      stylesheet is right and the module is wrong
- [x] The conditional form (`?:`) carries the same cases as `&&` / `||` / `??`

## What landed

The second of the two answers: the walk stops resolving the operand the language
never evaluates. `admit_value`'s `Expr::Bin` arm walks its left operand, then
asks `right_operand_runs`, and `Expr::Cond` walks its test, then asks
`arm_that_runs`. Both go through one `deciding_value_of`, which is where the
boundary lives.

Which operand a short circuit reaches is not restated in the guard. The
logical-expression node grew `evaluates_its_right_operand`, a one-line reading
of the `decide` the fold itself uses, so the walk and the fold cannot come to
disagree about which side a build reaches — including on the `??` guard that
tests truthiness where it meant nullishness, which settles nothing and makes
both sides carry. The conditional reads truthiness through
`evaluate_result_to_js_boolean`, the same bridge the conditional node reads.

`speculate` was left restoring exactly what it restored, on both paths, and
three unit cases now pin that rather than leaving it to the absence of a
`return` between the halves. The queue was not made to unwind, because a theme
group *does* legitimately cross a successful fold as the string its own
`toString` answers — `a_live_theme_read_keeps_its_import` is that case, and an
unwinding `speculate` would have failed it.

## The boundary that was chosen

Inside a callback body the module is not read for the decision at all. The
engine binds the callback's own names, so a guard written on a parameter holds a
different value per element and the module's answer for that spelling is a
different binding entirely; pruning on it would leave the surviving operand's
names unbound and the engine reaching for what nothing gave it. So a short
circuit inside a callback walks both sides, as everything did before. Narrower
than the scope check could be — a walk over the operand's free names would
decide more cases — and not worth the machinery until an input asks for it.

The same conservatism leaves a residue of the ticket's second half: where the
walk cannot tell which side runs — `0 ?? pick`, an unreadable guard, a guard on
a callback's own parameter — a dead leaf naming a module function is still
walked and still refuses the call. That is the rule being applied to an operand
the guard has to treat as live, not the bug this ticket names; what makes such a
refusal fatal rather than a hand-back belongs to
[32](./32-a-refusal-falls-through-rather-than-failing.md).

## Where upstream differs

`@stylexjs/babel-plugin@0.19.0` is eager here: it evaluates both sides under
forked states so a dead one may fail without deopting the whole. Measured on
fifteen shapes, the folded values and class names are identical; three things
diverge, and the divergence is this compiler's in each case.

- A dead theme read. Upstream queues `import 'tokens.stylex';` for a token no
  stylesheet holds; this branch does not. Not a new position — this compiler's
  logical node has been lazy since it was written, and `docs`-worthy only in that
  the fold's guard was walking through the laziness rather than with it. The same
  module with no call in it already emitted no such import.
- A dead operand that throws. `['a', false && null.x].join('-')` aborts upstream
  with `Cannot read properties of null (reading 'x')`; here it folds to `a-`.
- `0 ?? x`. Upstream aborts with `Unsupported expression: CallExpression`; here it
  refuses without aborting, which is #1265's position.

## Found and not fixed

`[colors.glow, '0 0 1px'].join(' ')` — a theme *member* read inside an array a
fold claims — fails the build here with `Cannot fold 'join' at compile time`,
where upstream folds it to `var(--x1savphn)`. Unrelated to the short circuit and
present with the guard true or absent; the hand-back for a carried theme
reference happens after the chain has already been refused. Filed as
[44](./44-a-theme-member-inside-a-claimed-fold.md).

`['a', 'b'].filter(Boolean).join(', ')` fails with `Referenced constant is not
defined.` — `Boolean` reached as a callback argument is not admitted, and the
fall-through has no sentence for it. Also pre-existing, filed as
[45](./45-a-global-as-a-callback-argument.md), and the reason the reproduction
in this ticket's own text could not be used verbatim as a test — every case
substitutes a plain `join`.

## Where it is proved

`short_circuited_operands.rs` at the transform seam — twelve cases, class names
and rule text measured against `@stylexjs/babel-plugin@0.19.0`. The pair that
matters is `a_dead_theme_read_leaves_no_import` against
`a_live_theme_read_keeps_its_import`: the same module, the same fold, and the
import list is the only thing that moves.

`short_circuited_walk_tests.rs` carries the fifteen unit cases — every form,
both arms, the leaf that would refuse, the leaf that would throw, the leaf past
the allocation ceiling, the leaf two hundred levels deep, and the callback where
none of it applies.

`logical_expression_tests.rs` pins the operand table itself, and
`speculation_tests.rs` pins what a speculation puts back.
