import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".color-x1e2nbdu{color:red}", 3000);
const styles = {
    red: {
        "color-kMwMTN": "color-x1e2nbdu",
        $$css: "npm-package:components/Foo.react.js:3"
    }
};
function Foo(props) {
    return <div id="test" {...stylex.props(props.style, styles.red)}>
      Hello World
    </div>;
}
