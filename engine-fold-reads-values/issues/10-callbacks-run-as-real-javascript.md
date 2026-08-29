# 10 — Callbacks run as real JavaScript

**What to build:** A callback stops being restricted to a shape the guard can
analyse.

Today a callback is admitted only if it is an arrow whose body is a single
expression reading nothing but its own parameters. Adding a statement,
destructuring a parameter, or reading a named value from the module all break
the build. None of those is hard for a JavaScript engine — the restriction
exists because the guard was analysing the arrow itself.

Printed into the same arrow transport, the engine parses the callback and the
restriction disappears.

```js
const unit = 'px';
const sizes = [1, 2, 3];
padding: sizes.map(n => `${n}${unit}`).join(' ')     // free variable
margin:  sizes.map(n => {
  const v = n * 2; return `${v}px`;
}).join(" ")   // block body
```

The separate path that compiles an arrow into a Rust closure is **not**
touched. It exists for dynamic and inline styles and for callbacks that reach
a StyleX function, and a callback of that kind keeps using it.

**Blocked by:** 06.

**Status:** resolved

- [x] A callback with a block body folds
- [x] A callback with destructured parameters folds
- [x] A callback closing over a named value that resolves folds
- [x] A callback receiving the index and the whole array folds
- [~] A callback touching a StyleX function or a dynamic parameter still takes
      the existing closure path, and dynamic styles are unaffected
- [x] A callback closing over a value that cannot be resolved refuses with a
      reason rather than folding to something wrong

## What the fifth box turned out to be

A dynamic parameter is unaffected, measured: the fold declines a name with no
compile-time value, and a dynamic style function compiles beside a folded
declaration in the same object.

A StyleX function is not. There is no closure path left below the fold for an
array method to take -- ticket 06 deleted the hand-written array methods when
`Array.prototype` moved into the engine -- so `a.map(x => firstThatWorks(x,
'serif')).join(',')` refuses where the reference compiler folds it. The guard is
right to hand the call back; what is missing is underneath it, and re-adding a
method table is what this effort exists to avoid. Split out as issue 17 and
pinned by a test, since the divergence predates this ticket.
