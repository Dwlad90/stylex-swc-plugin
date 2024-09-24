/** @type {import('next').NextConfig} */
const path = require('path');
const stylexPlugin = require('@stylexswc/nextjs-swc-plugin');
const rootDir =  __dirname;

module.exports = stylexPlugin({ rootDir })({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
  swcMinify: true,
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
