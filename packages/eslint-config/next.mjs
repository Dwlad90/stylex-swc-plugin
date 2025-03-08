import turboConfig from "eslint-config-turbo/flat";
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

const filteredBaseConfig = baseConfig.map(config => {
  if (config.plugins?.['@typescript-eslint']) {
    // Remove the typescript-eslint plugin from the base config
    const { plugins, ...rest } = config;
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const { '@typescript-eslint': _, ...newPlugins } = { ...plugins };  // Use _ instead of _a

    return {
      ...rest,
      plugins: Object.keys(newPlugins).length > 0 ? newPlugins : {}
    };
  }

  if (config?.name?.startsWith('typescript-eslint')) {
    // Remove the typescript-eslint plugin from the base config
    return {}
  }

  return config;
});

/** @type {import("eslint").FlatConfig[]} */
const nextBaseEslintConfig = [
  prettierConfig,
  ...eslintConfig,
  ...turboConfig,
  ...filteredBaseConfig,
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