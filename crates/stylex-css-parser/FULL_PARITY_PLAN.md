# StyleX CSS Parser – Full Parity Plan (Rust vs JS)

This document inventories all unfinished areas in the Rust rewrite and defines a step-by-step plan to reach 100% parity with the original JavaScript implementation (`packages/style-value-parser/src/**`). All file, class, function, and variable names must match the JS originals (camelCase exported names preserved via re-exports).

## Unfinished and placeholder areas

### token_types.rs
- SimpleTokenizer is a minimal scanner. It does not mirror tokenizer semantics from JS (whitespace/comments handling, block tokens, URL/string, etc.).
- Token-to-token mapping vs `@csstools/css-tokenizer` semantics is incomplete.
- TokenList must match JS: `peek`, `consumeNextToken`, `slice`, `setCurrentIndex`, `rewind`, including how whitespace/comments interact with lookahead.

### token_parser.rs
- Missing `fn(name)` helper like JS `TokenParser.fn('rgb')`.
- Missing a “tokens bag” (JS: `TokenParser.tokens`) exposing token-kind parsers with stable labels.
- `setOf` / `TokenParserSet` order-insensitive combinator not implemented.

### css_types/color.rs
- Parsers for `rgb()`/`rgba()`/`hsl()`/`hsla()` are placeholders. Need comma and space syntaxes, percent channels, slash alpha, hue units.
- Optional modern spaces (Lch, Oklch, Oklab) to be added only if present in the JS scope.

### css_types/common_types.rs
- `CssVariable` parser is unimplemented (returns `TokenParser::never`). Implement `var(--ident)` parity.

### css_types/calc.rs
- TODOs around full precedence/grouping and nested expressions.

### css_types/* function types
- `transform_function.rs`, `filter_function.rs`, `easing_function.rs`, `basic_shape.rs` contain `todo!()`.

### properties/border_radius.rs
- Parser is simplified. Needs full 1–4 value parsing plus slash-separated vertical radii and expansion rules.

### properties/box_shadow.rs
- Simplified parsing. Needs: color anywhere, optional blur/spread, `inset` anywhere, multiple shadows (comma-separated), and invalid-case handling.

### properties/transform.rs
- `TransformFunction` placeholder; `Transform::parser()` returns never. Needs all functions: `matrix`, `matrix3d`, `perspective`, `rotate*`, `scale*`, `translate*`, `skew*` with proper arguments and units.

### at_queries/media_query.rs
- `MediaQuery` only stores raw string; `parser()` returns never. Needs a real AST and parser for:
  - Keywords with `not`/`only`
  - Pair rules `(key: value)` and word rules `(color)`
  - AND/OR/NOT combinators and grouped parentheses
  - Inequality parsing (>, >=, <, <=) mapped to min-/max- with .01px normalization
  - Normalization and `to_string()` parity

### at_queries/media_query_transform.rs
- Transform returns input unchanged. Implement “last media query wins” interval logic identical to tests.

### tests
- Replace TODO’d tests with real assertions once parsers are complete.

---

## Step-by-step implementation plan

### Phase 1: Tokenization and parser helpers
1. Replace SimpleTokenizer with cssparser-backed tokenizer
   - Map `cssparser::Token` → `SimpleToken` for Ident/Function/Number/Percentage/Dimension/String/URL/Hash/Colon/Semicolon/Comma/Paren/Square/Curly/Whitespace/Comment.
   - Keep whitespace/comments available for JS-equivalent behavior.
   - Acceptance: token tests stay green; no behavior regressions.

2. Finalize TokenList parity
   - `peek`, `consume_next_token`, `slice`, `set_current_index`, `rewind` identical to JS semantics, especially around lookahead and whitespace.
   - Acceptance: integration tests remain green.

3. Add `TokenParser::fn(name)` and tokens bag
   - Implement `fn(name)` for matching function tokens.
   - Provide a static-like set exposing per-token parsers (`Ident`, `Function`, `Number`, `Dimension`, `Percentage`, `Whitespace`, `Colon`, `Comma`, `Semicolon`, brackets, parentheses) with labels matching JS.

4. Implement `setOf` / `TokenParserSet`
   - Order-insensitive selection with optional separator handling as in JS.

### Phase 2: Core CSS types
5. Complete color parsers
   - `rgb()`/`rgba()`: comma and space syntax; integer/percent channels; slash alpha; validation.
   - `hsl()`/`hsla()`: hue with deg/rad/turn; saturation/lightness percent; slash alpha; comma and modern syntax.
   - Acceptance: JS color tests mirrored and pass.

6. Implement `CssVariable` parser
   - Parse `var(--ident)` and error cases identical to JS.

7. Finish `calc.rs` precedence/grouping
   - Full expression parsing, nested groups, operator precedence; round-tripping display parity.

8. Implement function types
   - `transform_function.rs`, `filter_function.rs`, `easing_function.rs`, `basic_shape.rs` per JS files and tests.

### Phase 3: Properties
9. Finish `border_radius.rs`
   - 1–4 values; `/` vertical radii; expansion to 4 corners; shortest string output.

10. Finish `box_shadow.rs`
   - Color anywhere; optional blur/spread; `inset` anywhere; comma-separated lists; strict validation.

11. Implement `transform.rs`
   - `TransformFunction` variants and parser; whitespace-separated function sequence parsing.

### Phase 4: At-queries
12. Implement `MediaQuery` parser + AST
   - Keywords/not/only; word and pair rules; AND/OR/NOT; grouped parentheses; inequalities to ranges with .01 adjustments; normalization/flattening; `to_string()` parity.

13. Implement `lastMediaQueryWinsTransform`
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

## Immediate next actions
1) Implement `TokenParser::fn(name)` and add a `tokens`-like group for common tokens.
2) Swap in cssparser-backed tokenizer and align `TokenList` semantics.
3) Implement full color parsers.


