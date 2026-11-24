# A compiler function runs in the engine with the live build state

**Status:** accepted

A compiler function is an entry of the folded function map that takes one
evaluated argument and the build state: `keyframes`, `positionTry` and
`unstable_conditional` are the ones the transform registers. The engine fold now
calls one when the source calls it inside an expression the fold claims, a
callback body included. The function crosses as a native function of the
engine. When the engine calls it, the native function converts the first
argument back to the evaluator's own value, calls the Rust function with the
live `StateManager`, and builds the answer into the engine.

This replaces the earlier decision, recorded in the module doc of
`engine_stylex_functions.rs`, that the function map does not cross because its
functions write into the build.

## Why

The reference implementation runs a callback and every StyleX call in it:

```js
animationName: ['a']
  .map(() => stylex.keyframes({ from: { color: 'red' } }))
  .join();
```

declares `@keyframes xbjjx1r-B{from{color:red;}}` and
`.xsxtzgd{animation-name:xbjjx1r-B}`. A callback that never runs, as in
`[].map(() => stylex.keyframes(…)).join() || 'none'`, declares no rule. This
compiler refused all three shapes with `Cannot fold 'map' at compile time.`

The dispatch below the fold cannot answer them. It evaluates a StyleX call in
the receiver or in an argument first and writes in its value, but a call inside
a callback body has a value only each time the callback runs, and only the
engine runs the callback.

## What was rejected

**Folding the call before the engine runs.** This gives a callback that never
runs a rule it did not declare, and it cannot read a callback parameter.

**Calling the function once per distinct argument and caching the name.** The
evaluator memo already refuses to replay a fold that filed a rule
(`StateManager::note_nested_rule_filed`). A second cache would be the bug that
memo rule exists to stop.

## How the state reaches the engine

A native function of the engine is `'static`, so it cannot hold the `&mut
StateManager` of the fold. The fold opens a scope, `LiveState`, just before the
engine runs and drops it just after. The scope writes a raw pointer to the state
into one private thread-local cell. It keeps the pointer that was in the cell
before, and puts it back when it is dropped. Only one function,
`with_live_state`, reads the cell, and it holds the only `unsafe` block.

The scope is sound for these reasons:

- The cell is thread-local, and a fold runs on one thread. No other thread can
  see the pointer.
- The pointer comes from the `&mut StateManager` that the fold holds. The scope
  takes that borrow and holds it for its whole life (`LiveState<'a>`), so the
  borrow checker stops the fold from using the state while the scope is open.
  The fold opens the scope after the guard walk and drops it before it returns.
- The scope is dropped before the borrow ends, on a normal return and on an
  unwind. Outside a scope the cell is null, and the native function throws.
- A compiler function can evaluate again, and that evaluation can start a nested
  fold. The nested fold reborrows the state from the reference the native
  function made, opens its own scope, and puts the outer pointer back when it
  ends. So two references to the state are never in use at the same time.

## The engine slot

A fold takes the engine of the thread out of its slot and puts it back after
the engine has run. Before this change, the slot stayed borrowed while the
engine ran, so a nested fold would panic on the second borrow. Now the slot is
free while the engine runs. A nested fold finds it empty, builds an engine of
its own, and the outer engine goes back into the slot when the outer fold ends.
The nested engine leaks, as an engine left behind by an unwind already does.
This happens only when a compiler function starts a fold of its own, and no
function the transform registers does that.

## The memo of compiled scripts

The function is an argument of the printed arrow, not part of the printed
source. So two folds that print the same source share one compiled script even
when the name holds a different function in each, and each call gets its own
function. The parameter key needs no change for this.

## Consequences

- A compiler call inside a callback folds, and its rule is declared each time
  the callback runs. Its answer is the same as the reference implementation's
  for the three shapes above and for `positionTry` in a callback.
- A compiler call in an array or an argument of a method call now runs in the
  engine as well, where the dispatch below used to fold it first. The rules
  are the same.
- An argument the bridge cannot carry back, an answer with no engine form, or a
  call with no argument is a throw in the engine, and the fold reports it as a
  refusal.
