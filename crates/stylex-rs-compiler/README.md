# @stylexswc/rs-compiler

> High-performance StyleX compiler for Node.js, written in Rust on NAPI-RS and
> SWC. Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace.

<!-- stylex-compatibility:start -->

> [!NOTE]
> Compatibility target: this package has been updated through official
> StyleX v0.19.0. This is not an official Meta support guarantee.

<!-- stylex-compatibility:end -->

[StyleX](https://stylexjs.com) is Meta's CSS-in-JS library with compile-time
style extraction. The official toolchain compiles it with a Babel plugin; this
package is a from-scratch Rust implementation of that same transform, exposed to
Node.js as a native addon through [NAPI-RS](https://napi.rs) and parsed with
[SWC](https://swc.rs). It is designed as a drop-in replacement: your StyleX code
and its output do not change, but transforms run 5x to 10x faster than Babel —
see [performance](https://github.com/Dwlad90/stylex-swc-plugin#performance).

This is a community project and is not affiliated with or supported by Meta. It
requires Node.js 20 or newer; prebuilt binaries ship for macOS, Linux (glibc and
musl), and Windows on x64 and arm64.

Most projects should not call this package directly — use the integration for
your build tool, all of which drive this compiler under the hood:

| Build tool                              | Package                                                                                |
| --------------------------------------- | -------------------------------------------------------------------------------------- |
| Next.js (Webpack, Rspack, Turbopack)    | [`@stylexswc/nextjs-plugin`](https://www.npmjs.com/package/@stylexswc/nextjs-plugin)   |
| Vite, esbuild, Farm, Rsbuild, Nuxt, ... | [`@stylexswc/unplugin`](https://www.npmjs.com/package/@stylexswc/unplugin)             |
| webpack                                 | [`@stylexswc/webpack-plugin`](https://www.npmjs.com/package/@stylexswc/webpack-plugin) |
| Rspack                                  | [`@stylexswc/rspack-plugin`](https://www.npmjs.com/package/@stylexswc/rspack-plugin)   |
| Rollup                                  | [`@stylexswc/rollup-plugin`](https://www.npmjs.com/package/@stylexswc/rollup-plugin)   |
| PostCSS pipelines                       | [`@stylexswc/postcss-plugin`](https://www.npmjs.com/package/@stylexswc/postcss-plugin) |
| Jest                                    | [`@stylexswc/jest`](https://www.npmjs.com/package/@stylexswc/jest)                     |

Use this package directly when building your own tooling: custom bundler
plugins, codemods, or anything that needs the transformed code plus StyleX
metadata and source maps.

## Installation

```bash
npm install --save-dev @stylexswc/rs-compiler
```

## Usage

The main entry point is `transform`. It takes a filename, the source code, and
options, and returns the transformed code, metadata about the generated styles,
and an optional source map:

```ts
const { transform } = require('@stylexswc/rs-compiler');

const { code, metadata, map } = transform(
  filename,
  inputSourceCode,
  transformOptions
);
```

### Example

Input StyleX code:

```ts
import * as stylex from '@stylexjs/stylex';

const styles = stylex.create({
  root: {
    padding: 10,
  },
  element: {
    backgroundColor: 'red',
  },
});

export const styleProps = stylex.props(styles.root, styles.element);
```

Output code:

```ts
import * as stylex from '@stylexjs/stylex';
export const styleProps = {
  className: 'x7z7khe xrkmrrc',
};
```

### Output shape

Transforming the example above with source maps enabled
(`sourceMap: SourceMaps.True`) returns:

```json
{
  "code": "import * as stylex from '@stylexjs/stylex';\nexport const styleProps = {\n    className: \"x7z7khe xrkmrrc\"\n};\n",
  "metadata": {
    "stylex": [
      [
        "x7z7khe",
        {
          "ltr": ".x7z7khe{padding:10px}",
          "rtl": null
        },
        1000
      ],
      [
        "xrkmrrc",
        {
          "ltr": ".xrkmrrc{background-color:red}",
          "rtl": null
        },
        3000
      ]
    ]
  },
  "map": "{\"version\":3,\"sources\":[\"app/components/Button.tsx\"],\"names\":[],\"mappings\":\"AAAA;AAWA;;EAAoE\"}"
}
```

The `metadata.stylex` rules are what bundler plugins collect to build the final
CSS file.

The `map` above is abridged; by default it also carries `sourcesContent` and
column-accurate `mappings` — see [`inlineSourcesContent`](#inlinesourcescontent)
and [`emitSourceMapColumns`](#emitsourcemapcolumns).

> [!NOTE]
> Comments are preserved in `code`. That matters beyond readability: bundlers
> and minifiers read some of them — `/* webpackChunkName: "…" */` on dynamic
> imports names the emitted chunk, and `/* #__PURE__ */` is what lets a
> minifier drop an unused call.

## Path Filtering

> [!NOTE]
> The `include` and `exclude` options are exclusive to this compiler and
> are not available in the official StyleX Babel plugin.

The compiler exports a `shouldTransformFile` function to determine whether a
file should be transformed based on include/exclude patterns:

```ts
import { shouldTransformFile } from '@stylexswc/rs-compiler';

const shouldTransform = shouldTransformFile(
  '/path/to/file.tsx',
  ['src/**/*.{ts,tsx}'], // include patterns (optional)
  ['**/*.test.*', '**/__tests__/**'] // exclude patterns (optional)
);

if (shouldTransform) {
  // Transform the file
}
```

### Pattern Types

- **Glob patterns** (strings): standard glob syntax matched against file paths
  - `src/**/*.tsx` — all `.tsx` files in `src` and subdirectories
  - `**/*.test.*` — all test files
  - `**/node_modules/**` — all files in `node_modules`

- **Regular expressions**: RegExp objects for complex matching
  - `/\.test\./` — files containing `.test.`
  - `/^src\/.*\.tsx$/` — `.tsx` files directly in the `src` directory

  The Rust regex engine fully supports lookahead and lookbehind assertions,
  which the JavaScript-side patterns can rely on:
  - Negative lookahead `(?!...)`: `/node_modules(?!\/@stylexjs)/` excludes all
    of `node_modules` except `@stylexjs` packages
  - Positive lookahead `(?=...)`: `/.*\.test(?=\.tsx$)/` matches only
    `.test.tsx` files
  - Negative lookbehind `(?<!...)`: `/(?<!src\/).*\.tsx$/` excludes `.tsx` files
    outside `src/`
  - Positive lookbehind `(?<=...)`: `/(?<=components\/).*\.tsx$/` matches only
    `.tsx` files in `components/`

### Filtering Rules

1. If `include` patterns are specified and not empty, files must match at least
   one pattern
2. If `exclude` patterns are specified, files matching any pattern are excluded
3. Exclude patterns take precedence over include patterns
4. All paths are matched relative to the current working directory

### Common Use Cases

Exclude all of `node_modules` except one package:

```ts
shouldTransformFile(filePath, undefined, [
  /node_modules(?!\/@stylexjs\/open-props)/,
]);
```

Transform only specific packages from `node_modules`:

```ts
shouldTransformFile(
  filePath,
  [
    'src/**/*.{ts,tsx}',
    'node_modules/@stylexjs/open-props/**/*.js',
    'node_modules/@my-org/design-system/**/*.js',
  ],
  ['**/*.test.*']
);
```

## SWC Plugin Support

The `transform` function accepts an optional `swcPlugins` array, allowing you to
run SWC WASM plugins before the StyleX transformation:

```ts
const { transform } = require('@stylexswc/rs-compiler');

const { code, metadata, map } = transform('Button.tsx', sourceCode, {
  dev: true,
  // Other StyleX options...

  // SWC plugins to run before StyleX transformation
  swcPlugins: [
    // Plugin as [pluginPath, config]
    [
      '/path/to/swc_plugin_theme.wasm',
      {
        themeName: 'my-theme',
        customOption: 'value',
      },
    ],
    // You can chain multiple plugins
    [
      '@swc/plugin-emotion',
      {
        sourceMap: true,
      },
    ],
  ],
});
```

How it works:

1. **Plugin execution phase**: if `swcPlugins` are provided, the source code is
   first transformed using `@swc/core`'s `transformSync` with the specified WASM
   plugins
2. **StyleX transformation phase**: the plugin-transformed code is then passed
   to the StyleX compiler

Each entry in `swcPlugins` is a tuple of:

- **Plugin path** (string): an absolute path to a `.wasm` file
  (`/path/to/plugin.wasm`) or an npm package name (`@swc/plugin-emotion`)
- **Plugin config** (object): plugin-specific configuration options

## Configuration Options

The compiler accepts the standard StyleX options (`dev`, `debug`,
`importSources`, `unstable_moduleResolution`, and so on — see the
[StyleX configuration docs](https://stylexjs.com/docs/api/configuration/babel-plugin/))
plus the compiler-specific options below.

### `injectStylexSideEffects`

**Type:** `boolean` **Default:** `false`

Automatically injects side-effect imports for `.stylex` and `.consts` files to
prevent tree-shaking from removing them during bundling.

The problem: when build tools perform tree-shaking (webpack, Rollup, Vite),
imports from `.stylex` or `.consts` files may appear unused after the StyleX
transformation and get removed:

```ts
// Before StyleX transformation
import { colors } from './theme.stylex';
import { spacing } from './tokens.consts';

const styles = stylex.create({
  root: {
    backgroundColor: colors.primary, // Uses colors
    padding: spacing.md, // Uses spacing
  },
});

// After StyleX transformation
import { colors } from './theme.stylex'; // Appears unused!
import { spacing } from './tokens.consts'; // Appears unused!

const styles = {
  root: {
    backgroundColor: 'x1a2b3c',
    padding: 'x4d5e6f',
    $$css: true,
  },
};
```

The bundler may remove these "unused" imports, but they are needed for other
files to resolve the same StyleX/const references correctly.

With `injectStylexSideEffects: true`, the compiler adds side-effect imports to
preserve these modules:

```ts
// After transformation with injectStylexSideEffects: true
import { colors } from './theme.stylex';
import { spacing } from './tokens.consts';
import './theme.stylex'; // Side-effect import (prevents tree-shaking)
import './tokens.consts'; // Side-effect import (prevents tree-shaking)
```

When to use:

- Use `true` when your bundler runs the StyleX transformation **before** other
  optimizations (recommended), for example with webpack's `loaderOrder: 'first'`
- Use `false` when StyleX runs **after** tree-shaking (e.g. webpack's
  `loaderOrder: 'last'`)

> [!TIP]
> This option is automatically enabled when using
> `@stylexswc/webpack-plugin` with `loaderOrder: 'first'` (the default).

### `inputSourceMap`

**Type:** `string` (JSON source map) **Default:** `undefined`

Source map for the incoming `code`, produced by earlier tooling — for example a
loader chain that expands compile-time macros before the StyleX transformation
runs.

The problem: when the compiler receives code already rewritten by previous
tools, positions in that code no longer match the original authored file. Two
things degrade as a result:

- Debug source-map annotations (`$$css: "file.tsx:LINE"`, emitted with
  `debug: true`) point at lines of the intermediate code
- The emitted source map resolves to the intermediate code instead of the
  original file

When `inputSourceMap` is provided, the compiler:

1. Resolves each style namespace to its position using the namespace key's own
   span — exact, with no re-parsing — and maps it through the input map back to
   the original authored file
2. Chains the emitted source map onto the input map, so downstream tooling (e.g.
   devtools) resolves positions all the way back to the original file

```ts
const { code, metadata, map } = transform(filename, inputCode, {
  dev: true,
  debug: true,
  // Source map produced by the previous transformation step
  inputSourceMap: JSON.stringify(previousStepSourceMap),
});
```

This is also the fastest position-resolution path: two binary searches per
namespace instead of re-reading and re-parsing the source.

> [!TIP]
> The bundler plugins (`@stylexswc/rspack-plugin`,
> `@stylexswc/webpack-plugin`, `@stylexswc/turbopack-plugin`,
> `@stylexswc/rollup-plugin`, and `@stylexswc/unplugin` on Rollup-compatible
> hosts) forward the previous loader's / plugin's source map automatically — no
> configuration needed as long as source maps are enabled in the bundler.

An invalid map is ignored with a warning, and the compiler falls back to
locating positions in the source text as described under
[`useRealFileForSource`](#userealfileforsource).

### `inlineSourcesContent`

**Type:** `boolean` **Default:** `true`

Embeds the original source text in the emitted map's `sourcesContent`, so
tooling that reads the map can render the authored file without fetching
`sources[0]` separately. Chrome DevTools needs this under Next.js dev
(`eval-source-map`): without it the fetch goes out over `webpack-internal://`
and fails with `net::ERR_UNKNOWN_URL_SCHEME`.

```ts
const { map } = transform(filename, inputCode, {
  // Smaller production maps, or where the source shouldn't ship with the map
  inlineSourcesContent: false,
});
```

Set to `false` and the `sourcesContent` key is omitted from the map entirely.

> [!NOTE]
> This is the default for the compiler's own API. The bundler plugins narrow it
> to development builds — where the map is read by DevTools — so a production
> `.map` doesn't publish your source unless you ask for it. Set the option
> explicitly to override either way.

When [`inputSourceMap`](#inputsourcemap) is provided, the emitted map preserves
source text supplied by that map, and a missing entry for _this_ file is filled
in from the current input. Entries naming other files are left alone — the
compiler only has its own loader input, so text attached to an earlier authored
file would be plausible but wrong. If two entries resolve to this file, neither
is filled. Set this option to `false` to drop inherited source text as well.

### `emitSourceMapColumns`

**Type:** `boolean` **Default:** `true`

Emits column positions in the map's `mappings`, so devtools resolve individual
expressions rather than whole lines. Set to `false` for smaller,
line-granularity maps — the same trade-off webpack's `cheap-*` devtools make.

```ts
const { map } = transform(filename, inputCode, {
  emitSourceMapColumns: false,
});
```

> [!NOTE]
> Ignored when [`inputSourceMap`](#inputsourcemap) is provided. Chaining keeps
> the input map's own tokens and only shifts them, so the emitted granularity
> is the upstream map's, not this option's — and a line-granularity shift would
> apply one correction to every token on the line, misplacing all but the
> first. To get line-granularity output from a chain, emit the _input_ map
> without columns.

### `useRealFileForSource`

**Type:** `boolean` **Default:** `true`

Controls whether the compiler reads source files from disk for error reporting
and source map generation. Only relevant when no
[`inputSourceMap`](#inputsourcemap) is available — with an input map, debug
source-map annotations are resolved from the compiler's own parse and do not
depend on this option.

- **`true` (default)**: the compiler reads the actual source file from disk when
  generating error messages and source maps. This provides accurate line numbers
  and source context that match what you see in your editor. Style namespaces
  are located **by their key**, so positions resolve correctly even when the
  incoming code was already rewritten by earlier tooling (keys survive
  value-level transforms such as macro expansion).

- **`false`**: the compiler uses the transformed AST representation for error
  reporting. Useful for in-memory transformations, virtual file systems, or when
  skipping file I/O matters more than exact positions.

> [!WARNING]
> With `useRealFileForSource: false`, error messages may report
> incorrect line numbers: the AST may have been modified by previous
> transformations, comments and whitespace are normalized, and the structure may
> differ from the file on disk. Keep the default `true` during development, and
> provide an [`inputSourceMap`](#inputsourcemap) when the incoming code was
> already transformed by earlier tooling.

### `maxEvaluationDepth`

How many levels the compiler descends into a nested expression before it refuses
to evaluate it. Defaults to `32`.

The ceiling exists because the evaluator walks a nested expression recursively:
without it, a file nested deeply enough exhausts the stack and aborts the
process, which gives a bundler no message and no file to report. Past the
ceiling you get an ordinary StyleX error instead, naming the file and the key
path:

```bash
[StyleX] base > zIndex > Expression is too deeply nested to evaluate at compile time.
At most 32 levels of nested evaluation are supported.
```

Nesting this deep is not something a person writes, so the default is sized for
hand-written styles. If generated code needs more, raise it:

```js
const options = { maxEvaluationDepth: 256 };
```

> [!IMPORTANT]
> The number counts **evaluation steps**, not levels of nesting in your source.
> Reading a member spends two (the object, then the value under the key), an
> array element spends one for the array as well, and a parenthesis spends none
> because it is unwrapped before evaluation. So raise it by measuring the input
> that was refused, not by counting brackets in it.

The same value can be set process-wide with the
`STYLEX_MAX_EVALUATION_DEPTH` environment variable:

```bash
STYLEX_MAX_EVALUATION_DEPTH=256 npm run build
```

An explicit `maxEvaluationDepth` always wins over the environment, which in turn
overrides the built-in default -- so a stray value in a CI environment cannot
change what a project that configured the option compiles to. A value of zero,
or one that is not a number, is ignored rather than honoured.

## Debug Logging

Enable debug logging with the `STYLEX_DEBUG` environment variable. Available
levels: `error`, `warn` (default), `info`, `debug`, `trace`.

```bash
# Set to debug level
STYLEX_DEBUG=debug npm run build

# Set to trace for the most verbose output
STYLEX_DEBUG=trace npm run dev
```

For Windows Command Prompt:

```cmd
set STYLEX_DEBUG=debug && npm run build
```

For PowerShell:

```powershell
$env:STYLEX_DEBUG="debug"; npm run build
```

## Error Handling

The compiler produces structured error messages with a branded `[StyleX]`
prefix, replacing Rust's default panic boilerplate with readable diagnostics in
both the terminal and at the NAPI boundary.

All StyleX errors follow this format in the terminal:

```bash
[StyleX] message
  --> file:line:col
[Stack trace]: internal/source/location #shown only when STYLEX_DEBUG >= info
```

Errors are color-coded for readability:

| Category                   | Label             | Color         |
| -------------------------- | ----------------- | ------------- |
| Regular error              | _(none)_          | Red prefix    |
| Unimplemented feature      | `[UNIMPLEMENTED]` | Magenta label |
| Internal unreachable state | `[UNREACHABLE]`   | Blue label    |

## Class names that moved

A class name is a hash of the dashed property, the value, and the _modifier_
string — the sorted pseudo keys joined, then the sorted at-rules joined. Two
changes in this release correct how that pseudo list is sorted, so some class
names differ from the previous release. They now agree with
`@stylexjs/babel-plugin` 0.19.0, which is the point: markup built by one
compiler named a class the other's stylesheet never defined.

**Who is affected.** Only styles whose sorted pseudo list changes:

- **Three or more pseudo keys in one run**, nested in an order that is not
  already alphabetical. The sort used to close a run once it held a pair, so a
  third key was appended after an already-sorted pair instead of joining the
  sort. Two keys agreed from either nesting order, which is why nothing caught
  this earlier. `:hover > :focus > :active` named a different class before and
  names `x12rlomf` now, from any of the six nesting orders.
- **Two or more keys whose order differs between byte order and collation
  order.** The comparator used raw bytes and now reproduces `localeCompare`, so
  keys are weighed with symbols below digits below letters, and letters weighed
  without their case, whatever their bytes. Real pseudo-classes are lowercase
  ASCII words and sort the same either way; an attribute selector such as
  `[data-B]` beside `[data-a]` does not.

Everything else — a single pseudo key, two lowercase pseudo-classes, a plain
declaration, an at-rule — hashes exactly as it did.

**What to do.** Nothing, if your CSS is generated at build time and shipped
together with the markup that references it. If any of these are true, rebuild
and re-publish both halves together:

- extracted CSS committed to the repository or cached on a CDN separately from
  the JS bundle
- snapshot tests asserting class names or rule text
- visual-regression baselines — rerun `test:visual` and accept the diffs

## A namespace import of a theme file no longer resolves

```js
import * as tokens from './colors.stylex.js';
export const styles = stylex.create({ wrapper: { color: tokens.primary } });
```

This compiled before and is refused now, with
`Referenced constant is not defined.` — which is what
`@stylexjs/babel-plugin` 0.19.0 has always answered for it.

**Why the break is worth taking.** What it compiled _to_ was a
`var(--…)` naming a custom property the theme file never defines, because the
export name was synthesized from the local alias rather than read from the
module. A `var()` nothing defines renders as nothing and reports as nothing, so
these styles were already absent at runtime — the change turns a silent nothing
into a build error that names the reference. Read the same token through both
import kinds in one module and the old behaviour emitted two different custom
properties for it.

**The fix is a named import,** which both compilers resolve:

```js
import { colors } from './colors.stylex.js';
export const styles = stylex.create({ wrapper: { color: colors.primary } });
```

One spelling gets a different message. `import * as NaN from './colors.stylex.js'`
answers `Referenced constant is not initialized.`, because an alias spelled like
one of the three globals meets the globals check before the import is read.
Upstream answers the same, for the same reason.

## Deliberate divergences from `@stylexjs/babel-plugin`

Five values that upstream accepts are rejected here. Each rejection changes only
_which programs compile_, never the bytes of an accepted one — so none of them
can move a class name, which is the compatibility contract that matters. They
are listed here because until now they lived only in module docstrings, and a
build that fails on a value the reference compiler accepts is the kind of
surprise worth being able to look up.

| Rejected                                                                               | Upstream                         | Why                                                                                                                                                                                                                                                       |
| -------------------------------------------------------------------------------------- | -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `var(foo)` — a custom-property reference with no `--` prefix                           | emits it verbatim                | It resolves to nothing in a browser, with no diagnostic from anywhere. The rejection names the reference. Only top-level references are checked.                                                                                                          |
| A value carrying an unterminated `/*` comment                                          | emits it                         | The scanner invents the missing terminator, so the declaration would silently swallow whatever followed.                                                                                                                                                  |
| A `{`, `}` or `;` outside a string or comment in a custom-property value               | emits it                         | The same swallowing problem, one level up: the declaration would absorb the rest of the rule.                                                                                                                                                             |
| A value nested more than 64 levels deep                                                | throws a `RangeError`            | Spelling and dropping a token tree recurse, so past some depth the process aborts with no diagnostic at all. 64 is far above any real value and the failure is a named message rather than a crash.                                                       |
| An expression nested more than [`maxEvaluationDepth`](#maxevaluationdepth) levels deep | folds until the JS engine throws | The fold recurses, and upstream's only bound is the interpreter stack — whose failure is a process abort with no file, no message and no chance to finish the build. The default is sized for authored styles; a generated token file can need it raised. |

### A TypeScript module reads an unreferenced import as a type

One divergence goes the other way: a shape upstream rejects, this compiler
compiles — and only in a TypeScript file.

```ts
import { create, keyframes } from '@stylexjs/stylex';

export const styles = create({ dyn: keyframes => ({ height: keyframes }) });
```

| file           | upstream                     | here                                |
| -------------- | ---------------------------- | ----------------------------------- |
| `page.js`      | `Invalid pseudo or at-rule.` | `Invalid pseudo or at-rule.`        |
| `page.ts/.tsx` | `Invalid pseudo or at-rule.` | `.x16ye13r{height:var(--x-height)}` |

The parameter shadows the import. In JavaScript both compilers read `keyframes`
as the imported StyleX API and refuse; in TypeScript the type-stripping pass
runs first and removes an import specifier nothing references _as a value_,
because such a specifier may name a type and a type has no module to import at
runtime. That is `tsc`'s own rule, and `verbatim_module_syntax` — which turns it
off, and is what a `.js` file is compiled with — cannot be turned on here
without emitting imports of bindings that do not exist at runtime:
`@stylexjs/stylex` exports `StyleXStyles`, `Theme`, `VarGroup` and a dozen more
as types only, and `import { StyleXStyles } from '@stylexjs/stylex'` written
without the `type` keyword is ubiquitous.

So the parameter is just a parameter, and `height: keyframes` is an ordinary
dynamic value. Upstream reaches the other answer because Babel runs plugins
ahead of presets, so its StyleX plugin sees the import before
`@babel/preset-typescript` removes it — plugin ordering rather than a considered
TypeScript semantics.

**This is intended, and it will not be closed.** Making `.ts` refuse would turn
working `.tsx` builds into failing ones over a parameter name, to reproduce an
upstream answer that is the less defensible of the two. Nothing about it can
move a class name for a module that compiles under both. It is pinned in
`__test__/importElision.spec.ts` under _a TypeScript module keeps the elision_.

Everything else is parity, and the parity harness under
[`parity/`](./parity/README.md) is what keeps that claim honest — it runs a
corpus of declarations through both compilers and reports any that disagree.

## FAQ

### Is this a drop-in replacement for `@stylexjs/babel-plugin`?

Yes, by design. It implements the same transform, is validated against the
official StyleX test suite, and produces compatible output. It also adds
compiler-only capabilities: `include`/`exclude` filtering, SWC WASM plugin
chaining, `inputSourceMap` chaining, and structured metadata output.

Four values are deliberately rejected where upstream accepts them; see
[Deliberate divergences](#deliberate-divergences-from-stylexjsbabel-plugin).
None of them changes the output of a value that compiles.

### Do I need Rust installed to use it?

No. Prebuilt native binaries are published for each supported platform and
installed automatically as optional dependencies.

### Which package should I install for my app?

One of the bundler integrations listed at the top of this page. Install
`@stylexswc/rs-compiler` directly only when building custom tooling on top of
the `transform` API.

### Known limitations?

Resolution of the `exports` field in `package.json` is only partially supported.
If you hit a problem, please open an
[issue](https://github.com/Dwlad90/stylex-swc-plugin/issues/new) with a
reproduction link.

## Documentation

- [CSS value parity harness](./parity/README.md) — how output compatibility with
  `@stylexjs/babel-plugin` is measured
- [StyleX documentation](https://stylexjs.com)
- [NAPI-RS documentation](https://napi.rs)
- [SWC documentation](https://swc.rs)

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE).

This package is the one that ships compiled, so the notices travel with it.
It contains work that is somebody else's, all MIT:

- **StyleX** (<https://github.com/facebook/stylex>) — Copyright (c) Meta
  Platforms, Inc. and affiliates. This compiler is a community implementation
  of StyleX's, not affiliated with or officially supported by Meta.
- **`styleq`** (<https://github.com/necolas/styleq>) — Copyright (c) Nicolas
  Gallagher. The runtime class-name merger, ported so the merge can happen at
  compile time.
- **`postcss-value-parser`**
  (<https://github.com/TrySound/postcss-value-parser>) — Copyright (c) Bogdan
  Chadkin. The CSS declaration-value scanner.

[NOTICE.md](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/NOTICE.md)
is the full list, with each licence text.
