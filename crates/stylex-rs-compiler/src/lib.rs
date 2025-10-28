mod enums;
mod structs;
mod utils;
mod plugin_resolver;
mod plugin_executor;
use log::info;
use napi::{Env, Result};
use std::panic;
use std::{env, sync::Arc};
use structs::{StyleXMetadata, StyleXOptions, StyleXTransformResult};
use swc_compiler_base::{PrintArgs, SourceMapsConfig, print};

use stylex_shared::{
  StyleXTransform,
  shared::{
    structures::{
      plugin_pass::PluginPass,
      stylex_options::{StyleResolution, StyleXOptionsParams},
    },
    utils::log::logger,
  },
};
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

use swc_core::{
  common::{FileName, SourceMap},
  ecma::{
    ast::EsVersion,
    transforms::base::fixer::fixer,
    visit::{fold_pass, visit_mut_pass},
  },
  plugin::proxies::PluginCommentsProxy,
};

use napi_derive::napi;
use utils::extract_stylex_metadata;

use crate::enums::{ImportSourceUnion, PathFilterUnion, SourceMaps, StyleXModuleResolution};

fn extract_patterns(
  env: &Env,
  patterns_opt: &mut Option<Vec<napi::JsUnknown>>,
) -> Option<Vec<PathFilterUnion>> {
  patterns_opt.take().map(|patterns| {
    patterns
      .into_iter()
      .filter_map(|p| parse_js_pattern(env, p).ok())
      .collect()
  })
}

/// Parse SWC plugin config from JavaScript tuple format: [name, options]
fn parse_swc_plugin_config(_env: &Env, value: napi::JsUnknown) -> Result<structs::SwcPluginConfig> {
  use napi::JsObject;

  // Try to coerce to array
  let arr = value.coerce_to_object()?;

  // Get the first element (plugin name)
  let name: napi::JsString = arr.get_element(0)?;
  let plugin_name = name.into_utf8()?.as_str()?.to_owned();

  // Get the second element (plugin options)
  let options: JsObject = arr.get_element(1)?;

  Ok(structs::SwcPluginConfig {
    name: plugin_name,
    options,
  })
}

fn extract_swc_plugins(
  env: &Env,
  plugins_opt: &mut Option<Vec<napi::JsUnknown>>,
) -> Option<Vec<structs::SwcPluginConfig>> {
  plugins_opt.take().map(|plugins| {
    plugins
      .into_iter()
      .filter_map(|p| parse_swc_plugin_config(env, p).ok())
      .collect()
  })
}

#[napi]
pub fn transform(
  env: Env,
  filename: String,
  code: String,
  mut options: StyleXOptions,
) -> Result<StyleXTransformResult> {
  color_backtrace::install();
  logger::initialize();

  info!("Transforming source file: {}", filename);

  let mut include_opt = options.include.take();
  let mut exclude_opt = options.exclude.take();
  let mut swc_plugins_opt = options.swc_plugins.take();
  let include_patterns = extract_patterns(&env, &mut include_opt);
  let exclude_patterns = extract_patterns(&env, &mut exclude_opt);
  let swc_plugins = extract_swc_plugins(&env, &mut swc_plugins_opt);

  if !utils::should_transform_file(&filename, &include_patterns, &exclude_patterns) {
    return Ok(StyleXTransformResult {
      code,
      metadata: StyleXMetadata { stylex: vec![] },
      map: None,
    });
  }

  let result = panic::catch_unwind(|| {
    let cm: Arc<SourceMap> = Default::default();
    let filename = FileName::Real(filename.into());

    let fm = cm.new_source_file(filename.clone().into(), code);

    let cwd = env::current_dir()?;

    let plugin_pass = PluginPass {
      cwd: Some(cwd.clone()),
      filename: filename.clone(),
    };

    let source_map = match options.source_map.as_ref() {
      Some(SourceMaps::True) => SourceMapsConfig::Bool(true),
      Some(SourceMaps::False) => SourceMapsConfig::Bool(false),
      Some(SourceMaps::Inline) => SourceMapsConfig::Str("inline".to_string()),
      None => SourceMapsConfig::Bool(true),
    };

    let mut config: StyleXOptionsParams = options.try_into()?;

    let mut stylex: StyleXTransform<PluginCommentsProxy> =
      StyleXTransform::new(PluginCommentsProxy, plugin_pass, &mut config);

    let mut parser = Parser::new_from(Lexer::new(
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      EsVersion::latest(),
      StringInput::from(&*fm),
      None,
    ));

    let mut program = match parser.parse_program() {
      Ok(program) => program,
      Err(err) => {
        let error_message = format!("Failed to parse file `{}`: {:?}", filename, err);
        return Err(napi::Error::from_reason(error_message));
      }
    };

    // Apply SWC plugins if provided
    if let Some(plugins) = swc_plugins && !plugins.is_empty() {
      info!("Applying {} SWC plugin(s) before StyleX transformation", plugins.len());
      program = plugin_executor::apply_swc_plugins(
        &env,
        program,
        plugins,
        cm.clone(),
        &cwd,
      )?;
    }

    let program = program
      .apply(&mut fold_pass(&mut stylex))
      .apply(&mut visit_mut_pass(fixer(None)));

    let stylex_metadata = extract_stylex_metadata(env, &stylex)?;

    let transformed_code = print(
      cm,
      &program,
      PrintArgs {
        source_map,
        ..Default::default()
      },
    );

    let result = transformed_code.unwrap();

    let js_result = StyleXTransformResult {
      code: result.code,
      metadata: StyleXMetadata {
        stylex: stylex_metadata,
      },
      map: result.map,
    };

    Ok(js_result)
  });

  match result {
    Ok(res) => res,
    Err(error) => {
      let error_msg = match error.downcast_ref::<String>() {
        Some(s) => format!("Panic occurred during transformation: {}", s),
        None => match error.downcast_ref::<&str>() {
          Some(s) => format!("Panic occurred during transformation: {}", s),
          None => "Unknown panic occurred during transformation".to_string(),
        },
      };

      Err(napi::Error::from_reason(error_msg))
    }
  }
}

