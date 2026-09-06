# `stylex-enums`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Shared enum and type-alias definitions used throughout the StyleX compiler
crates. Extracted from the monolithic `stylex-shared` crate so that every
consumer can depend on a small, stable set of domain types without pulling in
transformation or CSS logic.

- Defines all domain enums for the StyleX pipeline in one place
- Zero runtime cost — every variant maps to a simple discriminant
- Consumed by five downstream crates, making it one of the most widely
  depended-on crates in the workspace

## Architecture

### Modules

| Module                     | Purpose                                    |
| -------------------------- | ------------------------------------------ |
| `aliases`                  | Type alias enums for shorthand mappings    |
| `core`                     | Core StyleX operation variants             |
| `counter_mode`             | CSS counter style modes                    |
| `css_syntax`               | CSS syntax classification types            |
| `import_path_resolution`   | Module resolution strategy variants        |
| `js`                       | JavaScript expression classification       |
| `misc`                     | Miscellaneous helper enums                 |
| `property_validation_mode` | Property validation strategy selection     |
| `style_resolution`         | Style merge and resolution strategies      |
| `style_vars_to_keep`       | Tracking which CSS variables to preserve   |
| `sx_prop_name_param`       | `stylex()` property name parameter types   |
| `theme_ref`                | Theme reference value wrappers             |
| `top_level_expression`     | Top-level call expression classification   |
| `value_with_default`       | Values carrying optional default fallbacks |

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
