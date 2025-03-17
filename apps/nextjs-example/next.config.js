/* eslint-disable @typescript-eslint/no-require-imports */
const path = require('path');
const stylexPlugin = require('@stylexswc/nextjs-plugin');
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
  }
})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
});
