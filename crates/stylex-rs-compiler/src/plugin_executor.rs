use log::{error, info};
use napi::{Env, Result};
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use swc_core::{common::SourceMap, ecma::ast::Program};
use wasmer::{Function, FunctionEnv, FunctionEnvMut, Instance, Module, Store, Value};
use wasmer_wasi::{Pipe, WasiEnv, WasiFunctionEnv, WasiState};

use crate::plugin_resolver::resolve_plugin_path;
use crate::structs::SwcPluginConfig;

/// Environment data for host functions
#[derive(Clone)]
struct PluginEnv {
  name: String,
}

/// Applies SWC WASM plugins using wasmer runtime.
///
/// This implementation directly executes SWC WASM plugins using wasmer,
/// avoiding the version conflicts with swc_plugin_runner.
///
/// ## Implementation Details
///
/// 1. **Load Plugin**: Read .wasm file and compile with wasmer
/// 2. **Setup WASI**: Initialize WASI environment (required by SWC plugins)
/// 3. **Provide Host Functions**: Stub implementations of SWC's host functions
/// 4. **Serialize AST**: Convert Program to bytes (currently using JSON as placeholder)
/// 5. **Execute Plugin**: Call __plugin_process_impl with serialized data
/// 6. **Deserialize Result**: Convert result bytes back to Program
///
/// ## Current Status
///
/// ✅ WASM module loading and instantiation
/// ✅ WASI environment setup
/// ✅ Plugin function calls
/// ⚠️  AST serialization/deserialization (using placeholder format)
///
/// The main challenge is matching SWC's exact rkyv serialization format.
/// For now, we demonstrate the execution flow with placeholder serialization.
pub fn apply_swc_plugins(
  env: &Env,
  mut program: Program,
  plugins: Vec<SwcPluginConfig>,
  _cm: Arc<SourceMap>,
  cwd: &Path,
) -> Result<Program> {
  if plugins.is_empty() {
    return Ok(program);
  }

  let plugin_count = plugins.len();
  info!(
    "Applying {} SWC plugin(s) using wasmer runtime",
    plugin_count
  );

  for plugin_config in plugins {
    info!("Processing plugin: {}", plugin_config.name);

    // Resolve and read the plugin WASM file
    let plugin_path = resolve_plugin_path(&plugin_config.name, cwd)?;
    info!("  Plugin path: {}", plugin_path.display());

    let wasm_bytes = std::fs::read(&plugin_path).map_err(|e| {
      napi::Error::from_reason(format!(
        "Failed to read plugin WASM file '{}': {}",
        plugin_config.name, e
      ))
    })?;

    info!("  Loaded {} bytes of WASM", wasm_bytes.len());

    // Execute the plugin
    program = execute_plugin_with_wasmer(env, program, &plugin_config, &wasm_bytes)?;

    info!("  ✓ Plugin '{}' completed", plugin_config.name);
  }

  info!("All {} SWC plugin(s) processed", plugin_count);
  Ok(program)
}

