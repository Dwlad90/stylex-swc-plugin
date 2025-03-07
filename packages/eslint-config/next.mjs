import { resolve } from "node:path";
import js from "@eslint/js";
import turboConfig from "eslint-config-turbo/flat";
import globals from "globals";
import prettierConfig from "eslint-config-prettier/flat";
import stylexPlugin from '@stylexjs/eslint-plugin';
import { FlatCompat } from '@eslint/eslintrc';
import baseConfig from '../../eslint.config.mjs';


const compat = new FlatCompat({
  baseDirectory: import.meta.dirname,
})

const eslintConfig = [
  ...compat.config({
    extends: ['eslint-config-next', 'next/core-web-vitals', 'next/typescript'],
  }),
]

const project = resolve(process.cwd(), "tsconfig.json");

/** @type {import("eslint").FlatConfig[]} */
const nextBaseEslintConfig = [
  {
    name: "next:base",
    linterOptions: {
      reportUnusedDisableDirectives: true,
    },
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
        React: true,
        JSX: true,
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
      "__tests__/**",
      "output/**"
    ],
  },
  js.configs.recommended,
  prettierConfig,
  ...eslintConfig,
  ...turboConfig,
  ...baseConfig,
  {
    name: "next:js-ts",
    files: ["*.js?(x)", "*.ts?(x)"],
  },
  {
    name: "next:stylex",
    plugins: {
      '@stylexjs': stylexPlugin,
    },
    rules: {
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          vars: 'all',
          args: 'after-used',
          ignoreRestSiblings: true,
          argsIgnorePattern: '^_',
          caughtErrors: 'none',
        },
      ],
      '@stylexjs/valid-styles': 'error',
      'ft-flow/space-after-type-colon': 0,
      'ft-flow/no-types-missing-file-annotation': 0,
      'ft-flow/generic-spacing': 0,
      'no-html-link-for-pages': 0,
      '@next/next/no-duplicate-head': 0
    },
  }
];

export default nextBaseEslintConfig;