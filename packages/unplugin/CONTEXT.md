# @stylexswc/unplugin

One plugin factory served to Vite, webpack, Rspack, Rollup, esbuild, Farm,
Rsbuild, Nuxt and Astro through `unplugin`. The shared factory is where the
hosts' differences are reconciled. Rsbuild consumes the `./rspack` entry, and
the Nuxt and Astro entries wrap `./vite`.

## Language

**Host**:
One bundler the factory is instantiated for. Hosts differ in how CSS reaches the
output, not in how the transform runs.
_Avoid_: target, platform, integration

**CSS placeholder**:
A marker string written into a CSS file, replaced with the extracted StyleX CSS
once compilation has produced it. `useCssPlaceholder` selects it and is off by
default; `true` means `'@stylex;'`, and a string that trims to empty falls back
to off rather than to the default marker. The first marker gets the rules and
later ones are stripped.
_Avoid_: token, sentinel, slot

**Build placeholder**:
The plugin's own marker, written over the CSS placeholder while a build runs,
because the rule set is not complete when the stylesheet is loaded. A `@layer`
statement rather than a comment: a comment is dropped by esbuild and Lightning
CSS, while a layer statement must be preserved because it declares layer order.
Written by the Vite and esbuild load hooks. Rollup and Vite replace it in
`generateBundle`; esbuild rewrites files already on disk in `onEnd`; webpack and
Rspack never see it and match the raw marker only.
_Avoid_: sentinel, temporary marker, internal token

**Injection target**:
One stylesheet the rules can go into, as `{ name, read, write }`, whichever way
the host stores it: a bundle asset, a webpack asset, or a file already on disk.
It covers the three build paths; Vite's dev path replaces the marker in file
contents with no target, and Farm never replaces it and warns. A target that was
written is handed back to the host afterwards, because its name outlives the
write and has to be settled there.
_Avoid_: sink, destination, output

**Written target**:
An injection target whose bytes the injection changed -- the one that received
the rules, and every other one that had a stray marker taken out. Reported as a
set, because a host that settles only the receiving one leaves a stale name on
the rest.
_Avoid_: touched target, dirty target, modified asset

**Stylesheet rename**:
Giving a written target a name that matches its new contents, because the rules
arrive after the host has named and hashed it. Vite and Rollup re-emit the final
source and let the host hash it; esbuild renames the file on disk with a hash of
our own, its own being unreproducible; webpack and Rspack rename the asset
themselves. Skipped where the name template carries no hash, and where Vite
splits CSS per chunk.
_Avoid_: rehash, cache busting, fingerprint

**Reference asset**:
A bundle entry that names stylesheets rather than being one: a document, or a
Vite build manifest. Both are emitted under names the host never hashes, so a
stylesheet rename is written into them in place. A reference outside the bundle,
such as a service-worker precache list, is out of reach and documented.
_Avoid_: consumer, referrer, index

**Injection asset**:
The CSS asset the extracted styles are appended to, picked by preference:
`index.css`, then `style.css`, then `main.css`, then the first `.css` asset,
each matched at a path boundary. Reached only in placeholder mode when no marker
was found; with the placeholder off, the plugin emits a standalone asset at
`buildEnd` instead.
_Avoid_: target file, output css, main stylesheet

**Stylesheet href**:
The URL written into the injected `<link>` tag, as opposed to the path the CSS
asset is emitted at. The emitted path stays root-relative while the href is
resolved against the host's base; a relative base climbs by document depth. In
development a third form applies: the served path, base-less, because that is
what HMR payloads carry.
_Avoid_: css url, link path, asset url

**Served stylesheet**:
The marker stylesheet under Vite's bundled dev server, where imported CSS is
compiled into JavaScript and never requested on its own. The import is
redirected to a runtime module that links the stylesheet from the dev server,
and a middleware renders it from the file and the current rules on each
request; one hot event asks the browser to refetch it. Only exists when
`experimental.bundledDev` is on; every other Vite path keeps the stylesheet in
the module graph.
_Avoid_: virtual stylesheet, external CSS, link mode

**Bundler source**:
An asset's content object. webpack and Rspack each declare their own
incompatible `Source`, so the shared injection helper is generic over it: under
`any`, a webpack `RawSource` handed to Rspack's `updateAsset` type-checks and
fails only at runtime.
_Avoid_: asset, content, raw source
