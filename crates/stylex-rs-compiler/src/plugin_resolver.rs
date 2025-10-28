use std::path::{Path, PathBuf};
use napi::Result;
use stylex_path_resolver::package_json::{find_closest_node_modules, recursive_find_node_modules};

/// Resolves a plugin name to an actual WASM file path.
///
/// Resolution strategy:
/// 1. Try as absolute path
/// 2. Try as relative path from CWD
/// 3. Try as npm package in node_modules
pub fn resolve_plugin_path(plugin_name: &str, cwd: &Path) -> Result<PathBuf> {
  let plugin_path = Path::new(plugin_name);

  // Strategy 1: Try as absolute path
  if plugin_path.is_absolute() {
    if plugin_path.exists() {
      return Ok(plugin_path.to_path_buf());
    }
    return Err(napi::Error::from_reason(format!(
      "Failed to load SWC plugin '{}': file not found at absolute path {}",
      plugin_name,
      plugin_path.display()
    )));
  }

  // Strategy 2: Try as relative path from CWD
  let cwd_relative = cwd.join(plugin_path);
  if cwd_relative.exists() {
    return Ok(cwd_relative);
  }

  // Strategy 3: Try as npm package in node_modules
  // First, find closest node_modules
  if let Some(closest_nm) = find_closest_node_modules(cwd) {
    // Try direct .wasm file: node_modules/{plugin_name}.wasm
    let direct_wasm = closest_nm.join(format!("{}.wasm", plugin_name));
    if direct_wasm.exists() {
      return Ok(direct_wasm);
    }

    // Try package directory: node_modules/{plugin_name}/*.wasm
    let package_dir = closest_nm.join(plugin_name);
    if package_dir.exists() && package_dir.is_dir()
      && let Ok(entries) = std::fs::read_dir(&package_dir) {
      for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
          return Ok(path);
        }
      }
    }
  }

  // Try recursively finding node_modules up the directory tree
  let all_node_modules = recursive_find_node_modules(cwd, None);
  for nm_path in all_node_modules {
    // Try direct .wasm file
    let direct_wasm = nm_path.join(format!("{}.wasm", plugin_name));
    if direct_wasm.exists() {
      return Ok(direct_wasm);
    }

    // Try package directory
    let package_dir = nm_path.join(plugin_name);
    if package_dir.exists() && package_dir.is_dir()
      && let Ok(entries) = std::fs::read_dir(&package_dir) {
      for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
          return Ok(path);
        }
      }
    }
  }

  // Plugin not found anywhere
  Err(napi::Error::from_reason(format!(
    "Failed to load SWC plugin '{}': plugin not found. Searched in:\n\
     - Absolute path: {}\n\
     - Relative to CWD: {}\n\
     - node_modules as npm package\n\
     \n\
     Please ensure the plugin is installed or provide the correct path.",
    plugin_name,
    plugin_path.display(),
    cwd_relative.display()
  )))
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn test_resolve_plugin_path_absolute() {
    // This test would need an actual .wasm file to work
    // Just testing the logic structure
    let cwd = env::current_dir().unwrap();
    let result = resolve_plugin_path("/nonexistent/plugin.wasm", &cwd);
    assert!(result.is_err());
  }

  #[test]
  fn test_resolve_plugin_path_not_found() {
    let cwd = env::current_dir().unwrap();
    let result = resolve_plugin_path("nonexistent-plugin", &cwd);
    assert!(result.is_err());
    if let Err(e) = result {
      assert!(e.reason.contains("Failed to load SWC plugin"));
    }
  }
}

