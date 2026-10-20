# `stylex-js`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

The ECMAScript semantics the compiler consults when it decides whether an
expression is safe to fold, and what it folds to. It is a separate crate so the
evaluator can ask these questions without the full transformation pipeline.

- Compile-time guards such as `is_valid_callee`, `is_mutation_expr` and
  `is_valid_callee_method`, which keep the evaluator to side-effect-free
  expressions
- The coercions `ToString`, `ToNumber`, `ToBoolean` and `ToObject` over an
  already-evaluated expression
- `evaluate_bin_expr`, which applies a numeric binary operator to two operands
- `JsValue`, a JavaScript value as data, with its coercions, and
  `array_tim_sort`, `Array.prototype.sort` as V8 sorts
- `string`, JavaScript string operations over Rust strings:
  `split_at_utf16_lengths` cuts a text at lengths that JavaScript counts in
  UTF-16 code units, `js_substitution` expands the `$` patterns of a
  replacement, and `js_replace_first` and `js_replace_all` are
  `String.prototype.replace` and `replaceAll` with a string pattern
- A low crate: it depends on primitives, macros and a few small libraries only

## Architecture

### Modules

| Module       | Purpose                                                                                      |
| ------------ | -------------------------------------------------------------------------------------------- |
| `helpers`    | JS runtime guards (`is_valid_callee`, `is_mutation_expr`, `is_valid_callee_method`, etc.)    |
| `coercions`  | What the language says a value converts to (`ToString`, `ToNumber`, `ToBoolean`, `ToObject`) |
| `operators`  | `evaluate_bin_expr`, the numeric binary operators                                            |
| `value`      | `JsValue`, a JavaScript value as data, and its coercions                                     |
| `array_sort` | `array_tim_sort`, `Array.prototype.sort` as V8 sorts                                         |
| `string`     | JavaScript string operations over Rust strings (`split_at_utf16_lengths`, `js_replace_all`)  |

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
