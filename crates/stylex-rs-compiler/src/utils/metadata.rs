#![allow(deprecated)]

use napi::{Env, Error, JsObject};
use stylex_transform::StyleXTransform;
use stylex_types::enums::data_structures::injectable_style::InjectableStyleBaseKind;
use swc_core::plugin::proxies::PluginCommentsProxy;

#[derive(Debug, PartialEq, Eq)]
struct MetadataStyleParts<'a> {
  ltr: &'a str,
  rtl: Option<&'a str>,
  const_key: Option<&'a str>,
  const_value: Option<&'a str>,
}

fn metadata_style_parts(style: &InjectableStyleBaseKind) -> MetadataStyleParts<'_> {
  match style {
    InjectableStyleBaseKind::Regular(styles) => MetadataStyleParts {
      ltr: &styles.ltr,
      rtl: styles.rtl.as_deref(),
      const_key: None,
      const_value: None,
    },
    InjectableStyleBaseKind::Const(styles) => MetadataStyleParts {
      ltr: &styles.ltr,
      rtl: styles.rtl.as_deref(),
      const_key: Some(&styles.const_key),
      const_value: Some(&styles.const_value),
    },
  }
}

/// Extracts StyleX metadata from the transformation state
pub(crate) fn extract_stylex_metadata(
  env: Env,
  stylex: &StyleXTransform<PluginCommentsProxy>,
) -> Result<Vec<JsObject>, Error> {
  let mut stylex_metadata = Vec::with_capacity(stylex.state.metadata().len());

  for value in stylex.state.metadata().values() {
    for meta in value {
      let mut metadata_value = env.create_array_with_length(3)?;

      metadata_value.set_element(0, env.create_string(meta.get_class_name())?)?;

      let mut style_value = env.create_object()?;
      let parts = metadata_style_parts(meta.get_style());

      set_metadata_ltr_and_rtl(
        env,
        &mut style_value,
        parts.ltr,
        parts.rtl,
        parts.const_key,
        parts.const_value,
      )?;

      metadata_value.set_element(1, style_value)?;
      metadata_value.set_element(2, env.create_double(*meta.get_priority())?)?;

      stylex_metadata.push(metadata_value);
    }
  }

  Ok(stylex_metadata)
}

/// Sets LTR and RTL metadata properties on a JS object
fn set_metadata_ltr_and_rtl(
  env: Env,
  style_value: &mut JsObject,
  ltr: &str,
  rtl: Option<&str>,
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

  let rtl_value = rtl.map(|v| env.create_string(v)).transpose()?;
  style_value.set_named_property("rtl", rtl_value)?;

  Ok(())
}

#[cfg(test)]
#[path = "../tests/metadata_tests.rs"]
mod tests;
