# stylex-evaluator

The JavaScript evaluator: what an authored expression folds to, or why it
cannot. A refusal is a normal answer here rather than a failure -- a value that
cannot be known at compile time becomes an inline style instead -- so nothing on
this path may abort the process.

Today the crate holds the bottom of that path: the stack every descent of a fold
runs on. The dispatcher, the node handlers and the
[engine fold](../stylex-transform/CONTEXT.md#engine-fold) are still with the
[transform](../stylex-transform/CONTEXT.md) and join this crate as one unit.

## Language

**Grown stack**:
Room a descent is given rather than room it inherited. Several descents of a
fold recurse without a budget of their own -- the evaluator's own walk, the
guard's walk towards the engine, the carriage of a value in and of an engine
value back, SWC's print of the source the engine is handed, and the engine's
parse of it. Overflowing any of them aborts the process from inside an
evaluation that is allowed to refuse, so none of them may run on whatever the
thread had left over. Nothing is allocated when the room is already underfoot,
which is what keeps a fold nested inside another fold from paying twice.
_Avoid_: stack growth, thread stack, recursion budget

**Asking by the level**:
The way a descent that can ask again at the next level is given room: it spends
one level and claims headroom for the one after it, so a walk that stops early
pays only for the levels it descended. Every walk this compiler writes asks this
way. _Avoid_: incremental growth, per-frame growth

**Claim**:
The whole descent asked for at once, for a caller that will not ask again --
SWC's printer and the engine's parser, neither of which this compiler writes.
Sized from the [text nesting](#text-nesting) of what those two are handed, so an
input that nests further gets the stack its nesting needs and an ordinary one
asks for a couple of megabytes it already has. A claim is made after the guard
has admitted a call, never around the walk, because the walk runs on every call
expression the evaluator visits and almost none of them fold.
_Avoid_: reservation, stack size, allocation

**Carriable**:
Whether a descent that deep is one a [claim](#claim) may be asked for. The
deepest carried nesting and the ceiling
[evaluation depth](../stylex-transform/CONTEXT.md#evaluation-depth) is clamped
to are one number, because they are one thing: how far down this compiler is
prepared to go. Past it the caller refuses, which is a diagnostic rather than an
abort. _Avoid_: within limit, allowed depth

**Text nesting**:
How deeply an expression nests, counted at the three node kinds that nest
without bound -- an expression, a statement and a binding pattern. Everything
between two of them is a fixed number of frames, so counting the three counts
the descent a printer or a parser makes through the text. Distinct from
[evaluation depth](../stylex-transform/CONTEXT.md#evaluation-depth), which
counts the levels a _fold_ spends: an operand a short circuit never reaches
costs the fold nothing and the parser its whole height.
_Avoid_: expression depth, source depth, nesting level
