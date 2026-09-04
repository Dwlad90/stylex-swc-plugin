# @stylexswc/jest

A Jest transformer, so StyleX components can be tested without Babel. Its
substance is cache correctness: Jest caches transform output aggressively, and
this transformer's output depends on a native binary Jest cannot see.

## Language

**Cache key**:
The digest Jest uses to decide whether it may replay a previous transform. Its
parts are joined with a NUL delimiter, so a `(source, path)` pair cannot
collide with another.
_Avoid_: hash, fingerprint, digest

**Compiler fingerprint**:
`COMPILER_FINGERPRINT` — `<version>|<size>:<mtimeMs>` of the `.node` addon the
compiler actually loaded, computed once per worker and folded into the cache
key. The declared package version is not enough during development: rebuilding
the crate replaces the binary in place without touching `package.json`, so
nothing in the key moves and Jest replays the previous build's output. The addon
is found by walking the loaded compiler entry's own dependency subtree.
_Avoid_: version, build id, checksum

**Guarded manifest read**:
Reading `@stylexswc/rs-compiler/package.json` inside a `try`. That subpath is
not declared in an `exports` map, so it resolves by legacy lookup only, and an
unguarded read would fail the entire Jest run for a cache-key ingredient. Both
failure paths answer with a placeholder rather than throwing.
_Avoid_: version lookup, require, import
