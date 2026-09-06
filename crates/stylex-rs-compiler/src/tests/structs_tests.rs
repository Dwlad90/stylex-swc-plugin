use super::*;
use stylex_structures::{
  ceiling::Ceiling,
  evaluation_depth::MAX_EVALUATION_DEPTH,
  fold_ceilings::{MAX_FOLDED_CHARACTERS, MAX_FOLDED_ENTRIES},
};

fn empty_options() -> StyleXOptions {
  StyleXOptions {
    style_resolution: None,
    max_evaluation_depth: None,
    max_folded_characters: None,
    max_folded_entries: None,
    enable_font_size_px_to_rem: None,
    runtime_injection: None,
    class_name_prefix: None,
    defined_stylex_css_variables: None,
    import_sources: None,
    treeshake_compensation: None,
    enable_inlined_conditional_merge: None,
    enable_media_query_order: None,
    enable_logical_styles_polyfill: None,
    enable_legacy_value_flipping: None,
    enable_ltr_rtl_comments: None,
    legacy_disable_layers: None,
    dev: None,
    test: None,
    debug: None,
    enable_debug_class_names: None,
    enable_debug_data_prop: None,
    enable_dev_class_names: None,
    enable_minified_keys: None,
    inject_stylex_side_effects: None,
    use_real_file_for_source: None,
    aliases: None,
    unstable_module_resolution: None,
    source_map: None,
    inline_sources_content: None,
    emit_source_map_columns: None,
    input_source_map: None,
    property_validation_mode: None,
    env: None,
    debug_file_path: None,
    sx_prop_name: None,
  }
}

#[test]
fn try_from_maps_supported_fields() {
  let options = StyleXOptions {
    style_resolution: Some("application-order".to_string()),
    runtime_injection: Some(RuntimeInjectionUnion::Regular("inject-path".to_string())),
    import_sources: Some(vec![
      ImportSourceUnion::Regular("@stylexjs/stylex".to_string()),
      ImportSourceUnion::Named(NamedImportSource {
        r#as: "sx".to_string(),
        from: "stylex".to_string(),
      }),
    ]),
    unstable_module_resolution: Some(StyleXModuleResolution {
      r#type: "commonJS".to_string(),
      root_dir: Some("/root".to_string()),
      theme_file_extension: Some(".stylex".to_string()),
    }),
    property_validation_mode: Some(PropertyValidationMode::Warn),
    sx_prop_name: Some(SxPropNameUnion::Name("sx".to_string())),
    ..empty_options()
  };

  let parsed = StyleXOptionsParams::try_from(options).unwrap();

  assert!(parsed.style_resolution.is_some());
  assert_eq!(
    parsed.runtime_injection,
    Some(RuntimeInjection::Regular("inject-path".to_string()))
  );
  assert_eq!(parsed.import_sources.as_ref().map(Vec::len), Some(2));
  assert_eq!(
    parsed.unstable_module_resolution.as_ref().map(|m| m.kind),
    Some(ModuleResolutionKind::CommonJs)
  );
  assert_eq!(
    parsed.property_validation_mode,
    Some(StylexPropertyValidationMode::Warn)
  );
  assert!(matches!(
    parsed.sx_prop_name,
    Some(SxPropNameParam::Enabled(ref value)) if value == "sx"
  ));
}

#[test]
fn try_from_returns_error_for_invalid_style_resolution() {
  let options = StyleXOptions {
    style_resolution: Some("not-a-style-resolution".to_string()),
    ..empty_options()
  };

  match StyleXOptionsParams::try_from(options) {
    Ok(_) => panic!("expected style resolution parsing to fail"),
    Err(error) => assert!(
      error
        .to_string()
        .contains("Failed to parse style resolution")
    ),
  }
}

#[test]
fn try_from_uses_dev_for_debug_when_debug_not_set() {
  let options = StyleXOptions {
    dev: Some(true),
    debug: None,
    ..empty_options()
  };

  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(parsed.debug, Some(true));
}

#[test]
fn try_from_maps_runtime_injection_bool_and_sx_disabled() {
  let options = StyleXOptions {
    runtime_injection: Some(RuntimeInjectionUnion::Boolean(true)),
    sx_prop_name: Some(SxPropNameUnion::Disabled),
    ..empty_options()
  };

  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(
    parsed.runtime_injection,
    Some(RuntimeInjection::Boolean(true))
  );
  assert!(matches!(
    parsed.sx_prop_name,
    Some(SxPropNameParam::Disabled)
  ));
}

