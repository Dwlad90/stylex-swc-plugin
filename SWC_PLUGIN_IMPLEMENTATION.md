# SWC Plugin Pre-Transpilation Implementation

## Summary

This document describes the implementation of conditional SWC WASM plugin pre-transpilation support in the StyleX NAPI compiler.

## What Was Implemented

### 1. Configuration Structure (`src/structs/mod.rs`)

Added support for specifying SWC plugins via `StyleXOptions`:

```typescript
interface StyleXOptions {
  // ... existing fields
  swcPlugins?: Array<[string, Record<string, any>]>;
}
```

The configuration follows Next.js's format: `[['plugin-name', {...options}]]`

**Implementation Details:**
- JavaScript sends plugins as tuples: `[['plugin-name', {options}]]`
- Rust receives them as `Vec<JsUnknown>` and parses each tuple
- Helper function `parse_swc_plugin_config` converts JavaScript arrays to `SwcPluginConfig` structs
- This approach allows proper interop between JavaScript tuples and Rust structs

### 2. Plugin Resolution Module (`src/plugin_resolver.rs`)

Implements plugin path resolution with the following strategy:
1. Try as absolute path
2. Try as relative path from current working directory
3. Try as npm package in `node_modules/`
   - Direct `.wasm` file: `node_modules/{plugin-name}.wasm`
   - Package directory: `node_modules/{plugin-name}/*.wasm`
4. Recursively search up directory tree for additional `node_modules` directories

### 3. Plugin Execution Module (`src/plugin_executor.rs`)

**Status: VALIDATION ONLY - EXECUTION NOT IMPLEMENTED**

The plugin executor currently:
- ✅ Validates that all requested plugins exist
- ✅ Resolves plugin paths correctly
- ✅ Converts JavaScript options to JSON
- ✅ Verifies WASM files are readable
- ✅ Logs detailed validation information
- ❌ Does NOT execute plugins (returns program unchanged)

**Why Execution is Not Implemented:**

The `swc_plugin_runner` v21 API is complex and requires:
1. Proper `PluginModuleCache` initialization
2. Seven parameters for `create_plugin_transform_executor`:
   - Plugin path
   - SourceMap
   - Unresolved mark
   - Top-level mark
   - Comments proxy
   - Plugin metadata context
   - Environment information
3. Correct method calls on the executor

The API is not well-documented for external usage and varies significantly between versions.

### 4. Integration (`src/lib.rs`)

The plugin execution is properly integrated into the transform pipeline:
1. File filtering check
2. Parse program
3. **Apply SWC plugins** (if provided) ← New step
4. Apply StyleX transformation
5. Apply fixer
6. Generate output

## Current Status

### ✅ Completed
- Configuration structure with TypeScript types
- Plugin resolution logic (CWD, npm packages)
- Integration into transform pipeline
- Error handling and validation
- Module structure and organization

### ⚠️ Stub Implementation
- **Plugin execution** - Plugins are validated but not executed

### ❌ TODO
- Complete WASM plugin execution using `swc_plugin_runner`
- Integration tests with actual WASM plugins

## Why Plugin Execution is Stubbed

The `swc_plugin_runner` crate has a complex API that varies significantly between versions. To properly implement plugin execution, we need to:

1. Determine the correct `swc_plugin_runner` version compatible with `swc_core 44.0.2`
2. Understand the full API signature for loading and executing WASM plugins
3. Handle plugin contexts, unresolved marks, and other SWC internals

## How to Complete the Implementation

### Current Status

✅ Dependencies added: `swc_plugin_runner = "21.0.0"`
✅ Full validation pipeline implemented
✅ JSON serialization of options working
❌ WASM execution not implemented

### Steps to Complete

1. **Study the swc_plugin_runner v21 API**
   - Review docs.rs/swc_plugin_runner/21.0.0
   - Study SWC's internal usage in the compiler itself
   - Understand the required parameters and their construction

2. **Implement proper initialization in `plugin_executor.rs`**:
   ```rust
   use swc_plugin_runner::{
       cache::PluginModuleCache,
       create_plugin_transform_executor,
   };
   use swc_core::{
       common::{Mark, comments::NoopComments},
       plugin::metadata::TransformPluginMetadataContext,
   };

   // Initialize cache and marks
   let cache = Arc::new(PluginModuleCache::new(/* config */));
   let unresolved_mark = Mark::new();
   let top_level_mark = Mark::new();
   let comments = NoopComments;
   let metadata_context = TransformPluginMetadataContext::new(/* ... */);

   // Create executor with all 7 required parameters
   let mut executor = create_plugin_transform_executor(
       &plugin_path,
       Arc::clone(&cm),
       unresolved_mark,
       top_level_mark,
       Arc::new(comments),
       Arc::new(metadata_context),
       // ... environment info
   );

   // Transform program
   program = executor.transform_program(&program, options_bytes)?;
   ```

3. **Handle errors and edge cases**
   - Plugin initialization failures
   - Runtime transformation errors
   - Invalid WASM modules
   - Incompatible plugin versions

4. **Test with actual WASM plugins**
   - Create a simple test plugin
   - Verify it executes correctly
   - Test error handling
   - Validate transformed output

## Usage Example

```typescript
import { transform } from '@stylexswc/rs-compiler';

const result = transform(
  'Button.tsx',
  sourceCode,
  {
    // ... other StyleX options
    swcPlugins: [
      ['@swc/plugin-emotion', { sourceMap: true }],
      ['./custom-plugin.wasm', { customOption: 'value' }],
    ],
  }
);
```

## Files Changed

- `crates/stylex-rs-compiler/Cargo.toml` - Added stylex_path_resolver dependency
- `crates/stylex-rs-compiler/src/lib.rs` - Integrated plugin execution
- `crates/stylex-rs-compiler/src/structs/mod.rs` - Added SwcPluginConfig and swc_plugins field
- `crates/stylex-rs-compiler/src/plugin_resolver.rs` - New file for plugin path resolution
- `crates/stylex-rs-compiler/src/plugin_executor.rs` - New file for plugin execution (stub)

## Testing

Currently tested:
- ✅ Empty plugin list (no-op)
- ✅ Plugin resolution (path validation)
- ✅ Build compilation

Needs testing:
- ❌ Actual WASM plugin execution
- ❌ Plugin option passing
- ❌ Multiple plugins in sequence
- ❌ Error handling for malformed WASM
- ❌ Integration with real SWC plugins

## References

- [SWC Plugin Documentation](https://swc.rs/docs/plugin/ecmascript/getting-started)
- [swc_plugin_runner crate](https://docs.rs/swc_plugin_runner/latest/swc_plugin_runner/)
- [Next.js SWC Plugin Configuration](https://nextjs.org/docs/architecture/nextjs-compiler#swc-plugins-experimental)

