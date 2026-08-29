# 45 — A global reached as a callback argument

**What to build:** A callable global handed to a method as its callback folds,
or declines with a sentence about itself.

```js
fontFamily: ['Arial', false].filter(Boolean).join(', ')
```

`@stylexjs/babel-plugin@0.19.0` folds this to `font-family: Arial`. This branch
fails the whole declaration with `Referenced constant is not defined.` — the
fall-through's sentence for a name nothing resolved, which is not what happened:
`Boolean` is a global the engine holds, and the guard simply does not admit it
in an argument position.

`admit_an_applied_global` admits `Boolean(x)` as a *call*. A bare `Boolean`
standing where a function belongs is a different shape and reaches
`admit_value`'s `Expr::Ident` arm, which asks the module for it, gets nothing,
and answers `Decline::NotACandidate`. The dispatch below then owns a call it
cannot fold either, and the sentence an author reads names a constant they never
wrote.

**Two things to decide separately.** Whether the global should be admitted —
`Boolean`, `Number`, `String` and `Array` are all held by the engine and would
print and run unchanged — and, whichever way that goes, what the refusal says. A
call the guard recognised and declined is supposed to name the rule that
declined it; this one borrows a sentence about something else.

**Blocked by:** none.

**Status:** resolved

- [x] `[…].filter(Boolean)` folds, or declines with a sentence naming the global
      rather than a missing constant
- [x] The other callable globals the engine holds are answered on the same terms,
      whatever those terms are
- [x] A global *shadowed* by a module binding is still the module's, as it is
      everywhere else
- [x] `Boolean` as a callback and `Boolean(x)` as a call agree about whether they
      fold

**Found while resolving:** [30](./30-a-speculated-read-leaves-no-import-behind.md)

**Resolved by:** the guard's value walk. A name that is one of the globals the
fold recognises — the five `VALID_CALLEES` plus the new `VALUE_ONLY_GLOBALS`,
which holds `Boolean` — and that no module binding shadows now refuses with
`global_as_a_value`, which names it. The rule is answered from the source, so it
sits in front of the resolution rather than among the arms that read one. Every
binding shadows, as it does for a callee, so a `const`, a `function` or an
import of the spelling is answered by the rules around it.

**What the rule does not claim.** A global the fold folds nothing of —
`parseInt`, `isNaN`, `Symbol` — is still handed back and still reads as a
missing constant. Naming those would mean writing down a list of globals
nothing else in the compiler uses, so the set stays the one the fold already
knows. The sentence names what a fold *does* answer for a global rather than a
position that would work, because `Boolean(x)` does not fold either and
pointing at the call would send an author to a second refusal.

**Measured, and the ticket's premise was wrong.** `@stylexjs/babel-plugin@0.19.0`
does *not* fold `['Arial', false].filter(Boolean).join(', ')`; it fails with
`Unsupported expression: CallExpression`. It folds no callable global as a
callback — `Number`, `String`, `Array`, `Object` and `Object.keys` all refuse —
and refuses `Boolean(x)` as a call as well. So the fold stays refused in both
compilers on every shape here, and only the sentence changed. The reference
compiler's own wording varies with the outermost node rather than with the
global, so there was no string to match, and the ticket's second option is the
one taken.
