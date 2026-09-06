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

You can also set this ceiling with the `STYLEX_MAX_EVALUATION_DEPTH`
environment variable. See [the three
ceilings](#the-three-ceilings-share-a-precedence).

The maximum value is `8192`. The compiler refuses a larger value, and does not
clamp it. The compiler reserves stack memory for the depth that you ask for.

### `maxFoldedCharacters`

The maximum length of a string that the compiler builds or carries during
evaluation, in UTF-16 code units. Defaults to `1000000`.

The ceiling applies to a string from a method call, a concatenation, an
interpolation, and a conversion of an array to a string. In a callback, the
compiler compares the ceiling with the product of the length and the number of
elements. The compiler refuses a call for all lengths if it cannot read the
count.

```js
const options = { maxFoldedCharacters: 4000000 };
```

A string costs approximately 19 bytes of peak memory for each code unit. Thus
the default is approximately 20 MB for one fold. The maximum value is
`40000000`. The compiler refuses a larger value, and does not clamp it.

### `maxFoldedEntries`

The maximum number of array elements and object properties that one
compile-time fold builds or carries. Defaults to `10000`. In a callback, the
ceiling applies to a product, as for
[`maxFoldedCharacters`](#maxfoldedcharacters).

This ceiling is separate, because an element as a syntax node costs much more
than a code unit as text.

```js
const options = { maxFoldedEntries: 50000 };
```

An entry costs approximately 190 bytes of peak memory. Thus the default is
approximately 2 MB. The maximum value is `1000000`. The compiler refuses a
larger value, and does not clamp it.

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

## FAQ

### Is this a drop-in replacement for `@stylexjs/babel-plugin`?

Yes, by design. It implements the same transform, is validated against the
official StyleX test suite, and produces compatible output. It also adds
compiler-only capabilities: `include`/`exclude` filtering, SWC WASM plugin
chaining, `inputSourceMap` chaining, and structured metadata output.

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
