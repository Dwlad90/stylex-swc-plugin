use crate::utils::{prelude::*, transform::stringify_js};
use rustc_hash::FxHashMap;

fn fixture_dir() -> std::path::PathBuf {
  std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/__fixtures__")
}

fn stylex_transform(
  comments: TestComments,
  aliases: FxHashMap<String, Vec<String>>,
  unstable_module_resolution: ModuleResolution,
) -> impl Pass {
  let fixture_path = fixture_dir();

  build_test_transform(comments, move |b| {
    b.with_cwd(fixture_path.clone())
      .with_filename(fixture_path.clone().join("main.stylex.js").into())
      .with_aliases(aliases)
      .with_unstable_module_resolution(unstable_module_resolution)
  })
}

fn transform_with_aliased_inline_consts(
  input: &str,
  aliases: FxHashMap<String, Vec<String>>,
  unstable_module_resolution: Option<ModuleResolution>,
) -> String {
  let fixture_path = fixture_dir();

  let mut merged_aliases = FxHashMap::default();
  merged_aliases.insert(
    "~fixture/*".to_string(),
    vec![fixture_path.join("*").to_string_lossy().to_string()],
  );
  merged_aliases.extend(aliases);

  let unstable_module_resolution = unstable_module_resolution.unwrap_or_else(|| ModuleResolution {
    root_dir: Some(fixture_path.to_string_lossy().to_string()),
    theme_file_extension: None,
    ..ModuleResolution::common_js(None)
  });

  stringify_js(input, ts_syntax(), move |tr| {
    stylex_transform(
      tr.comments.clone(),
      merged_aliases,
      unstable_module_resolution,
    )
  })
}

#[test]
fn resolves_root_placeholder_alias_paths_for_define_consts_imports() {
  let fixture_dir = fixture_dir();
  let project_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let relative_fixture_path = fixture_dir
    .strip_prefix(&project_root)
    .unwrap()
    .to_string_lossy()
    .replace('\\', "/");
  let root_fixture_glob = format!("/ROOT/{}/*", relative_fixture_path);

  let mut aliases = FxHashMap::default();
  aliases.insert("~root-fixture/*".to_string(), vec![root_fixture_glob]);

  let input = r#"
    import * as stylex from '@stylexjs/stylex';
    import { breakpoints } from '~root-fixture/constants.stylex';

    export const styles = stylex.create({
      root: {
        color: {
          default: 'red',
          [breakpoints.small]: 'blue',
        },
      },
    });
  "#;

  let output = transform_with_aliased_inline_consts(
    input,
    aliases,
    Some(ModuleResolution {
      root_dir: Some(project_root.to_string_lossy().to_string()),
      theme_file_extension: None,
      ..ModuleResolution::common_js(None)
    }),
  );
  insta::assert_snapshot!(
    "resolves_root_placeholder_alias_paths_for_define_consts_imports",
    output
  );
}

#[test]
fn resolves_aliased_absolute_paths_for_define_consts_imports() {
  let input = r#"
    import * as stylex from '@stylexjs/stylex';
    import { breakpoints } from '~fixture/constants.stylex';

    export const styles = stylex.create({
      root: {
        color: {
          default: 'red',
          [breakpoints.small]: 'blue',
        },
      },
    });
  "#;

  let output = transform_with_aliased_inline_consts(input, FxHashMap::default(), None);
  insta::assert_snapshot!(
    "resolves_aliased_absolute_paths_for_define_consts_imports",
    output
  );
}
