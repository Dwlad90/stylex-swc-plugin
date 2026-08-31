# `stylex-js`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

What ECMAScript says, asked while the compiler decides whether an expression is
safe to fold and what it folds to. Extracted into its own crate so the evaluator
can ask those questions without pulling in the full transformation pipeline.

- Compile-time guards such as `is_valid_callee`, `is_mutation_expr` and
  `is_invalid_method`, which keep the evaluator to side-effect-free expressions
- The coercions `ToString`, `ToNumber`, `ToBoolean` and `ToObject` over an
  already-evaluated expression
- `evaluate_bin_expr`, which applies a numeric binary operator to two operands
- Thin leaf crate with no transitive dependencies beyond primitives and macros

## Architecture

### Modules

| Module      | Purpose                                                                                      |
| ----------- | -------------------------------------------------------------------------------------------- |
| `helpers`   | JS runtime guards (`is_valid_callee`, `is_mutation_expr`, `is_invalid_method`, etc.)         |
| `coercions` | What the language says a value converts to (`ToString`, `ToNumber`, `ToBoolean`, `ToObject`) |
| `operators` | `evaluate_bin_expr`, the numeric binary operators                                            |

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
