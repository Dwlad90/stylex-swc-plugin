# SWC plugin for StyleX (\*\*unofficial)

> [!CAUTION]
> **DEPRECATED**: This package is deprecated as of version 0.5.0. Please migrate to [`@stylexswc/rs-compiler`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler) which is the official replacement.

## Overview

StyleX is a JavaScript library developed by Meta for defining styles optimized for user interfaces. You can find the official repository [here](https://www.github.com/facebook/stylex).

>**This is an unofficial plugin** for StyleX. It uses SWC instead of Babel for
build step, which allows us to completely ditch Babel and make StyleX faster.

This plugin successfully passes almost all tests from the official StyleX library and is designed to be a drop-in replacement for the official StyleX Babel plugin.


**The usage of StyleX does not change**, all changes are internal.

This plugin is particularly beneficial for Next.js projets as it allows the use of the [SWC Next.js Compiler](https://nextjs.org/docs/architecture/nextjs-compiler).

* [Next.js plugin](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/packages/nextjs-plugin)
* [StyleX Documentation](https://stylexjs.com)

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/swc-plugin
```


## Usage

Modify your bundler configuration to use the StyleX SWC plugin.

> [!NOTE]
> All awailable options the same as in the official StyleX Babel plugin
> and can be found on the
> [StyleX babel plugin documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/)
> page.

For example:

* Register SWC plugin in Next.js config:

```ts
module.exports = {
  experimental: {
    swcPlugins: [[
      "@stylexswc/swc-plugin",
      {
        dev: process.env.NODE_ENV === 'development',
        genConditionalClasses: true,
        treeshakeCompensation: true,
        aliases: {
          '@/*': [path.join(rootDir, '*')],
        },
        unstable_moduleResolution: {
          type: 'commonJS',
          rootDir: rootDir,
        },
      },
    ]],
  },
};
```

* Register SWC plugin in Webpack config:

```ts
module.exports = {
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: [
          {
            loader: 'swc-loader',
            options: {
              jsc: {
                experimental: {
                  plugins: [
                    [
                      '@stylexswc/swc-plugin',
                      {
                        dev: process.env.NODE_ENV === 'development',
                        genConditionalClasses: true,
                        treeshakeCompensation: true,
                        aliases: {
                          '@/*': [path.join(rootDir, '*')],
                        },
                        unstable_moduleResolution: {
                          type: 'commonJS',
                          rootDir: rootDir,
                        },
                        // ... other options
                      },
                    ],
                  ],
                },
              },
            },
          },
        ],
      },
    ],
  },
};
```

* Regiter SWC plugin in Rsbuild config:

```ts
export default {
  tools: {
    swc: {
      jsc: {
        experimental: {
          plugins: [
            [
              '@swc/plugin-styled-components',
              {
                dev: process.env.NODE_ENV === 'development',
                genConditionalClasses: true,
                treeshakeCompensation: true,
                aliases: {
                  '@/*': [path.join(rootDir, '*')],
                },
                unstable_moduleResolution: {
                  type: 'commonJS',
                  rootDir: rootDir,
                },
                // ... other options
              },
            ],
          ],
        },
      },
    },
  },
};
```

## Working with Metadata

Since SWC does not support receiving metadata after transformation, the process
of extracting CSS styles is a bit tricky and is based on searching for a
substring of metadata in the compiled application file and serializing it into
JSON.

To extract metadate from compiled code, you need to add the following code to your build script:

```ts
let metadataStr = '[]';

const code = sourceCodeString.replace(
  /\/\/*__stylex_metadata_start__(?<metadata>.+)__stylex_metadata_end__/,
  (...args) => {
    metadataStr = args.at(-1)?.metadata.split('"__stylex_metadata_end__')[0];

    return '';
  }
);

const metadata = { stylex: [] };

try {
  metadata.stylex = JSON.parse(metadataStr);
} catch (e) {
  console.error('error parsing metadata', e);
}
```

Example of metadata:

```json
[
  {
    "class_name": "x7z7khe",
    "style": {
      "rtl": null,
      "ltr": ".x7z7khe{padding:10px}"
    },
    "priority": 1000
  },
  {
    "class_name": "xrkmrrc",
    "style": {
      "rtl": null,
      "ltr": ".xrkmrrc{background-color:red}"
    },
    "priority": 3000
  }
]
```

Metadata can be used to extract CSS styles from the compiled code.

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
//__stylex_metadata_start__[{"class_name":"x7z7khe","style":{"rtl":null,"ltr":".x7z7khe{padding:10px}"},"priority":1000},{"class_name":"xrkmrrc","style":{"rtl":null,"ltr":".xrkmrrc{background-color:red}"},"priority":3000}]__stylex_metadata_end__
import * as stylex from '@stylexjs/stylex';
const styleProps = {
  className: 'x7z7khe xrkmrrc',
};
```

> [!IMPORTANT]
> The current resolution of the `exports` field from `package. json` is only partially supported, so if you encounter problems, please open an [issue](https://github.com/Dwlad90/stylex-swc-plugin/issues/new) with an attached link to reproduce the problem.

## License

StyleX is MIT licensed. Stylex SWC plugin is also MIT licensed.
