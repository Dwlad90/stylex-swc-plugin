# `stylex-utils`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Small standalone utilities that don't belong in any domain crate.
This crate (formerly `stylex-misc`) provides deterministic hashing
and SWC helper functions shared by several higher-level crates in
the workspace.

## Architecture

- **Layer**: 0 — Primitives (no internal deps)
- **Depends on**: None (leaf crate)
- **Depended on by**: ast, rs-compiler, transform, types

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
