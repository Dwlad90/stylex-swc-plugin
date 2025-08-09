# StyleX CSS Parser - Rust Rewrite Implementation Plan

## Project Overview

This document outlines the complete plan for rewriting the JavaScript StyleX CSS parser to Rust while maintaining exact API compatibility and full feature parity.

## Analysis Summary

### JavaScript Package Structure
- **Core Foundation**: `token-parser.js` (monadic parser combinators), `token-types.js` (tokenization), `base-types.js` (utilities)
- **CSS Types**: 22 CSS type parsers in `css-types/` (color, length, calc, angle, etc.)
- **Properties**: Specific property parsers (transform, box-shadow, border-radius)
- **At-Queries**: Media query parsing
- **Tests**: Comprehensive test coverage matching each module

### Previous Rust Attempt Assessment
- ✅ Good foundation with `cssparser` crate integration
- ✅ Basic token parser structure implemented
- ✅ Some CSS types working (colors, lengths)
- ❌ Incomplete - missing many CSS types and features
- ❌ Architecture needs refinement to match JavaScript exactly
- ❌ Tests incomplete

## Implementation Plan

### Phase 1: Project Foundation

#### Task 1: Project Structure Setup
**Goal**: Create new crate structure mirroring JavaScript package exactly

**Actions**:
- Create new crate `stylex-css-parser` in `crates/stylex-css-parser/`
- Mirror exact directory structure: `src/`, `src/css_types/`, `src/properties/`, `src/at_queries/`
- Set up `Cargo.toml` with dependencies: `cssparser`, `serde`, `regex`, test frameworks

**Success Criteria**:
- Directory structure matches JavaScript package
- All dependencies properly configured
- Crate compiles successfully

#### Task 2: Base Types Implementation
**Goal**: Port `base-types.js` → `base_types.rs`

**Actions**:
- Implement `SubString` struct with identical methods
- Methods: `starts_with`, `first`, `get`, `to_string`, `is_empty`
- Ensure exact API parity with JavaScript version

**Success Criteria**:
- All SubString methods implemented
- Unit tests pass
- API matches JavaScript exactly

#### Task 3: Token Types Implementation
**Goal**: Port `token-types.js` → `token_types.rs`

**Actions**:
- Implement `TokenList` struct using `cssparser::tokenize`
- Implement `TokenIterator` trait
- Methods: `consume_next_token`, `peek`, `set_current_index`, `rewind`, `slice`, `get_all_tokens`

**Success Criteria**:
- TokenList fully functional
- All methods implemented with exact behavior
- Integration with cssparser working

#### Task 4: Token Parser Implementation
**Goal**: Port `token-parser.js` → `token_parser.rs`

**Actions**:
- Implement `TokenParser<T>` struct with all monadic operations
- Core methods: `parse`, `parse_to_end`, `map`, `flat_map`, `or`, `where_fn`
- Utility methods: `surrounded_by`, `skip`, `optional`, `prefix`, `suffix`
- Static methods: `always`, `never`, `one_of`, `sequence`, `set_of`, `zero_or_more`, `one_or_more`
- Implement helper structs: `TokenParserSequence`, `TokenParserSet`, `TokenZeroOrMoreParsers`, `TokenOneOrMoreParsers`

**Success Criteria**:
- All parser combinators working
- Monadic operations functional
- Complex parsing scenarios supported

### Phase 2: CSS Types Foundation

#### Task 5: CSS Types Module Setup
**Goal**: Create foundation for CSS type implementations

**Actions**:
- Create `css_types/mod.rs` exporting all type modules
- Implement `common_types.rs` with `Percentage`, `Number`, basic shared types
- Set up module structure

**Success Criteria**:
- Module structure established
- Basic types implemented
- Foundation ready for complex types

#### Task 6: Basic CSS Types
**Goal**: Implement fundamental CSS value types

**Actions**:
- Implement `number.rs` with number parsing and validation
- Implement `dimension.rs` for generic dimension handling
- Ensure exact parity with JavaScript implementations

**Success Criteria**:
- Number parsing working correctly
- Dimension handling implemented
- Tests passing

### Phase 3: Core CSS Types

