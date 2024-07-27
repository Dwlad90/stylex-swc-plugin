# StyleX SWC plugin

This is a monorepo for an unofficial [SWC](https://swc.rs/) plugin for [StyleX](https://github.com/facebook/stylex). Using SWC allows us to completely ditch Babel and make StyleX faster.

This plugin passes almost 100% of tests of the official StyleX library. It is intended as a drop-in replacement for the official StyleX babel plugin.

This is specifically useful for Next.js projets as it allows us to use [SWC Next.js Compiler](https://nextjs.org/docs/architecture/nextjs-compiler).



## Packages

- [`eslint-config`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/eslint-config) - internal [ESLint](https://eslint.org/) configuration
- [`nextjs-plugin`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/nextjs-plugin) - wrapper for [`Next.JS config`](https://nextjs.org/docs/app/api-reference/next-config-js) that inject the StyleX SWC plugin to webpack processing
- [`swc-plugin`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/swc-plugin) - unofficial SWC implementation of the native [StyleX](https://github.com/facebook/stylex) plugin
- [`test-parser`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/test-parser) - parser for [StyleX](https://github.com/facebook/stylex) repo Jest tests that helps to understand last changes and keeps the project up to date
- [`typescript-config`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/typescript-config) - internal [Typescript](https://www.typescriptlang.org/docs/handbook/tsconfig-json.htm) configuration