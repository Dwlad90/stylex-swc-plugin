# @stylexswc/jest

> Jest transformer that compiles StyleX with a Rust compiler (NAPI-RS + SWC).
> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace.

Components that use [StyleX](https://stylexjs.com) cannot run in Jest without a
transform: `stylex.create` calls must be compiled before the test executes. This
transformer does that compilation with
[`@stylexswc/rs-compiler`](https://www.npmjs.com/package/@stylexswc/rs-compiler),
a Rust implementation of the StyleX transform, so tests see the same compiled
class names your production build produces — without pulling Babel into your
test pipeline.

This is a community project and is not affiliated with Meta. It tracks the
official StyleX releases
<!-- stylex-compatibility:start -->(currently compatible with StyleX v0.19.0)<!-- stylex-compatibility:end -->
and requires Node.js 20 or newer.

## Installation

```bash
npm install --save-dev @stylexswc/jest
```

## Configuration

Add the transformer to your Jest configuration:

```js
// jest.config.js
const path = require('path');

module.exports = {
  transform: {
    '^.+\\.(ts|tsx|js|jsx|mjs|cjs|html)$': [
      '@stylexswc/jest',
      {
        rsOptions: {
          aliases: {
            '@/*': [path.join(__dirname, '*')],
          },
          unstable_moduleResolution: {
            type: 'commonJS',
          },
        },
      },
    ],
  },
  setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
  testEnvironment: 'jsdom',
};
```

## Options

### `rsOptions`

- Type: `Partial<StyleXOptions>`
- Optional
- Description: StyleX compiler options passed to the transformer. For the
  standard options, see the
  [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

> [!NOTE]
> The `include` and `exclude` options are exclusive to the Rust compiler
> and are not available in the official StyleX Babel plugin.

#### `rsOptions.include`

- Type: `(string | RegExp)[]`
- Optional
- Description: Glob patterns or regular expressions selecting the files to
  transform. When specified, only files matching at least one pattern are
  transformed. Patterns are matched against paths relative to the current
  working directory. Regular expressions support lookahead and lookbehind.

#### `rsOptions.exclude`

- Type: `(string | RegExp)[]`
- Optional
- Description: Glob patterns or regular expressions excluding files from the
  transform. A file matching any exclude pattern is skipped even if it matches
  an `include` pattern. Patterns are matched against paths relative to the
  current working directory. Regular expressions support lookahead and
  lookbehind.

### Path Filtering Examples

Include only specific directories:

```js
{
  rsOptions: {
    include: ['src/**/*.{ts,tsx}', 'app/**/*.{ts,tsx}'],
  },
}
```

Exclude test files:

```js
{
  rsOptions: {
    exclude: ['**/*.test.*', '**/*.spec.*', '**/__tests__/**'],
  },
}
```

Exclude `node_modules` except specific packages (negative lookahead):

```js
{
  rsOptions: {
    exclude: [/node_modules(?!\/@stylexjs\/open-props)/],
  },
}
```

## Chaining with other transformers

This transformer only compiles StyleX — TypeScript and JSX still need their own
transform. Chain it with `@swc/jest` (or another transformer) using
`jest-chain-transform`:

```js
// jest.config.js
const path = require('path');

module.exports = {
  setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
  testEnvironment: 'jsdom',
  transform: {
    '^.+\\.(ts|tsx|js|jsx|mjs|cjs|html)$': [
      'jest-chain-transform',
      {
        transformers: [
          [
            '@stylexswc/jest',
            {
              rsOptions: {
                aliases: {
                  '@/*': [path.join(__dirname, '*')],
                },
                unstable_moduleResolution: {
                  type: 'commonJS',
                },
              },
            },
          ],
          [
            '@swc/jest',
            {
              jsc: {
                parser: {
                  syntax: 'typescript',
                  tsx: true,
                },
                transform: {
                  react: {
                    runtime: 'automatic',
                  },
                },
                target: 'esnext',
              },
              module: {
                type: 'es6',
              },
            },
          ],
        ],
      },
    ],
  },
};
```

A complete working configuration lives in the
[Next.js example app](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/apps/nextjs-example/jest.config.js).

## FAQ

### Why do my tests fail with "stylex.create should never be called"?

That error means the StyleX code reached the runtime uncompiled. Check that this
transformer is registered for the failing file's extension and that the file is
not excluded by your `rsOptions.include`/`exclude` patterns.

### Do I still need `babel-jest` or `@stylexjs/babel-plugin`?

No. This transformer replaces the Babel-based StyleX setup in Jest. For
TypeScript/JSX, chain it with `@swc/jest` as shown above instead of Babel.

### My theme tokens from `.stylex.ts` files don't resolve in tests

Set `unstable_moduleResolution` (usually `{ type: 'commonJS' }`) and mirror your
path aliases in `rsOptions.aliases`, exactly as in your build config.

### Is this an official StyleX package?

No. It is a community-maintained alternative to the official tooling and is not
affiliated with or supported by Meta.

## Documentation

- [StyleX documentation](https://stylexjs.com)
- [`@stylexswc/rs-compiler` compiler options](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
