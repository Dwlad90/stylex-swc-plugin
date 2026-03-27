# NAPI-RS compiler for StyleX (\*\*unofficial)

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

StyleX is a JavaScript library developed by Meta for defining styles optimized
for user interfaces. You can find the [official StyleX repository](https://www.github.com/facebook/stylex) here.

> [!WARNING] This is an unofficial style compiler for StyleX.

## Overview

This package provides an unofficial, high-performance NAPI-RS compiler for
StyleX, a popular library from Meta for building optimized user interfaces.
It is the top-level consumer crate that exposes the full StyleX pipeline to
Node.js, leveraging SWC for parsing and transformation.

> [!IMPORTANT] The usage of StyleX does not change. All changes are internal.

- Faster Build Times: By utilizing SWC instead of Babel, you can potentially
  experience significant speed improvements during StyleX processing.
- Seamless Integration: This compiler seamlessly integrates with Next.js's
  default SWC Compiler, ensuring a smooth workflow.
- Drop-in Replacement: Designed to be a drop-in replacement for the official
  StyleX Babel plugin, minimizing disruption to existing codebases.
- Advanced Tooling Capabilities: NAPI-RS compiler unlocks access to StyleX
  metadata and source maps, enabling the creation of advanced plugins and tools
  for StyleX, ex. for creating a plugin for Webpack, Rollup, or other tools.

## Advantages of a `NAPI-RS` compiler versus a `SWC plugin`

- Compability with SWC: under the hood, the NAPI-RS compiler uses SWC for
  parsing and transforming JavaScript code, ensuring compatibility with the
  latest ECMAScript features.
- Direct Access to Node.js APIs: NAPI-RS allows you to directly access Node.js
  APIs from your Rust code, providing greater flexibility and control.
- Improved Performance: NAPI-RS can often offer better performance than
  traditional Node.js addons, especially for computationally intensive tasks.
- Simplified Development: NAPI-RS simplifies the process of developing Node.js
  addons in Rust, making it easier to create high-performance and efficient
  tools.

## Architecture

- **Layer**: 9 — Compilers (top-level consumer)
- **Depends on**: `stylex-ast`, `stylex-enums`, `stylex-logs`,
  `stylex-macros`, `stylex-regex`, `stylex-structures`,
  `stylex-transform`, `stylex-types`, `stylex-utils`
- **Depended on by**: None (top-level entry point)

### Public API

- `transform()` — Main entry point: takes source code + options, returns
  transformed output
- `should_transform_file()` — File filtering based on path patterns
- `normalize_rs_options()` — Options normalization and validation

### Modules

- `enums` — Compiler-specific enum types
- `structs` — Compiler-specific struct types
- `utils::fn_parser` — Function argument parsing
- `utils::metadata` — Build metadata handling
- `utils::path_filter` — File path filtering logic

## Dependency Graph

<details>
<summary><h3>Dependency Graph</h3></summary>

```mermaid
graph TD
  subgraph L0["Primitives"]
    stylex_constants["constants"]
    stylex_regex["regex"]
    stylex_utils["utils"]
  end

  subgraph L1["Proc Macros"]
    stylex_macros["macros"]
  end

  subgraph L2["Domain Leaves"]
    stylex_enums["enums"]
    stylex_css_values["css-values"]
    stylex_js["js"]
    stylex_logs["logs"]
    stylex_css_parser["css-parser"]
    stylex_path_resolver["path-resolver"]
  end

  subgraph L3["Core Data Structures"]
    stylex_structures["structures"]
  end

  subgraph L4["Type System"]
    stylex_types["types"]
    stylex_css_utils["css-utils"]
  end

  subgraph L5["CSS Foundations & AST"]
    stylex_css_order["css-order"]
    stylex_ast["ast"]
  end

  subgraph L6["Evaluation"]
    stylex_evaluator["evaluator"]
  end

  subgraph L7["CSS Processing"]
    stylex_css["css"]
  end

  subgraph L8["StyleX Transform"]
    stylex_transform["transform"]
  end

  subgraph L9["Compilers"]
    stylex_compiler_rs["rs-compiler"]
  end

  stylex_macros        --> stylex_constants

  stylex_enums         --> stylex_macros
  stylex_css_values    --> stylex_macros
  stylex_js            --> stylex_constants
  stylex_js            --> stylex_macros
  stylex_logs          --> stylex_macros
  stylex_css_parser    --> stylex_macros
  stylex_path_resolver --> stylex_macros

  stylex_structures    --> stylex_constants
  stylex_structures    --> stylex_enums
  stylex_structures    --> stylex_macros

  stylex_types         --> stylex_constants
  stylex_types         --> stylex_enums
  stylex_types         --> stylex_macros
  stylex_types         --> stylex_structures
  stylex_types         --> stylex_utils
  stylex_css_utils     --> stylex_structures

  stylex_css_order     --> stylex_constants
  stylex_css_order     --> stylex_css_values
  stylex_css_order     --> stylex_structures
  stylex_css_order     --> stylex_types
  stylex_ast           --> stylex_constants
  stylex_ast           --> stylex_macros
  stylex_ast           --> stylex_types
  stylex_ast           --> stylex_utils

  stylex_evaluator     --> stylex_ast
  stylex_evaluator     --> stylex_constants
  stylex_evaluator     --> stylex_js
  stylex_evaluator     --> stylex_macros
  stylex_evaluator     --> stylex_path_resolver
  stylex_evaluator     --> stylex_types

  stylex_css           --> stylex_ast
  stylex_css           --> stylex_constants
  stylex_css           --> stylex_css_order
  stylex_css           --> stylex_css_parser
  stylex_css           --> stylex_css_utils
  stylex_css           --> stylex_css_values
  stylex_css           --> stylex_enums
  stylex_css           --> stylex_evaluator
  stylex_css           --> stylex_macros
  stylex_css           --> stylex_regex
  stylex_css           --> stylex_structures
  stylex_css           --> stylex_types

  stylex_transform     --> stylex_ast
  stylex_transform     --> stylex_constants
  stylex_transform     --> stylex_css
  stylex_transform     --> stylex_css_order
  stylex_transform     --> stylex_css_parser
  stylex_transform     --> stylex_css_utils
  stylex_transform     --> stylex_css_values
  stylex_transform     --> stylex_enums
  stylex_transform     --> stylex_logs
  stylex_transform     --> stylex_macros
  stylex_transform     --> stylex_path_resolver
  stylex_transform     --> stylex_regex
  stylex_transform     --> stylex_structures
  stylex_transform     --> stylex_types
  stylex_transform     --> stylex_utils

  stylex_compiler_rs   --> stylex_ast
  stylex_compiler_rs   --> stylex_enums
  stylex_compiler_rs   --> stylex_logs
  stylex_compiler_rs   --> stylex_macros
  stylex_compiler_rs   --> stylex_regex
  stylex_compiler_rs   --> stylex_structures
  stylex_compiler_rs   --> stylex_transform
  stylex_compiler_rs   --> stylex_types
  stylex_compiler_rs   --> stylex_utils

  classDef l0 fill:#e8e8e8,stroke:#999,color:#333
  classDef l1 fill:#dce8ff,stroke:#6699cc,color:#333
  classDef l2 fill:#dcf5dc,stroke:#66aa66,color:#333
  classDef l3 fill:#fff3dc,stroke:#cc9933,color:#333
  classDef l4 fill:#ffe8dc,stroke:#cc6633,color:#333
  classDef l5 fill:#f5dcff,stroke:#9933cc,color:#333
  classDef l6 fill:#dcfff5,stroke:#33aaaa,color:#333
  classDef l7 fill:#ffdcdc,stroke:#cc3333,color:#333
  classDef l8 fill:#fffdc0,stroke:#aaaa33,color:#333
  classDef l9 fill:#ffc0c0,stroke:#cc0000,color:#333

  class stylex_constants,stylex_regex,stylex_utils l0
  class stylex_macros l1
  class stylex_enums,stylex_css_values,stylex_js,stylex_logs,stylex_css_parser,stylex_path_resolver l2
  class stylex_structures l3
  class stylex_types,stylex_css_utils l4
  class stylex_css_order,stylex_ast l5
  class stylex_evaluator l6
  class stylex_css l7
  class stylex_transform l8
  class stylex_compiler_rs l9
```

</details>

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/rs-compiler
```

### Transformation Process

Internally, this compiler takes your StyleX code and transforms it into a format
optimized for further processing.

```ts
var { transform } = require('@stylexswc/rs-compiler');

/// ...other logic

const { code, metadata, sourcemap } = transform(
  filename,
  inputSourceCode,
  transformOptions
);

/// ...other logic
```

### Path Filtering

> [!NOTE] **New Feature:** The `include` and `exclude` options are exclusive to
> this NAPI-RS compiler implementation and are not available in the official
> StyleX Babel plugin. They provide powerful file filtering capabilities to
> control which files are transformed.

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

#### Pattern Types

- **Glob patterns** (strings): Use standard glob syntax to match file paths
  - `src/**/*.tsx` - All `.tsx` files in `src` directory and subdirectories
  - `**/*.test.*` - All test files
  - `**/node_modules/**` - All files in `node_modules`

- **Regular expressions**: Use RegExp objects for complex pattern matching
  - `/\.test\./` - Files containing `.test.`
  - `/^src\/.*\.tsx$/` - `.tsx` files directly in the `src` directory

  **Advanced: Lookahead/Lookbehind Support**

  The Rust regex engine fully supports lookahead and lookbehind assertions,
  enabling sophisticated filtering patterns:
  - **Negative Lookahead** `(?!...)`: Match if NOT followed by pattern
    - `/node_modules(?!\/@stylexjs)/` - Exclude all node_modules except
      @stylexjs packages
    - `/\.tsx(?!\.test)/` - Match .tsx files that are NOT test files

  - **Positive Lookahead** `(?=...)`: Match if followed by pattern
    - `/.*\.test(?=\.tsx$)/` - Match only .test.tsx files

  - **Negative Lookbehind** `(?<!...)`: Match if NOT preceded by pattern
    - `/(?<!src\/).*\.tsx$/` - Exclude .tsx files not in src/

  - **Positive Lookbehind** `(?<=...)`: Match if preceded by pattern
    - `/(?<=components\/).*\.tsx$/` - Match only .tsx files in components/

#### Filtering Rules

1. If `include` patterns are specified and not empty, files must match at least
   one pattern
2. If `exclude` patterns are specified, files matching any pattern are excluded
3. Exclude patterns take precedence over include patterns
4. All paths are matched relative to the current working directory

#### Common Use Cases

**Exclude all node_modules except specific packages:**

```ts
// Exclude all node_modules except @stylexjs/open-props
shouldTransformFile(filePath, undefined, [
  /node_modules(?!\/@stylexjs\/open-props)/,
]);
```

**Transform only specific packages from node_modules:**

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

**Exclude multiple node_modules packages except a few:**

```ts
// Exclude all node_modules except @stylexjs packages
shouldTransformFile(filePath, undefined, [/node_modules(?!\/@stylexjs)/]);
```

### SWC Plugin Support

> [!NOTE] **New Feature:** The compiler now supports running SWC WASM plugins
> before StyleX transformation. This allows you to chain transformations and
> integrate custom SWC plugins seamlessly.

The `transform` function accepts an optional `swcPlugins` array in the options
object, allowing you to run SWC WASM plugins before the StyleX transformation:

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

#### How It Works

1. **Plugin Execution Phase**: If `swcPlugins` are provided, the source code is
   first transformed using `@swc/core`'s `transformSync` with the specified WASM
   plugins
2. **StyleX Transformation Phase**: The plugin-transformed code is then passed
   to the StyleX compiler

#### Plugin Configuration

Each plugin in the `swcPlugins` array is a tuple of:

- **Plugin Path** (string): Can be:
  - An absolute path to a `.wasm` file: `/path/to/plugin.wasm`
  - An npm package name: `@swc/plugin-emotion`
- **Plugin Config** (object): Plugin-specific configuration options

#### Example: Custom Theme Plugin

```ts
transform(filename, code, {
  dev: true,
  swcPlugins: [
    [
      '/Users/me/plugins/swc_plugin_theme.wasm',
      {
        themeName: 'theme-name',
        themeConfig: {
          primaryColor: 'blue',
          spacing: 8,
        },
      },
    ],
  ],
});
```

#### Benefits

- ✅ Chain multiple transformations seamlessly
- ✅ Leverage the SWC plugin ecosystem
- ✅ Custom preprocessing before StyleX transformation
- ✅ Full compatibility with SWC WASM plugins
- ✅ No additional build configuration needed

### Output

The output from the compiler includes the transformed code, metadata about the
generated styles, and an optional source map.

```json
{
  "code": "import * as stylex from '@stylexjs/stylex';\nexport const styles = {\n    default: {\n        backgroundColor: \"xrkmrrc\",\n        color: \"xju2f9n\",\n        $$css: true\n    }\n};\n",
  "metadata": {
    "stylex": {
      "styles": [
        [
          "xrkmrrc",
          {
            "ltr": ".xrkmrrc{background-color:red}",
            "rtl": null
          },
          3000
        ],
        [
          "xju2f9n",
          {
            "ltr": ".xju2f9n{color:blue}",
            "rtl": null
          },
          3000
        ]
      ]
    }
  },
  "map": "{\"version\":3,\"sources\":[\"<anon>\"],\"names\":[],\"mappings\":\"AACE;AACA;;;;;;EAKG\"}"
}
```

## Example

Below is a simple example of input StyleX code:

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

const styleProps = stylex.props(styles.root, styles.element);
```

Output code:

```ts
import * as stylex from '@stylexjs/stylex';
const styleProps = {
  className: 'x7z7khe xrkmrrc',
};
```

## Compatibility

> [!IMPORTANT] The current resolution of the `exports` field from
> `package. json` is only partially supported, so if you encounter problems,
> please open an
> [issue](https://github.com/Dwlad90/stylex-swc-plugin/issues/new) with an
> attached link to reproduce the problem.

## Configuration Options

### `injectStylexSideEffects`

**Type:** `boolean` **Default:** `false`

Automatically injects side-effect imports for `.stylex` and `.consts` files to
prevent tree-shaking from removing them during bundling.

#### Problem

When using build tools that perform tree-shaking (like webpack, rollup, vite),
imports from `.stylex` or `.consts` files may appear unused after StyleX
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

The bundler may remove these "unused" imports, but they're needed for other
files to resolve the same StyleX/const references correctly.

#### Solution

When `injectStylexSideEffects: true`, the compiler automatically adds
side-effect imports to preserve these modules:

```ts
// After transformation with injectStylexSideEffects: true
import { colors } from './theme.stylex';
import { spacing } from './tokens.consts';
import './theme.stylex'; // Side-effect import (prevents tree-shaking)
import './tokens.consts'; // Side-effect import (prevents tree-shaking)

const styles = {
  root: {
    backgroundColor: 'x1a2b3c',
    padding: 'x4d5e6f',
    $$css: true,
  },
};
```

#### When to Use

- ✅ **Use `true`** when your bundler runs StyleX transformation **before**
  other optimizations (recommended)
- ✅ **Use `true`** with webpack's `loaderOrder: 'first'` option
- ❌ **Use `false`** when StyleX runs **after** tree-shaking (e.g., webpack's
  `loaderOrder: 'last'`)

> [!TIP] This option is automatically enabled when using
> `@stylexswc/webpack-plugin` with `loaderOrder: 'first'` (the default).

### `useRealFileForSource`

**Type:** `boolean` **Default:** `true`

Controls whether the compiler should read source files from disk for error
reporting and source map generation.

#### Behavior

- **`true` (default)**: The compiler reads the actual source file from disk when
  generating error messages and source maps. This provides accurate line numbers
  and source context that match what you see in your editor.

- **`false`**: The compiler uses the transformed AST representation for error
  reporting. This is useful when:
  - Working with in-memory transformations
  - Source files are not available on disk
  - You want faster compilation (skips file I/O)

#### Example

```ts
transform(filename, code, {
  use_real_file_for_source: true, // Use actual source files (default)
  dev: true,
  // ... other options
});
```

#### Use Cases

**Use `true` (recommended for development):**

- Local development with files on disk
- Accurate error messages with real line numbers
- Better debugging experience
- Source maps match your actual files

**Use `false` (for special cases):**

- In-memory transformations without disk access
- Virtual file systems
- Performance optimization when error accuracy is less critical
- Build pipelines where source files are not available

> [!TIP] Keep the default `true` value for most use cases. Only set it to
> `false` if you have specific requirements for in-memory transformations or
> performance-critical scenarios where file I/O is a bottleneck.

> [!WARNING] When `useRealFileForSource` is set to `false`, error messages may
> report **incorrect line numbers**. The compiler will use the transformed AST
> representation instead of the original source code, which can lead to line
> number mismatches. This happens because:
>
> - The AST may have been modified by previous transformations
> - Comments and whitespace are normalized in the AST
> - The structure may differ from what's in your actual source file
>
> For accurate error reporting and debugging, always use
> `useRealFileForSource: true` (the default) during development.

## Debug

You can enable debug logging for the StyleX compiler using the `STYLEX_DEBUG`
environment variable. This is useful for troubleshooting and understanding the
internal processing of StyleX code.

### Log Levels

The following log levels are available:

- `error`: Only shows error messages
- `warn`: Shows warnings and errors (default)
- `info`: Shows informational messages, warnings, and errors
- `debug`: Shows debug information and all above levels
- `trace`: Shows very detailed execution information

### Usage

Set the environment variable before running your build command:

```bash
# Set to debug level
STYLEX_DEBUG=debug npm run build

# Set to trace for most verbose output
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

The compiler produces clean, structured error messages with a branded `[StyleX]`
prefix, replacing Rust's default panic boilerplate with user-friendly
diagnostics in both the terminal and at the NAPI boundary.

### Error Format

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

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
