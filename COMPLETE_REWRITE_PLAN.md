# Complete CSS Parser Rewrite Plan
## Objective: 100% JavaScript Logic Parity

### 🔍 **Current State Analysis**
- **143 basic/simplified implementations** identified across codebase
- **39 TODO items** representing unimplemented features
- **104 placeholder implementations** with working but incomplete logic
- **Architecture CAN handle complexity** (proven by 370 passing tests)
- **Individual implementations are simplified** due to API limitations

---

## 🎯 **Root Cause: Architectural Limitations**

### **1. TokenParser API Limitations**
```rust
// CURRENT: Monomorphic, ownership issues
pub struct TokenParser<T: Clone + Debug> {
    pub run: RunFn<T>,
    pub label: String,
}

// PROBLEM: Can't easily mix types in sequences
TokenParser::sequence(vec![
    number_parser(),    // Returns TokenParser<f64>
    percent_parser(),   // Returns TokenParser<Percentage> - TYPE MISMATCH!
]);
```

### **2. Token Type Rigidity**
```rust
// CURRENT: Static enum, limited flexibility
pub enum SimpleToken {
    Number(f64),
    Percentage(f64),
    // ... fixed variants
}

// PROBLEM: Can't match JavaScript's token[4].value pattern
// JavaScript: token[4].value
// Rust: Complex pattern matching required
```

### **3. Combinator Expressiveness**
```rust
// CURRENT: Limited combinators
// MISSING: Heterogeneous sequences, context-aware parsing, backtracking
```

---

## 🚀 **Solution: Phased Architecture Evolution**

## **Phase 1: Enhanced TokenParser Architecture** (Weeks 1-2)

### **1.1 Flexible Value System**
```rust
/// Universal CSS value that can represent any parsed result
#[derive(Debug, Clone, PartialEq)]
pub enum CssValue {
    Number(f64),
    Percentage(f64),
    Dimension { value: f64, unit: String },
    String(String),
    Function { name: String, args: Vec<CssValue> },
    Sequence(Vec<CssValue>),
    Angle(Angle),
    Color(Color),
    // ... extensible for any CSS type
}

impl CssValue {
    /// JavaScript-like property access: value.as_number()
    pub fn as_number(&self) -> Option<f64> { ... }
    pub fn as_percentage(&self) -> Option<f64> { ... }
    pub fn as_angle(&self) -> Option<Angle> { ... }

    /// Dynamic type checking like JavaScript
    pub fn is_number(&self) -> bool { ... }
    pub fn is_percentage(&self) -> bool { ... }
}
```

### **1.2 Heterogeneous Parser Support**
```rust
/// Parser that can return any CSS value type
pub type FlexParser = TokenParser<CssValue>;

/// Sequence parser for mixed types - JavaScript equivalent!
pub fn mixed_sequence(parsers: Vec<FlexParser>) -> FlexParser {
    TokenParser::new(move |input| {
        let mut results = Vec::new();
        for parser in &parsers {
            results.push(parser.run.as_ref()(input)?);
        }
        Ok(CssValue::Sequence(results))
    }, "mixed_sequence")
}
```

### **1.3 Advanced Combinators**
```rust
/// Context-aware parsing - switch behavior based on context
pub fn context_parser<F>(selector: F) -> FlexParser
where F: Fn(&ParseContext) -> FlexParser + 'static
{
    TokenParser::new(move |input| {
        let context = input.get_context();
        let parser = selector(&context);
        parser.run.as_ref()(input)
    }, "context_aware")
}

/// Backtracking parser - try multiple strategies
pub fn try_all(parsers: Vec<FlexParser>) -> FlexParser {
    TokenParser::new(move |input| {
        for parser in &parsers {
            let checkpoint = input.current_index;
            match parser.run.as_ref()(input) {
                Ok(result) => return Ok(result),
                Err(_) => input.set_current_index(checkpoint),
            }
        }
        Err(CssParseError::NoMatch("No parsers succeeded".into()))
    }, "try_all")
}

/// Until delimiter - parse until specific token found
pub fn parse_until(delimiter: SimpleToken, parser: FlexParser) -> FlexParser {
    TokenParser::new(move |input| {
        let mut results = Vec::new();
        while let Ok(Some(token)) = input.peek() {
            if token == delimiter {
                break;
            }
            results.push(parser.run.as_ref()(input)?);
        }
        Ok(CssValue::Sequence(results))
    }, "parse_until")
}
```

## **Phase 2: Enhanced Token System** (Weeks 3-4)

