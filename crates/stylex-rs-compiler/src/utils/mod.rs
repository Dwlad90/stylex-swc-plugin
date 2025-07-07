use napi::{Env, Error, JsObject};
use stylex_shared::{
  StyleXTransform, shared::enums::data_structures::injectable_style::InjectableStyleBaseKind,
};
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

      match styles {
        InjectableStyleBaseKind::Regular(styles) => {
          set_metadata_ltr_and_rtl(env, &mut style_value, &styles.ltr, &styles.rtl, None, None)?;
        }
        InjectableStyleBaseKind::Const(styles) => {
          set_metadata_ltr_and_rtl(
            env,
            &mut style_value,
            &styles.ltr,
            &styles.rtl,
            Some(&styles.const_key),
            Some(&styles.const_value),
          )?;
        }
      }

      metadata_value.set_element(1, style_value)?;
      metadata_value.set_element(2, env.create_double(*meta.get_priority())?)?;

      stylex_metadata.push(metadata_value);
    }
  }

  Ok(stylex_metadata)
}

fn set_metadata_ltr_and_rtl(
  env: Env,
  style_value: &mut JsObject,
  ltr: &str,
  rtl: &Option<String>,
  consts_key: Option<&str>,
  consts_value: Option<&str>,
) -> Result<(), Error> {
  if let Some(consts_key) = consts_key {
    style_value.set_named_property("constKey", consts_key)?;
  }

  if let Some(consts_value) = consts_value {
    style_value.set_named_property("constVal", consts_value)?;
  }

  style_value.set_named_property("ltr", ltr)?;

  style_value.set_named_property(
    "rtl",
    rtl
      .as_ref()
      .map_or(env.get_null()?.into_unknown(), |rtl_str| {
        env.create_string(rtl_str).unwrap().into_unknown()
      }),
  )?;

  Ok(())
}
