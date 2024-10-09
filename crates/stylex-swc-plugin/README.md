# SWC plugin for StyleX (\*\*unofficial)

> [!WARNING]
> **Deprecated**: This package is deprecated as of version `0.3.0` and may be removed in the future. Please use the [`rs-compiler`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-rs-compiler) instead.

## Overview

StyleX is a JavaScript library developed by Meta for defining styles optimized for user interfaces. You can find the official repository [here](https://www.github.com/facebook/stylex).

>**This is an unofficial plugin** for StyleX. It uses SWC instead of Babel for
build step, which allows us to completely ditch Babel and make StyleX faster.

This plugin successfully passes almost all tests from the official StyleX library and is designed to be a drop-in replacement for the official StyleX Babel plugin.


**The usage of StyleX does not change**, all changes are internal.

This plugin is particularly beneficial for Next.js projets as it allows the use of the [SWC Next.js Compiler](https://nextjs.org/docs/architecture/nextjs-compiler).

* [Next.js plugin](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/nextjs-plugin)
* [StyleX Documentation](https://stylexjs.com)

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/swc-plugin
```

## Example

Below is a simple example of using StyleX:

```ts
import * as stylex from '@stylexjs/stylex';

const styles = stylex.create({
  root: {
    padding: 10,
  },
  element: {
    backgroundColor: 'red',
  },
});

const styleProps = stylex.props(styles.root, styles.element);
```

> [!IMPORTANT]
> The current resolution of the `exports` field from `package. json` is only partially supported, so if you encounter problems, please open an [issue](https://github.com/Dwlad90/stylex-swc-plugin/issues/new) with an attached link to reproduce the problem.

## License

StyleX is MIT licensed. Stylex SWC plugin is also MIT licensed.
