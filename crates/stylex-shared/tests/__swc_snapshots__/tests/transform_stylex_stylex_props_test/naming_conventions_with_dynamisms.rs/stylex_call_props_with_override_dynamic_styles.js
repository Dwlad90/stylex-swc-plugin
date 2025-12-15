import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".xju2f9n{color:blue}",
    priority: 3000
});
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
