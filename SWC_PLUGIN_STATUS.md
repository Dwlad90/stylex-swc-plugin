# SWC Plugin Pre-Transpilation - Implementation Status

## ✅ What's Completed

### 1. Full Configuration Support
- ✅ Added `swcPlugins` field to `StyleXOptions`
- ✅ Accepts Next.js format: `[['plugin-name', {options}]]`
- ✅ Proper TypeScript types generated
- ✅ Tuple parsing from JavaScript to Rust structs

### 2. Comprehensive Plugin Resolution
- ✅ Absolute path resolution
- ✅ Relative path resolution (from CWD)
- ✅ npm package resolution in `node_modules/`
- ✅ Recursive directory tree searching
- ✅ Detailed error messages for missing plugins

### 3. Full Validation Pipeline
- ✅ Plugin path validation
- ✅ WASM file accessibility checks
- ✅ File size logging
- ✅ JavaScript to JSON option conversion
- ✅ Pre-execution verification

### 4. Integration
- ✅ Integrated into transform pipeline
- ✅ Positioned after file filtering, before StyleX transformation
- ✅ Proper error propagation
- ✅ Logging at all stages

### 5. Dependencies
- ✅ Added `swc_plugin_runner` v21.0.0
- ✅ Added `stylex_path_resolver` dependency
- ✅ All code compiles without errors

## ⚠️ What's NOT Completed

### WASM Plugin Execution

**Status:** Validation only - plugins are NOT executed

**Reason:** Multiple approaches attempted, all blocked by API limitations:

### Attempted Approach 1: swc_plugin_runner v20-21

- **Problem:** Complex API requiring 7+ parameters
- **Issue:** Poorly documented for external use
- **Blocker:** API designed for internal SWC usage, not NAPI modules

### Attempted Approach 2: common_plugin_transform Feature

- **Source:** Community guide suggesting `swc_core::Compiler` with `common_plugin_transform`
- **Problem:** API mismatch with swc_core 44.0.2
- **Issue:** Types like `swc_core::base::config` don't exist in our version
- **Blocker:** Guide appears to be for older SWC versions

### Attempted Approach 3: swc_core::Compiler Direct Usage

- **Problem:** `Compiler` type not exposed in swc_core 44.0.2 public API
- **Issue:** Only available in `swc_compiler_base` which has different structure
- **Blocker:** No clear way to integrate with existing parsed Program AST

### Root Cause

The SWC plugin system is primarily designed for:
1. **Internal use within SWC compiler itself**
2. **CLI usage via configuration files**
3. **Not** for programmatic external usage in NAPI modules

External plugin loading requires deep knowledge of SWC internals and version-specific APIs that change frequently.

## Current Behavior

When you specify `swcPlugins`:

```typescript
rsOptions: {
  swcPlugins: [
    ['@haaretz/l-swc-plugin-theme', { site: 'htz' }]
  ],
}
```

The system will:

1. ✅ Parse the configuration correctly
2. ✅ Resolve the plugin path
3. ✅ Validate the WASM file exists and is readable
4. ✅ Convert options to JSON
5. ⚠️ Log a warning that execution is not implemented
6. ✅ Return the program **unchanged**
7. ✅ Continue with StyleX transformation

## Logs You'll See

```
[INFO] Transforming source file: MyComponent.tsx
[INFO] SWC Plugin pre-transpilation requested for 1 plugin(s)
[INFO] Validating plugin: @haaretz/l-swc-plugin-theme
[INFO]   Found plugin at: /path/to/node_modules/@haaretz/l-swc-plugin-theme/plugin.wasm
[INFO]   Plugin file size: 1234567 bytes
[INFO]   Plugin options: {"site":"htz"}
[INFO]   Plugin validated successfully
[WARN] SWC Plugin execution not yet implemented for swc_plugin_runner v21.
[WARN] All 1 plugin(s) have been validated but NOT executed.
[WARN] Program returned unchanged. See plugin_executor.rs for implementation details.
```

## How to Complete

Given the challenges encountered, here are realistic options:

### Option 1: Wait for SWC Stabilization
- Wait for SWC team to provide stable external plugin API
- Monitor SWC releases for improved documentation
- Track swc_core updates for public plugin interfaces

### Option 2: Subprocess Approach
- Execute SWC CLI as subprocess with plugin config
- Simpler but slower (process overhead)
- More reliable (uses official SWC implementation)
```rust
// Pseudo-code
std::process::Command::new("swc")
  .arg("--config")
  .arg(temp_config_with_plugins)
  .arg(temp_input_file)
  .output()?
```

### Option 3: JavaScript Layer Pre-processing
- Run SWC with plugins in JavaScript/TypeScript layer
- Pass already-transformed code to Rust
- Cleaner separation of concerns
```typescript
// In JavaScript wrapper
const swcResult = await transform(code, {
  jsc: {
    experimental: {
      plugins: swcPlugins
    }
  }
});
// Then pass to Rust compiler
rust_compiler.transform(swcResult.code, options);
```

### Option 4: Deep SWC Internals Dive
- Study SWC's own source code
- Reverse-engineer current internal APIs
- High maintenance burden (API changes frequently)
- Not recommended unless absolutely necessary

**Recommended:** Option 3 (JavaScript layer) - Most maintainable and reliable

## Files Changed

- `Cargo.toml` - Added swc_plugin_runner dependency
- `crates/stylex-rs-compiler/Cargo.toml` - Added dependencies
- `crates/stylex-rs-compiler/src/lib.rs` - Parsing, integration
- `crates/stylex-rs-compiler/src/structs/mod.rs` - Types, JSON conversion
- `crates/stylex-rs-compiler/src/plugin_resolver.rs` - Path resolution (new)
- `crates/stylex-rs-compiler/src/plugin_executor.rs` - Validation (new)

## Testing

### Currently Works
- ✅ Empty plugin list (no-op)
- ✅ Plugin path resolution
- ✅ Validation pipeline
- ✅ Error handling for missing plugins
- ✅ Compilation without errors

### Needs Implementation
- ❌ Actual WASM execution
- ❌ Plugin transformation application
- ❌ Runtime error handling
- ❌ End-to-end integration tests

## Impact

**For Users:**
- The feature is safe to use (won't break builds)
- Plugins are validated but not executed
- Clear warnings in logs
- No impact on StyleX transformation
- Can be safely configured in anticipation of completion

**For Development:**
- Foundation is solid and complete
- Only execution layer needs implementation
- API complexity is the main blocker
- Once completed, will require minimal changes to user code

