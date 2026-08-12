//! Class name hashes for values carrying non-ASCII characters.
//!
//! The hash is defined over UTF-16 code units masked to their low byte, so a
//! non-ASCII value must not be hashed as UTF-8 bytes. Regression coverage for
//! https://github.com/Dwlad90/stylex-swc-plugin/issues/1248, where the emitted
//! CSS text was already byte-identical to `@stylexjs/babel-plugin` but the class
//! names diverged — the two compilers could not be mixed across SSR and client.
//!
//! Runtime injection is enabled so each snapshot records the emitted CSS text
//! alongside the class name: the class name is what regressed, but pinning the
//! rule text next to it is what proves the two still agree. The `content`
//! snapshot must read `.xe0tt08{content:"•"}` and `.xwywlkd{content:'•'}`, which
//! is exactly what `@stylexjs/babel-plugin@0.19.0` emits for the same input.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

stylex_test!(
  content_with_non_ascii_character,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { content: '•' },
      b: { content: "'•'" },
    });
  "#
);

stylex_test!(
  font_family_with_non_ascii_name,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { fontFamily: '日本語' },
    });
  "#
);

stylex_test!(
  custom_property_with_non_ascii_name,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { '--épaisseur': '1px' },
    });
  "#
);

stylex_test!(
  content_with_astral_character,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: { content: '🎉' },
    });
  "#
);
