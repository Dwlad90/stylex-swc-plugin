# @stylexswc/nextjs-plugin

`withStyleX(...)`, the Next.js config wrapper, plus separate `./turbopack` and
`./rspack` entry points. Its vocabulary is largely Next's internals.

## Language

**Config wrapper**:
The `withStyleX(pluginOptions)(nextConfig)` shape Next.js plugins take. It
spreads the Next config into a new object, and mutates the _webpack_ config in
place inside the callback, because Next's CSS rules must be found and amended
where they are.
_Avoid_: plugin, hoc, middleware

**App Router registry mode**:
`nextjsAppRouterMode`, on by default — whether to use the
[cross-compiler registry](../plugin-shared/CONTEXT.md) that lets server-only
modules' rules reach the client compiler's CSS asset. While it is on, the plugin
force-sets `experimental.webpackBuildWorker` to `false` and warns, because
separate worker processes share no `globalThis`.
_Avoid_: app mode, ssr mode, shared mode

**CSS container rule**:
The `oneOf` rule inside Next's webpack config that owns stylesheet handling. The
plugin locates it and inserts its own loaders there, and throws a named error
identifying the incompatible Next version when it cannot find it.
_Avoid_: css rule, loader chain, style rule

**Carrier CSS**:
The `carrierCss` option, naming which stylesheet the extracted CSS is appended
to — the Next.js-side selection of
[plugin-shared's carrier stylesheet](../plugin-shared/CONTEXT.md). Resolved to
an absolute path once and shared by the CSS rule's `test` and the plugin, so the
two cannot disagree. It defaults to the webpack plugin's copy.
_Avoid_: output path, css entry

**Turbopack entry**:
The separate `turbopack` export, served by a loader from
[@stylexswc/turbopack-plugin](../turbopack-plugin/CONTEXT.md). Turbopack reads a
rule list as alternatives rather than as a chain, so the loader is added to
every item, and `loaderOrder: 'first'` means last in the array.
_Avoid_: turbo mode, alternative build
