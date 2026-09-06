# StyleX Path Resolver

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Import path resolution and `package.json` parsing utilities for the StyleX SWC
transformation. Extracted into its own crate so that the evaluator and transform
layers can resolve module paths without depending on the full compiler.

> [!WARNING]
> The current resolution of the `exports` field from `package.json`
> is only partially supported, so if you encounter problems, please open an
> [issue](https://github.com/Dwlad90/stylex-swc-plugin/issues/new) with an
> attached link to reproduce the problem.

- Resolves bare and relative import specifiers to filesystem paths, mirroring
  Node.js module resolution semantics
- Parses `package.json` files with partial support for the `exports` field
  (conditions, subpath patterns)
- Leaf crate with minimal dependencies — only `stylex-macros`

## Architecture

### Modules

| Module         | Purpose                                                     |
| -------------- | ----------------------------------------------------------- |
| `package_json` | `package.json` parsing with partial `exports` field support |
| `resolvers`    | Import path resolution (bare specifiers, relative paths)    |
| `utils`        | Path manipulation and normalization helpers                 |

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
