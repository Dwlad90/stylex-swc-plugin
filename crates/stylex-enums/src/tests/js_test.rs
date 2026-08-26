use super::CallableGlobalJS;

#[test]
fn callable_global_js_try_from_maps_foldable_callees() {
  assert_eq!(
    CallableGlobalJS::try_from("String"),
    Ok(CallableGlobalJS::String)
  );
  assert_eq!(
    CallableGlobalJS::try_from("Number"),
    Ok(CallableGlobalJS::Number)
  );
  assert_eq!(
    CallableGlobalJS::try_from("Array"),
    Ok(CallableGlobalJS::Array)
  );
  assert_eq!(
    CallableGlobalJS::try_from("Object"),
    Ok(CallableGlobalJS::Object)
  );
  // `Math` is a valid callee because its methods fold; calling it is not.
  assert_eq!(CallableGlobalJS::try_from("Math"), Err(()));
  assert_eq!(CallableGlobalJS::try_from("console"), Err(()));
}

#[test]
fn callable_global_js_name_is_the_name_it_was_mapped_from() {
  for global in [
    CallableGlobalJS::String,
    CallableGlobalJS::Number,
    CallableGlobalJS::Array,
    CallableGlobalJS::Object,
  ] {
    assert_eq!(CallableGlobalJS::try_from(global.name()), Ok(global));
  }
}
