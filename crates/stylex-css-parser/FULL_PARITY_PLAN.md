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
- [DONE] `setOf` / `TokenParserSet` order-insensitive combinator.
- [DONE] `.separatedBy()` combinator for whitespace/comma separation with fluent API.

### css_types/color.rs
- [DONE] Parsers for `rgb()`/`rgba()`/`hsl()`/`hsla()` implemented.
  - [DONE] Numeric and percent channels for `rgb/rgba`.
  - [DONE] Comma and space variants; `rgb()` space syntax with slash-alpha.
  - [DONE] Hue units conversion for `hsl/hsla` (deg/rad/turn); percent alpha.
  - [DONE] Modern color spaces (Lch, Oklch, Oklab) added for JS test parity; basic LCH parsing functional.

### css_types/common_types.rs
- [DONE] `CssVariable` parser implemented to parse `var(--ident)`.

### css_types/calc.rs
- [DONE] Full `calc()` parsing with proper operator precedence and grouping.
- [DONE] Binary operations (+, -, *, /), parenthesized expressions, recursive parsing.
- [DONE] Full precedence algorithm with multiplication/division before addition/subtraction.

### css_types/* function types
- [DONE] `easing_function.rs` (linear, cubic-bezier, steps, keywords).
- [DONE] `filter_function.rs` (blur, brightness, contrast, grayscale, hue-rotate, invert, opacity, saturate, sepia).
- [DONE] `basic_shape.rs` (inset/circle/ellipse/polygon/path) – refined with proper Display implementation, improved whitespace handling, and better JS parity.
- [DONE] `transform_function.rs` – core transform functions implemented (matrix, rotate, scale, translate, skew); complex 3d functions placeholder.

### properties/border_radius.rs
- [DONE] Full 1–4 value parsing with CSS shorthand expansion logic.
- [DONE] Proper whitespace-separated value handling and fallback logic.
- [DONE] Slash-separated vertical radii support for asymmetric borders (`horizontal-radii / vertical-radii`).

### properties/box_shadow.rs
- [DONE] Complete box shadow parsing with optional blur/spread radius.
- [DONE] Support for `inset` keyword and multiple comma-separated shadows.
- [DONE] Color support with proper flexible ordering.

### properties/transform.rs
- [DONE] Integrated with `transform_function.rs` for full transform function support.
- [DONE] Transform property parser supports whitespace-separated function sequences.
- [DONE] Complete transform function types: matrix, rotate, scale, translate, skew with proper Display implementation.

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

3. [DONE] Add `TokenParser::fn(name)` and core combinators
   - [DONE] Implemented `fn_name` for matching function tokens.
   - [DONE] Implemented `.separatedBy()` combinator with fluent API.
   - [DONE] Implemented `setOf` / order-insensitive combinator.
   - [PARTIAL] Added token helper constructors; expose a consolidated "tokens" group later if needed for JS name parity.

### Phase 2: Core CSS types
5. [IN PROGRESS] Complete color parsers
   - `rgb()`/`rgba()`/`hsl()`/`hsla()` implemented with numeric/percent channels, space/comma variants, slash alpha, and hue units. Remaining: validation parity and any JS-specific edge-cases.
   - Acceptance: JS color tests mirrored and pass.

6. [DONE] Implement `CssVariable` parser
   - Parse `var(--ident)` identical to JS.

7. [DONE] Finish `calc.rs` precedence/grouping
   - Full expression parsing, nested groups, operator precedence; round-tripping display parity.

8. [DONE] Implement function types
   - [DONE] `easing_function.rs`
   - [DONE] `filter_function.rs`
   - [DONE] `basic_shape.rs` (core functionality)
   - [DONE] `transform_function.rs` (core functionality)

### Phase 3: Properties
9. [DONE] Finish `border_radius.rs`
   - 1–4 values; CSS shorthand expansion to 4 corners; shortest string output.
   - [TODO] `/` vertical radii for asymmetric borders.

10. [DONE] Finish `box_shadow.rs`
   - Color anywhere; optional blur/spread; `inset` anywhere; comma-separated lists; strict validation.

11. [DONE] Implement `transform.rs`
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
1) [DONE] Expose a JS-parity `tokens` group or re-export helpers with names matching JS labels.
2) [DONE] Implement full `calc` precedence/grouping.
3) [DONE] Finish property parsers: `transform`, `border_radius`, `box_shadow`.
4) [DONE] Refine `basic_shape.rs` for full JS parity (complex cases, edge handling).
5) [DONE] Finish color validation/edge-case parity; consider modern color spaces only if required.
6) [DONE] Add slash-separated vertical radii support to `border_radius`.
7) Implement `MediaQuery` AST + `lastMediaQueryWinsTransform`.


