# A refused fold borrows the diagnostic it would have reached

**Status:** superseded by
[0008](./0008-the-fold-guard-reads-values-and-the-engine-is-permanent.md)

The borrowing this recorded is gone, and the body that described it has been
deleted rather than left to be believed: it named a dispatch that no longer
exists. `String(x)`, `Number(x)`, `Array(n)` and `Object(x)` are native
JavaScript functions and are folded by being called, so `Object('red')` builds a
real wrapper object inside the engine and is refused on the way _back_ in the
fold's own words — `Cannot carry a folded object back from the engine` — rather
than by raising the style-value rejection early. A function argument is the same.

The option the deleted body rejected — representing the boxed wrapper in the
evaluator's value type — is moot for the same reason: the wrapper is the engine's
own object and never crosses the bridge, so there is nothing to represent.

## What survives

**A global that only contributes methods is refused by name.** `Math` is a valid
callee because its statics fold, so `Math(1)` reaches the fold with everything
else in place, and the reference implementation answers it by leaking a
`TypeError` from inside its own evaluator — a null dereference, not a designed
error. That is a defect rather than a contract, so only the observable outcome is
preserved: the program does not compile, and this compiler says why.

**Whether a global can be applied is asked of the language.** The global object
holds the value and the value says, so there is no list of names here to fall
behind the language. The engine's own sentence for applying a non-function is
`not a callable function`, which names neither the global nor the mistake, so the
refusal is this compiler's.

## Consequences

**The reference implementation's wording is not matched everywhere.** Both
compilers reject these programs; only that is common ground. Message text is not
a parity obligation — the comparison harness compares class name, rule text and
style-object shape, never sentences.