#### Task 7: Color Implementation
**Goal**: Port `css-types/color.js` → `css_types/color.rs`

**Actions**:
- Implement all color types: `NamedColor`, `HashColor`, `Rgb`, `Rgba`, `Hsl`, `Hsla`, `Lch`, `Oklch`, `Oklab`
- Include complete named colors list
- Implement hex validation
- Support RGB/HSL parsing with comma/space syntax
- Handle alpha channel parsing

**Success Criteria**:
- All color formats supported
- Named colors working
- Hex colors with validation
- RGB/RGBA with both syntaxes
- HSL/HSLA with both syntaxes
- Modern color spaces (LCH, OKLCH, OKLAB)

#### Task 8: Length Implementation
**Goal**: Port `css-types/length.js` → `css_types/length.rs`

**Actions**:
- Implement `Length` with all unit categories:
  - Font-relative: `ch`, `em`, `ex`, `ic`, `lh`, `rem`, `rlh`
  - Viewport: `vh`, `svh`, `lvh`, `dvh`, `vw`, `svw`, `lvw`, `dvw`, `vmin`, `svmin`, `lvmin`, `dvmin`, `vmax`, `svmax`, `lvmax`, `dvmax`
  - Container: `cqw`, `cqi`, `cqh`, `cqb`, `cqmin`, `cqmax`
  - Absolute: `px`, `cm`, `mm`, `in`, `pt`
- Handle zero values without units
- Implement unit conversion utilities

**Success Criteria**:
- All length units supported
- Unit categorization working
- Zero value handling
- Proper validation

#### Task 9: Calc Implementation
**Goal**: Port calc expression parsing

**Actions**:
- Port `css-types/calc.js` → `css_types/calc.rs`
- Port `css-types/calc-constant.js` → `css_types/calc_constant.rs`
- Implement `Calc` with full expression parsing
- Handle operator precedence correctly
- Support nested parentheses
- Include all calc constants and operations

**Success Criteria**:
- Mathematical expressions parsed correctly
- Operator precedence respected
- Nested expressions working
- Constants supported

### Phase 4: Additional CSS Types

#### Task 10: Standard CSS Types
**Goal**: Complete remaining standard CSS types

**Actions**:
- `angle.rs`, `angle_percentage.rs` - angle values and mixed types
- `time.rs`, `frequency.rs`, `resolution.rs` - time/frequency/resolution units
- `length_percentage.rs` - mixed length/percentage values
- `alpha_value.rs` - alpha channel values
- `custom_ident.rs`, `dashed_ident.rs` - identifier types

**Success Criteria**:
- All standard types implemented
- Mixed value types working
- Identifier validation correct

#### Task 11: Advanced CSS Types
**Goal**: Implement complex CSS value types

**Actions**:
- `position.rs` - CSS position values
- `transform_function.rs` - transform function parsing
- `filter_function.rs` - filter function parsing
- `easing_function.rs` - easing function parsing
- `basic_shape.rs` - CSS basic shapes
- `blend_mode.rs` - blend mode values
- `flex.rs` - flex property values

**Success Criteria**:
- Function parsing working
- Complex value combinations supported
- Shape parsing implemented

### Phase 5: Properties & At-Rules

#### Task 12: Properties Module
**Goal**: Implement property-specific parsers

**Actions**:
- Port `properties/transform.js` → `properties/transform.rs`
- Port `properties/box-shadow.js` → `properties/box_shadow.rs`
- Port `properties/border-radius.js` → `properties/border_radius.rs`
- Implement property-specific validation and parsing

**Success Criteria**:
- Property parsers working
- Validation rules implemented
- Complex property combinations supported

#### Task 13: At-Queries Module
**Goal**: Implement at-rule parsers

**Actions**:
- Port `at-queries/media-query.js` → `at_queries/media_query.rs`
- Port `at-queries/media-query-transform.js` → `at_queries/media_query_transform.rs`
- Implement media query parsing and transformation

**Success Criteria**:
- Media query parsing working
- Transformations implemented
- Complex queries supported

### Phase 6: Integration & Exports

#### Task 14: Main Library
**Goal**: Complete the public API

