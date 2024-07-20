# StyleX SWC plugin

This is a monorepo for an unofficial [SWC](https://swc.rs/) plugin for [StyleX](https://github.com/facebook/stylex). Using SWC allows us to completely ditch Babel and make StyleX faster.

This plugin passes 100% of tests of the offical StyleX library. It is intended as a drop-in replacement for the official Babel-based StyleX library.

This is specifically useful for Next.js projets as it allows us to use [SWC Next.js Compiler](https://nextjs.org/docs/architecture/nextjs-compiler).



## Packages

- [`eslint-config`](https://github.com/talovski/stylex-swc-plugin/tree/master/packages/eslint-config)
- [`nextjs-plugin`](https://github.com/talovski/stylex-swc-plugin/tree/master/packages/nextjs-plugin)
- [`swc-plugin`](https://github.com/talovski/stylex-swc-plugin/tree/master/packages/swc-plugin)
- [`test-parser`](https://github.com/talovski/stylex-swc-plugin/tree/master/packages/test-parser)
- [`typescript-config`](https://github.com/talovski/stylex-swc-plugin/tree/master/packages/typescript-config)
