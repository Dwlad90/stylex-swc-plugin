#![allow(deprecated)]

mod enums;
mod structs;
mod utils;
use log::info;
use napi::{Env, Result};
use std::{env, panic, sync::Arc};
use structs::{StyleXMetadata, StyleXOptions, StyleXTransformResult};
use stylex_logs::initializer::initialize as initialize_logger;
use stylex_macros::stylex_error::{SuppressPanicStderr, format_panic_message};
use swc_compiler_base::{PrintArgs, SourceMapsConfig, print};

use stylex_structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams};
use stylex_transform::StyleXTransform;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

use swc_core::{
  common::{FileName, GLOBALS, Globals, Mark, SourceMap},
  ecma::{
    ast::EsVersion,
    transforms::{
      base::{fixer::fixer, hygiene::hygiene, resolver},
      typescript::strip as typescript_strip,
    },
    visit::fold_pass,
  },
  plugin::proxies::PluginCommentsProxy,
};

use napi_derive::napi;
use utils::extract_stylex_metadata;

use crate::enums::SourceMaps;

fn source_maps_config(source_map: Option<&SourceMaps>) -> SourceMapsConfig {
  match source_map {
    Some(SourceMaps::True) => SourceMapsConfig::Bool(true),
    Some(SourceMaps::False) => SourceMapsConfig::Bool(false),
    Some(SourceMaps::Inline) => SourceMapsConfig::Str("inline".to_string()),
    None => SourceMapsConfig::Bool(true),
  }
}

#[napi]
pub fn transform(
  env: Env,
  filename: String,
  code: String,
  mut options: StyleXOptions,
) -> Result<StyleXTransformResult> {
  initialize_logger();

  info!("Transforming source file: {}", filename);

  // Parse the env object separately since it needs the napi::Env for JS function
  // references.
  let parsed_env = options
    .env
    .take()
    .map(|ref env_obj| utils::fn_parser::parse_env_object(&env, env_obj))
    .transpose()?;

  // Parse debugFilePath separately since it needs the napi::Env for JS function
  // references.
  let parsed_debug_file_path = options
    .debug_file_path
    .take()
    .map(|unknown_ref| {
      let value = unknown_ref.get_value(&env)?;
      utils::fn_parser::parse_debug_file_path(&env, value)
    })
    .transpose()?;

  let _suppress = SuppressPanicStderr::new();
  let result = panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
    let cm: Arc<SourceMap> = Default::default();
    let filename = FileName::Real(filename.into());

    let fm = cm.new_source_file(filename.clone().into(), code);

    let cwd = env::current_dir()?;

    let plugin_pass = PluginPass {
      cwd: Some(cwd),
      filename: filename.clone(),
    };

    let source_map = source_maps_config(options.source_map.as_ref());

    let mut config: StyleXOptionsParams = options.try_into()?;

    // Set the parsed env and debugFilePath on the config
    config.env = parsed_env;
    config.debug_file_path = parsed_debug_file_path;

    let mut parser = Parser::new_from(Lexer::new(
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      EsVersion::latest(),
      StringInput::from(&*fm),
      None,
    ));

    let program = match parser.parse_program() {
      Ok(program) => program,
      Err(err) => {
        let error_message = format!("Failed to parse file `{}`: {:?}", filename, err);
        return Err(napi::Error::from_reason(error_message));
      },
    };

    let globals = Globals::default();
    GLOBALS.set(&globals, || {
      // Set the NAPI env in thread-local storage so env functions can call back to JS
      utils::fn_parser::with_napi_env(&env, || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        let mut stylex: StyleXTransform<PluginCommentsProxy> =
          StyleXTransform::new(PluginCommentsProxy, plugin_pass, &mut config);

        let program = program
          .apply(resolver(unresolved_mark, top_level_mark, true))
          .apply(typescript_strip(unresolved_mark, top_level_mark))
          .apply(&mut fold_pass(&mut stylex))
          .apply(hygiene())
          .apply(&mut fixer(None));

        let stylex_metadata = extract_stylex_metadata(env, &stylex)?;

        let transformed_code = print(
          cm,
          &program,
          PrintArgs {
            source_map,
            ..Default::default()
          },
        );

        let result = match transformed_code {
          Ok(output) => output,
          Err(e) => {
            return Err(napi::Error::from_reason(format!(
              "[StyleX] Failed to print transformed code: {}",
              e
            )));
          },
        };

        let js_result = StyleXTransformResult {
          code: result.code,
          metadata: StyleXMetadata {
            stylex: stylex_metadata,
          },
          map: result.map,
        };

        Ok(js_result)
      })
    })
  }));

  match result {
    Ok(res) => res,
    Err(error) => {
      let error_msg = format_panic_message(&error);

      Err(napi::Error::from_reason(error_msg))
    },
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn source_maps_config_defaults_to_true() {
    assert!(matches!(
      source_maps_config(None),
      SourceMapsConfig::Bool(true)
    ));
  }

  #[test]
  fn source_maps_config_maps_explicit_values() {
    assert!(matches!(
      source_maps_config(Some(&SourceMaps::True)),
      SourceMapsConfig::Bool(true)
    ));
    assert!(matches!(
      source_maps_config(Some(&SourceMaps::False)),
      SourceMapsConfig::Bool(false)
    ));

    match source_maps_config(Some(&SourceMaps::Inline)) {
      SourceMapsConfig::Str(value) => assert_eq!(value, "inline"),
      _ => panic!("expected inline source map string"),
    }
  }
}
