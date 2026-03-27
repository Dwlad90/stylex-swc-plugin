# `stylex-js`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

JavaScript runtime guard functions used during compile-time evaluation
of JS expressions. Extracted into its own crate so the evaluator can
depend on a focused set of AST-inspection helpers without pulling in
the full transformation pipeline.

## Architecture

- **Layer**: 2 — Domain Leaves
- **Depends on**: `stylex-constants`, `stylex-macros`
- **Depended on by**: `stylex-evaluator`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
