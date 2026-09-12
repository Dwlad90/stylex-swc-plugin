use crate::utils::prelude::*;
use stylex_enums::sx_prop_name_param::SxPropNameParam;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(b)
      .with_filename(FileName::Real(
        "/js/node_modules/npm-package/dist/components/Foo.react.js".into(),
      ))
      .with_debug(true)
      .with_dev(true)
      .with_enable_debug_class_names(true)
      .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
      .with_runtime_injection()
  })
}

stylex_test!(
  sx_attr_instead_of_stylex_props,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    function Foo() {
      return (
        <>
          <div id="test" sx={styles.red}>Hello World</div>
          <div className="test" sx={styles.red} id="test">Hello World</div>
          <div id="test" sx={styles.red} className="test">Hello World</div>
        </>
      );
    }
  "#
);

stylex_test!(
  sx_attr_with_custom_sx_prop_name,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_sx_prop_name(SxPropNameParam::Enabled("css".to_string()))
  }),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    function Foo() {
      return <div css={styles.red}>Hello World</div>;
    }
  "#
);

stylex_test!(
  sx_attr_disabled_when_sx_prop_name_is_false,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_sx_prop_name(SxPropNameParam::Disabled)
  }),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    function Foo() {
      return <div sx={styles.red}>Hello World</div>;
    }
  "#
);

stylex_test!(
  sx_attr_not_applied_to_component_elements,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    function Foo() {
      return <MyComponent sx={styles.red}>Hello World</MyComponent>;
    }
  "#
);

// sx={[styles.a, styles.b]} passes the array expression through to stylex.props.
stylex_test!(
  sx_attr_array_syntax,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      card: {
        borderRadius: 4,
      },
      blueBg: {
        backgroundColor: 'blue',
      }
    });
    function Foo() {
      return <div sx={[styles.card, styles.blueBg]}>Hello World</div>;
    }
  "#
);

// Compiled JSX form: _jsx("div", { sx: styles.main }) — no JSX syntax
stylex_test!(
  sx_attr_compiled_jsx_form,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      }
    });
    function App() {
      return _jsx("div", {
          sx: styles.main,
          children: "Hello World"
        });
      }
  "#
);

// Compiled JSX form with array: _jsx("div", { sx: [styles.card, styles.blueBg]
// })
stylex_test!(
  sx_attr_compiled_jsx_form_array_syntax,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      card: {
        borderRadius: 4,
      },
      blueBg: {
        backgroundColor: 'blue',
      }
    });
    function App() {
      return _jsx("div", {
          sx: [styles.card, styles.blueBg],
          children: "Hello World"
        });
      }
  "#
);

// Compiled JSX: uppercase component names are NOT transformed
stylex_test!(
  sx_attr_compiled_jsx_not_applied_to_components,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      }
    });
    function App() {
      return _jsx(MyComponent, {
          sx: styles.main,
          children: "Hello World"
        });
      }
  "#
);

// Compiled JSX, object shorthand: `_jsx("div", { sx })`. A JSX-compiling pass
// collapses `sx={sx}` to this form, which names the same prop. A shorthand
// that names something else is passed over, on a host element that is scanned.
stylex_test!(
  sx_attr_compiled_jsx_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export function Leaf({ id, sx }) {
      return _jsx("div", {
          id,
          sx,
          children: "Hello World"
        });
      }
  "#
);

// A host element whose only shorthand names something else keeps every prop.
stylex_test!(
  sx_attr_compiled_jsx_other_shorthand_unchanged,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export function Leaf({ id }) {
      return _jsx("div", {
          id,
          children: "Hello World"
        });
      }
  "#
);

// The shorthand follows the configured prop name, like every other form.
stylex_test!(
  sx_attr_compiled_jsx_shorthand_with_custom_sx_prop_name,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_sx_prop_name(SxPropNameParam::Enabled("css".to_string()))
  }),
  r#"
    import stylex from 'stylex';
    export function Leaf({ css }) {
      return _jsx("div", {
          css,
          children: "Hello World"
        });
      }
  "#
);

// A computed key whose text is known at compile time names the prop, whether
// it is written as a string literal or as a template literal with no
// expressions.
stylex_test!(
  sx_attr_compiled_jsx_computed_key,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      },
      card: {
        borderRadius: 4,
      }
    });
    function App() {
      return _jsx("div", {
          ["sx"]: styles.main,
          children: _jsx("span", {
              [`sx`]: styles.card
            })
        });
      }
  "#
);

