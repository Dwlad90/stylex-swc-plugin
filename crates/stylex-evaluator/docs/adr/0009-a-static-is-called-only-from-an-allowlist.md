# A static is called only from an allowlist

**Status:** accepted

A method call on `String`, `Number`, `Math`, `Object` or `Array` is folded only
where that global's own list of static methods holds the name. Every other
static on those globals is refused by name, before the engine sees the call.

This is a list of method names in the compiler, which is the thing
[0008](./0008-the-fold-guard-reads-values-and-the-engine-is-permanent.md)
removed. That record deleted two tables and said why a table is the wrong shape
for a prototype surface: "a table is finite and a language is not", and "the
method nobody listed is the next bug report". Both sentences are still true.
This record says why one list comes back anyway, and what makes it a different
kind of list.

## What the list is for

The deleted tables decided **what a call answers**. They held an implementation
per method, so a method nobody listed had no answer and the call refused. Adding
a method meant writing the language's behaviour again, by hand, and being wrong
about an edge of it.

This list decides **which calls are offered to the engine at all**. The engine
still answers every one of them, with the language's own semantics, so no entry
carries an implementation and no entry can disagree with the language. A method
that leaves the list loses nothing but the offer.

That is the whole of the difference, and it is what makes the two costs
different. A missing entry in a deleted table was a wrong value or a lost fold
whose replacement had to be written. A missing entry here is one refused call
and one line to add.

## Why an allowlist and not a denylist

`Object` carries statics that answer with an object from the prototype chain:
`getPrototypeOf`, `create`, `getOwnPropertyDescriptor`,
`getOwnPropertyDescriptors`, `setPrototypeOf`. Any one of them is a step from a
plain object to `Object.prototype`, whose `constructor` is `Object`, whose
`constructor` is `Function` — which compiles a string into a function. A file
being compiled could therefore run code at build time.

A denylist closes the names it holds. This surface is one where the name nobody
listed is not a missing fold but an open door, and it grows: `Object.groupBy`
and `Math.f16round` are both newer than this compiler. The engine makes that
worse rather than better, because the engine's surface is its own — it carries
`Math.sumPrecise`, which Node 24 does not, so a denylist written against the
host language would not even have the name to list.

So the direction is reversed for this one surface: the list says what is
allowed, and anything new is refused until somebody looks at it.

## What bounds the list

The list is the reference implementation's, name for name — 59 entries over five
globals. It is not a judgement about which methods are safe. Two entries make
that concrete:

- `Math.random` is absent although it reaches no prototype, because a build has
  to answer the same value every time it reads the same source.
- `Math.f16round` is present although no test names it. The engine carries it
  and Node 22 does not, so a case written against it would answer one way here
  and another way there. It is listed because the surface is the language's.

Keeping the list identical is what keeps the two compilers folding and refusing
the same calls, which is the property the parity corpus measures. A name added
here for a local reason would be a divergence that nothing else in the project
can express.

## What this record does not change

The guard still reads values, not syntax. Which _expressions_ qualify is
unchanged — a receiver may be written out, bound by a callback, or named by the
module — and the allowlist is asked only after that, about the one case where
the receiver is an unshadowed global. Inside the list the surface is still the
language's: a listed static needs no entry of its own to fold, and the engine
answers it.

The prototype surfaces — `String.prototype`, `Array.prototype`,
`Object.prototype` — carry no list at all. They are reached through a value the
source wrote, and the property rules refuse the reads that lead off it.

That was written as "there is no door for a list to close", which was too
strong. The property rules read a name the syntax spells or the evaluator
resolves, and a fold that crosses into the engine whole has a third kind of key:
one that is a name only once the engine has run.
[0010](./0010-the-printed-fold-reads-and-calls-through-a-check.md) closes that
one, by making the printed source read through a check rather than through the
language's index operator. The surfaces still carry no list; what changed is
where the property rules are applied.
