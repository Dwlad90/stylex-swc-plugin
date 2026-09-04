# `stylex-evaluator`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

The JavaScript evaluator: what an authored expression folds to, or why it
cannot. Every `stylex.*` call reaches the stylesheet through such a fold. A fold
answers with a value or refuses, and a refusal is a normal answer — a value that
is not known at compile time becomes an inline style. Nothing on this path
aborts the process.

## Architecture

- **Dispatch** — `evaluate` reads the shape of an expression and calls the
  handler for that shape. A refusal carries the expression it stopped at and the
  message to report, so the caller can make an inline style or a diagnostic from
  it without guessing. `cache` memoizes each subtree under a hash of that
  subtree.
- **Node handlers** — `evaluate::nodes`, one module per expression kind:
  identifiers, members, calls, objects, arrays, templates, the two operators and
  the arrow. `binding` resolves a name against the declarations `stylex-state`
  records.
- **Engine fold** — `evaluate::engine_fold` hands a self-contained method call
  to a JavaScript engine instead of matching its name against a table, because a
  table is finite and the method it omits is the next bug report. `guard`
  decides what may cross, `transport` carries the resolved values inward,
  `amplification` answers how much a call would build, and `outward` reads the
  answer back.
- **Convertors** — `convertors` reads an expression back as a number or a
  string. They sit above the literal convertors in `stylex-state` because they
  can only answer by evaluating.
- **Growable stack** — `growable_stack` gives a descent more stack than it
  inherited. `grown_per_level` asks again at each level and is used by every
  walk this compiler writes. `grown_for_depth` claims the whole descent in
  advance, for the SWC printer and the engine parser, which recurse through a
  nested literal without asking.

A `grown_for_depth` claim is sized from `nesting_of`, which measures how deeply
an expression nests at the three node kinds that nest without bound: an
expression, a statement and a binding pattern. Measuring rather than guessing
ties the claim to the descent it must carry. `carriable` answers whether a depth
can be claimed at all; past that depth the caller refuses, which is a diagnostic
and not an abort. `stylex-structures` supplies the depth limit, so the
configured ceiling and the claim behind it cannot disagree.

`stylex_first_that_works` lives here and not with the transformers because the
embedded engine calls it while a fold is standing. No transformer calls it.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