// A computed key that is only known at run time names no prop at compile time,
// so it is left alone.
stylex_test!(
  sx_attr_compiled_jsx_dynamic_computed_key_unchanged,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      }
    });
    function App(key) {
      return _jsx("div", {
          [key]: styles.main,
          children: "Hello World"
        });
      }
  "#
);

// A getter, a setter or a method carries no value expression to forward, so it
// is left alone — the same way raw markup skips an attribute that is not an
// expression container.
stylex_test!(
  sx_attr_compiled_jsx_accessor_and_method_unchanged,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      }
    });
    function App() {
      return _jsx("div", {
          get sx() {
            return styles.main;
          },
          children: [
            _jsx("span", {
                sx() {
                  return styles.main;
                }
              }),
            _jsx("b", {
                set sx(value) {
                  this.value = value;
                }
              })
          ]
        });
      }
  "#
);

// A spread names no key at compile time, so it is not inspected — the same way
// a spread attribute in raw markup is not inspected. A spread of an object
// that does name the prop is left alone too, which is what tells the two
// apart: were the spread inspected, this one would transform.
stylex_test!(
  sx_attr_compiled_jsx_spread_unchanged,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      }
    });
    function App(rest) {
      return _jsx("div", {
          ...{
            sx: styles.main
          },
          children: _jsx("span", {
              ...rest
            })
        });
      }
  "#
);

// Only a host element carries the prop: the shorthand on a component is left
// alone, like the explicit property.
stylex_test!(
  sx_attr_compiled_jsx_shorthand_not_applied_to_components,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export function Leaf({ sx }) {
      return _jsx(MyComponent, {
          sx,
          children: "Hello World"
        });
      }
  "#
);

// The prop written twice: the first occurrence wins, matching raw markup,
// which stops at the first matching attribute.
stylex_test!(
  sx_attr_compiled_jsx_duplicate_prop_takes_the_first,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      },
      card: {
        borderRadius: 4,
      }
    });
    function App() {
      return _jsx("div", {
          sx: styles.main,
          ["sx"]: styles.card,
          children: "Hello World"
        });
      }
  "#
);

// A deep tree of compiled calls, mixing every matched and every skipped form
// at once. The props object of a call holds the whole subtree below it, so
// this is also where a scan that copies before it matches would cost the most.
stylex_test!(
  sx_attr_compiled_jsx_deeply_nested_mixed_forms,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      a: { color: 'red' },
      b: { borderRadius: 4 },
      c: { backgroundColor: 'blue' },
      d: { display: 'flex' }
    });
    function App({ sx, rest }) {
      return _jsx("section", {
          sx,
          id: "outer",
          children: _jsxs("div", {
              "sx": styles.a,
              className: "middle",
              children: [
                _jsx("span", {
                    ["sx"]: [styles.b, styles.c],
                    children: _jsx("em", {
                        ...rest,
                        [`sx`]: styles.d,
                        children: _jsx(MyComponent, {
                            sx: styles.a
                          })
                      })
                  }),
                _jsx("p", {
                    get sx() {
                      return styles.b;
                    }
                  })
              ]
            })
        });
      }
  "#
);

// Vue: _createElementBlock / _createElementVNode with sx prop
stylex_test!(
  sx_attr_vue_create_element_block,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      },
      card: {
        borderRadius: 4,
      }
    });
    function App() {
      return _createElementBlock("div", {
          sx: styles.main
        }, [
          _createElementVNode("div", {
              sx: [styles.card]
            }, "Hello World")
        ]);
      }
  "#
);

// Solid.js: _$setAttribute(el, "sx", value) → _$spread(el, _$mergeProps(() =>
// stylex.props(value)), false, true)
stylex_test!(
  sx_attr_solid_js_set_attribute,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      },
      card: {
        borderRadius: 4,
      },
      blueBg: {
        backgroundColor: 'blue',
      }
    });
    function App() {
      _$setAttribute(_el$, "sx", styles.main);
      _$spread(_el$2, _$mergeProps(() => stylex.props(styles.card, styles.blueBg)), false, true);
    }
  "#
);

// Vite: jsx runtime call with sx prop
stylex_test!(
  sx_attr_vite_jsx_runtime_call,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      },
      card: {
        borderRadius: 4,
      },
      blueBg: {
        backgroundColor: 'blue',
      }
    });
    function App() {
      return jsx("div", {
          sx: styles.main
        }, [
          jsx("div", {
              sx: [styles.card]
            }, "Hello World")
        ]);
      }
  "#
);

