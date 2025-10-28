use anyhow::{Context, Result};
use log::{info, error};
use std::path::Path;
use std::sync::Arc;

use swc_core::{
    common::{
        plugin::metadata::TransformPluginMetadataContext,
        Globals,
        Mark,
        SourceMap,
        GLOBALS,
    },
    ecma::ast::Program,
};
use swc_plugin_runner::{
    create_plugin_transform_executor,
    plugin_module_bytes::{CompiledPluginModuleBytes, RawPluginModuleBytes},
};

/// Configuration for a single SWC plugin
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub path: String,
    pub config_json: String,
}

/// Execute a list of SWC plugins on a program AST
///
/// This function uses swc_plugin_runner to load and execute WASM plugins.
/// Each plugin is applied in sequence to the AST.
///
/// # Arguments
/// * `program` - The AST to transform
/// * `plugins` - List of plugin configurations
/// * `source_map` - The source map for the compilation (as Arc)
/// * `unresolved_mark` - Mark for unresolved references
///
/// # Returns
/// The transformed AST or an error
pub fn apply_swc_plugins(
    program: Program,
    plugins: Vec<PluginConfig>,
    source_map: Arc<SourceMap>,
    unresolved_mark: Mark,
) -> Result<Program> {
    if plugins.is_empty() {
        return Ok(program);
    }

    info!("Applying {} SWC plugin(s) using swc_plugin_runner", plugins.len());

    // Create metadata context (filename, env, experimental)
    let metadata_context = Arc::new(TransformPluginMetadataContext::new(
        None,  // filename
        "development".to_string(),  // env
        None,  // experimental config
    ));

    // Serialize the program using rkyv
    use swc_core::plugin::serialized::{PluginSerializedBytes, VersionedSerializable};

    let mut serialized_program = PluginSerializedBytes::try_serialize(
        &VersionedSerializable::new(program)
    ).context("Failed to serialize program")?;

    // Apply each plugin in sequence
    for plugin_config in plugins {
        info!("Processing plugin: {}", plugin_config.name);

        // Load the plugin WASM file
        let plugin_path = Path::new(&plugin_config.path);

        if !plugin_path.exists() {
            error!("Plugin WASM file not found: {}", plugin_config.path);
            return Err(anyhow::anyhow!(
                "Plugin WASM file not found: {}",
                plugin_config.path
            ));
        }

        let wasm_bytes = std::fs::read(plugin_path)
            .with_context(|| format!("Failed to read plugin WASM file: {}", plugin_config.path))?;

        info!("  Loaded {} bytes of WASM from {}", wasm_bytes.len(), plugin_config.path);

        // Wrap in GLOBALS.set() to provide SWC's global context
        serialized_program = GLOBALS.set(&Globals::new(), || -> Result<PluginSerializedBytes> {
            // Create raw plugin bytes with identifier and bytes
            let identifier = format!("file://{}", plugin_config.path);
            let raw_bytes = RawPluginModuleBytes::new(identifier.clone(), wasm_bytes);

            // Create compiled plugin module - no runtime parameter in v18
            let compiled = CompiledPluginModuleBytes::new(
                identifier,
                raw_bytes,
            );

            // Parse plugin config
            let plugin_config_value: Option<serde_json::Value> = if plugin_config.config_json.is_empty() {
                None
            } else {
                Some(serde_json::from_str(&plugin_config.config_json)
                    .context("Failed to parse plugin config JSON")?)
            };

            // Create the transform executor (v18 uses Option<()> for runtime)
            let mut executor = create_plugin_transform_executor(
                &source_map,
                &unresolved_mark,
                &metadata_context,
                None, // plugin_env_vars
                Box::new(compiled),
                plugin_config_value,
                None, // runtime parameter (not used in v18)
            );

            // Execute the transformation
            let transformed = executor
                .transform(&serialized_program, None)
                .with_context(|| format!("Plugin transformation failed: {}", plugin_config.name))?;

            info!("  ✓ Plugin '{}' completed", plugin_config.name);

            Ok(transformed)
        })?;
    }

    // Deserialize the final result
    let result: Program = serialized_program
        .deserialize()
        .context("Failed to deserialize transformed program")?
        .into_inner();

    info!("All {} SWC plugin(s) processed successfully", plugins.len());
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_plugins() {
        let source_map = Arc::new(SourceMap::default());
        let program = Program::Module(swc_core::ecma::ast::Module {
            span: Default::default(),
            body: vec![],
            shebang: None,
        });

        let result = GLOBALS.set(&Globals::new(), || {
            apply_swc_plugins(
                program,
                vec![],
                source_map,
                Mark::fresh(Mark::root()),
            )
        });

        assert!(result.is_ok());
    }
}
