# 06 — `Object(…)` folds and a bare `Math(…)` call is rejected

**What to build:** The two rejection-shaped callees. Grouped because neither
produces a useful value and both end in a diagnostic; kept out of ticket 03
because each needs its own reasoning recorded.

## `Object(…)`

`null` and `undefined` give an empty object; an object argument is the
identity. A primitive argument produces a boxed wrapper object upstream, and
that wrapper's *only* observable effect is the existing "a style value can only
contain an array, string or number" rejection — so map straight to that
rejection rather than adding a wrapper representation to the evaluator's value
type. Modelling a boxed wrapper would thread a new variant through the whole
evaluator and style pipeline to make one error message arrive by a longer route,
and no valid program reaches that path.

## Bare `Math(…)`

`Math` is not a function. It sits in the foldable-callee set because its
*methods* fold, so a bare call reaches this branch and must be rejected.

Upstream leaks a raw `TypeError` from inside its own evaluator here — a
null-dereference artifact, not a designed error and not a contract. This
compiler raises its own diagnostic naming the real problem instead. The
observable outcome that matters — this program does not compile — is preserved,
and an author gets a message about their code rather than a stack trace from
inside the compiler.

Record in the transform crate's glossary why some cases in this area deopt while
others hard-error. That split is invisible in the code and expensive to
re-derive; without it, the deliberately-missing wrapper-object modelling above
reads as an oversight.

**Blocked by:** 03 — reuses the globals enum, the callback variant, and the
apply-site wiring introduced there.

**Status:** done

- [x] `Object(null)` and `Object(undefined)` compile to no rules
- [x] `Object(obj)` is the identity and emits what the bare object would
- [x] `Object(primitive)` fails with the existing style-value rejection,
      asserted by its diagnostic
- [x] A bare `Math(…)` call fails with a diagnostic naming the real problem,
      not a reproduction of the upstream stack trace
- [x] The transform crate's glossary records the deopt-versus-hard-error split
- [x] Expected values are taken from measured reference output

**Found while measuring:** a function argument needs the same rejection as a
wrapper, and for a reason the wrapper reasoning above does not cover.
`ToObject` returns a function unchanged, so the identity is the faithful fold —
but this evaluator's function values reduce to whatever the function returns,
so `Object(() => 'red')` folded to `color:red` where upstream fails the build.
Wrong output rather than a failed build is the one outcome worth diverging from
the letter of the coercion to avoid, so the coercion reports a function apart
from the other objects and the fold refuses it.

Two positions report the refusal in their own words rather than the fold's,
which is pre-existing and not the fold's to fix: inside a dynamic style
function a refusal leaves the call for the runtime, and `defineVars` replaces
the recorded reason with `Only static values are allowed inside of a
defineVars() call.` Upstream's `defineVars` diverges in wording here too — it
reports the missing default the wrapper leaves behind — so only the build
failure itself is common ground.