// Solid.js: array syntax in _$setAttribute
stylex_test!(
  sx_attr_solid_js_set_attribute_array,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      card: {
        borderRadius: 4,
      },
      blueBg: {
        backgroundColor: 'blue',
      }
    });
    function App() {
      _$setAttribute(_el$, "sx", [styles.card, styles.blueBg]);
      }
  "#
);

stylex_test!(
  sx_attr_and_props_calls_are_equivalent,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      },
      blueBg: {
        backgroundColor: 'blue',
      }
    });
    function Foo() {
      return (
        <>
          <div id="test" sx={styles.red}>Hello World</div>
          <div id="test" {...stylex.props(styles.red)}>Hello World</div>
          <div className="test" sx={[styles.red, color && styles.blueBg]} id="test">Hello World</div>
          <div className="test" {...stylex.props(styles.red, color && styles.blueBg)} id="test">Hello World</div>
          <div id="test" sx={styles.blueBg} className="test">Hello World</div>
          <div id="test" {...stylex.props(styles.blueBg)} className="test">Hello World</div>
        </>
      );
    }
  "#
);

stylex_test!(
  sx_attr_import_name_as_default,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import sx from '@stylexjs/stylex';
    const styles = sx.create({
      red: {
        color: 'red',
      }
    });
    function Foo({overrideProps= []}) {
      return <div sx={[styles.red, ...overrideProps]}>Hello World</div>;
    }
  "#
);

stylex_test!(
  sx_attr_import_name_as_namespace,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as sx from '@stylexjs/stylex';
    const styles = sx.create({
      red: {
        color: 'red',
      }
    });
    function Foo({overrideProps= []}) {
      return <div sx={[styles.red, ...overrideProps]}>Hello World</div>;
    }
  "#
);

stylex_test!(
  sx_attr_import_name_as_named,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import {create, props as sx} from '@stylexjs/stylex';
    const styles = create({
      red: {
        color: 'red',
      }
    });
    function Foo({overrideProps= []}) {
      return <div sx={[styles.red, ...overrideProps]}>Hello World</div>;
    }
  "#
);

stylex_test!(
  sx_attr_import_name_as_props,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import {create, props} from '@stylexjs/stylex';
    const styles = create({
      red: {
        color: 'red',
      }
    });
    function Foo({overrideProps= []}) {
      return <div sx={[styles.red, ...overrideProps]}>Hello World</div>;
    }
  "#
);

// sx attribute runtime binding: ensure a value-level `stylex` namespace import
// exists before emitting `stylex.props(...)`, reusing an existing runtime import
// when present and respecting configured import sources.
fn stylex_transform_runtime_binding(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b).with_runtime_injection())
}

