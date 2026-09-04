# `stylex-state`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

The state one file is compiled against, and the value types that state composes.

- **State manager** — `StateManager` holds everything the compiler learns about
  one file while it walks it: the imports it declares, the top-level
  expressions, the injected styles, the modules already parsed, and the caches
  that keep all of that cheap to query. One instance per file, passed by mutable
  reference through the whole visitor.
- **Value vocabulary** — the types the state manager holds or returns: an
  evaluated value, an evaluation result, a function config, a theme reference, a
  compiled style value, and the maps built over them.
- **Common helpers** — the writers that fill the state from a module, and the
  stateless readers over an object literal that the layers above share.
- **Declaration resolution** — `resolution` answers what a name resolves to: the
  declarator that binds it, the import declaration and specifier that bound it,
  and what a declaration spells when read literally. Each lookup is one hash
  probe of the indices the state manager fills while it walks the file.

## Architecture

The value types stay in this crate and not one layer lower because they name
each other and the state manager in a cycle: a function config carries a theme
reference, a theme reference reads the state manager, and an evaluated value can
be a function config.

`resolution` is a module here and not a crate of its own because nothing else
depends on it and every index it reads belongs to this crate. It only reads a
declaration back as written.

Nothing here evaluates an expression. A conversion that must evaluate one — a
binary expression read as a number, a template read through its substitutions —
lives above this crate. That split lets the evaluator depend on this crate with
no cycle.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
