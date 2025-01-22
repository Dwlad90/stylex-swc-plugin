mod structs;
mod utils;
use napi::{Env, Result};
use std::panic;
use std::{env, sync::Arc};
use structs::{
  SourceMaps, StyleXMetadata, StyleXModuleResolution, StyleXOptions, StyleXTransformResult,
};
use swc_compiler_base::{print, PrintArgs, SourceMapsConfig};

use stylex_shared::{
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
  shared::utils::log::logger,
  StyleXTransform,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};

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

#[napi]
pub fn transform(
  env: Env,
  filename: String,
  code: String,
  options: StyleXOptions,
) -> Result<StyleXTransformResult> {
  color_backtrace::install();
  logger::initialize();

  let result = panic::catch_unwind(|| {
    let cm: Arc<SourceMap> = Default::default();
    let filename = FileName::Real(filename.into());

    let fm = cm.new_source_file(filename.clone().into(), code);

    let cwd = env::current_dir()?;

    let plugin_pass = PluginPass {
      cwd: Some(cwd),
      filename,
    };

    let source_map = match options.source_map.as_ref() {
      Some(SourceMaps::True) => SourceMapsConfig::Bool(true),
      Some(SourceMaps::False) => SourceMapsConfig::Bool(false),
      Some(SourceMaps::Inline) => SourceMapsConfig::Str("inline".to_string()),
      None => SourceMapsConfig::Bool(true),
    };

    let mut config: StyleXOptionsParams = options.into();

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

    let program = parser.parse_program().unwrap();

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
pub fn normalize_rs_options(options: StyleXOptions) -> Result<StyleXOptions> {
  Ok(StyleXOptions {
    dev: options
      .dev
      .or_else(|| env::var("NODE_ENV").ok().map(|env| env == "development")),
    use_rem_for_font_size: options.use_rem_for_font_size.or(Some(true)),
    runtime_injection: options.runtime_injection.or(Some(false)),
    treeshake_compensation: options.treeshake_compensation.or(Some(true)),
    import_sources: options.import_sources.or(Some(vec![
      "stylex".to_string(),
      "@stylexjs/stylex".to_string(),
    ])),
    unstable_module_resolution: options.unstable_module_resolution.or_else(|| {
      Some(StyleXModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: None,
        theme_file_extension: None,
      })
    }),
    ..options
  })
}
