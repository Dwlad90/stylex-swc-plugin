import storybookPlugin from 'eslint-plugin-storybook'
import baseConfig from '../../eslint.config.mjs';

const appSpecificRules = [
  {
    name: "storybook",
    files: ["**/*.stories.ts?(x)", "**/*.mdx", "**/*.(mjs|ts|tsx)"],
    plugins: {
      ...storybookPlugin.configs['flat/recommended'],
    },
  },
  // Only add app-specific rules that should override the base config
];

/** @type {import('eslint').FlatConfig[]} */
const nextElintConfg = [
  ...baseConfig,
  ...appSpecificRules,
];

export default nextElintConfg;