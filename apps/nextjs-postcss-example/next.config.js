const path = require('path');
const stylexPlugin = require('@stylexswc/nextjs-plugin');
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
      rootDir
    },
  },
  extractCSS: false,
})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
});
