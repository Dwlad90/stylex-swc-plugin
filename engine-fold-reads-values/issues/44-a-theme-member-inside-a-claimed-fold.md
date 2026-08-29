# 44 — A theme member read inside a call the fold claimed

**What to build:** A `defineVars` member read inside an expression the fold
claims compiles to the variable reference it always did, rather than failing the
build.

```js
import { colors } from './tokens.stylex.js';
boxShadow: [colors.glow, '0 0 1px'].join(' ')
```

`@stylexjs/babel-plugin@0.19.0` folds this to `box-shadow: var(--x1savphn) 0 0
1px`. This branch fails with `Cannot fold 'join' at compile time. Its receiver
or one of its arguments is not in a form the compiler can evaluate.`

**Where the hand-back is too late.** A theme reference crosses the bridge as the
string its own `toString` answers, which has none of the group's members — so
`fold` refuses the whole expression when it sees `carried_a_theme_reference()`
*and* `read_a_property_as_a_value()`, and does so as `Decline::NotACandidate`
precisely so the dispatch below can resolve the member itself. That much is
right. What is wrong is the order: the array's own admission has already
answered `Decline::rule(..)` for the member, and a rule reaches the caller as a
refusal rather than as a hand-back, so the dispatch never gets its turn.

The same read outside a call compiles here today — `color: colors.glow` is fine
— which is what says the fold is what broke it.

**What it turned out to be, measured.** The hand-back is not late: `fold` does
answer `NotACandidate` and the dispatch does get its turn. The dispatch has
nothing left to answer with — `Array.prototype` moved into the engine, so a
`join` arriving below the fold reads `unfoldable_call` and fails the build. So
the fix is not an ordering one: the fold has to *answer*, which means the group
has to cross carrying its members.

**What was built.** A group stores no members — every name is derived from the
group's identity as it is read — so what crosses is that identity and a proxy
over it, which is the same arrangement the reference implementation holds a group
in. One Rust function derives a member's name and both sides call it, so the
engine and the evaluator cannot answer the same read differently. The
`read_a_property_as_a_value` rule is deleted with the string crossing it existed
to make safe.

One thing the stand-in cannot work out for itself: a chain of two or more names
is a single token — `colors.brand.primary` names `brand.primary` — and which
chains those are is a question about the source rather than about any value. The
guard reads them off the source with the same two helpers the dispatch below
uses, under the name each chain was read through and only where the guard's own
scope does not bind it, and names them for the group.

The outward walk gained one arm it did not have: a group *inside* an answer
converts to the text it answers for itself. Not scope creep but the cost of the
crossing — `Array(colors, 1, 2, 3).length` answers an array holding the group,
and before this the group was already a string by the time it got there. The
group standing *alone* as the answer is still handed back, which is what
`Object(colors).primary` needs.

**Blocked by:** none.

**Status:** resolved

- [x] The module above emits the rule the reference implementation emits, with
      treeshake compensation on and off
- [x] The member read is admitted at every depth a value can sit at: an array
      element, a template hole, an object value, and an argument
- [x] A theme *group* carried as a value still folds as it does now — the group's
      own hash, not a member
- [x] Where the fold genuinely cannot answer, it hands back rather than refusing,
      and the value the dispatch produces is what reaches the stylesheet

**Measured.** Thirty-seven expressions over a theme group — members at every
depth, computed keys including one a callback is handed, dotted token paths, the
group as a value, and the four questions that ask what a group *holds* — answer
what `@stylexjs/babel-plugin` 0.19.0 answers, class name and rule text alike. The
one row that still differs is `[vars.primary].join('').repeat(2)`, which the
allocation ceiling refuses for a receiver-set length; `['a'].join('').repeat(2)`
refuses identically, so it is that rule and not this one.

**Found while resolving:** [30](./30-a-speculated-read-leaves-no-import-behind.md)
