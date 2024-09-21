# SWC Stylex plugin for Next.js

A Next.js plugin for the community-maintained
[`StyleX SWC`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-swc-plugin)
plugin.

## Why SWC instead of Babel

Since version 12, Next.js uses SWC Compiler by default.
[According to Vercel](https://nextjs.org/docs/architecture/nextjs-compiler),
SWC compiles JavaScript up to 17x faster than Babel.

However, if you have a Babel configconfiguration, Next.js will disable the SWC Compiler and continue using Babel.

This plugin lets you use StyleX with the SWC Compiler, so you can take full advantage of its speed and efficiency.

**The usage of StyleX does not change**, all changes are internal. All you need
to do, is install SWC StyleX plugin and update Next.js config.

## Installation

Install the package and SWC plugin by using:

```bash
npm install --save-dev @stylexswc/nextjs-plugin
```

Please install `@stylexswc/swc-plugin` if you haven't done so already:

```bash
npm install --save-dev @stylexswc/swc-plugin
```

## Usage

Modify Next.js config. Here’s an example:

```js
/** @type {import('next').NextConfig} */
const stylexPlugin = require('@stylexswc/nextjs-plugin');

const nextConfig = {
  // Configure `pageExtensions` to include MDX files. Include custom page extensions if needed
  pageExtensions: ['js', 'jsx', 'mdx', 'ts', 'tsx'],
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
  swcMinify: true,
  experimental: {
    swcPlugins: [
      '@stylexswc/swc-plugin',
      {
        dev: false,
        runtimeInjection: false,
        genConditionalClasses: true,
        treeshakeCompensation: true,
        unstable_moduleResolution: {
          type: 'commonJS',
          rootDir: __dirname,
        },
      },
    ],
  },
};

module.exports = stylexPlugin({
  rootDir: __dirname,
})(nextConfig);
```

## Examples

- [Example repo](https://github.com/Dwlad90/nextjs-app-dir-stylex)
- [CodeSandbox with example repo](https://codesandbox.io/p/github/Dwlad90/nextjs-app-dir-stylex/main)

## Documentation

- [StyleX Documentation](https://stylexjs.com)
- [SWC plugin for StyleX](https://github.com/Dwlad90/stylex-swc-plugin/tree/master/packages/swc-plugin)