stylex_test!(
  injects_a_value_namespace_import_alongside_a_type_namespace_import,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    import type * as stylex from '@stylexjs/stylex';
    function Foo(props) {
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

stylex_test!(
  injects_a_value_namespace_import_alongside_a_named_type_import,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    import type { StyleXStyles } from '@stylexjs/stylex';
    function Foo(props) {
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

stylex_test!(
  injects_a_value_namespace_import_when_there_is_no_stylex_import,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    function Foo(props) {
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

stylex_test!(
  injects_from_configured_import_source_when_there_is_no_stylex_import,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| {
    b.with_import_sources(vec![ImportSources::Regular(
      "custom-stylex-path".to_string(),
    )])
  }),
  r#"
    function Foo(props) {
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

stylex_test!(
  injects_from_existing_type_only_custom_import_source,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| {
    b.with_import_sources(vec![ImportSources::Regular(
      "custom-stylex-path".to_string(),
    )])
  }),
  r#"
    import type { StyleXStyles } from 'custom-stylex-path';
    function Foo(props) {
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

stylex_test!(
  reuses_an_existing_value_namespace_import,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function Foo(props) {
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

stylex_test!(
  avoids_a_local_stylex_binding_that_would_shadow_the_injected_import,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    function Foo(props) {
      const stylex = props.stylex;
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

stylex_test!(
  injects_one_value_namespace_import_for_multiple_sx_attributes,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    function Foo(props) {
      const x = props.x;
      return (
        <>
          <svg sx={x} />
          <svg sx={props.y} />
        </>
      );
    }
  "#
);

stylex_test!(
  does_not_transform_sx_on_capitalized_components,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    function Foo(props) {
      const x = props.x;
      return <CustomComponent sx={x} />;
    }
  "#
);

// With multiple configured import sources and no existing import, the first
// configured source wins deterministically. Guards the insertion-ordered
// `IndexSet` against the non-determinism a hash set would reintroduce.
stylex_test!(
  injects_from_the_first_configured_import_source_when_multiple_are_configured,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| {
    b.with_import_sources(vec![
      ImportSources::Regular("custom-stylex-a".to_string()),
      ImportSources::Regular("custom-stylex-b".to_string()),
    ])
  }),
  r#"
    function Foo(props) {
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

// A value-level `stylex` import that a local binding shadows must not be
// reused; a uid namespace import is injected instead.
stylex_test!(
  injects_a_uid_namespace_import_when_the_stylex_import_is_locally_shadowed,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function Foo(props) {
      const stylex = props.stylex;
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

// When the generated `_stylex` uid is already bound in the module, generation
// skips to `_stylex2`.
stylex_test!(
  injects_a_deduped_uid_namespace_import_when_the_uid_is_already_bound,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    function Foo(props) {
      const stylex = props.stylex;
      const _stylex = props.theme;
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

// A local binding that rebinds `stylex` in a *sibling* scope must NOT block
// reuse at the `sx` site: the import is still in scope there. The shadow check
// is position-aware — a module-wide shadow test would wrongly inject a uid
// import here.
stylex_test!(
  reuses_the_value_namespace_import_when_only_a_sibling_scope_rebinds_it,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function Bar() {
      const stylex = 1;
      return stylex;
    }
    function Foo(props) {
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

// An expression-bodied arrow does not have an enclosing return statement. The
// runtime binding must use the JSX opening element span itself; otherwise the
// top-level variable statement span makes the `stylex` parameter look out of
// scope and the transform incorrectly reuses the imported namespace.
stylex_test!(
  injects_a_uid_namespace_import_when_an_arrow_param_shadows_stylex,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const Foo = ({stylex, x}) => <svg sx={x} />;
  "#
);

// A named function expression binds its name inside its own body. Reusing the
// imported namespace would emit `stylex.props(...)` where `stylex` resolves to
// the function itself.
stylex_test!(
  injects_a_uid_namespace_import_when_a_named_function_expression_shadows_stylex,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const Foo = function stylex(props) {
      return <svg sx={props.x} />;
    };
  "#
);

// A catch binding shadows imports only inside the catch clause, not throughout
// the surrounding function body.
stylex_test!(
  reuses_the_value_namespace_import_after_a_sibling_catch_scope_rebinds_it,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    function Foo(props) {
      try {
        props.run();
      } catch (stylex) {
        props.onError(stylex);
      }
      return <svg sx={props.x} />;
    }
  "#
);

// A block-scoped class declaration binds its name but does not shadow `stylex`;
// the plain `stylex` namespace import is still injected and the class name is
// left untouched.
stylex_test!(
  injects_a_value_namespace_import_with_a_class_declaration_present,
  |tr| stylex_transform_runtime_binding(tr.comments.clone(), |b| b),
  r#"
    class Bar {}
    function Foo(props) {
      const x = props.x;
      return <svg sx={x} />;
    }
  "#
);

// The spread of the props call takes the place the prop held, so a later
// property of the same name wins. A `className` after the prop therefore
// replaces the class the spread just supplied, and one before it does not.
// The order is the author's to control; these snapshots pin both directions.
stylex_test!(
  sx_attr_compiled_jsx_class_name_order,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      }
    });
    function App() {
      return _jsxs("div", {
          children: [
            _jsx("div", { className: "before", sx: styles.main }),
            _jsx("div", { sx: styles.main, className: "after" })
          ]
        });
      }
  "#
);

// A name that is not a plain identifier reaches the prop only as a quoted or
// computed key. The compiler compares the name as text, so any name works.
stylex_test!(
  sx_attr_compiled_jsx_non_identifier_prop_name,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_sx_prop_name(SxPropNameParam::Enabled("data-sx".to_string()))
  }),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      }
    });
    function App() {
      return _jsx("div", {
          "data-sx": styles.main,
          children: "Hello World"
        });
      }
  "#
);

// An empty prop name disables nothing on its own, so no property can match it.
stylex_test!(
  sx_attr_compiled_jsx_blank_prop_name_unchanged,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_sx_prop_name(SxPropNameParam::Enabled(String::new()))
  }),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      main: {
        color: 'red',
      }
    });
    function App() {
      return _jsx("div", { sx: styles.main, children: "Hello World" });
    }
  "#
);