/// Execute a single SWC plugin using wasmer
fn execute_plugin_with_wasmer(
  env: &Env,
  program: Program,
  plugin_config: &SwcPluginConfig,
  wasm_bytes: &[u8],
) -> Result<Program> {
  // Create wasmer store and compile module
  let mut store = Store::default();

  let module = Module::new(&store, wasm_bytes).map_err(|e| {
    napi::Error::from_reason(format!(
      "Failed to compile WASM module '{}': {}",
      plugin_config.name, e
    ))
  })?;

  info!("  WASM module compiled successfully");

  // Setup WASI environment (required by SWC plugins)
  // Create pipes for stdin/stdout/stderr to capture plugin output
  let stdout = Pipe::new();
  let stderr = Pipe::new();

  let wasi_state = WasiState::new(&plugin_config.name)
    .stdout(Box::new(stdout.clone()))
    .stderr(Box::new(stderr.clone()))
    .build()
    .map_err(|e| {
      napi::Error::from_reason(format!(
        "Failed to build WASI state for plugin '{}': {}",
        plugin_config.name, e
      ))
    })?;

  let wasi_env = WasiEnv::new(wasi_state);
  let wasi_fn_env = WasiFunctionEnv::new(&mut store, wasi_env);

  // Create import object with WASI imports
  let wasi_import_object = wasi_fn_env
    .import_object(&mut store, &module)
    .map_err(|e| {
      napi::Error::from_reason(format!(
        "Failed to generate WASI imports for plugin '{}': {}",
        plugin_config.name, e
      ))
    })?;

  let mut import_object = wasi_import_object;

  // Create plugin environment for host functions
  let plugin_env = PluginEnv {
    name: plugin_config.name.clone(),
  };
  let env_data = FunctionEnv::new(&mut store, plugin_env);

  // Create host function stubs that SWC plugins may call
  // These are optional - only add them if they don't conflict with existing imports
  let comments_get = Function::new_typed_with_env(&mut store, &env_data, host_comments_get);
  let comments_add = Function::new_typed_with_env(&mut store, &env_data, host_comments_add);
  let source_map_lookup =
    Function::new_typed_with_env(&mut store, &env_data, host_source_map_lookup);
  let plugin_error = Function::new_typed_with_env(&mut store, &env_data, host_plugin_error);

  // Create additional host functions
  let emit_diagnostics = Function::new_typed_with_env(&mut store, &env_data, host_emit_diagnostics);
  let set_transform_result = Function::new_typed_with_env(&mut store, &env_data, host_set_transform_result);
  let set_core_pkg_diagnostics = Function::new_typed_with_env(&mut store, &env_data, host_set_core_pkg_diagnostics);
  let get_plugin_config = Function::new_typed_with_env(&mut store, &env_data, host_get_plugin_config);

  // Register host functions in the "env" namespace
  import_object.define("env", "__swc_plugin_comments_get", comments_get);
  import_object.define(
    "env",
    "__swc_plugin_comments_add_leading",
    comments_add.clone(),
  );
  import_object.define("env", "__swc_plugin_comments_add_trailing", comments_add);
  import_object.define("env", "__swc_plugin_source_map_lookup", source_map_lookup);
  import_object.define("env", "__swc_plugin_error", plugin_error);
  import_object.define("env", "__emit_diagnostics", emit_diagnostics);
  import_object.define("env", "__set_transform_result", set_transform_result);
  import_object.define("env", "__set_transform_plugin_core_pkg_diagnostics", set_core_pkg_diagnostics);
  import_object.define("env", "__get_transform_plugin_config", get_plugin_config);

  // Instantiate the WASM module
  let instance = Instance::new(&mut store, &module, &import_object).map_err(|e| {
    napi::Error::from_reason(format!(
      "Failed to instantiate plugin '{}': {}",
      plugin_config.name, e
    ))
  })?;

  info!("  Plugin instantiated successfully");

  // Get plugin options as JSON
  let options_json = plugin_config.options_to_json(env)?;
  info!("  Plugin options: {}", options_json);

  // Serialize program to rkyv bytes
  let program_serialized = serialize_program(&program)?;

  // Allocate memory in WASM for the program data
  let alloc_fn = instance.exports.get_function("__alloc").map_err(|e| {
    napi::Error::from_reason(format!(
      "Plugin '{}' doesn't export __alloc function: {}",
      plugin_config.name, e
    ))
  })?;

  let program_len = program_serialized.len() as i32;
  let program_ptr = alloc_fn
    .call(&mut store, &[Value::I32(program_len)])
    .map_err(|e| {
      napi::Error::from_reason(format!("Failed to allocate memory for program data: {}", e))
    })?[0]
    .i32()
    .ok_or_else(|| napi::Error::from_reason("Invalid return value from __alloc"))?;

  info!(
    "  Allocated {} bytes at WASM address {}",
    program_len, program_ptr
  );

  // Write program data to WASM memory
  let memory = instance.exports.get_memory("memory").map_err(|e| {
    napi::Error::from_reason(format!(
      "Plugin '{}' doesn't export memory: {}",
      plugin_config.name, e
    ))
  })?;

  let memory_view = memory.view(&store);
  memory_view
    .write(program_ptr as u64, &program_serialized)
    .map_err(|e| {
      napi::Error::from_reason(format!(
        "Failed to write program data to WASM memory: {}",
        e
      ))
    })?;

  // Allocate memory for plugin config
  let config_bytes = options_json.as_bytes();
  let config_len = config_bytes.len() as i32;

  let config_ptr = alloc_fn
    .call(&mut store, &[Value::I32(config_len)])
    .map_err(|e| {
      napi::Error::from_reason(format!("Failed to allocate memory for config: {}", e))
    })?[0]
    .i32()
    .ok_or_else(|| napi::Error::from_reason("Invalid return value from __alloc for config"))?;

  memory_view
    .write(config_ptr as u64, config_bytes)
    .map_err(|e| {
      napi::Error::from_reason(format!("Failed to write config to WASM memory: {}", e))
    })?;

  info!(
    "  Allocated {} bytes for config at WASM address {}",
    config_len, config_ptr
  );

  // Call the plugin's transform function
  let transform_fn = instance
    .exports
    .get_function("__plugin_process_impl")
    .map_err(|e| {
      napi::Error::from_reason(format!(
        "Plugin '{}' doesn't export __plugin_process_impl: {}",
        plugin_config.name, e
      ))
    })?;

  info!("  Calling __plugin_process_impl...");

  let result = transform_fn
    .call(
      &mut store,
      &[
        Value::I32(program_ptr),
        Value::I32(program_len),
        Value::I32(config_ptr),
        Value::I32(config_len),
      ],
    )
    .map_err(|e| {
      napi::Error::from_reason(format!(
        "Plugin '{}' transform failed: {}",
        plugin_config.name, e
      ))
    })?;

  let result_ptr = result[0]
    .i32()
    .ok_or_else(|| napi::Error::from_reason("Invalid return value from __plugin_process_impl"))?;

  info!("  Plugin returned result at address: {}", result_ptr);

  // Read result length (first 4 bytes)
  let mut len_bytes = [0u8; 4];
  memory_view
    .read(result_ptr as u64, &mut len_bytes)
    .map_err(|e| napi::Error::from_reason(format!("Failed to read result length: {}", e)))?;
  let result_len = u32::from_le_bytes(len_bytes) as usize;

  info!("  Result data size: {} bytes", result_len);

  // Read the transformed data
  let mut result_data = vec![0u8; result_len];
  memory_view
    .read((result_ptr + 4) as u64, &mut result_data)
    .map_err(|e| napi::Error::from_reason(format!("Failed to read result data: {}", e)))?;

  // Clean up WASM memory
  if let Ok(free_fn) = instance.exports.get_function("__free") {
    let _ = free_fn.call(
      &mut store,
      &[Value::I32(program_ptr), Value::I32(program_len)],
    );
    let _ = free_fn.call(
      &mut store,
      &[Value::I32(config_ptr), Value::I32(config_len)],
    );
    // Note: Don't free result_ptr as it may be managed by the plugin
  }

  // Deserialize result back to Program
  let transformed_program = deserialize_program(&result_data, &program)?;

  // Read any output from plugin's stdout/stderr for debugging
  let mut stdout_buffer = String::new();
  if let Ok(size) = stdout.clone().read_to_string(&mut stdout_buffer)
    && size > 0
  {
    info!("  Plugin stdout: {}", stdout_buffer.trim());
  }

  let mut stderr_buffer = String::new();
  if let Ok(size) = stderr.clone().read_to_string(&mut stderr_buffer)
    && size > 0
  {
    info!("  Plugin stderr: {}", stderr_buffer.trim());
  }

  info!("  ✓ Plugin transformation completed successfully");

  Ok(transformed_program)
}

