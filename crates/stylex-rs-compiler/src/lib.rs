mod structs;
mod utils;
use napi::{Env, Result};
use std::env;
use structs::{StyleXMetadata, StyleXOptions, StyleXTransformResult};
use swc_compiler_base::{print, PrintArgs, SourceMapsConfig};

use stylex_shared::{shared::structures::plugin_pass::PluginPass, StyleXTransform};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};

use swc_core::{
  common::{sync::Lrc, FileName, SourceMap},
  ecma::{ast::EsVersion, transforms::base::fixer::fixer, visit::FoldWith},
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

  let cm: Lrc<SourceMap> = Default::default();
  let filename = FileName::Real(filename.into());

  let fm = cm.new_source_file(filename.clone(), code);

  let cwd = env::current_dir()?;

  let plugin_pass = PluginPass {
    cwd: Some(cwd),
    filename,
  };

  let mut config = options.into();

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

  let program = program.fold_with(&mut stylex).fold_with(&mut fixer(None));

  let transformed_code = print(
    cm,
    &program,
    PrintArgs {
      source_map: SourceMapsConfig::Bool(true),
      ..Default::default()
    },
  );

  let result = transformed_code.unwrap();

  let stylex_metadata = extract_stylex_metadata(env, &stylex)?;

  let js_result = StyleXTransformResult {
    code: result.code,
    metadata: StyleXMetadata {
      stylex: stylex_metadata,
    },
    map: result.map,
  };

  Ok(js_result)
}
