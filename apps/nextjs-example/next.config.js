const path = require('path');
/** @type {import('next').NextConfig} */
const stylexPlugin = require('@stylexswc/nextjs-plugin');
const rootDir = __dirname;

module.exports = stylexPlugin({
  rootDir,
  // Add any Stylex options here
  dev: process.env.NODE_ENV === 'development',
  genConditionalClasses: true,
  treeshakeCompensation: true,
  aliases: {
    '@/*': [
      path.join(rootDir, '*'),
    ],
  },
  unstable_moduleResolution: {
    type: 'commonJS',
    rootDir
  },
})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
  swcMinify: true,
});
