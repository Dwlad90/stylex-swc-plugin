# CSS Parser Architecture Redesign Proposal

## Problem Statement
Current TokenParser<T> design conflicts with Rust's ownership model, preventing JavaScript-equivalent parsing logic implementation.

## Core Issues
1. **Ownership Conflicts**: Parser consumption prevents reusability
2. **Type Rigidity**: Static typing limits dynamic parser composition
3. **Closure Complexity**: Ownership requires extensive cloning patterns

## Proposed Solution: Functional Parser Architecture

### 1. Parser as Function Reference
```rust
// Core parser type - cheaply cloneable
pub type Parser<T> = Rc<dyn Fn(&mut TokenList) -> Result<T, CssParseError>>;

pub struct ParserContext {
    pub label: String,
    pub parser: Parser<T>,
}

// Factory functions for common patterns
pub fn parser<T, F>(f: F, label: &str) -> Parser<T>
where
    F: Fn(&mut TokenList) -> Result<T, CssParseError> + 'static,
{
    Rc::new(f)
}
```

### 2. Combinator Library
```rust
// Reusable combinators without ownership issues
pub fn sequence<T>(parsers: Vec<Parser<T>>) -> Parser<Vec<T>> {
    parser(move |input| {
        let mut results = Vec::new();
        for p in &parsers {
            results.push(p(input)?);
        }
        Ok(results)
    }, "sequence")
}

pub fn one_of<T>(parsers: Vec<Parser<T>>) -> Parser<T> {
    parser(move |input| {
        for p in &parsers {
            let checkpoint = input.current_index;
            match p(input) {
                Ok(result) => return Ok(result),
                Err(_) => input.set_current_index(checkpoint),
            }
        }
        Err(CssParseError::NoMatch)
    }, "one_of")
}
```

### 3. Color Parser Implementation
```rust
// JavaScript-equivalent logic without ownership issues
pub fn rgb_parser() -> Parser<Rgb> {
    let rgb_fn = parser(|_| Ok("rgb".to_string()), "rgb_fn");
    let number = parser(|input| { /* number parsing */ }, "number");
    let comma = parser(|input| { /* comma parsing */ }, "comma");

    sequence(vec![
        rgb_fn,
        number.clone(), // Cheap Rc::clone!
        comma.clone(),
        number.clone(),
        comma.clone(),
        number.clone(),
    ])
    .map(|[_fn, r, _c1, g, _c2, b]| Rgb::new(r, g, b))
}
```

### 4. Token System Enhancement
```rust
pub struct DynamicToken {
    pub kind: TokenKind,
    pub value: String,
    pub position: (usize, usize),
    pub properties: HashMap<String, TokenValue>,
}

impl DynamicToken {
    // JavaScript-like property access: token[4].value
    pub fn get_nested(&self, path: &[&str]) -> Option<&TokenValue> {
        // Implement nested property access
    }
}
```

## Migration Strategy

### Phase 1: Proof of Concept
- Implement functional parser architecture
- Port color parsing to new system
- Validate performance and ergonomics

### Phase 2: Gradual Migration
- Create compatibility layer
- Migrate complex parsers (calc, transform)
- Maintain test compatibility

### Phase 3: Full Adoption
- Remove old TokenParser system
- Optimize for performance
- Add advanced parsing features

## Expected Benefits

1. **JavaScript Parity**: Same parsing capabilities without ownership conflicts
2. **Maintainability**: Simpler, more readable parser implementations
3. **Performance**: Rc<dyn Fn> should be zero-cost after compilation
4. **Extensibility**: Easy to add new parsing patterns

## Implementation Timeline

- Week 1: Core functional parser architecture
- Week 2: Color parser migration and testing
- Week 3: Complex parser migration (calc, transform)
- Week 4: Performance optimization and documentation

This architectural shift solves the fundamental mismatch between JavaScript's functional approach and Rust's ownership model while maintaining the same parsing power.
