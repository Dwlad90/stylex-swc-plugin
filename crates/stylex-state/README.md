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
- **Common helpers** — the writers that fill the state from a module, and the
  stateless readers over an object literal that the phases above share.
- **Declaration resolution** — `resolution`, which answers what a name resolves
  to against those records: the declarator a name is bound by, the import
  declaration and specifier that bound it, and what a declaration spells when
  read literally. Each lookup is one hash probe of the indices the state manager
  fills while it walks the file.

## Architecture

The value types sit in this crate and not one layer lower because they name each
other and the state manager in a knot with no cut: a function config carries a
theme reference, a theme reference reads the state manager, and an evaluated
value can be a function config.

`resolution` is a module here and not a layer of its own because nothing else
in the crate depends on it, so no cycle forces a boundary, and every index it
reads is this crate's own. It only reads a declaration back as written.

Nothing here evaluates an expression. A conversion that must evaluate one — a
binary expression read as a number, a template read through its substitutions —
stays above this crate, and that split is what lets the evaluator depend on this
one with no cycle.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
