import turboConfig from "eslint-config-turbo/flat";
import prettierConfig from "eslint-config-prettier/flat";
import stylexPlugin from '@stylexjs/eslint-plugin';
import baseConfig from '../../eslint.config.mjs';
import nextVitals from 'eslint-config-next/core-web-vitals'
import nextTypescript from 'eslint-config-next/typescript'
import { globalIgnores } from 'eslint/config'


const filteredBaseConfig = baseConfig.map(config => {
  if (config.plugins?.['@typescript-eslint']) {
    // Remove the typescript-eslint plugin from the base config
    const { plugins, ...rest } = config;
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const { '@typescript-eslint': _, ...newPlugins } = { ...plugins };

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

/** @type {import("eslint").Linter.Config[]} */
const nextBaseEslintConfig = [
  prettierConfig,
  ...nextVitals,
  ...nextTypescript,
  {
    name: "next:react-internal",
    settings: {
      react: { version: "19" },
    },
  },
  ...turboConfig,
  ...filteredBaseConfig,
  {
    name: "next:js-ts",
    files: ["**/*.{js,jsx,ts,tsx}"],
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
  },
  globalIgnores([
    // Default ignores of eslint-config-next:
    '.next/**',
    'out/**',
    'build/**',
    'next-env.d.ts',
  ]),
];

export default nextBaseEslintConfig;
