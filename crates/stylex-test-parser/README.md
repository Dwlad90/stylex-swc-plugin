# `StyleX Test Parser`

This project offers a command-line application (CLI) to parse Jest tests from
the official [StyleX](https://github.com/facebook/stylex) repository. This tool
helps maintain compatibility between the unofficial StyleX SWC plugin and the
official StyleX library by detecting changes in StyleX tests.

## Features

- Test Parsing: Extracts tests from the official StyleX repository.
- Compatibility Checks: Assists in ensuring compatibility between StyleX SWC
  plugin and official StyleX tests.
- Version Tracking: Enables you to stay updated with changes in StyleX tests and
  features.

## Using the CLI

1. Compile release version of the CLI app by running next command:
   `pnpm --filter=@stylexswc/test-parser run build`
2. Clone official StyleX [repo](https://github.com/facebook/stylex), preferably
   next to this repository or update it if exist
3. Run next command `pnpm --filter=@stylexswc/test-parser start` for parsing
   tests
4. Check `git diff` to see updates and changes to tests
5. Coding new features

## CLI Arguments

_-p, --stylex-path `PATH`_ - Absolute or relative path to cloned
[StyleX](https://github.com/facebook/stylex) repository. Default value:
`../../../stylex/packages`

> [!NOTE]
> All parsed tests are saved in the
> [**tests**](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-test-parser/output/__tests__)
> directory separated by the source package name.
