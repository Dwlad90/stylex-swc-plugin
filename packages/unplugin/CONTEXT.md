# @stylexswc/unplugin

One plugin factory served to Vite, webpack, Rspack, Rollup, esbuild, Farm,
Rsbuild, Nuxt and Astro through `unplugin`. Each host gets a thin entry file;
the shared factory is where the differences are actually reconciled.

## Language

**Host**:
One bundler the factory is instantiated for. Hosts differ in how CSS reaches the
output, not in how the transform runs — so a term that means something in one
host's hook may mean nothing in another's.
_Avoid_: adapter, target, platform, integration

**CSS placeholder**:
A marker string written into a CSS file, replaced with the extracted StyleX CSS
once compilation has produced it. The alternative to appending to a chosen
asset, and what `useCssPlaceholder` selects.
_Avoid_: token, sentinel, slot

**Injection asset**:
The CSS asset the extracted styles are appended to when no placeholder is in
use, picked by preference: `index.css`, then `style.css`, then `main.css`, then
the first `.css` asset present.
_Avoid_: target file, output css, main stylesheet

**Stylesheet href**:
The URL written into the injected `<link>` tag, as opposed to the path the CSS
asset is emitted at. The two differ whenever a host serves the bundle from
somewhere other than the origin root — Vite's `base` — so the emitted path stays
root-relative while the href is resolved against that base. In dev there is a
third form again: the served path, which stays base-less because that is what
HMR payloads carry and what the CSS middleware matches.
_Avoid_: css url, link path, asset url

**Bundler source**:
An asset's content object. webpack and Rspack each declare their own
incompatible `Source`, so the shared injection helper is generic over it —
naming either concretely would force a cast at the other call site, and under
`any` a webpack `RawSource` handed to Rspack's `updateAsset` type-checks and
fails only at runtime.
_Avoid_: asset, content, raw source
