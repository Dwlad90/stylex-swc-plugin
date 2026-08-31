# `stylex-state-index`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

The lookup structures the StyleX state manager composes so that "which
declarator, which call, which span" is answered with one hash probe instead of a
scan of the module. Both are pure lookup machinery: neither holds the entries it
points at, and neither decides what a style means.

- **Candidate index** — `CandidateIndex<K, H>`, a bucket map from a narrowing
  key to the handles of the entries that may hold what the key describes. The
  key only narrows; the caller still confirms a candidate by equality. It
  replaces a walk of the whole collection that made the transform quadratic in
  the number of `stylex.*` calls a module makes.
- **Key span index** — `KeySpanIndex`, every authored position a style namespace
  key could resolve to in one module, collected in a single walk. It is what the
  `file:line` annotation on `$$css` is resolved from. A key that two namespaces
  spell gives several candidates, ranked by how much of the compiled call each
  reproduces; a tie resolves to nothing, because a wrong `file:line` is worse
  than none.

## Architecture

`stylex-ast` reads the object keys, and `stylex-utils` supplies the stable hash
the cache key is built from. The transform is the one consumer: its state
manager holds the indices and shares them by `Rc`.

Positions are compared as `FileOffset`, never as raw `BytePos`. The index is
built from a module re-parsed into the code frame's shared source map, while the
call it places is read out of the per-transform one, so two `BytePos` can name
the same character and hold different numbers. A `FileOffset` can only be built
from a position and the `ModuleBase` it belongs to, which makes the wrong
comparison unspellable.

A lookup is split in two. `CallLookup` carries everything that belongs to the
`stylex.create` call — the sibling keys, the proximity anchor, the cache-key
digest and the call wrapped as an expression — and is built once per call.
`NamespaceKeyQuery` carries the one namespace being placed. Building the call
half per namespace would make a call quadratic in its own namespace count.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
