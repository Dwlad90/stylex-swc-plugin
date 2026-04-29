use super::*;

pub(super) fn build_runtime_function_map<C>(transform: &mut StyleXTransform<C>) -> Box<FunctionMap>
where
  C: Comments,
{
  let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
  let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

  let first_that_works_fn = FunctionConfig {
    fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
    takes_path: false,
  };

  let keyframes_fn = get_keyframes_fn();
  let position_try_fn = get_position_try_fn();

  if let Some(set) = transform
    .state
    .get_stylex_api_import(ImportKind::FirstThatWorks)
  {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
      );
    }
  }

  if let Some(set) = transform.state.get_stylex_api_import(ImportKind::Keyframes) {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
      );
    }
  }

  if let Some(set) = transform
    .state
    .get_stylex_api_import(ImportKind::PositionTry)
  {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
      );
    }
  }

  if let Some(set) = transform
    .state
    .get_stylex_api_import(ImportKind::DefaultMarker)
  {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::IndexMap(
          stylex_default_marker::stylex_default_marker(&transform.state.options)
            .as_values()
            .unwrap_or_else(|| stylex_panic!("{}", EXPECTED_COMPILED_STYLES))
            .clone(),
        )),
      );
    }
  }

  if let Some(set) = transform.state.get_stylex_api_import(ImportKind::When) {
    for name in set {
      identifiers.insert(
        name.clone(),
        Box::new(FunctionConfigType::Regular(FunctionConfig {
          fn_ptr: FunctionType::DefaultMarker(Arc::clone(LazyLock::force(&STYLEX_WHEN_MAP))),
          takes_path: false,
        })),
      );
    }
  }

  for name in transform.state.stylex_imports() {
    member_expressions.entry(name.clone()).or_default();

    let member_expression = match member_expressions.get_mut(name) {
      Some(me) => me,
      #[cfg_attr(coverage_nightly, coverage(off))]
      None => stylex_panic!("Could not resolve the member expression for the StyleX import."),
    };

    member_expression.insert(
      STYLEX_FIRST_THAT_WORKS.into(),
      Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
    );

    member_expression.insert(
      STYLEX_KEYFRAMES.into(),
      Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
    );

    member_expression.insert(
      STYLEX_POSITION_TRY.into(),
      Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
    );

    member_expression.insert(
      STYLEX_DEFAULT_MARKER.into(),
      Box::new(FunctionConfigType::IndexMap(
        stylex_default_marker::stylex_default_marker(&transform.state.options)
          .as_values()
          .unwrap_or_else(|| {
            #[cfg_attr(coverage_nightly, coverage(off))]
            {
              stylex_panic!("{}", EXPECTED_COMPILED_STYLES)
            }
          })
          .clone(),
      )),
    );

    identifiers
      .entry(name.get_import_str().into())
      .and_modify(|func_type| {
        if let Some(map) = func_type.as_map_mut() {
          map.insert(
            STYLEX_WHEN.into(),
            FunctionConfig {
              fn_ptr: FunctionType::DefaultMarker(Arc::clone(LazyLock::force(&STYLEX_WHEN_MAP))),
              takes_path: false,
            },
          );
        }
      })
      .or_insert_with(|| {
        let mut map = FxHashMap::default();
        map.insert(
          STYLEX_WHEN.into(),
          FunctionConfig {
            fn_ptr: FunctionType::DefaultMarker(Arc::clone(LazyLock::force(&STYLEX_WHEN_MAP))),
            takes_path: false,
          },
        );
        Box::new(FunctionConfigType::Map(map))
      });
  }

  transform
    .state
    .apply_stylex_env(&mut identifiers, &mut member_expressions);

  Box::new(FunctionMap {
    identifiers,
    member_expressions,
    disable_imports: false,
  })
}