**Actions**:
- Create `lib.rs` exactly matching `index.js` exports
- Export `tokenParser`, `properties`, `lastMediaQueryWinsTransform`
- Ensure API compatibility

**Success Criteria**:
- Public API matches JavaScript
- All exports working
- Documentation complete

### Phase 7: Testing

#### Task 15: Base Tests
**Goal**: Test foundation components

**Actions**:
- Port `__tests__/token-parser-test.js` → `tests/token_parser_test.rs`
- Test all TokenParser operations, sequences, sets, zero-or-more, one-or-more

**Success Criteria**:
- All parser combinator tests passing
- Complex parsing scenarios covered

#### Task 16: CSS Types Tests
**Goal**: Achieve complete test coverage for CSS types

**Actions**:
- Port all `css-types/__tests__/*.js` → `tests/css_types/*_test.rs`
- Exact test case parity for every CSS type
- Include edge cases, error conditions, and boundary tests

**Success Criteria**:
- 100% test parity achieved
- All edge cases covered
- Error handling tested

#### Task 17: Properties Tests
**Goal**: Test property-specific functionality

**Actions**:
- Port `properties/__tests__/*.js` → `tests/properties/*_test.rs`
- Test property-specific parsing and validation

**Success Criteria**:
- Property parsing tests complete
- Validation edge cases covered

#### Task 18: At-Queries Tests
**Goal**: Test at-rule functionality

**Actions**:
- Port `at-queries/__tests__/*.js` → `tests/at_queries/*_test.rs`
- Test media query parsing and transformations

**Success Criteria**:
- Media query tests complete
- Transformation tests working

### Phase 8: Performance & Documentation

#### Task 19: Benchmarks
**Goal**: Performance testing and optimization

**Actions**:
- Port `__benchmarks__/*.bench.mjs` → `benches/*.rs`
- Performance testing for all major CSS types
- Comparison benchmarks with JavaScript implementation

**Success Criteria**:
- Benchmarks established
- Performance meets or exceeds JavaScript
- Optimization opportunities identified

#### Task 20: Documentation
**Goal**: Comprehensive documentation

**Actions**:
- Comprehensive rustdoc for all public APIs
- README with usage examples
- Migration guide from JavaScript version

**Success Criteria**:
- API documentation complete
- Usage examples working
- Migration guide helpful

#### Task 21: Integration Testing
**Goal**: Final validation and testing

**Actions**:
- Cross-validation tests comparing Rust vs JavaScript results
- Property-based testing for edge cases
- Performance regression testing

**Success Criteria**:
- Cross-validation passing
- No behavioral differences
- Performance acceptable

## Key Implementation Guidelines

### Exact API Parity Requirements
- **Function Names**: All function names must match exactly (camelCase → snake_case conversion)
- **Class Names**: All class/struct names must match exactly
- **Method Signatures**: Input/output types must be equivalent
- **Error Handling**: Error messages and types should match JavaScript behavior
- **Parser Behavior**: Exact same parsing logic, precedence, and edge case handling

### Technical Considerations
- **Cssparser Integration**: Use `cssparser::tokenize()` instead of `@csstools/css-tokenizer`
- **Memory Management**: Efficient string handling and minimal allocations
- **Error Types**: Proper Rust error types while maintaining JS-compatible error messages
- **Serialization**: Serde support for JSON compatibility with JavaScript
- **Performance**: Target performance equal to or better than JavaScript implementation

### Testing Strategy
- **Test Porting**: Every JavaScript test must have an equivalent Rust test
- **Property Testing**: Use `proptest` for additional edge case coverage
- **Cross-Validation**: Automated testing comparing Rust vs JavaScript outputs
- **Regression Testing**: Ensure no behavior changes during development

## Success Metrics

- [ ] 100% API parity with JavaScript implementation
- [ ] 100% test coverage parity
- [ ] Performance equal to or better than JavaScript
- [ ] Zero breaking changes in behavior
- [ ] Complete documentation coverage
- [ ] All benchmarks implemented and passing

This plan ensures a complete, faithful, and high-performance Rust rewrite of the JavaScript CSS parser while maintaining exact API compatibility and full feature parity.
