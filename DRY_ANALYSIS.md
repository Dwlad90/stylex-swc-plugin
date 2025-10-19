# DRY Principle Violation Analysis & Refactoring Recommendations

## Executive Summary
This analysis identifies repetitive code patterns across the stylex-shared workspace and provides actionable refactoring recommendations using function extraction and macros.

---

## 🔴 Critical Violations (High Impact)

### 1. **Error Handling with unwrap_or_else Pattern**
**Occurrences**: 12 instances across 3 files
- `evaluate.rs`: 6 instances
- `convertors.rs`: 5 instances
- `native_functions.rs`: 1 instance

**Pattern**:
```rust
expr_to_num(&arg, state, traversal_state, fns)
  .unwrap_or_else(|error| panic!("{}", error))
```

**Solution**: Create a macro for cleaner error unwrapping
```rust
/// Add to error_helpers.rs
#[macro_export]
macro_rules! unwrap_or_panic {
  ($result:expr) => {
    $result.unwrap_or_else(|error| panic!("{}", error))
  };
  ($result:expr, $context:expr) => {
    $result.unwrap_or_else(|error| panic!("{}: {}", $context, error))
  };
}

// Usage:
let value = unwrap_or_panic!(expr_to_num(&arg, state, traversal_state, fns));
// Or with context:
let value = unwrap_or_panic!(expr_to_num(&arg, state, traversal_state, fns), "Failed to convert to number");
```

**Impact**: Reduces ~150 lines of repetitive error handling code

---

### 2. **ParenExpr Wrapper Creation**
**Occurrences**: 69 instances across 2 files
- `evaluate.rs`: 68 instances
- `transform_stylex_create_call.rs`: 1 instance

**Pattern**:
```rust
&Expr::Paren(ParenExpr {
  span: DUMMY_SP,
  expr: Box::new(path.clone()),
})
```

**Solution**: Create a utility function
```rust
/// Add to ast/factories.rs
#[inline]
pub fn wrap_in_paren(expr: Expr) -> Expr {
  Expr::Paren(ParenExpr {
    span: DUMMY_SP,
    expr: Box::new(expr),
  })
}

#[inline]
pub fn wrap_in_paren_ref(expr: &Expr) -> Expr {
  Expr::Paren(ParenExpr {
    span: DUMMY_SP,
    expr: Box::new(expr.clone()),
  })
}

// Usage:
build_code_frame_error_and_panic(
  &wrap_in_paren_ref(path),
  path,
  "Error message",
  traversal_state,
)
```

**Impact**: Reduces ~276 lines (4 lines per usage * 69)

---

### 3. **build_code_frame_error_and_panic with ParenExpr Pattern**
**Occurrences**: 114 instances across 4 files
- `evaluate.rs`: 69 instances
- `validators.rs`: 40 instances
- `transform_stylex_create_call.rs`: 4 instances
- `build_code_frame_error.rs`: 1 instance

**Pattern**:
```rust
build_code_frame_error_and_panic(
  &Expr::Paren(ParenExpr {
    span: DUMMY_SP,
    expr: Box::new(path.clone()),
  }),
  path,
  "Error message",
  traversal_state,
)
```

**Solution**: Create a macro combining both patterns
```rust
/// Add to error_helpers.rs
#[macro_export]
macro_rules! panic_with_context {
  ($expr:expr, $state:expr, $msg:expr) => {
    $crate::shared::utils::log::build_code_frame_error::build_code_frame_error_and_panic(
      &$crate::shared::utils::ast::factories::wrap_in_paren_ref($expr),
      $expr,
      $msg,
      $state,
    )
  };
}

// Usage:
panic_with_context!(path, traversal_state, "Unary expression not implemented");
```

**Impact**: Reduces ~570 lines (5 lines saved per usage * 114)

---

## 🟡 Medium Violations (Moderate Impact)

### 4. **Unary Expression to Number Pattern**
**Occurrences**: 3 instances in `evaluate.rs` (lines 760-777)

**Pattern**:
```rust
UnaryOp::Plus => {
  let value = expr_to_num(&arg, state, traversal_state, fns)
    .unwrap_or_else(|error| panic!("{}", error));
  Some(EvaluateResultValue::Expr(number_to_expression(value)))
}
UnaryOp::Minus => {
  let value = expr_to_num(&arg, state, traversal_state, fns)
    .unwrap_or_else(|error| panic!("{}", error));
  Some(EvaluateResultValue::Expr(number_to_expression(-value)))
}
UnaryOp::Tilde => {
  let value = expr_to_num(&arg, state, traversal_state, fns)
    .unwrap_or_else(|error| panic!("{}", error));
  Some(EvaluateResultValue::Expr(number_to_expression(
    (!(value as i64)) as f64,
  )))
}
```

