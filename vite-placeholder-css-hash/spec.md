# Placeholder CSS keeps its filename after StyleX-only edits

`@stylexswc/unplugin` in `useCssPlaceholder` mode puts the compiled StyleX rules
into a stylesheet **after** the bundler has named and content-hashed that file.
A change to StyleX code only therefore changed the CSS bytes and the JavaScript
bundle, but left the stylesheet's name alone. Documents and manifests kept
pointing at the same URL, so a browser or a CDN holding the old stylesheet
served CSS without the classes the new JavaScript asks for: elements ship
unstyled, with nothing in the build to say so.

## Verdict per host

| Host | Verdict | Action |
| --- | --- | --- |
| Vite / Rollup | defective | re-emit the final source and let the host hash it |
| esbuild | defective | rename the file on disk, with a hash of ours |
| webpack | correct already | real-content-hash renames the asset itself |
| Rspack | correct already | settled by a real production build, see below |
| Farm | no placeholder support | unchanged |

Rspack was the one verdict that could not be read from source. A production
build with `cssFilename: '[name].[contenthash].css'`, run twice over the same
app with one StyleX-only edit between the runs, gave two different names for two
different files. No change needed.

## Out of scope

- **`build.cssCodeSplit: true`**, which is Vite's default. A split build folds
  stylesheet names into the JavaScript chunk hashes and writes them inside the
  chunks for dynamic-import preloading, so a rename there cascades into
  JavaScript that is already hashed. Detected, left alone, documented, and filed
  as `issues/02`.
- **SSR builds**, which emit no stylesheet of their own.
- **The dev path**, which has no bundle step and does not hash.

## Follow-ups

- `issues/01` -- the pre-pass, which is the architecturally correct fix.
- `issues/02` — `cssCodeSplit: true` support.

## Outcome

Landed on `fix_vite-placeholder-css-keeps-same-filename-after-stylex-only-edits`
in five commits, from `refactor(unplugin): report which stylesheets the
injection wrote` to `docs(unplugin): say what placeholder mode does to
stylesheet names`.

Two questions that could not be answered from source were settled by probe
before any code was written:

- A plugin can emit a correctly hashed asset from its own `generateBundle`.
  `emitFile` succeeds, `getFileName` resolves, the file is written, the hash
  follows the modified source, and `delete bundle[old]` drops the old file. The
  name keeps the host's own shape.
- The hook's existing `order: 'post'` already runs after the stylesheet, the
  documents and the manifests are all in the bundle. The second `enforce: 'post'`
  plugin the plan held in reserve was therefore not needed; the documents and
  the manifests are rewritten in place instead.

Two injection bugs in the esbuild path surfaced while the tests were written and
are fixed in the same series: the output scan was not recursive, and relative
metafile names were resolved against the process directory rather than the
build's own. Either one left the marker in the output.
