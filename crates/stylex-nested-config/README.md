# `stylex-nested-config`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Reads the arbitrarily nested configuration objects that `defineVars`,
`defineConsts` and `createTheme` accept, and flattens them into the single map
of dot-joined keys those APIs emit. Every function is stateless and operates
only on SWC AST nodes and primitive values.

- **Nested configuration values** — `NestedVarsValue` and the
  `object_lit_to_nested_*_config` readers turn a `defineVars`, `createTheme` or
  `defineConsts` object literal into a nested map, and the
  `flatten_nested_*_config` writers collapse one back to the flat keys those
  APIs emit
- **Configuration shape tests** — `is_vars_leaf` answers whether a nested value
  stops flattening, while `is_css_type_object` and `is_conditional_object` read
  an `ObjectLit` for the two shapes a key may hold: a `syntax`/`value` CSS type,
  and a `default` with at-rule alternatives
- **Value emission** — `to_vars_config_value` and `value_with_default_to_expr`
  turn a nested value back into the SWC expression the transform writes out

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
