//! A variable group declared in a file whose name cannot be hashed.
//!
//! The four producers that declare variables or constants name each one after
//! the file it is declared in, so the file has to be one the compiler can build
//! a name from -- a `*.stylex.js`, or the `*.stylex.consts.js` a constants-only
//! file is written as. Every other file name has nothing to hash, and the call
//! is refused for the name rather than for anything the author wrote inside it.
//!
//! Each producer reads the same rule and each one says its own name, so all four
//! are asked here. The rest of the suite compiles these calls in a theme file,
//! which is why the refusal had no case.
//!
//! The sentence is written out at each case: `should_panic(expected = …)` takes
//! a literal, so a shared constant cannot reach it.

use crate::utils::prelude::*;
use swc_core::common::FileName;

fn app_file_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_runtime_injection()
      .with_filename(FileName::Real("/stylex/packages/app.js".into()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        "/stylex/packages/".to_string(),
      )))
  })
}

stylex_test_panic!(
  define_vars_refuses_a_file_name_it_cannot_hash,
  "Unable to generate hash for defineVars().",
  |tr| app_file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({ sm: '4px' });
  "#
);

stylex_test_panic!(
  define_vars_nested_refuses_a_file_name_it_cannot_hash,
  "Unable to generate hash for unstable_defineVarsNested().",
  |tr| app_file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.unstable_defineVarsNested({ space: { sm: '4px' } });
  "#
);

stylex_test_panic!(
  define_consts_refuses_a_file_name_it_cannot_hash,
  "Unable to generate hash for defineConsts().",
  |tr| app_file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const consts = stylex.defineConsts({ sm: '4px' });
  "#
);

stylex_test_panic!(
  define_consts_nested_refuses_a_file_name_it_cannot_hash,
  "Unable to generate hash for unstable_defineConstsNested().",
  |tr| app_file_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const consts = stylex.unstable_defineConstsNested({ space: { sm: '4px' } });
  "#
);
