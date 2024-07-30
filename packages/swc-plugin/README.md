# SWC plugin for StyleX (\*\*unofficial)

StyleX is a JavaScript library for defining styles for optimized user
interfaces, developed by Meta. The official repo is
[here](https://www.github.com/facebook/stylex).

**This is an unofficial plugin** for StyleX. It uses SWC instead of Babel for
build step, which allows us to completely ditch Babel and make StyleX faster.

This plugin passes almost 100% of tests of the official StyleX library. It is
intended as a drop-in replacement for the official StyleX babel plugin.

**The usage of StyleX does not change**, all changes are internal.

This is specifically useful for Next.js projets as it allows us to use
[SWC Next.js Compiler](https://nextjs.org/docs/architecture/nextjs-compiler).

- [Next.js plugin](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/nextjs-plugin)
- [StyleX Documentation](https://stylexjs.com)

## Installation

Install the package by using:

```bash
npm install --save-dev @stylexswc/nextjs-plugin
```

## Example

Here is a simple example of StyleX use:

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

## License

StyleX is MIT licensed. Stylex SWC plugin is also MIT licensed.
