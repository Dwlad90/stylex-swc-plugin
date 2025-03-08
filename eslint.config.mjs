import stylexLibraryConfig from '@stylexswc/eslint-config/library.mjs';
import stylexJsonConfig from '@stylexswc/eslint-config/json.mjs';
import tsParser from '@typescript-eslint/parser';

/** @type {import("eslint").FlatConfig[]} */
const eslintConfig = [{
  name: "base",
  ignores: ["__tests__/**", "output/**"],
  languageOptions: {
    parser: tsParser,
    parserOptions: {
      ecmaVersion: 2021,
      sourceType: 'module',
      ecmaFeatures: {
        jsx: true,
      },
      warnOnUnsupportedTypeScriptVersion: true,
    },
  },
},
...stylexLibraryConfig,
...stylexJsonConfig,
{
  ignores: ["**/dist", "**/node_modules", "**/output", "**/fixture", "**/__swc_snapshots__", "**/__snapshots__", "**/.next"],
},
];

export default eslintConfig;