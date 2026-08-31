# `stylex-evaluator`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

The JavaScript evaluator: what an authored expression folds to, or why it
cannot. Every `stylex.*` call reaches the stylesheet through such a fold, and a
fold either answers with a value or refuses. Refusing is a normal answer — a
value that cannot be known at compile time becomes an inline style instead — so
nothing on this path may abort the process.

The crate is being filled from the bottom. What is here today is the stack every
descent of a fold runs on; the dispatcher, the node handlers and the engine fold
are still with `stylex-transform` and move here as one unit.

## Architecture

- **Growable stack** — `growable_stack`, the room a descent is given rather than
  the room it inherited. Two ways of asking, and one rule that decides which a
  descent gets: a descent that can ask again at the next level does.
  `grown_per_level` is that ask, and every walk this compiler writes uses it.
  `grown_for_depth` is the other: the whole descent claimed up front, for SWC's
  printer and the engine's parser, which recurse through a nested literal
  without ever asking.

A claim is sized from `nesting_of`, which measures how deeply an expression
nests at the three node kinds that nest without bound — an expression, a
statement and a binding pattern. Measuring rather than guessing is what ties the
claim to the descent it has to carry: an operand a short circuit never reaches
costs the fold nothing and the parser its whole height. `carriable` answers
whether a depth is one a claim may be asked for at all; past it the caller
refuses, which is a diagnostic rather than an abort.

`stylex-structures` supplies the depth limit the deepest carried nesting is
equal to, so the ceiling a project configures and the claim behind it can never
disagree.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
