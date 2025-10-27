import nextBaseConfig from '../../packages/eslint-config/next.mjs';

const appSpecificRules = [
  // Only add app-specific rules that should override the base config
];


/** @type {import('eslint').FlatConfig[]} */
const nextESlintConfg = [
  ...nextBaseConfig,
  ...appSpecificRules
];

export default nextESlintConfg;