#[test]
fn try_from_empty_options_produces_all_none() {
  let options = empty_options();
  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert!(parsed.style_resolution.is_none());
  assert!(parsed.enable_font_size_px_to_rem.is_none());
  assert!(parsed.runtime_injection.is_none());
  assert!(parsed.class_name_prefix.is_none());
  assert!(parsed.defined_stylex_css_variables.is_none());
  assert!(parsed.import_sources.is_none());
  assert!(parsed.treeshake_compensation.is_none());
  assert!(parsed.enable_inlined_conditional_merge.is_none());
  assert!(parsed.unstable_module_resolution.is_none());
  assert!(parsed.property_validation_mode.is_none());
  assert!(parsed.sx_prop_name.is_none());
  assert!(parsed.env.is_none());
  assert!(parsed.debug_file_path.is_none());
}

#[test]
fn try_from_maps_all_boolean_options() {
  let options = StyleXOptions {
    enable_font_size_px_to_rem: Some(true),
    treeshake_compensation: Some(true),
    enable_inlined_conditional_merge: Some(false),
    enable_media_query_order: Some(false),
    enable_logical_styles_polyfill: Some(true),
    enable_legacy_value_flipping: Some(true),
    enable_ltr_rtl_comments: Some(true),
    dev: Some(true),
    test: Some(true),
    debug: Some(false),
    enable_debug_class_names: Some(true),
    enable_minified_keys: Some(false),
    inject_stylex_side_effects: Some(true),
    use_real_file_for_source: Some(false),
    ..empty_options()
  };

  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(parsed.enable_font_size_px_to_rem, Some(true));
  assert_eq!(parsed.treeshake_compensation, Some(true));
  assert_eq!(parsed.enable_inlined_conditional_merge, Some(false));
  assert_eq!(parsed.enable_media_query_order, Some(false));
  assert_eq!(parsed.enable_logical_styles_polyfill, Some(true));
  assert_eq!(parsed.enable_legacy_value_flipping, Some(true));
  assert_eq!(parsed.enable_ltr_rtl_comments, Some(true));
  assert_eq!(parsed.dev, Some(true));
  assert_eq!(parsed.test, Some(true));
  assert_eq!(parsed.debug, Some(false));
  assert_eq!(parsed.enable_debug_class_names, Some(true));
  assert_eq!(parsed.enable_minified_keys, Some(false));
  assert_eq!(parsed.inject_stylex_side_effects, Some(true));
  assert_eq!(parsed.use_real_file_for_source, Some(false));
}

#[test]
fn try_from_maps_class_name_prefix() {
  let options = StyleXOptions {
    class_name_prefix: Some("myprefix".to_string()),
    ..empty_options()
  };

  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(parsed.class_name_prefix, Some("myprefix".to_string()));
}

#[test]
fn try_from_maps_aliases() {
  let mut aliases = FxHashMap::default();
  aliases.insert(
    "@components".to_string(),
    vec!["src/components".to_string()],
  );
  let options = StyleXOptions {
    aliases: Some(aliases.clone()),
    ..empty_options()
  };

  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(parsed.aliases, Some(aliases));
}

#[test]
fn try_from_maps_defined_stylex_css_variables() {
  let mut vars = FxHashMap::default();
  vars.insert("--primary".to_string(), "blue".to_string());
  let options = StyleXOptions {
    defined_stylex_css_variables: Some(vars.clone()),
    ..empty_options()
  };

  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(parsed.defined_stylex_css_variables, Some(vars));
}

#[test]
fn try_from_maps_property_validation_throw() {
  let options = StyleXOptions {
    property_validation_mode: Some(PropertyValidationMode::Throw),
    ..empty_options()
  };
  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(
    parsed.property_validation_mode,
    Some(StylexPropertyValidationMode::Throw)
  );
}

#[test]
fn try_from_maps_property_validation_silent() {
  let options = StyleXOptions {
    property_validation_mode: Some(PropertyValidationMode::Silent),
    ..empty_options()
  };
  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(
    parsed.property_validation_mode,
    Some(StylexPropertyValidationMode::Silent)
  );
}

