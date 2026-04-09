#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use utils::prelude::*;

stylex_test!(
  transform_simple_css_class,
  r#"
    import s from "@stylexjs/stylex";

    const c = s.create({
      base: {
        backgroundColor: 'red',
        color: 'blue',
      },
    });
  "#
);

stylex_test!(
  transform_multiple_simple_css_classes,
  r#"
    import s from "@stylexjs/stylex";

    const c = s.create({
      base: {
        color: "red",
        borderColor: "blue",
      },
      test:{
        borderColor: "pink",
        padding: "10px",
      },
      wrapper: {
        color: "red",
        borderColor: "pink",
      },
      container:{
        marginLeft: "10px",
        padding: "10px",
      }
    });
  "#
);

stylex_test!(
  transform_multiple_simple_css_classes_and_inject_to_react_component,
  r#"
    import s from "@stylexjs/stylex";

    const c = s.create({
      base: {
        color: "red",
        borderColor: "blue",
      },
      test:{
        borderColor: "pink",
        padding: "10px",
      },
      wrapper: {
        color: "red",
        borderColor: "pink",
      },
      container:{
        marginLeft: "10px",
        padding: "10px",
      }
    });

    export default function Home() {
      const { className, style } = s.props(c.base, c.test);

      return (
        <main className={className} style={style}>
          Main
        </main>
      );
    }
  "#
);