/// Serializes a Program to rkyv bytes for plugin consumption
///
/// Uses rkyv serialization with aligned buffer to match SWC's format.
/// SWC plugins expect data in the rkyv AlignedVec format.
fn serialize_program(program: &Program) -> Result<Vec<u8>> {
  use rkyv::rancor;

  // Use rkyv::to_bytes which creates an AlignedVec automatically
  let bytes = rkyv::to_bytes::<rancor::Error>(program).map_err(|e| {
    napi::Error::from_reason(format!("Failed to serialize Program with rkyv: {:?}", e))
  })?;

  info!("  Serialized Program to {} bytes with rkyv", bytes.len());
  Ok(bytes.to_vec())
}

/// Deserializes rkyv bytes back to a Program
///
/// Safely deserializes the plugin's output using rkyv with validation.
/// The data format matches what SWC plugins produce.
fn deserialize_program(data: &[u8], _original: &Program) -> Result<Program> {
  use rkyv::rancor;

  if data.is_empty() {
    return Err(napi::Error::from_reason(
      "Cannot deserialize empty data from plugin",
    ));
  }

  // Deserialize using rkyv::from_bytes
  let program: Program = rkyv::from_bytes::<Program, rancor::Error>(data).map_err(|e| {
    napi::Error::from_reason(format!(
      "Failed to deserialize Program from plugin output: {:?}",
      e
    ))
  })?;

  info!(
    "  Successfully deserialized Program from {} bytes",
    data.len()
  );
  Ok(program)
}