#[test]
fn try_from_maps_runtime_injection_string() {
  let options = StyleXOptions {
    runtime_injection: Some(RuntimeInjectionUnion::Regular("inject-path".to_string())),
    ..empty_options()
  };
  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(
    parsed.runtime_injection,
    Some(RuntimeInjection::Regular("inject-path".to_string()))
  );
}

#[test]
fn try_from_maps_runtime_injection_false() {
  let options = StyleXOptions {
    runtime_injection: Some(RuntimeInjectionUnion::Boolean(false)),
    ..empty_options()
  };
  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(
    parsed.runtime_injection,
    Some(RuntimeInjection::Boolean(false))
  );
}

#[test]
fn try_from_maps_sx_prop_name_string() {
  let options = StyleXOptions {
    sx_prop_name: Some(SxPropNameUnion::Name("css".to_string())),
    ..empty_options()
  };
  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert!(matches!(
    parsed.sx_prop_name,
    Some(SxPropNameParam::Enabled(ref value)) if value == "css"
  ));
}

#[test]
fn try_from_maps_module_resolution_minimal() {
  let options = StyleXOptions {
    unstable_module_resolution: Some(StyleXModuleResolution {
      r#type: "haste".to_string(),
      root_dir: None,
      theme_file_extension: None,
    }),
    ..empty_options()
  };
  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(
    parsed.unstable_module_resolution.as_ref().map(|m| m.kind),
    Some(ModuleResolutionKind::Haste)
  );
  assert!(
    parsed
      .unstable_module_resolution
      .as_ref()
      .unwrap()
      .root_dir
      .is_none()
  );
}

#[test]
fn try_from_maps_import_sources_regular_only() {
  let options = StyleXOptions {
    import_sources: Some(vec![
      ImportSourceUnion::Regular("stylex".to_string()),
      ImportSourceUnion::Regular("@stylexjs/stylex".to_string()),
    ]),
    ..empty_options()
  };
  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  let sources = parsed.import_sources.unwrap();
  assert_eq!(sources.len(), 2);
  assert!(matches!(&sources[0], ImportSources::Regular(s) if s == "stylex"));
  assert!(matches!(&sources[1], ImportSources::Regular(s) if s == "@stylexjs/stylex"));
}

#[test]
fn try_from_maps_import_sources_empty() {
  let options = StyleXOptions {
    import_sources: Some(vec![]),
    ..empty_options()
  };
  let parsed = StyleXOptionsParams::try_from(options).unwrap();
  assert_eq!(parsed.import_sources.unwrap().len(), 0);
}

#[test]
fn try_from_all_style_resolutions() {
  for (input, _) in &[
    ("application-order", "application-order"),
    ("property-specificity", "property-specificity"),
    ("legacy-expand-shorthands", "legacy-expand-shorthands"),
  ] {
    let options = StyleXOptions {
      style_resolution: Some(input.to_string()),
      ..empty_options()
    };
    assert!(
      StyleXOptionsParams::try_from(options).is_ok(),
      "style_resolution '{}' should be valid",
      input
    );
  }
}

// ── the three ceilings ─────────────────────────────────────────────

/// One ceiling, named the two ways a test needs it: the declaration the boundary
/// resolves through, and the field on either side of that boundary.
///
/// The option a project writes and the cap it may not pass are read off the
/// declaration rather than restated here, so a test cannot assert against a name
/// the compiler does not use -- which is the whole of what these cases check.
///
/// A table rather than three copies of every case, because the rule is one rule
/// -- and a fourth ceiling added without a refusal of its own should fail here
/// rather than quietly accept a `NaN`.
struct CeilingUnderTest {
  declared: &'static Ceiling,
  write: fn(&mut StyleXOptions, Option<ConfiguredCeiling>),
  read: fn(&StyleXOptionsParams) -> Option<usize>,
}

