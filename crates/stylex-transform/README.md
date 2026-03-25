# `StyleX Transform`

## Overview

This package provides the SWC Fold implementation and transform orchestration for the StyleX compiler. Contains `StyleXTransform<C>`, `StateManager` (1,043 lines), and all SWC `Fold` method implementations. ~5,000 lines.

## Architecture

Layer 2 orchestration crate. Depends on all lower crates (Layer 0 & 1). This is the top-level plugin integration point and replaces the original `stylex-shared` crate.

## Key Components

### `StyleXTransform<C>`

Main SWC plugin struct that implements the `Fold` trait:

```rust
pub struct StyleXTransform<C: Comments> {
  state: StateManager,
  _phantom: PhantomData<C>,
}

impl<C: Comments> Fold for StyleXTransform<C> {
  // Fold implementations for all AST node types
}
```

- Generic over `C: Comments` for SWC integration
- Walks the entire AST and transforms StyleX calls
- Maintains mutable `StateManager` across traversal

### `StateManager`

Central state container implementing all three traits:

```rust
pub struct StateManager {
  // Declarations and imports tracking
  pub declarations: Vec<VarDeclarator>,
  pub top_imports: Vec<ImportDecl>,
  // Style tracking
  pub style_map: FxHashMap<String, Rc<StylesObjectMap>>,
  pub stylex_import: FxHashSet<ImportSources>,
  // ... 20+ fields
}

impl StyleOptions for StateManager { ... }
impl EvaluationContext for StateManager { ... }
impl TransformState for StateManager { ... }
```

**Responsibilities:**
- **Declarations tracking**: variable declarations, imports, function names
- **Style registration**: collect and deduplicate styles
- **Import tracking**: track which StyleX functions are imported
- **Caching**: span cache for evaluation results
- **Code frame context**: filename, source code for error reporting

## Key Modules

- **`transform/`** — SWC Fold implementation
  - `stylex/` — StyleX function call handlers
    - `stylex_create/` — `stylex.create()` transformation
    - `stylex_define_vars/` — `stylex.defineVars()` transformation
    - ... other StyleX API handlers
  - Fold methods for: Module, ImportDecl, FunctionDecl, CallExpression, etc.

- **`state_manager.rs`** — Central state container
  - Import and declaration tracking
  - Style map management
  - Variable counter management
  - Span cache for expensive computations

## Trait Implementation Pattern

`StateManager` implements all three trait tiers:

1. **`StyleOptions`** — Minimal CSS-generation interface
   - Direct access via field reads

2. **`EvaluationContext`** — Extended JS evaluation interface
   - Delegate to fields or compute on demand

3. **`TransformState`** — Full transform state with imports
   - All StyleX import set accessors
   - Style map queries

This allows lower-layer crates to accept flexible trait bounds:

- Functions needing only CSS metadata: `&mut dyn StyleOptions`
- Functions needing evaluation: `<T: EvaluationContext + Clone>`
- Functions needing import tracking: `<T: TransformState + Clone>`

## Dependencies

- **Layer 0**: `stylex-constants`, `stylex-types`, `stylex-ast`
- **Layer 1**: `stylex-evaluator`, `stylex-css`, `stylex-transformers`
- **SWC**: `swc_core` (full), `swc_ecma_parser`, `swc_compiler_base`
- **Utilities**: `once_cell` (lazy statics), `log`

## Usage

The NAPI compiler (`stylex-rs-compiler`) uses `StyleXTransform`:

```rust
use stylex_transform::StyleXTransform;
use swc_core::common::{FileName, SourceMap};

// Create transform for a module
let cm = Arc::new(SourceMap::default());
let transform = StyleXTransform::new(
  &PluginPass { /* ... */ },
  cm.clone(),
);

// Apply to module (used by SWC plugin system)
let transformed = module.fold_with(&mut transform);
```

## Integration Points

1. **SWC Plugin System**: Registered as `Plugin::Transform`
2. **NAPI Boundary**: Called from `stylex-rs-compiler`
3. **Metadata Export**: Collects styles for output metadata
4. **Source Maps**: Preserves span information for debugging
