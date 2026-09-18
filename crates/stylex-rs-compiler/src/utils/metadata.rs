#![allow(deprecated)]

use napi::{Env, Error, JsObject};
use stylex_state::flat_compiled_styles_value::FlatCompiledStylesValue;
use stylex_transform::StyleXTransform;
use stylex_types::enums::data_structures::injectable_style::InjectableStyleBaseKind;
use swc_core::common::comments::Comments;

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

/// Extracts StyleX metadata from the transformation state.
///
/// Generic over the comment store: only `state` is read, so which store the
/// transform carries is none of this function's business.
pub(crate) fn extract_stylex_metadata<C: Comments>(
  env: Env,
  stylex: &StyleXTransform<C>,
) -> Result<Vec<JsObject>, Error> {
  // Count the rules and not the groups. `len` gave the number of keys in the
  // map, and the loop below pushes one entry for each rule inside a key, so
  // every file with more than one rule made the vector grow again.
  let mut stylex_metadata = Vec::with_capacity(
    stylex
      .state
      .metadata()
      .values()
      .map(|rules| rules.len())
      .sum(),
  );

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
    // The constant travels as JSON, so that the kind the author gave it
    // survives the trip out of the compiler. A reader of this metadata sees a
    // number as a number and text as text, rather than text it would have to
    // guess a kind back out of.
    //
    // This carrier and the injected `stylex.inject(...)` call read the same
    // JSON, but they do not write the same answer, and that is deliberate. A
    // constant set to `null` keeps its `constKey`/`constVal` pair here and
    // loses it in the injected call, because the reference makes the same two
    // choices: its metadata pushes the rule object whole, and only the call it
    // builds drops a pair whose value is `null` or `undefined`. A guard added
    // here would put this carrier out of step with the reference, not into
    // step with the other one.
    let value = FlatCompiledStylesValue::from_json_text(consts_value);

    match value.as_number() {
      // A number JSON has no word for still crosses as the number it is, which
      // is what the reference hands a reader of its own metadata.
      Some(number) => style_value.set_named_property("constVal", env.create_double(number)?)?,
      None => {
        style_value.set_named_property("constVal", env.to_js_value(&value.as_json_value())?)?
      },
    }
  }

  style_value.set_named_property("ltr", ltr)?;

  let rtl_value = rtl.map(|v| env.create_string(v)).transpose()?;
  style_value.set_named_property("rtl", rtl_value)?;

  Ok(())
}

#[cfg(test)]
#[path = "../tests/metadata_tests.rs"]
mod tests;