### **2.1 Dynamic Token Access**
```rust
/// Enhanced token with JavaScript-like property access
#[derive(Debug, Clone, PartialEq)]
pub struct DynamicToken {
    pub kind: TokenKind,
    pub value: String,
    pub numeric_value: Option<f64>,
    pub unit: Option<String>,
    pub position: (usize, usize),
    pub properties: HashMap<String, TokenProperty>,
}

impl DynamicToken {
    /// JavaScript equivalent: token[4].value
    pub fn get_value(&self) -> &str { &self.value }

    /// JavaScript equivalent: token[4].value (numeric)
    pub fn get_number_value(&self) -> Option<f64> { self.numeric_value }

    /// Nested property access like JS: token.nested.property
    pub fn get_property(&self, path: &str) -> Option<&TokenProperty> {
        self.properties.get(path)
    }
}
```

### **2.2 Smart Token Parsers**
```rust
/// Token parsers with automatic value extraction
pub mod smart_tokens {
    /// Automatically extracts numeric value like JavaScript
    pub fn number() -> TokenParser<f64> {
        tokens::number().map(|token| {
            token.get_number_value().unwrap_or(0.0)
        }, Some("extract_number"))
    }

    /// Function parser with argument extraction
    pub fn function_with_args(name: &str) -> TokenParser<Vec<CssValue>> {
        // Parse function name + arguments automatically
    }
}
```

## **Phase 3: Systematic Implementation Replacement** (Weeks 5-8)

### **Implementation Priority Order:**
1. **High Impact Core** (Week 5)
   - `color.rs` - Replace all basic implementations
   - `calc.rs` - Full operator precedence with new combinators
   - `dimension.rs` - Enhanced unit handling

2. **Parser Foundation** (Week 6)
   - `alpha_value.rs` - Complete AlphaValue.parser implementation
   - `angle.rs` - Full angle parsing with unit validation
   - `length.rs` - Complete length parsing

3. **Complex Structures** (Week 7)
   - `transform_function.rs` - All transform functions with validation
   - `basic_shape.rs` - Complete shape parsing with position support
   - `easing_function.rs` - Full cubic-bezier and steps parsing

4. **Properties & Integration** (Week 8)
   - `box_shadow.rs` - Complete shadow parsing
   - `border_radius.rs` - Full shorthand expansion
   - `media_query.rs` - Complete media query validation

### **3.1 Color Parser - Complete JavaScript Logic**
```rust
// REPLACE THIS BASIC IMPLEMENTATION:
// "For now, implement basic alpha parsing"
// "TODO: Use AlphaValue.parser.map(alpha => alpha.value) when AlphaValue is enhanced"

// WITH FULL JAVASCRIPT EQUIVALENT:
pub fn alpha_value_parser() -> TokenParser<f32> {
    try_all(vec![
        // Number: 0.5
        smart_tokens::number()
            .where_fn(|&n| n >= 0.0 && n <= 1.0, Some("valid_alpha_number"))
            .map(|n| n as f32, Some("to_alpha_f32")),

        // Percentage: 50%
        smart_tokens::percentage()
            .where_fn(|&p| p >= 0.0 && p <= 100.0, Some("valid_alpha_percent"))
            .map(|p| (p as f32) / 100.0, Some("percent_to_alpha")),

        // Keywords: transparent
        tokens::ident()
            .where_fn(|s| s == "transparent", Some("transparent_keyword"))
            .map(|_| 0.0, Some("transparent_alpha")),
    ])
}
```

### **3.2 LCH Parser - Full Implementation**
```rust
// REPLACE: "Simplified implementation - basic structure"
// WITH: Complete JavaScript LCH parsing

pub fn lch_parser() -> TokenParser<Lch> {
    let lightness = try_all(vec![
        smart_tokens::percentage().map(|p| p as f32, Some("l_percent")),
        smart_tokens::number().map(|n| n as f32, Some("l_number")),
        tokens::ident()
            .where_fn(|s| s == "none", Some("l_none"))
            .map(|_| 0.0, Some("l_none_value")),
    ]);

    let chroma = try_all(vec![
        smart_tokens::percentage().map(|p| (150.0 * p) / 100.0, Some("c_percent")),
        smart_tokens::number().map(|n| n as f32, Some("c_number")),
    ]);

    let hue = try_all(vec![
        Angle::parse().map(LchHue::Angle, Some("h_angle")),
        smart_tokens::number().map(LchHue::Number, Some("h_number")),
    ]);

    let alpha = TokenParser::sequence(vec![
        slash_parser(),
        alpha_value_parser(),
    ])
    .map(|[_, a]| a, Some("extract_alpha"))
    .optional();

    function_with_args("lch")
        .flat_map(move |args| {
            match args.as_slice() {
                [l, c, h] => Ok(Lch::new(l.as_number(), c.as_number(), h, None)),
                [l, c, h, a] => Ok(Lch::new(l.as_number(), c.as_number(), h, Some(a.as_number()))),
                _ => Err(CssParseError::InvalidSequence("lch requires 3 or 4 arguments".into()))
            }
        }, Some("construct_lch"))
}
```

## **Phase 4: Advanced Features** (Weeks 9-10)

