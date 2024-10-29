# Next.js plugin with NAPI-RS StyleX compiler integration

Next.js plugin for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

## Overview

This package combines two solutions to enhance your Next.js development experience with StyleX:

### StyleX SWC Plugin

* Integrates StyleX with the SWC compiler, potentially leading to faster build times compared to using Babel.
* Maintains high compatibility with official StyleX tests, ensuring a reliable experience.
* Integrates seamlessly with Next.js SWC Compiler for a streamlined workflow.

### Stylex NAPI-RS Compiler

* Utilizes NAPI-RS to compile StyleX code, offering advantages over the SWC plugin approach.
* Provides access to StyleX metadata and source maps, enabling advanced plugin and tool development.

## Why choose this approach?

* Leverage SWC's speed: Benefit from Next.js's default SWC compiler for potentially faster build times.
* Maintain StyleX familiarity: The usage of StyleX remains unchanged for developers.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/nextjs-plugin
```

Please install `@stylexswc/rs-compiler` if you haven't done so already:

```bash
npm install --save-dev @stylexswc/rs-compiler
```

## Usage

Modify Next.js config. For example:

```js
/** @type {import('next').NextConfig} */
const stylexPlugin = require('@stylexswc/nextjs-plugin');

const nextConfig = {
  // Configure `pageExtensions` to include MDX files
  pageExtensions: ['js', 'jsx', 'mdx', 'ts', 'tsx'],
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
  swcMinify: true,
};

module.exports = stylexPlugin({
  rootDir: __dirname,
  // Stylex RS compiler options
  dev: false,
  runtimeInjection: false,
  genConditionalClasses: true,
  treeshakeCompensation: true,
  unstable_moduleResolution: {
    type: 'commonJS',
    rootDir: __dirname,
  },
})(nextConfig);
```

## Examples

* [Example repo](https://github.com/Dwlad90/nextjs-app-dir-stylex)
* [CodeSandbox with example repo](https://codesandbox.io/p/github/Dwlad90/nextjs-app-dir-stylex/main)

## Documentation

* [StyleX Documentation](https://stylexjs.com)
* [NAPI-RS compiler for StyleX](https://github.com/Dwlad90/stylex-swc-plugin/tree/master/crates/stylex-rs-compiler)