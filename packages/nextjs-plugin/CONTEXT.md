# @stylexswc/nextjs-plugin

`withStyleX(...)`, the Next.js config wrapper. It reaches into Next's own
webpack configuration, so its vocabulary is largely Next's internals — which are
not a public API and move between releases.

## Language

**Config wrapper**:
The `withStyleX(pluginOptions)(nextConfig)` shape Next.js plugins take. It
mutates the config Next hands it rather than composing a fresh one, because
Next's CSS rules must be found and amended in place.
_Avoid_: plugin, hoc, middleware

**App Router registry mode**:
`nextjsAppRouterMode` (on by default) — whether to use the
[cross-compiler registry](../plugin-shared/CONTEXT.md) that lets server-only
modules' rules reach the client compiler's CSS asset.
_Avoid_: app mode, ssr mode, shared mode

**CSS container rule**:
The `oneOf` rule inside Next's webpack config that owns stylesheet handling. The
plugin locates it and inserts its own loaders there, so a Next release that
reshapes that rule breaks the plugin rather than degrading it.
_Avoid_: css rule, loader chain, style rule

**Carrier CSS**:
The `carrierCss` option, naming which stylesheet the extracted CSS replaces —
the Next.js-side selection of
[plugin-shared's carrier stylesheet](../plugin-shared/CONTEXT.md).
_Avoid_: output path, css entry

**Turbopack entry**:
The separate `turbopack` export. Turbopack does not run webpack plugins, so it
is served by a loader from
[@stylexswc/turbopack-plugin](../turbopack-plugin/CONTEXT.md) rather than by
this plugin's webpack path.
_Avoid_: turbo mode, alternative build
