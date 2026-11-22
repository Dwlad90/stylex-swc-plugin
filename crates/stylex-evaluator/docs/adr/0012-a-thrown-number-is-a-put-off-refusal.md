# A thrown number is a put-off refusal

**Status:** accepted

The printed source of a fold can hold a call of the global `__sxPutOff` with a
number. The call throws that number. The number is the position of a part that
the guard refused, and the fold reads a thrown number as the decline it
recorded for that position. This is the
[put-off refusal](../../CONTEXT.md#put-off-refusal).

## Why

The evaluator evaluates both sides of `||`, `&&` and `??`. A refusal on the
side that loses has no effect, but a compiler call in front of it on that side
still declares its rule. Inside a callback, only the engine can tell which side
loses. So the guard cannot refuse the part before the engine runs, and it
cannot print the part either, because it did not carry the names the part
reads.

So the part becomes a call that throws. The parts in front of it run first,
and `__sxEither` ignores the throw where the left side decides. Where the left
side does not decide, the throw leaves the engine, and the fold gives the
decline the guard recorded.

## The invariant

A thrown number means a put-off refusal only because no other source throws
one:

- The guard admits no `throw` statement. A callback body admits expression,
  `return`, declaration, block, `if` and empty statements only.
- Every native function of the engine throws an error object, not a number.
- `__sxPutOff` and `__sxEither` are properties of the global object that are
  not writable, not enumerable and not configurable, and the guard refuses
  each name of these spellings, with `__sxRead`, `__sxCall` and the native
  that computes `**`. No binding can hide them.

A thrown number that is not the position of a put-off refusal of this fold is
read as an ordinary throw. A source that forged one could only change the
reason of a refusal, never the value of a fold.

`__sxEither` stops at once on any other throw of its left side, as a throw does
in the language. It runs the right side first only for a thrown number,
because the evaluator evaluates the right side when the left side refuses.

## Consequences

- If the guard ever admits a `throw`, this record has to be read again.
- A refusal of a whole declaration that crosses as a parameter default is not
  held in the default, because a default runs before any operator decides. An
  operator inside the default can still hold one, because that operator decides
  before its own parts run.
