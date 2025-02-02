# @stylexswc/jest

Jest transformer that enables StyleX SWC integration for JavaScript and
TypeScript testing with Jest.

## Overview

The `@stylexswc/jest` package provides a Jest transformer that integrates with
the StyleX RS compiler. This allows to transform source code using StyleX during
Jest tests, ensuring that styles are correctly processed and applied.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/jest
```

## Configuration

To use the transformer, add it to Jest configuration. Here is an example
configuration:

```javascript
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
- Description: StyleX compiler options that will be passed to the transformer.
  See
  [StyleX configuration docs](https://stylexjs.com/docs/api/configuration/babel-plugin/)
  for details.

## Example

Here is an example of how to use `@swc/jest` with other transformers:

```javascript
// jest.config.js
const nextJest = require('next/jest');
const path = require('path');

const createJestConfig = nextJest({
  dir: process.cwd(),
});

const customJestConfig = {
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
              $schema: 'https://json.schemastore.org/swcrc',
              jsc: {
                parser: {
                  syntax: 'typescript',
                  tsx: true,
                  dynamicImport: true,
                  decorators: true,
                  dts: true,
                },
                transform: {
                  react: {
                    useBuiltins: true,
                    runtime: 'automatic',
                  },
                },
                target: 'esnext',
                loose: false,
                externalHelpers: false,
                keepClassNames: true,
                baseUrl: './',
                paths: {
                  '@/*': ['./*'],
                },
              },
              module: {
                type: 'es6',
              },
              minify: false,
            },
          ],
        ],
      },
    ],
  },
};

module.exports = customJestConfig;
```

Real example can be found in the
[@stylexswc/next-example](../../apps/nextjs-example/jest.config.js)

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE)
file for details.
