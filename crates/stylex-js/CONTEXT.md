# stylex-js

Predicates and coercions over JavaScript semantics, asked while deciding whether
an expression is safe to evaluate at compile time and what it evaluates to. The
predicates answer one yes/no question about an AST node, or read a name off one;
the coercions answer what the language says a value converts to. Neither kind
transforms anything.

The two live in separate modules -- `helpers` and `coercions` -- and neither is
re-exported from the crate root, so an import site says which kind of helper it
is reaching for.

The evaluator in `stylex-transform` is the consumer, and these are the only
copies -- a change to the set of foldable callees lands in one place.

## Language

**Valid callee**:
A call target the compiler is willing to evaluate — the members of
`VALID_CALLEES`, not an arbitrary function. Anything else makes the surrounding
expression unevaluable.
_Avoid_: allowed function, safe call, whitelisted callee

**Coercion**:
What ECMAScript says a value converts to when another type is asked for --
`ToString`, `ToNumber`, `ToBoolean` and `ToObject` over an already-evaluated
expression.
Answers only what the language answers: a value with no compile-time form of
that type gets `None`, and the caller
[deopts](../stylex-transform/CONTEXT.md) rather than inventing one. `NaN` is not
that case -- it is a value the language produces, and it is returned rather
than refused.
_Avoid_: conversion, cast, stringify, formatting

**Object coercion**:
Which kind of object `ToObject` answers with, reported rather than carried out:
a function, or every other object -- one the value already is, a boxed wrapper
for a primitive, and the fresh object the nullish pair takes. Reported this
coarsely because its one caller is `typeof`, which tells a function from
everything else and nothing else does; `Object(x)` is folded by the engine,
which answers with a real object rather than with a name for one.
_Avoid_: boxing, object conversion, wrapping

**Nullish**:
`null`, `undefined` and `void x` -- the values `??` takes its right side for. A
plain question about an expression rather than a coercion, and it sits beside
them because that is what `??` asks before it asks for a boolean. Answered as a
plain no for a value the crate cannot read, since both spellings are syntax it
always recognises.
_Avoid_: empty, falsy, missing, absent

**Mutation expression**:
An expression that writes through a member — `a.x = 1`, `++a.x`, `delete a.x`.
Recognised because a value that is mutated after its declaration cannot be read
from its initializer.
_Avoid_: side effect, write, assignment

**Mutating method**:
A method known to mutate its receiver: `push`/`splice` on arrays,
`Object.assign` and friends on objects. Tracked apart from mutation expressions
because the syntax gives nothing away — the call looks the same as a pure one.
_Avoid_: impure method, unsafe method

**Invalid method**:
A method the compiler refuses outright, regardless of mutation —
`INVALID_METHODS`.
_Avoid_: banned method, unsupported method