**Solution**: Extract helper function
```rust
#[inline]
fn evaluate_unary_numeric(
  arg: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
  transform: impl FnOnce(f64) -> f64,
) -> Option<EvaluateResultValue> {
  let value = unwrap_or_panic!(expr_to_num(arg, state, traversal_state, fns));
  Some(EvaluateResultValue::Expr(number_to_expression(transform(value))))
}

// Usage:
UnaryOp::Plus => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| v),
UnaryOp::Minus => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| -v),
UnaryOp::Tilde => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| (!(v as i64)) as f64),
```

**Impact**: Reduces ~20 lines

---

### 5. **InjectableStyle Creation Pattern**
**Occurrences**: 20 instances across 15 files

**Pattern**:
```rust
Rc::new(InjectableStyleKind::Regular(InjectableStyle {
  ltr: css_string,
  rtl: None,
  priority: Some(priority_value),
}))
```

**Solution**: Create builder functions
```rust
/// Add to structures/injectable_style.rs
impl InjectableStyle {
  #[inline]
  pub fn regular(ltr: String, priority: Option<f64>) -> Rc<InjectableStyleKind> {
    Rc::new(InjectableStyleKind::Regular(InjectableStyle {
      ltr,
      rtl: None,
      priority,
    }))
  }

  #[inline]
  pub fn with_rtl(ltr: String, rtl: String, priority: Option<f64>) -> Rc<InjectableStyleKind> {
    Rc::new(InjectableStyleKind::Regular(InjectableStyle {
      ltr,
      rtl: Some(rtl),
      priority,
    }))
  }
}

// Usage:
InjectableStyle::regular(css_string, Some(priority_value))
InjectableStyle::with_rtl(ltr_css, rtl_css, Some(priority))
```

**Impact**: Reduces ~60 lines, improves readability

---

### 6. **Expression Cloning and Extraction Chain**
**Occurrences**: 9 instances across 2 files

**Pattern**:
```rust
spread_expression
  .and_then(|spread| spread.as_expr().cloned())
  .and_then(|expr| expr.as_object().cloned())
```

**Solution**: Create utility function
```rust
/// Add to enums/data_structures/evaluate_result_value.rs
impl EvaluateResultValue {
  #[inline]
  pub fn into_object(self) -> Option<ObjectLit> {
    self.as_expr().cloned().and_then(|expr| expr.as_object().cloned())
  }

  #[inline]
  pub fn into_array(self) -> Option<ArrayLit> {
    self.as_expr().cloned().and_then(|expr| expr.as_array().cloned())
  }
}

// Usage:
let Some(new_props) = spread_expression.into_object() else {
  // error handling
};
```

**Impact**: Reduces ~36 lines, improves intent clarity

---

## 🟢 Minor Violations (Low Impact, High Value)

### 7. **Hash Creation with Formatting**
**Occurrences**: 44 instances across 14 files

**Pattern**:
```rust
format!("{}-{}", base_name, create_hash(value))
```

**Solution**: Create helper function
```rust
/// Add to utils/common.rs
#[inline]
pub fn create_hashed_name(base: &str, value: &str) -> String {
  format!("{}-{}", base, create_hash(value))
}

// Usage:
let class_name = create_hashed_name("x", &some_value);
```

**Impact**: Reduces ~88 lines, centralizes naming logic

---

### 8. **Default Collection Initialization**
**Occurrences**: Multiple patterns

**Pattern**:
```rust
let mut map: IndexMap<String, T> = IndexMap::default();
let mut hash_map: FxHashMap<String, T> = FxHashMap::default();
```

**Solution**: Type inference optimization
```rust
// Instead of explicit type annotations, use inference:
let mut map = IndexMap::<String, T>::default();
let mut hash_map = FxHashMap::<String, T>::default();

// Or create type aliases for common collections:
type StyleMap = IndexMap<String, Box<FlatCompiledStyles>>;
type FunctionMapIdentifiers = FxHashMap<String, Box<FunctionConfigType>>;

// Usage:
let mut map = StyleMap::default();
```

