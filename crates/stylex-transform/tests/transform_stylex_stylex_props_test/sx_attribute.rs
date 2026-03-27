use stylex_enums::sx_prop_name_param::SxPropNameParam;
use stylex_structures::{
  plugin_pass::PluginPass,
  stylex_options::{StyleXOptions, StyleXOptionsParams},
};
use stylex_transform::StyleXTransform;
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test,
  },
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_instead_of_stylex_props,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      sx_prop_name: Some(SxPropNameParam::Enabled("css".to_string())),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_with_custom_sx_prop_name,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      sx_prop_name: Some(SxPropNameParam::Disabled),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_disabled_when_sx_prop_name_is_false,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_not_applied_to_component_elements,
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
test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_array_syntax,
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
test!(
  Syntax::Typescript(TsSyntax {
    tsx: false,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_compiled_jsx_form,
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
test!(
  Syntax::Typescript(TsSyntax {
    tsx: false,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_compiled_jsx_form_array_syntax,
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
test!(
  Syntax::Typescript(TsSyntax {
    tsx: false,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_compiled_jsx_not_applied_to_components,
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
test!(
  Syntax::Typescript(TsSyntax {
    tsx: false,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_vue_create_element_block,
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
test!(
  Syntax::Typescript(TsSyntax {
    tsx: false,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_solid_js_set_attribute,
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
test!(
  Syntax::Typescript(TsSyntax {
    tsx: false,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_vite_jsx_runtime_call,
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
test!(
  Syntax::Typescript(TsSyntax {
    tsx: false,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_solid_js_set_attribute_array,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_and_props_calls_are_equivalent,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_import_name_as_default,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_import_name_as_namespace,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_import_name_as_named,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      dev: Some(true),
      enable_debug_class_names: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/js".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  sx_attr_import_name_as_props,
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