/// A function rather than a `const`, because a `Ceiling` caches its environment
/// read behind a `OnceLock` and so cannot be looked at while a constant is being
/// evaluated.
fn ceilings() -> [CeilingUnderTest; 3] {
  [
    CeilingUnderTest {
      declared: &MAX_EVALUATION_DEPTH,
      write: |options, value| options.max_evaluation_depth = value,
      read: |parsed| parsed.max_evaluation_depth,
    },
    CeilingUnderTest {
      declared: &MAX_FOLDED_CHARACTERS,
      write: |options, value| options.max_folded_characters = value,
      read: |parsed| parsed.max_folded_characters,
    },
    CeilingUnderTest {
      declared: &MAX_FOLDED_ENTRIES,
      write: |options, value| options.max_folded_entries = value,
      read: |parsed| parsed.max_folded_entries,
    },
  ]
}

/// The options a project would send with one ceiling configured.
fn options_with(ceiling: &CeilingUnderTest, value: Option<ConfiguredCeiling>) -> StyleXOptions {
  let mut options = empty_options();
  (ceiling.write)(&mut options, value);
  options
}

#[test]
fn a_ceiling_that_is_a_count_is_taken_as_written() {
  for ceiling in &ceilings() {
    for value in [1, 7, ceiling.declared.limit] {
      let options = options_with(ceiling, Some(ConfiguredCeiling::Number(value as f64)));

      match StyleXOptionsParams::try_from(options) {
        Ok(parsed) => assert_eq!(
          (ceiling.read)(&parsed),
          Some(value),
          "{}: {value} is a usable count",
          ceiling.declared.option
        ),
        Err(error) => panic!(
          "{}: {value} was refused -- {error}",
          ceiling.declared.option
        ),
      }
    }
  }
}

#[test]
fn an_absent_ceiling_leaves_the_environment_and_the_default_to_answer() {
  for ceiling in &ceilings() {
    match StyleXOptionsParams::try_from(options_with(ceiling, None)) {
      Ok(parsed) => assert_eq!((ceiling.read)(&parsed), None, "{}", ceiling.declared.option),
      Err(error) => panic!(
        "{}: an absent ceiling was refused -- {error}",
        ceiling.declared.option
      ),
    }
  }
}

#[test]
fn a_ceiling_that_is_not_a_count_is_refused_by_name() {
  for ceiling in &ceilings() {
    let past_the_cap = (ceiling.declared.limit + 1) as f64;

    let refused = [
      ("a fraction", ConfiguredCeiling::Number(1.5), "1.5"),
      ("NaN", ConfiguredCeiling::Number(f64::NAN), "NaN"),
      (
        "an infinity",
        ConfiguredCeiling::Number(f64::INFINITY),
        "Infinity",
      ),
      (
        "a negative infinity",
        ConfiguredCeiling::Number(f64::NEG_INFINITY),
        "-Infinity",
      ),
      ("a negative", ConfiguredCeiling::Number(-1.0), "-1"),
      ("zero", ConfiguredCeiling::Number(0.0), "0"),
      (
        "a value past the cap",
        ConfiguredCeiling::Number(past_the_cap),
        &past_the_cap.to_string(),
      ),
    ];

    for (label, value, written) in refused {
      match StyleXOptionsParams::try_from(options_with(ceiling, Some(value))) {
        Ok(_) => panic!("{}: {label} was accepted", ceiling.declared.option),
        Err(error) => {
          let message = error.reason;

          // The three things an author needs from the sentence: which option,
          // what the cap is, and what they actually wrote.
          assert!(
            message.contains(ceiling.declared.option),
            "{}: {label} does not name the option -- {message}",
            ceiling.declared.option
          );
          assert!(
            message.contains(&ceiling.declared.limit.to_string()),
            "{}: {label} does not name the cap -- {message}",
            ceiling.declared.option
          );
          assert!(
            message.contains(written),
            "{}: {label} does not name what was configured -- {message}",
            ceiling.declared.option
          );
        },
      }
    }
  }
}

/// A value with no number in it at all has nothing to hand back, so the sentence
/// names the option and the cap and stops there.
#[test]
fn a_ceiling_that_is_not_a_number_is_refused_by_name() {
  for ceiling in &ceilings() {
    match StyleXOptionsParams::try_from(options_with(ceiling, Some(ConfiguredCeiling::NotANumber)))
    {
      Ok(_) => panic!("{}: a non-number was accepted", ceiling.declared.option),
      Err(error) => assert_eq!(
        error.reason,
        format!(
          "{} must be a whole number between 1 and {}.",
          ceiling.declared.option, ceiling.declared.limit
        )
      ),
    }
  }
}
