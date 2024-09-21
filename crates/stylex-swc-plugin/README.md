# SWC plugin for StyleX (Community-Maintained)

StyleX is a JavaScript library for defining styles for optimized user
interfaces, developed by Meta (formerly Facebook). The official repo is
[here](https://www.github.com/facebook/stylex).

>**This is a Community-Maintained plugin** for StyleX.
>>This plugin replaces Babel with SWC in the build process, significantly improving performance by reducing build times and optimizing the final bundle size, making StyleX faster and more efficient.

This plugin successfully passes the majority of tests from the official StyleX library, ensuring high compatibility, and, is intended as a drop-in replacement for the official StyleX babel plugin.

**The usage of StyleX does not change**, all changes are internal.

This is specifically useful for Next.js projects as it allows us to use
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

> **Warning**
>
> The current resolution of the `exports` field from `package. json` is only partially supported, so if you encounter problems, please open an [issue](https://github.com/Dwlad90/stylex-swc-plugin/issues/new) with an attached link to reproduce the problem.

## License

StyleX is MIT licensed. Stylex SWC plugin is also MIT licensed.
