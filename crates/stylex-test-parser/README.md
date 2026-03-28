# StyleX Test Parser

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Standalone CLI tool that parses Jest tests from the official
[StyleX](https://github.com/facebook/stylex) repository to maintain
compatibility between this unofficial SWC plugin and Meta's official StyleX
library. It is **not** part of the compiler pipeline DAG — it has zero internal
crate dependencies and nothing depends on it. The tool exists purely as a
development-time utility to keep the SWC plugin's test suite synchronised with
upstream changes in the official StyleX repo.

- **Automated test extraction** — walks the official StyleX repository tree and
  extracts Jest test files, converting them into a format consumable by this
  workspace's test harness.
- **Compatibility tracking** — by re-running the parser after upstream StyleX
  releases, developers can immediately see which tests have been added,
  modified, or removed via `git diff`.
- **Version awareness** — enables the team to stay current with new StyleX
  features and API changes by surfacing test-level deltas.
- **Zero internal dependencies** — the tool depends only on external crates
  (e.g., `clap`, `serde`, standard I/O) and can be built and run independently
  of the compiler pipeline.

## Architecture

- **Layer**: — _(standalone CLI, not part of the compiler pipeline DAG)_
- **Depends on**: None (no internal workspace crate dependencies)
- **Depended on by**: None

## Features

- **Test Parsing**: Extracts tests from the official StyleX repository.
- **Compatibility Checks**: Assists in ensuring compatibility between the StyleX
  SWC plugin and official StyleX tests.
- **Version Tracking**: Enables you to stay updated with changes in StyleX tests
  and features.

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

> [!NOTE] All parsed tests are saved in the
> [**tests**](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-test-parser/output/__tests__)
> directory separated by the source package name.

## Dependency Graph

<details>
<summary><h3>Dependency Graph</h3></summary>

```mermaid
graph TD
  subgraph L0["Primitives"]
    stylex_constants["constants"]
    stylex_regex["regex"]
    stylex_utils["utils"]
  end

  subgraph L1["Proc Macros"]
    stylex_macros["macros"]
  end

  subgraph L2["Domain Leaves"]
    stylex_enums["enums"]
    stylex_js["js"]
    stylex_logs["logs"]
    stylex_css_parser["css-parser"]
    stylex_path_resolver["path-resolver"]
  end

  subgraph L3["Core Data Structures"]
    stylex_structures["structures"]
  end

  subgraph L4["Type System"]
    stylex_types["types"]
  end

  subgraph L5["AST Foundations"]
    stylex_ast["ast"]
  end

  subgraph L6["Evaluation"]
    stylex_evaluator["evaluator"]
  end

  subgraph L7["CSS Processing"]
    stylex_css["css"]
  end

  subgraph L8["StyleX Transform"]
    stylex_transform["transform"]
  end

  subgraph L9["Compilers"]
    stylex_compiler_rs["rs-compiler"]
  end

  stylex_macros        --> stylex_constants

  stylex_enums         --> stylex_macros
  stylex_js            --> stylex_constants
  stylex_js            --> stylex_macros
  stylex_logs          --> stylex_macros
  stylex_css_parser    --> stylex_macros
  stylex_path_resolver --> stylex_macros

  stylex_structures    --> stylex_constants
  stylex_structures    --> stylex_enums
  stylex_structures    --> stylex_macros

  stylex_types         --> stylex_constants
  stylex_types         --> stylex_enums
  stylex_types         --> stylex_macros
  stylex_types         --> stylex_structures
  stylex_types         --> stylex_utils

  stylex_ast           --> stylex_constants
  stylex_ast           --> stylex_macros
  stylex_ast           --> stylex_types
  stylex_ast           --> stylex_utils

  stylex_evaluator     --> stylex_ast
  stylex_evaluator     --> stylex_constants
  stylex_evaluator     --> stylex_js
  stylex_evaluator     --> stylex_macros
  stylex_evaluator     --> stylex_path_resolver
  stylex_evaluator     --> stylex_types

  stylex_css           --> stylex_ast
  stylex_css           --> stylex_constants
  stylex_css           --> stylex_css_parser
  stylex_css           --> stylex_enums
  stylex_css           --> stylex_evaluator
  stylex_css           --> stylex_macros
  stylex_css           --> stylex_regex
  stylex_css           --> stylex_structures
  stylex_css           --> stylex_types

  stylex_transform     --> stylex_ast
  stylex_transform     --> stylex_constants
  stylex_transform     --> stylex_css
  stylex_transform     --> stylex_css_parser
  stylex_transform     --> stylex_enums
  stylex_transform     --> stylex_logs
  stylex_transform     --> stylex_macros
  stylex_transform     --> stylex_path_resolver
  stylex_transform     --> stylex_regex
  stylex_transform     --> stylex_structures
  stylex_transform     --> stylex_types
  stylex_transform     --> stylex_utils

  stylex_compiler_rs   --> stylex_ast
  stylex_compiler_rs   --> stylex_enums
  stylex_compiler_rs   --> stylex_logs
  stylex_compiler_rs   --> stylex_macros
  stylex_compiler_rs   --> stylex_regex
  stylex_compiler_rs   --> stylex_structures
  stylex_compiler_rs   --> stylex_transform
  stylex_compiler_rs   --> stylex_types
  stylex_compiler_rs   --> stylex_utils

  classDef l0 fill:#e8e8e8,stroke:#999,color:#333
  classDef l1 fill:#dce8ff,stroke:#6699cc,color:#333
  classDef l2 fill:#dcf5dc,stroke:#66aa66,color:#333
  classDef l3 fill:#fff3dc,stroke:#cc9933,color:#333
  classDef l4 fill:#ffe8dc,stroke:#cc6633,color:#333
  classDef l5 fill:#f5dcff,stroke:#9933cc,color:#333
  classDef l6 fill:#dcfff5,stroke:#33aaaa,color:#333
  classDef l7 fill:#ffdcdc,stroke:#cc3333,color:#333
  classDef l8 fill:#fffdc0,stroke:#aaaa33,color:#333
  classDef l9 fill:#ffc0c0,stroke:#cc0000,color:#333

  class stylex_constants,stylex_regex,stylex_utils l0
  class stylex_macros l1
  class stylex_enums,stylex_js,stylex_logs,stylex_css_parser,stylex_path_resolver l2
  class stylex_structures l3
  class stylex_types l4
  class stylex_ast l5
  class stylex_evaluator l6
  class stylex_css l7
  class stylex_transform l8
  class stylex_compiler_rs l9
```

</details>

## Development

```bash
# Build
make crate-test-parser-build

# Lint
make crate-test-parser-lint

# Generate docs
make crate-test-parser-docs
```

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
