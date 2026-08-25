# Notice

This project is MIT licensed; the terms are in [LICENSE](./LICENSE) and the
copyright there is the project's own.

It also contains, and builds on, work that is somebody else's. That work is
listed here with its copyright holder, its licence and where the licence text
lives. Everything below is MIT, so nothing here restricts what
[LICENSE](./LICENSE) already permits — the list exists because MIT requires the
notices travel with the code, and because a reader deserves to know which parts
of a repository were not written here.

## StyleX

- **Upstream:** <https://github.com/facebook/stylex>
- **Copyright (c) Meta Platforms, Inc. and affiliates.**
- **Licence:** MIT

StyleX is Meta's CSS-in-JS library. This project is a community implementation
of its compiler, written in Rust, and is not affiliated with or officially
supported by Meta.

The attribution is not incidental. The compiler's observable behaviour is
StyleX's — the class-name hashing, the property expansion tables, the value
canonicalization, the rule priorities — and in places the implementation
follows the structure of Meta's as well, because the two have to agree
character for character to be interchangeable. Where that is true of a
particular module, the module says so.

## styleq

- **Upstream:** <https://github.com/necolas/styleq>
- **Copyright (c) Nicolas Gallagher**
- **Licence:** MIT, text at
  [crates/stylex-styleq/LICENSE](./crates/stylex-styleq/LICENSE)

`styleq` is the class-name merger the StyleX runtime uses.
[`crates/stylex-styleq`](./crates/stylex-styleq) is a Rust port of it, so that
the merge can happen at compile time and the bundle can carry a literal class
string instead of a call.

## postcss-value-parser

- **Upstream:** <https://github.com/TrySound/postcss-value-parser>
- **Copyright (c) Bogdan Chadkin**
- **Licence:** MIT, text at
  [crates/postcss-value-parser/LICENSE](./crates/postcss-value-parser/LICENSE)

A loose CSS declaration-value scanner.
[`crates/postcss-value-parser`](./crates/postcss-value-parser) carries it as a
crate of its own, with no dependencies, because it is not this project's code
and the boundary is worth being able to see.

## Boa

- **Upstream:** <https://github.com/boa-dev/boa>, tag `v0.21.1`
- **Copyright (c) 2019 Jason Williams and the Boa contributors**
- **Licence:** MIT or Unlicense, at the user's option; taken here under MIT,
  with both texts at [vendor/boa/LICENSE-MIT](./vendor/boa/LICENSE-MIT) and
  [vendor/boa/LICENSE-UNLICENSE](./vendor/boa/LICENSE-UNLICENSE)

Boa is a JavaScript engine written in Rust. The compiler hands it a method call
on a literal — `'documentation'.startsWith('doc')` and the rest of
`String.prototype`, `Array.prototype` and `Object.prototype` — rather than
matching the method's name against a table of its own, because a table is finite
and the reference implementation's reflection is not.

[`vendor/boa`](./vendor/boa) carries nine of its crates as source. Only two
dependency bounds in its workspace manifest are changed, for a version conflict
[`vendor/boa/README.md`](./vendor/boa/README.md) explains; no engine source file
is modified.

## Everything else

Dependencies resolved from a registry are not listed here. They arrive with
their own licence files, and `pnpm-lock.yaml` and `Cargo.lock` are the record of
which versions. This file covers only third-party work that is _in_ the
repository — carried, ported, or reimplemented.
