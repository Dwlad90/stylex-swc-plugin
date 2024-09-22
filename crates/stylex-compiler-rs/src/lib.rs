use std::sync::Arc;
use swc_compiler_base::{parse_js, print, IsModule, PrintArgs, SourceMapsConfig};

use stylex_swc_plugin::{
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
  ModuleTransformVisitor,
};
use swc_ecma_parser::Syntax;

use swc_core::{
  common::{
    errors::{ColorConfig, Handler},
    FileName, SourceMap,
  },
  ecma::{ast::EsVersion, visit::FoldWith},
  plugin::proxies::PluginCommentsProxy,
};

use napi::{bindgen_prelude::*, JsObject};
use napi_derive::napi;

#[napi]
pub fn transform(env: Env, code: String) -> Result<JsObject> {
  let cm: Arc<SourceMap> = Default::default();
  let fm = cm.new_source_file(FileName::Anon, code);

  let plugin_pass = Box::new(PluginPass {
    cwd: None,
    filename: FileName::Anon,
  });

  let mut config = StyleXOptionsParams::default();

  let mut stylex: ModuleTransformVisitor<PluginCommentsProxy> =
    ModuleTransformVisitor::new(PluginCommentsProxy, plugin_pass, &mut config, false);

  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

  let program = parse_js(
    cm.clone(),
    fm,
    &handler,
    EsVersion::EsNext,
    Syntax::Typescript(Default::default()),
    IsModule::Bool(true),
    None,
  )
  .unwrap();

  let program = program.fold_with(&mut stylex);

  let transformed_code = print(
    cm,
    &program,
    PrintArgs {
      source_map: SourceMapsConfig::Bool(true),
      ..Default::default()
    },
  );

  let result = transformed_code.unwrap();

  let mut stylex_metadata = env.create_object()?;

  let mut js_metadata = env.create_object()?;

  for (key, value) in stylex.state.metadata {
    let mut styles_value = env.create_array_with_length(value.len())?;

    for (i, meta) in value.into_iter().enumerate() {
      let mut metadata_value = env.create_object()?;

      metadata_value.set_named_property("className", meta.get_class_name())?;

      let mut style_value = env.create_object()?;

      let styles = meta.get_style();

      style_value.set_named_property("ltr", styles.ltr.clone())?;
      style_value.set_named_property("rtl", styles.rtl.clone())?;

      metadata_value.set_named_property("style", style_value)?;

      metadata_value.set_named_property("priority", *meta.get_priority())?;

      styles_value.set_element(i as u32, metadata_value)?;
    }
    js_metadata.set_named_property(&key, styles_value)?;
  }

  let mut js_result = env.create_object()?;

  let _ = stylex_metadata.set_named_property("stylex", js_metadata);

  js_result.set_named_property("code", env.create_string(&result.code)?)?;
  js_result.set_named_property("metadata", stylex_metadata)?;

  if let Some(src_map) = result.map {
    js_result.set_named_property("sourcemap", env.create_string(&src_map)?)?;
  }

  Ok(js_result)
}
