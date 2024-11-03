use std::ops::Deref;

use napi::{Env, Error, JsObject};
use stylex_shared::StyleXTransform;
use swc_core::plugin::proxies::PluginCommentsProxy;

pub(crate) fn extract_stylex_metadata(
  env: Env,
  stylex: &StyleXTransform<PluginCommentsProxy>,
) -> Result<Vec<JsObject>, Error> {
  let mut stylex_metadata = Vec::with_capacity(stylex.state.metadata.len());

  for value in stylex.state.metadata.values() {
    for meta in value {
      let mut metadata_value = env.create_array_with_length(3)?;

      metadata_value.set_element(0, env.create_string(meta.get_class_name())?)?;

      let mut style_value = env.create_object()?;
      let styles = meta.get_style();

      style_value.set_named_property("ltr", styles.ltr.deref())?;

      if let Some(rtl) = styles.rtl.as_deref() {
        style_value.set_named_property("rtl", rtl)?;
      } else {
        style_value.set_named_property("rtl", env.get_null())?;
      }

      metadata_value.set_element(1, style_value)?;
      metadata_value.set_element(2, env.create_double(*meta.get_priority())?)?;

      stylex_metadata.push(metadata_value);
    }
  }

  Ok(stylex_metadata)
}
