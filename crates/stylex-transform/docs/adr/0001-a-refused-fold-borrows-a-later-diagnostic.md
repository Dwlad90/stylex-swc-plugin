# A refused fold borrows the diagnostic it would have reached

**Status:** accepted

A [refused fold](../../CONTEXT.md) normally reports what the fold itself could
not do: an argument it cannot coerce, a length that is not one. Two refusals in
the callable-global folds instead raise a diagnostic that belongs further down
the pipeline, naming a problem with the _value_ rather than with the call.

`Object(primitive)` is one. The argument is a boxed wrapper object, and that
wrapper's only observable effect anywhere in a stylesheet is the style-value
rejection — it is not an array, a string or a number, and no operation turns it
into one. So the refusal _is_ that rejection, raised at the fold.

A function argument joins it. `ToObject` returns a function unchanged, so the
identity is the faithful coercion — but this evaluator's function values reduce
to whatever the function returns, so folding `Object(() => 'red')` faithfully
emitted `color:red` where the reference implementation fails the build. Wrong
output is the one outcome worth diverging from the letter of the coercion to
avoid, so the coercion reports a function apart from the other objects and the
fold refuses it at the same rejection a wrapper reaches.

A bare `Math(x)` is the second, and the opposite case: there is no value at all,
so it names the callee.

## Considered options

**Represent the boxed wrapper in the evaluator's value type.** The faithful
model, and the one most likely to be proposed again, because the fold currently
raises an error about a value it never built. Rejected because the wrapper would
thread a new variant through every `match` in the evaluator and the style
pipeline — each of which must then decide what a wrapper means — for the sole
purpose of delivering the same message by a longer route. Nothing consumes a
wrapper: no valid program reaches the path, and no coercion of one produces a
value a stylesheet accepts.

**Refuse with the fold's own wording instead of borrowing.** Rejected because
the author's problem is the value, not the call. `Object('red')` is a perfectly
good call; what fails is that its result cannot be a style value, which is what
the borrowed message says.

**Mirror the reference implementation's bare-`Math` behaviour.** It leaks a
`TypeError` from inside its own evaluator — a null dereference, not a designed
error. Rejected as a defect rather than a contract; only the observable outcome,
that the program does not compile, is preserved.

## Consequences

**A missing wrapper representation is a decision, not an oversight.** Anyone
reading the `Object` fold will find an error raised where a value would be
expected, and this is the reason.

**The reference implementation's wording is not matched everywhere.** In
`defineVars` it reports the missing default the wrapper leaves behind, and this
compiler reports its own refusal one step earlier. Both fail the build; only
that is common ground.

**The coercion tells a function apart from other objects even though the fold
treats them alike.** The distinction is load-bearing — it is what prevents the
wrong-output case above — and lives in the coercion because it is a fact about
the language, not about this fold.
