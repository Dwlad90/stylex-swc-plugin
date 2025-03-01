# NAPI-RS compiler for StyleX (\*\*unofficial)

StyleX is a JavaScript library developed by Meta for defining styles optimized
for user interfaces. You can find the official repository
[here](https://www.github.com/facebook/stylex).

> [!WARNING]
> This is an unofficial style compiler for StyleX.

## Overview

This package provides an unofficial, high-performance compiler for StyleX, a
popular library from Meta for building optimized user interfaces. It leverages
the power of NAPI-RS and SWC to achieve several key advantages:

> [!IMPORTANT]
> The usage of StyleX does not change. All changes are internal.

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

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/rs-compiler
```

### Transformation Process

Internally, this compiler takes your StyleX code and transforms it into a format
optimized for further processing.

```ts
var { transform } = require('@stylexswc/compiler-rs');

/// ...other logic

const { code, metadata, sourcemap } = transform(
  filename,
  inputSourceCode,
  transformOptions
);

/// ...other logic
```

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

> [!IMPORTANT]
> The current resolution of the `exports` field from
> `package. json` is only partially supported, so if you encounter problems,
> please open an
> [issue](https://github.com/Dwlad90/stylex-swc-plugin/issues/new) with an
> attached link to reproduce the problem.

## License

StyleX is MIT licensed. StyleX NAPI-RS compiler is also MIT licensed.
