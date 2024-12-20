const path = require('path');
const stylexPlugin = require('@stylexswc/nextjs-swc-plugin');
const rootDir = __dirname;

module.exports = stylexPlugin({})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
  experimental: {
    swcPlugins: [[
      "@stylexswc/swc-plugin",
      {
        dev: process.env.NODE_ENV === 'development',
        genConditionalClasses: true,
        treeshakeCompensation: true,
        aliases: {
          '@/*': [path.join(rootDir, '*')],
        },
        unstable_moduleResolution: {
          type: 'commonJS',
          rootDir: rootDir,
        },
      },
    ]],
  },
});
