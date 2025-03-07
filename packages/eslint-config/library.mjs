import { resolve } from 'node:path';
import globals from 'globals';

const project = resolve(process.cwd(), "tsconfig.json");

/** @type {import("eslint").FlatConfig[]} */
export default [
  {
    name: "library:base",
    linterOptions: {
      reportUnusedDisableDirectives: true,
    },
    languageOptions: {
      globals: {
        React: true,
        JSX: true,
        ...globals.browser,
        ...globals.node,
      },
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module"
      }
    },
    settings: {
      "import/resolver": {
        typescript: {
          project,
        },
      },
    },
    ignores: [
      ".*.js",
      "node_modules/",
      "dist/",
    ],
    rules: {
      'no-unused-vars': [
        'error',
        {
          vars: 'all',
          args: 'after-used',
          ignoreRestSiblings: true,
          argsIgnorePattern: '^_',
          caughtErrors: 'none',
        },
      ],
    }
  },
  {
    name: "library:js-ts",
    files: ["**/*.js", "**/*.jsx", "**/*.ts", "**/*.tsx"],
  }
];