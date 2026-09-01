# `stylex-declarations`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

What a name resolves to, against the declarations the StyleX compilation state
recorded.

- **Declaration lookup** — the declarator a name is bound by, the import
  declaration and specifier that bound it, and the span and initializer a caller
  reading a declarator actually needs. Each is one hash probe of the indices the
  state manager fills while it walks the file.
- **Spelled value** — what an expression says when read literally: the string a
  literal or a chain of identifiers spells, the expression a declaration was
  initialized with, and a template with each substituted identifier replaced by
  its initializer.

## Architecture

The state below this crate records _that_ a file declares something. This crate
answers the next question. Both the visitor and the evaluator above ask it, so
this is a layer of its own and not part of either.

Nothing here folds an expression. A conversion that must evaluate a binary
expression, or read a template through its substitutions, stays above this
crate. The split is deliberate. Those conversions call the evaluator back, and
keeping them out lets the evaluator depend on this crate without a cycle.

A read stops at the first thing that is neither a literal nor another
identifier, and answers nothing there. It never decides what a non-literal
means. One caller treats a non-literal as a value it can skip; another treats it
as a hard error.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
