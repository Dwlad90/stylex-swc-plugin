use crate::utils::prelude::*;
use stylex_enums::sx_prop_name_param::SxPropNameParam;
use swc_core::{common::FileName, ecma::transforms::testing::test};

stylex_test!(
  sx_attr_instead_of_stylex_props,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_sx_prop_name(SxPropNameParam::Enabled("css".to_string()))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_sx_prop_name(SxPropNameParam::Disabled)
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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

// sx={[styles.a, styles.b]} — array syntax maps to stylex.props(styles.a, styles.b)
stylex_test!(
  sx_attr_array_syntax,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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

// Compiled JSX form with array: _jsx("div", { sx: [styles.card, styles.blueBg] })
stylex_test!(
  sx_attr_compiled_jsx_form_array_syntax,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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

// Vue: _createElementBlock / _createElementVNode with sx prop
stylex_test!(
  sx_attr_vue_create_element_block,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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

// Solid.js: _$setAttribute(el, "sx", value) → _$spread(el, _$mergeProps(() => stylex.props(value)), false, true)
stylex_test!(
  sx_attr_solid_js_set_attribute,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some("/js".to_string())))
    .with_runtime_injection()
    .into_pass(),
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
