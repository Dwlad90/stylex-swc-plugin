# `stylex-path-resolver`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Import path resolution and `package.json` parsing utilities for the
StyleX SWC transformation. Extracted into its own crate so that the
evaluator and transform layers can resolve module paths without
depending on the full compiler.

> [!WARNING]
> The current resolution of the `exports` field from `package.json`
> is only partially supported, so if you encounter problems, please
> open an
> [issue](https://github.com/Dwlad90/stylex-swc-plugin/issues/new)
> with an attached link to reproduce the problem.

## Architecture

- **Layer**: 2 — Domain Leaves
- **Depends on**: `stylex-macros`
- **Depended on by**: `stylex-evaluator`, `stylex-transform`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
