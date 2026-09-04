# @stylexswc/plugin-shared

The core the webpack, Rspack and Next.js plugins are built from: the loader, the
rule registry, and the CSS-extraction machinery.

## Language

**Carrier stylesheet**:
The physical `stylex.css` a consumer imports once at its entrypoint. During
`processAssets` the plugin **appends** the extracted CSS to that asset; it never
replaces the content. The file is real and nearly empty, which is what makes the
import resolvable before anything is extracted. It is resolved to an absolute
path and matched by that path, with the filename patterns as a fallback. Each
wrapper ships its own copy, because npm `exports` cannot point across packages,
and `carrierCss` replaces the default match rather than adding to it.
_Avoid_: virtual css, output css, bundle, entrypoint

**Rule transport**:
How a compiler's rules reach the plugin core, and the two hosts differ. Webpack
carries them on `buildInfo` under a reserved key, which survives webpack's
filesystem cache when the loader does not re-run. Rspack cannot persist
`buildInfo` across the native boundary, so it uses the dummy import's resource
query and re-collects from module identifiers. Every webpack/Rspack asymmetry
here follows from this.
_Avoid_: rules map, cache, store

**Dummy import**:
The per-module import of `stylex-virtual.css` the loader appends, carrying that
module's serialized rules in its resource query. The content the virtual loader
stamps is development-only, for HMR invalidation, but the import itself is
load-bearing in production: it is the Rspack rule transport, and it routes the
module into the StyleX chunk through the cache group's test.
_Avoid_: marker import, side-effect import, virtual module

**Transformed flag**:
The comment the loader appends after a successful transform, so a module fed
through the loader chain twice — which Next.js App Router does — is not
transformed again.
_Avoid_: sentinel, guard, cache key

**Transformable extension**:
A file extension the StyleX loader compiles: the eight JavaScript and TypeScript
extensions `INCLUDE_EXTENSIONS` names. Every bundler plugin reads that one list,
as the list, the path form `INCLUDE_REGEXP`, or the glob form
`buildIncludeGlob`, and the three always agree. A plugin that leaves an
extension out sends the StyleX in it to the browser uncompiled. The list has its
own `./constants` entry point, which loads no compiler, because a config file
reads it.
_Avoid_: page extension, source extension, supported file type

**Next.js global registry**:
The `globalThis` registry that lets Next.js App Router's three compilers
(client, server, edge-server) share rules, since server-only modules are never
seen by the client compiler. Only the client compiler merges and emits. It
requires `experimental.webpackBuildWorker` off, which
[nextjs-plugin](../nextjs-plugin/CONTEXT.md) enforces, and in development the
merged CSS can lag one invalidation behind.
_Avoid_: global cache, shared state, singleton
