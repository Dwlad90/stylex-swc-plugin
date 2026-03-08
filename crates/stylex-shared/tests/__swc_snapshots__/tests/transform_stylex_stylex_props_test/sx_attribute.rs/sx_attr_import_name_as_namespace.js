import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as sx from '@stylexjs/stylex';
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
const styles = {
    red: {
        "color-kMwMTN": "color-x1e2nbdu",
        $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
    }
};
function Foo({ overrideProps = [] }) {
    return <div {...sx.props(styles.red, ...overrideProps)}>Hello World</div>;
}
