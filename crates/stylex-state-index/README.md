# `stylex-state-index`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

The lookup structures the StyleX state manager composes, so that "which
declarator, which call, which span" costs one hash probe instead of a scan of
the module. Both are pure lookup machinery: neither owns the entries it points
at, and neither decides what a style means.

- **Candidate index** — `CandidateIndex<K, H>`, a bucket map from a narrowing
  key to the handles of the entries that can hold what the key describes. The
  key only narrows; the caller confirms a candidate by equality. It replaces a
  full walk that made the transform quadratic in the number of `stylex.*` calls
  in a module.
- **Key span index** — `KeySpanIndex`, every authored position a style namespace
  key can resolve to in one module, collected in a single walk. The `file:line`
  annotation on `$$css` is resolved from it. When two namespaces spell the same
  key, candidates are ranked by how much of the compiled call each reproduces. A
  tie resolves to nothing, because a wrong `file:line` is worse than none.

## Architecture

`stylex-ast` reads the object keys and `stylex-utils` supplies the stable hash
for the cache key. The transform is the one consumer: its state manager holds
the indices and shares them by `Rc`.

Positions are compared as `FileOffset`, never as raw `BytePos`. The index is
built from a module re-parsed into the shared source map of the code frame,
while the call it places is read from the per-transform map, so two `BytePos`
can name the same character with different numbers. A `FileOffset` can only be
built from a position and the `ModuleBase` it belongs to, which makes the wrong
comparison unspellable.

A lookup has two parts. `CallLookup` carries what belongs to the `stylex.create`
call — the sibling keys, the proximity anchor, the cache-key digest and the call
as an expression — and is built once per call. `NamespaceKeyQuery` carries the
one namespace being placed. Building the call half per namespace would make a
call quadratic in its own namespace count.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
