import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1e2nbdu{color:red}", 3000);
_inject2(".xju2f9n{color:blue}", 3000);
const styles = {
    "primary-variant": {
        kMwMTN: "x1e2nbdu",
        $$css: true
    },
    secondaryVariant: {
        kMwMTN: "xju2f9n",
        $$css: true
    }
};
function TestComponent({ variant }) {
    return <div {...stylex.props(styles.secondaryVariant, styles[`${variant}-variant`])}/>;
}
