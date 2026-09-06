# `stylex-regex`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Pre-compiled regular expressions shared across the StyleX compiler. Every
`lazy_static!` pattern lives in this crate so that regexes are compiled exactly
once at startup and reused by all consumers, giving zero per-call regex
compilation overhead. The crate was extracted to provide shared patterns without
pulling in any compiler logic.

- **Compiled once at startup** — all patterns use `lazy_static!` for one-time
  initialisation
- **Zero per-call overhead** — downstream crates reference pre-compiled `Regex`
  values
- **No compiler dependencies** — leaf crate with no workspace deps

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