// Host function stubs that SWC plugins may call

fn host_comments_get(_env: FunctionEnvMut<PluginEnv>, _ptr: i32, _len: i32) -> i32 {
  // Return 0 (NULL) to indicate no comments
  0
}

fn host_comments_add(
  _env: FunctionEnvMut<PluginEnv>,
  _span_ptr: i32,
  _comment_ptr: i32,
  _comment_len: i32,
) {
  // Stub - do nothing
}

fn host_source_map_lookup(_env: FunctionEnvMut<PluginEnv>, _line: i32, _column: i32) -> i32 {
  // Return 0 to indicate no source map data
  0
}

fn host_plugin_error(env: FunctionEnvMut<PluginEnv>, msg_ptr: i32, msg_len: i32) {
  let plugin_name = &env.data().name;
  error!(
    "Plugin '{}' reported error at ptr={}, len={}",
    plugin_name, msg_ptr, msg_len
  );
}

fn host_emit_diagnostics(env: FunctionEnvMut<PluginEnv>, diagnostic_ptr: i32, diagnostic_len: i32) {
  let plugin_name = &env.data().name;
  info!(
    "Plugin '{}' emitted diagnostics at ptr={}, len={}",
    plugin_name, diagnostic_ptr, diagnostic_len
  );
  // In a full implementation, you would:
  // 1. Read the diagnostic data from WASM memory
  // 2. Deserialize it
  // 3. Forward it to the host's diagnostic system
}

fn host_set_transform_result(env: FunctionEnvMut<PluginEnv>, result_ptr: i32, result_len: i32) {
  let plugin_name = &env.data().name;
  info!(
    "Plugin '{}' set transform result at ptr={}, len={}",
    plugin_name, result_ptr, result_len
  );
  // This is called by the plugin to set the transformation result
  // The actual result is read via __plugin_process_impl return value
  // This is likely for additional metadata or alternative result storage
}

fn host_set_core_pkg_diagnostics(env: FunctionEnvMut<PluginEnv>, diag_ptr: i32, diag_len: i32) {
  let plugin_name = &env.data().name;
  info!(
    "Plugin '{}' set core package diagnostics at ptr={}, len={}",
    plugin_name, diag_ptr, diag_len
  );
  // This is called by plugins via __get_transform_plugin_core_pkg_diag
  // to report plugin metadata (AST schema version, package version, git SHA, etc.)
  // In a full implementation, you would:
  // 1. Read the diagnostics data from WASM memory
  // 2. Deserialize PluginCorePkgDiagnostics
  // 3. Verify compatibility with host
  // 4. Log or store the version information
}

fn host_get_plugin_config(env: FunctionEnvMut<PluginEnv>, _config_key: i32) -> i32 {
  let plugin_name = &env.data().name;
  info!(
    "Plugin '{}' requested plugin config via __get_transform_plugin_config",
    plugin_name
  );
  // This function is called by plugins to retrieve their configuration
  // The config is already passed via __plugin_process_impl parameters,
  // so we return 0 (NULL) to indicate no additional config is available
  // In a full implementation with alternative config storage:
  // 1. Look up stored config based on config_key
  // 2. Serialize config to WASM memory
  // 3. Return pointer to the serialized config
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  use swc_core::ecma::ast::Module;

  #[test]
  fn test_apply_swc_plugins_empty_list() {
    let cm = Arc::new(SourceMap::default());
    let program = Program::Module(Module {
      span: Default::default(),
      body: vec![],
      shebang: None,
    });

    // Empty list should return program unchanged
    // (Would need NAPI env to actually test)
  }
}
