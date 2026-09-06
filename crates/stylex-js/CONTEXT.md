# stylex-js

Predicates and coercions over JavaScript semantics, asked while deciding whether
an expression is safe to evaluate at compile time and what it evaluates to. A
predicate answers one yes/no question about an AST node, or reads a name off
one; a coercion answers what the language says a value converts to. Neither kind
transforms anything.

The three modules — `helpers`, `coercions` and `operators` — are not re-exported
from the crate root, so an import site says which kind of helper it reaches for.
[stylex-evaluator](../stylex-evaluator/CONTEXT.md) is the consumer. The named
sets themselves live in [stylex-constants](../stylex-constants/CONTEXT.md).

## Language

**Valid callee**:
A call target the compiler is willing to evaluate — a member of `VALID_CALLEES`,
not an arbitrary function. Anything else makes the surrounding expression
unevaluable.
_Avoid_: allowed function, safe call, whitelisted callee

**Coercion**:
What ECMAScript says a value converts to when another type is asked for —
`ToString`, `ToNumber`, `ToBoolean` and `ToObject` over an already-evaluated
expression. It answers only what the language answers: a value with no
compile-time form of that type is refused, and the caller
[deopts](../stylex-evaluator/CONTEXT.md) rather than inventing one. `NaN` is not
that case — the language produces it, so it is returned rather than refused.
_Avoid_: conversion, cast, stringify, formatting

**Binary operator**:
One of the twelve operators that read two numbers and answer a number: the four
arithmetic ones, `%`, `**`, the three bitwise ones and the three shifts. A
comparison or a logical operator is not one. `evaluate_bin_expr` panics on
anything outside the set rather than guessing, because an operator arriving
there is a fault in the caller and not in the source being compiled.
_Avoid_: arithmetic, binary expression, math operator

**Streamed coercion**:
The two coercions that reach their answer through a _string_ — `ToString`, and
`ToNumber` of anything that is not already a number — written into a **sink**
piece by piece rather than handed back whole. Every writer calls `write_piece`,
so a sink's refusal is lifted into the coercion's two endings in one place. A
number's reading also says where its answer came from: the value's own number
where it has one, and otherwise the number of the text the sink was given.
_Avoid_: incremental conversion, chunked coercion, callback coercion

**Sink**:
Where a streamed coercion writes what it is building, one piece at a time, and
the only thing allowed to refuse a piece. A plain `String` refuses nothing; a
caller with an [allocation ceiling](../stylex-structures/CONTEXT.md) to spend
measures each piece as it arrives and refuses the one that passes it.

The number sink also _stops caring_: `ToNumber` keeps no string, so it drops the
text at the first character no numeric literal can hold and answers `NaN` from
there. That test is **sound rather than exact** — every character a numeric
literal can hold is admitted, so a rejection proves the whole text is `NaN`
however it continues.
_Avoid_: writer, buffer, accumulator, visitor

**Object coercion**:
Which kind of object `ToObject` answers with, reported rather than carried out:
a function, or every other object. Reported this coarsely because its one caller
is `typeof`, which tells a function from everything else and nothing else does.
_Avoid_: boxing, object conversion, wrapping

**Nullish**:
`null`, `undefined` and `void x` — the values `??` takes its right side for. A
plain question about an expression rather than a coercion. Answered as a plain
no for a value the crate cannot read, since all three spellings are syntax it
always recognises.
_Avoid_: empty, falsy, missing, absent

**Mutation expression**:
An expression that writes through a member — `a.x = 1`, `++a.x`, `delete a.x`.
Recognised because a value that is mutated after its declaration cannot be read
from its initializer.
_Avoid_: side effect, write, assignment

**Mutating method**:
A method known to mutate its receiver. This crate answers for the object half,
`is_mutating_object_method` over `MUTATING_OBJECT_METHODS`; the array half,
`MUTATING_ARRAY_METHODS`, is read directly by
[stylex-transform](../stylex-transform/CONTEXT.md). Tracked apart from a
mutation expression because the syntax gives nothing away.
_Avoid_: impure method, unsafe method

**Invalid method**:
A method the compiler refuses outright, regardless of mutation —
`INVALID_METHODS`.
_Avoid_: banned method, unsupported method
