# @stylexswc/plugin-shared

The core the webpack, Rspack and Next.js plugins are built from: the loader, the
rule registry, and the CSS-extraction machinery. Its vocabulary is webpack's
plus the workarounds this pipeline needs.

## Language

**Carrier stylesheet**:
The physical `stylex.css` a consumer imports once at its entrypoint. The plugin
replaces the emitted asset's _content_ with the extracted CSS during
`processAssets` — the file itself is real and nearly empty, which is what makes
the import resolvable before anything has been extracted. Each wrapper plugin
ships its own copy, because npm `exports` cannot point across packages, so it is
matched by filename pattern rather than exact path.
_Avoid_: virtual css, output css, bundle, entrypoint

**Dummy import**:
The per-module import of `stylex-virtual.css` the loader appends, carrying that
module's serialized rules in its resource query. It exists only to invalidate
HMR in development; in production it passes through unchanged.
_Avoid_: marker import, side-effect import, virtual module

**Transformed flag**:
The comment the loader appends after a successful transform, so a module fed
through the loader chain twice — which Next.js App Router does — is not
transformed again.
_Avoid_: sentinel, guard, cache key

**Rules map**:
A module path to `StyleXRule[]` mapping, published by one compiler.
`RegisterStyleXRules` is how the loader hands rules to the plugin core.
_Avoid_: cache, store, collection

**Next.js global registry**:
The `globalThis` registry that lets Next.js App Router's three compilers
(client, server, edge-server) share rules. Server-only modules are never seen by
the client compiler, so without it their rules are simply lost. It requires
`experimental.webpackBuildWorker` off — separate worker processes share no
`globalThis` — and in development the merged CSS can lag one invalidation
behind.
_Avoid_: global cache, shared state, singleton
