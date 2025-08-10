# StyleX CSS Parser – Full Parity Plan (Rust vs JS)

This document inventories all unfinished areas in the Rust rewrite and defines a step-by-step plan to reach 100% parity with the original JavaScript implementation (`packages/style-value-parser/src/**`). All file, class, function, and variable names must match the JS originals (camelCase exported names preserved via re-exports).

## Unfinished and placeholder areas (updated status)

### token_types.rs
- [DONE] Replace `SimpleTokenizer` with `cssparser`-backed tokenization; preserve whitespace tokens.
- [DONE] Map `cssparser::Token` → `SimpleToken` for Ident/Function/Number/Percentage/Dimension/String/Hash/Colon/Semicolon/Comma/Paren/Bracket/Brace/Whitespace/Delim.
- [DONE] `TokenList` parity for `peek`, `consume_next_token`, `slice`, `set_current_index`, `rewind` (tests updated to account for whitespace).

### token_parser.rs
- [DONE] `fn(name)` helper implemented (`TokenParser::fn_name`).
- [PARTIAL] Common token helpers added (ident, colon, semicolon, comma, parens/brackets/braces, whitespace, number, percentage, dimension, string, hash, at-keyword). A full "tokens" bag struct is not exposed yet.
- [TODO] `setOf` / `TokenParserSet` order-insensitive combinator.

### css_types/color.rs
- [IN PROGRESS] Parsers for `rgb()`/`rgba()`/`hsl()`/`hsla()` implemented to a working baseline (numeric channels, comma and space variants).
- [TODO] Percent channels, slash alpha, hue units conversion; validation parity; optional modern spaces (Lch, Oklch, Oklab) only if in JS scope.

### css_types/common_types.rs
- [DONE] `CssVariable` parser implemented to parse `var(--ident)`.

### css_types/calc.rs
- [PARTIAL] Basic `calc()` parsing with single value; value parser for numbers/dimensions/percentages/constants.
- [TODO] Full precedence/grouping and nested expressions with correct operator associativity.

### css_types/* function types
- [TODO] Implement `transform_function.rs`, `filter_function.rs`, `easing_function.rs`, `basic_shape.rs` per JS.

### properties/border_radius.rs
- [TODO] Parser is simplified. Needs full 1–4 value parsing plus slash-separated vertical radii and expansion rules.

### properties/box_shadow.rs
- [TODO] Simplified parsing. Needs: color anywhere, optional blur/spread, `inset` anywhere, multiple shadows (comma-separated), and invalid-case handling.

### properties/transform.rs
- [TODO] `TransformFunction` placeholder; `Transform::parser()` returns never. Implement all functions: `matrix`, `matrix3d`, `perspective`, `rotate*`, `scale*`, `translate*`, `skew*` with proper arguments and units.

### at_queries/media_query.rs
- [TODO] `MediaQuery` placeholder. Implement real AST and parser for:
  - Keywords with `not`/`only`
  - Pair rules `(key: value)` and word rules `(color)`
  - AND/OR/NOT combinators and grouped parentheses
  - Inequality parsing (>, >=, <, <=) mapped to min-/max- with .01px normalization
  - Normalization and `to_string()` parity

### at_queries/media_query_transform.rs
- [TODO] Transform returns input unchanged. Implement “last media query wins” interval logic identical to tests.

### tests
- Replace TODO’d tests with real assertions once parsers are complete.

---

## Step-by-step implementation plan (progress)

### Phase 1: Tokenization and parser helpers
1. [DONE] Replace SimpleTokenizer with cssparser-backed tokenizer
   - [DONE] Map `cssparser::Token` → `SimpleToken` for core token kinds; preserve whitespace.
   - [DONE] Acceptance: token tests updated and green.

2. [DONE] Finalize TokenList parity
   - [DONE] Implemented `peek`, `consume_next_token`, `slice`, `set_current_index`, `rewind`.

3. [PARTIAL] Add `TokenParser::fn(name)` and tokens bag
   - [DONE] Implemented `fn_name` for matching function tokens.
   - [PARTIAL] Added token helper constructors; expose a consolidated "tokens" group later if needed for JS name parity.

4. [TODO] Implement `setOf` / `TokenParserSet`
   - Order-insensitive selection with optional separator handling as in JS.

### Phase 2: Core CSS types
5. [IN PROGRESS] Complete color parsers
   - `rgb()`/`rgba()`/`hsl()`/`hsla()` basics implemented; add percent channels, slash alpha, hue units, and validation.
   - Acceptance: JS color tests mirrored and pass.

6. [DONE] Implement `CssVariable` parser
   - Parse `var(--ident)` identical to JS.

7. [TODO] Finish `calc.rs` precedence/grouping
   - Full expression parsing, nested groups, operator precedence; round-tripping display parity.

8. [TODO] Implement function types
   - `transform_function.rs`, `filter_function.rs`, `easing_function.rs`, `basic_shape.rs` per JS files and tests.

### Phase 3: Properties
9. [TODO] Finish `border_radius.rs`
   - 1–4 values; `/` vertical radii; expansion to 4 corners; shortest string output.

10. [TODO] Finish `box_shadow.rs`
   - Color anywhere; optional blur/spread; `inset` anywhere; comma-separated lists; strict validation.

11. [TODO] Implement `transform.rs`
   - `TransformFunction` variants and parser; whitespace-separated function sequence parsing.

### Phase 4: At-queries
12. [TODO] Implement `MediaQuery` parser + AST
   - Keywords/not/only; word and pair rules; AND/OR/NOT; grouped parentheses; inequalities to ranges with .01 adjustments; normalization/flattening; `to_string()` parity.

13. [TODO] Implement `lastMediaQueryWinsTransform`
   - Convert sequences of max/min width/height to non-overlapping intervals; handle mixed cases per tests.

### Phase 5: Tests, cleanup, and docs
14. Enable TODO tests and add any missing ones mirroring JS files.
15. Remove all `todo!()` and `TokenParser::never()` placeholders.
16. Update benches to use real parsers.

---

## Execution notes
- Preserve exported names exactly as JS: use re-exports to expose `tokenParser`, `properties`, `lastMediaQueryWinsTransform`, etc.
- Keep error labels/messages identical where tests assert strings.
- Land changes incrementally and run `cargo test -p stylex_css_parser` after every step.

## Immediate next actions (updated)
1) Expose a JS-parity `tokens` group or re-export helpers with names matching JS labels.
2) Implement `setOf` / order-insensitive combinator.
3) Finish color parser parity: percent channels, slash alpha, hue units, validation.
4) Implement full `calc` precedence/grouping.
5) Implement function types: `transform_function`, `filter_function`, `easing_function`, `basic_shape`.
6) Finish property parsers: `border_radius`, `box_shadow`, `transform`.
7) Implement `MediaQuery` AST + `lastMediaQueryWinsTransform`.


