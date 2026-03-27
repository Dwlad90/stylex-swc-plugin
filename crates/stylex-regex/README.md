# `stylex-regex`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Pre-compiled regular expressions used throughout the StyleX compiler.
This crate centralises every `lazy_static!` regex pattern into one
place so that patterns are compiled exactly once and shared across
all downstream consumers.

## Architecture

- **Layer**: 0 — Primitives (no internal deps)
- **Depends on**: None (leaf crate)
- **Depended on by**: css, rs-compiler, transform

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
