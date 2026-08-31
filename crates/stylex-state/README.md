# `stylex-state`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

The state one file is compiled against, and the value vocabulary that state
composes.

- **State manager** — `StateManager`, everything the compiler learns about a
  single file while it walks it: the imports it declares, the top-level
  expressions it holds, the styles it has injected, the modules it has already
  parsed, and the caches that keep all of that cheap to ask about. One per file,
  passed by mutable reference through the whole visitor. It stays one struct
  with one method surface, because it corresponds to a single unit on the
  comparison side.
- **Value vocabulary** — the types the state manager holds or hands back: an
  evaluated value, an evaluation result, a function config, a theme reference,
  a compiled style value and the maps built over them.
- **Common helpers** — the readers that answer a question about a declaration or
  an object literal against the state manager.

## Architecture

The value types sit in this crate and not one layer lower because they name each
other and the state manager in a knot with no cut: a function config carries a
theme reference, a theme reference reads the state manager, and an evaluated
value can be a function config.

Nothing here evaluates an expression. The state manager is _what_ an evaluation
reads and writes; the crate that decides what an expression folds to sits above
this one and depends on it.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
