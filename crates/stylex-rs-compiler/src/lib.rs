mod structs;
use std::env;
use structs::{StyleXOptions, StyleXTransformResult, StyleXMetadata};
use swc_compiler_base::{print, PrintArgs, SourceMapsConfig};

use stylex_shared::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};

use swc_core::{
  common::{sync::Lrc, FileName, SourceMap},
  ecma::{ast::EsVersion, transforms::base::fixer::fixer, visit::FoldWith},
  plugin::proxies::PluginCommentsProxy,
};

use napi::{bindgen_prelude::*, JsObject};
use napi_derive::napi;

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

  let plugin_pass = Box::new(PluginPass {
    cwd: Some(cwd),
    filename,
  });

  let mut config = options.into();

  let mut stylex: ModuleTransformVisitor<PluginCommentsProxy> =
    ModuleTransformVisitor::new(PluginCommentsProxy, plugin_pass, &mut config);

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

  let mut stylex_metadata: Vec<JsObject> = vec![];

  for (_, value) in stylex.state.metadata {
    for meta in value.into_iter() {
      let mut metadata_value = env.create_array_with_length(3)?;

      metadata_value.set_element(0, env.create_string(meta.get_class_name())?)?;

      let mut style_value = env.create_object()?;

      let styles = meta.get_style();

      style_value.set_named_property("ltr", styles.ltr.clone())?;
      style_value.set_named_property("rtl", styles.rtl.clone())?;

      metadata_value.set_element(1, style_value)?;

      metadata_value.set_element(2, env.create_double(*meta.get_priority())?)?;

      stylex_metadata.push(metadata_value);
    }
  }

  let js_result = StyleXTransformResult {
    code: result.code,
    metadata: StyleXMetadata {
      stylex: stylex_metadata,
    },
    map: result.map,
  };

  Ok(js_result)
}
