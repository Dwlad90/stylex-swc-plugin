# stylex-js

Predicates and coercions over JavaScript semantics, asked while deciding whether
an expression is safe to evaluate at compile time and what it evaluates to. The
predicates answer one yes/no question about an AST node, or read a name off one;
the coercions answer what the language says a value converts to. Neither kind
transforms anything.

The three live in separate modules -- `helpers`, `coercions` and `operators` --
and none is re-exported from the crate root, so an import site says which kind
of helper it is reaching for.

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
that type is refused, and the caller
[deopts](../stylex-transform/CONTEXT.md) rather than inventing one. `NaN` is not
that case -- it is a value the language produces, and it is returned rather
than refused.
_Avoid_: conversion, cast, stringify, formatting

**Binary operator**:
One of the operators that reads two numbers and answers a number -- the four
arithmetic ones, `%`, `**`, the three bitwise ones and the three shifts. A
comparison or a logical operator is not one: those answer a boolean or pick a
side, and a different reader handles them. `evaluate_bin_expr` panics on
anything outside the set rather than guessing, because an operator arriving
there is a fault in the caller and not in the source being compiled.
_Avoid_: arithmetic, binary expression, math operator

**Streamed coercion**:
The two coercions that reach their answer through a _string_ -- `ToString`, and
`ToNumber` of anything that is not already a number -- written into a
[sink](#sink) piece by piece rather than handed back whole. An array is why: its
string is the join of every element, each rendered before the join copies them
all again, so a caller with a bound that read the finished join had already paid
for the whole of it. Every writer of one calls `write_piece`, so a sink's refusal
is lifted into the coercion's own two endings in one place.

A number's reading also says where its answer came from: the value's own number
where it has one -- a number, a boolean, `null`, an object with an own `valueOf`
-- and otherwise the number of the text the sink was given.
_Avoid_: incremental conversion, chunked coercion, callback coercion

**Sink**:
Where a [streamed coercion](#streamed-coercion) writes what it is building, one
piece at a time, and the only thing allowed to refuse a piece. A plain `String`
refuses nothing, which is what a caller collecting the whole answer wants; a
caller with an [allocation
ceiling](../stylex-structures/CONTEXT.md) to spend measures each piece as it
arrives and refuses the one that passes it.

The number sink is the one that also _stops caring_. `ToNumber` keeps no string,
so its sink drops the text at the first character no numeric literal can hold and
answers `NaN` from there -- and the test for that is deliberately **sound rather
than exact**: every character a numeric literal can hold is admitted, so a
rejection proves the whole text is `NaN` however it continues.
_Avoid_: writer, buffer, accumulator, visitor

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