#[napi]
pub fn should_transform_file(
  env: Env,
  file_path: String,
  include: Option<Vec<napi::JsUnknown>>,
  exclude: Option<Vec<napi::JsUnknown>>,
) -> Result<bool> {
  let include_patterns = include.map(|patterns| {
    patterns
      .into_iter()
      .filter_map(|p| parse_js_pattern(&env, p).ok())
      .collect()
  });
  let exclude_patterns = exclude.map(|patterns| {
    patterns
      .into_iter()
      .filter_map(|p| parse_js_pattern(&env, p).ok())
      .collect()
  });

  Ok(utils::should_transform_file(
    &file_path,
    &include_patterns,
    &exclude_patterns,
  ))
}

/// Parse a JS value (string or RegExp)
fn parse_js_pattern(_env: &Env, value: napi::JsUnknown) -> Result<PathFilterUnion> {
  // Try to coerce to object first to check if it's a RegExp
  if let Ok(obj) = value.coerce_to_object() {
    // Check if it's a RegExp by trying to get 'source' and 'flags' properties
    if let (Ok(source), Ok(flags)) = (
      obj.get_named_property::<napi::JsString>("source"),
      obj.get_named_property::<napi::JsString>("flags"),
    ) {
      // It's a RegExp object - convert JS flags to inline modifiers
      let source_str = source.into_utf8()?.as_str()?.to_owned();
      let flags_str = flags.into_utf8()?.as_str()?.to_owned();

      // Convert JavaScript flags to regex inline modifiers
      // Note: 'g' (global) and 'y' (sticky) are not relevant for single-string matching
      let mut inline_flags = String::new();
      if flags_str.contains('i') {
        inline_flags.push('i'); // case insensitive
      }
      if flags_str.contains('m') {
        inline_flags.push('m'); // multiline
      }
      if flags_str.contains('s') {
        inline_flags.push('s'); // dotAll
      }

      // Prepend inline flags if any exist
      let pattern = if !inline_flags.is_empty() {
        format!("(?{}){}", inline_flags, source_str)
      } else {
        source_str
      };

      return Ok(PathFilterUnion::Regex(pattern));
    }

    // Not a RegExp, try to get it as a string
    if let Ok(str_val) = obj.coerce_to_string() {
      let pattern_str = str_val.into_utf8()?.as_str()?.to_owned();
      return Ok(PathFilterUnion::from_string(&pattern_str));
    }
  }

  Err(napi::Error::from_reason(
    "Invalid pattern: must be string or RegExp",
  ))
}

#[napi]
pub fn normalize_rs_options(options: StyleXOptions) -> Result<StyleXOptions> {
  let normalized_options = StyleXOptions {
    dev: options
      .dev
      .or_else(|| env::var("NODE_ENV").ok().map(|env| env == "development")),
    enable_font_size_px_to_rem: options.enable_font_size_px_to_rem.or(Some(false)),
    enable_minified_keys: options.enable_minified_keys.or(Some(true)),
    runtime_injection: options.runtime_injection.or(Some(false)),
    treeshake_compensation: options.treeshake_compensation.or(Some(false)),
    import_sources: options.import_sources.or(Some(vec![
      ImportSourceUnion::Regular("stylex".to_string()),
      ImportSourceUnion::Regular("@stylexjs/stylex".to_string()),
    ])),
    unstable_module_resolution: options.unstable_module_resolution.or_else(|| {
      Some(StyleXModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: None,
        theme_file_extension: None,
      })
    }),
    enable_inlined_conditional_merge: options.enable_inlined_conditional_merge.or(Some(true)),
    enable_logical_styles_polyfill: options.enable_logical_styles_polyfill.or(Some(false)),
    enable_legacy_value_flipping: options.enable_legacy_value_flipping.or(Some(false)),
    enable_ltr_rtl_comments: options.enable_ltr_rtl_comments.or(Some(false)),
    style_resolution: options
      .style_resolution
      .or(Some("property-specificity".to_string())),
    legacy_disable_layers: options.legacy_disable_layers.or(Some(false)),
    ..options
  };

  // Validate styleResolution if provided
  if let Some(ref style_resolution) = normalized_options.style_resolution {
    // Try to parse it to validate
    serde_plain::from_str::<StyleResolution>(style_resolution)
      .map_err(|e| napi::Error::from_reason(format!("Failed to parse style resolution: {}", e)))?;
  }

  Ok(normalized_options)
}