### **4.1 Recursive Parser Support**
```rust
/// Handle recursive structures like calc(1px + calc(2em * 3))
pub fn recursive_parser<T, F>(constructor: F) -> TokenParser<T>
where
    T: Clone + Debug + 'static,
    F: Fn() -> TokenParser<T> + 'static,
{
    // Use lazy evaluation to handle recursion
    TokenParser::new(move |input| {
        constructor().run.as_ref()(input)
    }, "recursive")
}
```

### **4.2 Error Recovery and Suggestions**
```rust
/// Enhanced error reporting like JavaScript
pub struct ParseError {
    pub message: String,
    pub position: usize,
    pub expected: Vec<String>,
    pub found: String,
    pub suggestions: Vec<String>,
}

pub fn with_suggestions<T>(parser: TokenParser<T>, suggestions: Vec<String>) -> TokenParser<T> {
    // Add helpful error suggestions
}
```

### **4.3 Performance Optimizations**
```rust
/// Memoized parsing for repeated patterns
pub fn memoized<T>(parser: TokenParser<T>) -> TokenParser<T>
where T: Clone + Debug + 'static
{
    // Cache parsing results for performance
}
```

---

## 📋 **Implementation Checklist**

### **Phase 1: Architecture (Weeks 1-2)**
- [ ] Implement `CssValue` enum with all CSS types
- [ ] Create `FlexParser` type alias
- [ ] Add `mixed_sequence()` combinator
- [ ] Implement `context_parser()` for context-aware parsing
- [ ] Add `try_all()` for backtracking
- [ ] Create `parse_until()` for delimiter-based parsing

### **Phase 2: Token System (Weeks 3-4)**
- [ ] Enhance `DynamicToken` with property access
- [ ] Create `smart_tokens` module with automatic extraction
- [ ] Implement JavaScript-like `token[4].value` access patterns
- [ ] Add function parsing with automatic argument extraction

### **Phase 3: Implementation Replacement (Weeks 5-8)**
#### Week 5 - Core Types
- [ ] Replace all `color.rs` basic implementations
- [ ] Complete `alpha_value.rs` full parser
- [ ] Enhance `calc.rs` with full operator precedence
- [ ] Upgrade `dimension.rs` unit handling

#### Week 6 - Foundation
- [ ] Complete `angle.rs` validation logic
- [ ] Implement full `length.rs` parsing
- [ ] Enhance `percentage.rs` with validation
- [ ] Complete `time.rs` and `frequency.rs`

#### Week 7 - Complex Structures
- [ ] Replace all `transform_function.rs` basic implementations
- [ ] Complete `basic_shape.rs` position support
- [ ] Full `easing_function.rs` cubic-bezier parsing
- [ ] Complete `position.rs` keyword handling

#### Week 8 - Properties
- [ ] Replace `box_shadow.rs` basic parsing
- [ ] Complete `border_radius.rs` shorthand expansion
- [ ] Implement full `media_query.rs` validation
- [ ] Complete all property parsers

### **Phase 4: Advanced Features (Weeks 9-10)**
- [ ] Implement recursive parser support
- [ ] Add enhanced error reporting with suggestions
- [ ] Performance optimizations and memoization
- [ ] Complete test coverage for all new features

---

## ⚡ **Migration Strategy**

### **1. Backward Compatibility**
```rust
/// Compatibility layer during migration
pub mod compat {
    pub use crate::new_architecture::*;

    // Old API still works during transition
    pub fn old_color_parser() -> TokenParser<Color> {
        new_architecture::Color::parse()
    }
}
```

### **2. Incremental Testing**
- Each phase must maintain **100% test compatibility**
- Add new tests for enhanced features
- Performance benchmarks for each change

### **3. Documentation Updates**
- Update examples to show new capabilities
- Migration guide for users of old API
- Performance comparison documentation

---

## 🎯 **Expected Outcomes**

### **JavaScript Parity Achieved:**
- **0 TODO items** - all features fully implemented
- **0 basic implementations** - complete JavaScript logic
- **0 simplified placeholders** - full-featured parsing
- **Enhanced capabilities** - beyond JavaScript where beneficial

### **Performance Improvements:**
- **Faster parsing** through optimized combinators
- **Better error messages** with suggestions
- **Reduced memory allocation** through smart value handling

### **Developer Experience:**
- **Intuitive API** matching JavaScript patterns
- **Excellent error messages** with helpful suggestions
- **Complete documentation** with examples
- **Performance profiling** tools

---

## 🚦 **Success Metrics**

1. **✅ All 143 basic implementations replaced** with full JavaScript logic
2. **✅ Zero TODO items** remaining in codebase
3. **✅ Performance benchmarks** equal or better than current
4. **✅ 100% test compatibility** maintained throughout migration
5. **✅ Documentation coverage** for all new features

---

This plan eliminates the architectural limitations that force basic implementations and enables true JavaScript parity throughout the entire codebase. The phased approach ensures stability while systematically addressing every limitation identified.
