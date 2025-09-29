/* eslint-disable @typescript-eslint/no-require-imports */
const path = require('path');
const stylexPlugin = require('@toss/stylexswc-nextjs-plugin');
/* eslint-enable @typescript-eslint/no-require-imports */

const rootDir = __dirname;

module.exports = stylexPlugin({
  // Add any StyleX options here
  rsOptions: {
    aliases: {
      '@/*': [
        path.join(rootDir, '*'),
      ],
    },
    unstable_moduleResolution: {
      type: 'commonJS',
    },
    isDev: process.env.NODE_ENV === 'development',
    runtimeInjection: false,
    treeshakeCompensation: true,
  },
  extractCSS: false,
})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
});

