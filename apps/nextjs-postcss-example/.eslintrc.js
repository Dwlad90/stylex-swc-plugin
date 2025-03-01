const path = require('path');


module.exports = {
  ignorePatterns: ["__tests__/**", "output/**"],
  extends: [
    'next',
    'next/core-web-vitals',
    path.join("../..", '.eslintrc.js'),

  ],
  plugins: ['@stylexjs'],
  rules: {
    // The Eslint rule still needs work, but you can
    // enable it to test things out.
    '@stylexjs/valid-styles': 'error',
    'ft-flow/space-after-type-colon': 0,
    'ft-flow/no-types-missing-file-annotation': 0,
    'ft-flow/generic-spacing': 0,
    'no-html-link-for-pages': 0,
  },
};
