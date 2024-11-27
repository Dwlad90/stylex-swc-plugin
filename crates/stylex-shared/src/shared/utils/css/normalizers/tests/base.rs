#[cfg(test)]

mod normalizers {
  use crate::shared::utils::css::common::{stringify, swc_parse_css};
  use crate::shared::utils::css::normalizers::base::base_normalizer;

  #[test]
  fn should_normalize() {
    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css("* {{ transitionProperty: opacity, margin-top; }}")
          .0
          .unwrap(),
        false
      )),
      "*{{transitionproperty:opacity,margin-top}}"
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css("* {{ boxShadow: 0px 2px 4px var(--shadow-1); }}")
          .0
          .unwrap(),
        false
      )),
      "*{{boxshadow:0 2px 4px var(--shadow-1)}}"
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css("* {{ opacity: 0.5; }}").0.unwrap(),
        false
      )),
      "*{{opacity:.5}}"
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css("* {{ transitionDuration: 500ms; }}")
          .0
          .unwrap(),
        false
      )),
      "*{{transitionduration:.5s}}"
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css("* {{ boxShadow: 1px 1px #000; }}").0.unwrap(),
        false
      )),
      "*{{boxshadow:1px 1px#000}}"
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css(r#"* {{ quotes: '""'; }}"#).0.unwrap(),
        false
      )),
      r#"*{{quotes:""}}"#
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css(r#"* {{ quotes: '"123"'; }}"#).0.unwrap(),
        false
      )),
      r#"*{{quotes:"123"}}"#
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css(r#"* {{ gridTemplateAreas: '"content"'; }}"#)
          .0
          .unwrap(),
        false
      )),
      r#"*{{gridtemplateareas:"content"}}"#
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css(r#"* {{ gridTemplateAreas: '"content" "sidebar"'; }}"#)
          .0
          .unwrap(),
        false
      )),
      r#"*{{gridtemplateareas:"content" "sidebar"}}"#
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css("* {{ color: oklch(from var(--xs74gcj) l c h / 0.5) }}")
          .0
          .unwrap(),
        false
      )),
      "*{{color:oklch(from var(--xs74gcj) l c h / 0.5)}}"
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css("* {{ color: oklab(from #0000FF calc(l + 0.1) a b / calc(alpha * 0.9)) }}")
          .0
          .unwrap(),
        false
      )),
      "*{{color:oklab(from #0000FF calc(l + 0.1) a b / calc(alpha * 0.9))}}"
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css("* {{ color: oklab(from hsl(180 100% 50%) calc(l - 0.1) a b) }}")
          .0
          .unwrap(),
        false
      )),
      "*{{color:oklab(from hsl(180 100% 50%) calc(l - 0.1) a b)}}"
    );

    assert_eq!(
      stringify(&base_normalizer(
        swc_parse_css("* {{ color: oklab(from green l a b / 0.5) }}")
          .0
          .unwrap(),
        false
      )),
      "*{{color:oklab(from green l a b / 0.5)}}"
    );
  }
}