**Impact**: Improves consistency, reduces cognitive load

---

### 9. **Confident Check Pattern in Evaluation**
**Occurrences**: ~30 instances in `evaluate.rs`

**Pattern**:
```rust
if elem_value.confident {
  arr.push(elem_value.value);
} else {
  return None;
}
```

**Solution**: Create a helper macro
```rust
/// Add to error_helpers.rs
#[macro_export]
macro_rules! collect_confident {
  ($eval_result:expr, $collection:expr) => {
    if $eval_result.confident {
      $collection.push($eval_result.value);
    } else {
      return None;
    }
  };
}

// Usage:
for elem in arr_path.elems.iter().flatten() {
  let elem_value = evaluate(&elem.expr, traversal_state, &state.functions);
  collect_confident!(elem_value, arr);
}
```

**Impact**: Reduces ~60 lines, improves consistency

---

## 📊 Refactoring Priority Matrix

| Violation | Files Affected | Lines Saved | Complexity Reduction | Priority |
|-----------|----------------|-------------|----------------------|----------|
| ParenExpr + panic pattern | 4 | ~570 | High | 🔴 Critical |
| ParenExpr wrapper | 2 | ~276 | Medium | 🔴 Critical |
| unwrap_or_else panic | 3 | ~150 | Low | 🔴 Critical |
| Confident check | 1 | ~60 | Medium | 🟡 Medium |
| InjectableStyle creation | 15 | ~60 | Low | 🟡 Medium |
| Expression cloning chain | 2 | ~36 | Medium | 🟡 Medium |
| Hash name creation | 14 | ~88 | Low | 🟢 Low |
| Unary numeric ops | 1 | ~20 | High | 🟡 Medium |

**Total Estimated Reduction**: ~1,260 lines of code
**Estimated Time**: 4-6 hours for all refactorings

---

## 🚀 Implementation Roadmap

### Phase 1: Critical Macros (Week 1)
1. Add `unwrap_or_panic!` to `error_helpers.rs`
2. Add `panic_with_context!` to `error_helpers.rs`
3. Add `wrap_in_paren` functions to `ast/factories.rs`
4. Update all call sites in `evaluate.rs`
5. Run full test suite

### Phase 2: Utility Functions (Week 2)
1. Add `InjectableStyle::regular()` and `::with_rtl()` builders
2. Add `EvaluateResultValue::into_object()` and `::into_array()`
3. Add `create_hashed_name()` to `common.rs`
4. Update call sites across workspace
5. Run full test suite

### Phase 3: Minor Optimizations (Week 3)
1. Add `evaluate_unary_numeric()` helper
2. Add `collect_confident!` macro
3. Create type aliases for common collections
4. Update call sites
5. Run full test suite
6. Performance benchmarks

### Phase 4: Documentation & Cleanup
1. Update macro documentation with examples
2. Add migration guide to existing code
3. Update CONTRIBUTING.md with DRY guidelines
4. Code review and polish

---

## 📝 Example PR Structure

```
feat: Apply DRY principle to error handling macros

- Add `unwrap_or_panic!` macro to error_helpers.rs
- Add `panic_with_context!` macro for common error pattern
- Add `wrap_in_paren` utility functions to ast/factories.rs
- Replace 114 instances of repetitive error handling
- Reduce codebase by ~570 lines
- All tests passing

BREAKING CHANGE: None (additive changes only)
```

---

## 🎯 Success Metrics

- **Lines of Code Reduced**: Target 1,200+ lines
- **Test Coverage**: Maintain 100% of existing coverage
- **Build Time**: No regression (or improvement)
- **Readability**: Improved intent clarity in error paths
- **Maintainability**: Centralized error handling patterns

---

## ⚠️ Risks & Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Breaking existing code | High | Low | Comprehensive test suite |
| Performance regression | Medium | Very Low | Inline macros, benchmarks |
| Over-abstraction | Low | Medium | Keep macros simple, well-documented |
| Resistance to change | Low | Low | Show clear benefits, gradual rollout |

---

## 🔍 Code Review Checklist

- [ ] All new macros have comprehensive documentation
- [ ] Usage examples provided for each macro
- [ ] All existing tests pass
- [ ] No new clippy warnings
- [ ] Performance benchmarks show no regression
- [ ] Code review by 2+ team members
- [ ] Update CHANGELOG.md

---

*Generated: $(date)*
*Analyzer: DRY Principle Violation Detector v1.0*

