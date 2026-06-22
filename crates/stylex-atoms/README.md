# `stylex-atoms`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Compile-time helpers for authoring StyleX atomic styles inline. It detects and
compiles the inline-style syntax used inside `stylex.props(...)`:

```js
import * as stylex from '@stylexjs/stylex';
import x from '@stylexjs/atoms';

<div
  {...stylex.props(
    x.display.flex, //               static value
    x.padding._16px, //              leading-underscore value
    x.width['calc(100% - 20cqi)'], // computed key for complex values
    x.color(color), //              dynamic value
  )}
/>;
```

There is no new runtime styling system — everything compiles to the same output
as `stylex.create`. The compiler treats atomic styles from this crate as if they
were authored locally, enabling the same optimizations as normal StyleX styles.

- **Detectors** — `get_static_style_from_path`, `get_dynamic_style_from_path`,
  `is_utility_styles_identifier`, `get_prop_key`, `normalize_value` inspect SWC
  AST nodes and the recorded atom imports.
- **Compile helpers** — `compile_static_style` / `compile_dynamic_style` rewrite
  the detected nodes into compiled style objects (or hoisted dynamic-style
  functions).
- **Visitor** — `create_utility_styles_visitor` returns a `VisitMut` pass that
  rewrites member and call expressions in place.

## Architecture

The heavy style-compilation utilities (`styleXCreateSet`, `convertObjectToAST`,
`hoistExpression`, `injectDevClassNames`, and the needed `StateManager` access)
are injected through the `Compile` trait, which receives a `compile` object.
This keeps the crate dependency-light and avoids a circular dependency on
`stylex-transform`.

- **Depends on**: `swc_core`, `rustc-hash`, and
  [`stylex-constants`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-constants)
  (only for the shared `$$css` compiled-key constant)
- **Depended on by**:
  [`stylex-transform`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-transform),
  which implements `Compile` for its `StyleXTransform` and runs the visitor
  between the `stylex.create` and `stylex.props` transformation passes.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
