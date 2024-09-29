use napi::{Env, Error, JsObject};
use stylex_shared::StyleXTransform;
use swc_core::plugin::proxies::PluginCommentsProxy;

pub(crate) fn extract_stylex_metadata(
  env: Env,
  stylex: &StyleXTransform<PluginCommentsProxy>,
) -> Result<Vec<JsObject>, Error> {
  let mut stylex_metadata: Vec<JsObject> = vec![];

  for (_, value) in &stylex.state.metadata {
    for meta in value.iter() {
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

  Ok(stylex_metadata)
}
