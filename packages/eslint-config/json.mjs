import jsoncPlugin from 'eslint-plugin-jsonc';
import jsoncParser from 'jsonc-eslint-parser';

/** @type {import("eslint").FlatConfig[]} */
export default [
  {
    name: 'json:base',
    plugins: {
      jsonc: jsoncPlugin,
    },
    languageOptions: {
      parser: jsoncParser,
    },
    files: ['**/*.json'],
    rules: {
      ...jsoncPlugin.configs['recommended-with-json'].rules,
    },
  },
  {
    name: 'json:config-files',
    files: [
      '**/.eslintrc.json',
      '**/tsconfig.*.json',
      '**/tsconfig.json',
    ],
    rules: {
      'jsonc/no-comments': 'off',
      'max-len': 'off',
    },
  },
];