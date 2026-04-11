//! Tests for StyleXOptions default values, From<StyleXOptionsParams> conversion,
//! and module resolution factories.

use crate::named_import_source::{NamedImportSource, RuntimeInjection};
use crate::stylex_options::*;

#[test]
fn stylex_options_params_default_key_fields() {
  let params = StyleXOptionsParams::default();
  assert_eq!(params.class_name_prefix, Some("x".to_string()));
  assert_eq!(params.dev, Some(false));
  assert_eq!(params.test, Some(false));
  assert!(params.debug.is_none());
  assert_eq!(params.treeshake_compensation, Some(true));
  assert_eq!(params.enable_inlined_conditional_merge, Some(true));
  assert!(params.aliases.is_none());
  assert!(params.unstable_module_resolution.is_none());
}

#[test]
fn stylex_options_default_key_fields() {
  let opts = StyleXOptions::default();
  assert_eq!(opts.class_name_prefix, "x");
  assert!(!opts.dev);
  assert!(!opts.test);
  assert!(!opts.debug);
  assert!(opts.enable_minified_keys);
  assert!(opts.enable_debug_data_prop);
  assert!(!opts.treeshake_compensation);
  assert!(matches!(opts.runtime_injection, RuntimeInjection::Boolean(false)));
  assert!(matches!(
    opts.unstable_module_resolution,
    CheckModuleResolution::CommonJS(_)
  ));
  assert_eq!(opts.sx_prop_name, Some("sx".to_string()));
}

#[test]
fn from_stylex_options_params_defaults() {
  let params = StyleXOptionsParams::default();
  let opts: StyleXOptions = params.into();
  assert_eq!(opts.class_name_prefix, "x");
  assert!(!opts.dev);
  assert!(!opts.debug);
  assert!(matches!(opts.runtime_injection, RuntimeInjection::Boolean(false)));
  assert_eq!(opts.import_sources.len(), 2);
}

#[test]
fn from_stylex_options_params_runtime_injection_true() {
  let mut params = StyleXOptionsParams::default();
  params.runtime_injection = Some(RuntimeInjection::Boolean(true));
  let opts: StyleXOptions = params.into();
  assert!(matches!(opts.runtime_injection, RuntimeInjection::Regular(_)));
}

#[test]
fn from_stylex_options_params_runtime_injection_named() {
  let mut params = StyleXOptionsParams::default();
  params.runtime_injection = Some(RuntimeInjection::Named(
    NamedImportSource {
      r#as: "inject".into(),
      from: "my-pkg".into(),
    },
  ));
  let opts: StyleXOptions = params.into();
  assert!(matches!(opts.runtime_injection, RuntimeInjection::Named(_)));
}

#[test]
fn from_stylex_options_params_haste_resolution() {
  let mut params = StyleXOptionsParams::default();
  params.unstable_module_resolution = Some(ModuleResolution {
    r#type: "haste".to_string(),
    root_dir: Some("/root".to_string()),
    theme_file_extension: None,
  });
  let opts: StyleXOptions = params.into();
  assert!(matches!(
    opts.unstable_module_resolution,
    CheckModuleResolution::Haste(_)
  ));
}

#[test]
fn from_stylex_options_params_cross_file_parsing_resolution() {
  let mut params = StyleXOptionsParams::default();
  params.unstable_module_resolution = Some(ModuleResolution {
    r#type: "cross-file-parsing".to_string(),
    root_dir: None,
    theme_file_extension: None,
  });
  let opts: StyleXOptions = params.into();
  assert!(matches!(
    opts.unstable_module_resolution,
    CheckModuleResolution::CrossFileParsing(_)
  ));
}

#[test]
fn get_haste_module_resolution() {
  let res = ModuleResolution::haste(Some("/root".to_string()));
  assert_eq!(res.r#type, "haste");
  assert_eq!(res.root_dir, Some("/root".to_string()));
  assert!(res.theme_file_extension.is_none());
}

#[test]
fn get_common_js_module_resolution() {
  let res = ModuleResolution::common_js(None);
  assert_eq!(res.r#type, "commonjs");
  assert!(res.root_dir.is_none());
}
