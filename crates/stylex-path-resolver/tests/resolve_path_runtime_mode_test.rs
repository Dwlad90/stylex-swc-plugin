//! Integration test to cover runtime-only (`cfg(not(test))`) resolve_path
//! branches.

use std::path::PathBuf;

use rustc_hash::FxHashMap;
use stylex_path_resolver::{package_json::PackageJsonExtended, resolvers::resolve_path};

/// In runtime mode, resolve_path should reject non-file inputs and include the
/// original path.
#[test]
#[should_panic(expected = "Resolve path must be a file")]
fn resolve_path_panics_for_non_file_input_in_runtime_mode() {
  let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("fixtures")
    .join("application-pnpm");
  let processing_path = root.join("src/components");

  resolve_path(
    &processing_path,
    &root,
    &mut FxHashMap::<String, PackageJsonExtended>::default(),
  );
}
