# @stylexswc/jest

A Jest transformer, so StyleX components can be tested without Babel. Almost all
of its substance is cache correctness: Jest caches transform output aggressively
and this transformer's output depends on a native binary Jest cannot see.

## Language

**Cache key**:
The digest Jest uses to decide whether it may replay a previous transform. It
must move whenever the output could differ, which is why it takes in more than
the file's contents.
_Avoid_: hash, fingerprint, digest

**Binary identity**:
The size and mtime of the `.node` addon the compiler actually `dlopen`'d, read
out of `require.cache`, and folded into the cache key. The declared package
version is not enough during development: rebuilding the crate replaces the
binary in place without touching `package.json`, so nothing in the key moves and
Jest replays output from the previous build.
_Avoid_: version, build id, checksum

**Guarded manifest read**:
Reading `@stylexswc/rs-compiler/package.json` inside a `try`. That subpath is
not declared in an `exports` map, so it resolves only by legacy lookup — if the
package ever gains an `exports` map without it, an unguarded read would throw at
module evaluation and fail the entire Jest run for a cache-key ingredient.
_Avoid_: version lookup, require, import
