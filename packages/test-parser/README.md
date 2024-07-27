# `test-parser`

Small CLI application that helps parse Jest tests of official [StyleX](https://github.com/facebook/stylex) repo and keeps the project up to date.

## Using

1. Compile release version of the CLI app by running next command: `pnpm --filter=@stylexswc/test-parser run build`
2. Clone official StyleX [repo](https://github.com/facebook/stylex), preferably next to this repository or update it if exist
3. Run next command `pnpm --filter=@stylexswc/test-parser start` for parsing tests
4. Check `git diff` to see updates and changes to tests
5. Coding new features

#CLI Arguments

*-p, --stylex-path <PATH>* - Absolute or relative path to cloned [StyleX](https://github.com/facebook/stylex) repository. Default value: `../../../stylex/packages`

**_NOTE:_** All parsed tests are saved in the [__tests__](https://github.com/Dwlad90/stylex-swc-plugin/tree/master/packages/test-parser/output/__tests__) directory separated by the source package name.