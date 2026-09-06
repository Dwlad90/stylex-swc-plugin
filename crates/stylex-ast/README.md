# `stylex-ast`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

SWC AST manipulation utilities — factory functions for creating AST nodes and
pure convertor functions for extracting and coercing values. This crate was
split out so that any layer needing to construct or inspect SWC `Expr` / `Lit` /
`Ident` nodes can do so without pulling in the full transform or CSS pipelines.
All 54+ public functions follow a strict semantic naming convention (`create_*`,
`convert_*`, `extract_*`, `coerce_*`) that makes intent immediately clear at the
call site.

- **Factories** — ~36 `create_*` functions that construct AST nodes from
  primitive values (literals, identifiers, properties, expressions, JSX
  attributes, variable declarators)
- **Convertors** — ~20 `convert_*` / `extract_*` / `expand_*` functions that
  transform between AST types, extract inner values, and expand shorthand
  properties
- **Type-specific suffixes** — `_lit`, `_expr`, `_prop`, `_or_spread` encode the
  output SWC type directly in the function name
- **Error strategy** — `Result<T>` for fallible conversions, `Option<T>` for
  nullable extractions, `stylex_panic!` for invariant violations

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
