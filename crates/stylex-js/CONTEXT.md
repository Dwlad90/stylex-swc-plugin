# stylex-js

Predicates over JavaScript semantics, asked while deciding whether an expression
is safe to evaluate at compile time. Every function here answers one yes/no
question about an AST node; none of them transforms anything.

## Language

**Valid callee**:
A call target the compiler is willing to evaluate — the members of
`VALID_CALLEES`, not an arbitrary function. Anything else makes the surrounding
expression unevaluable.
_Avoid_: allowed function, safe call, whitelisted callee

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
