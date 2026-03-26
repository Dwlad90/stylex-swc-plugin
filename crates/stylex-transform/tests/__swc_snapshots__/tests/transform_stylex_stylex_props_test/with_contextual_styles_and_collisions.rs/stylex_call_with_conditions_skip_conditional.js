import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".xrkmrrc{background-color:red}",
    priority: 3000
});
_inject2({
    ltr: ".xju2f9n{color:blue}",
    priority: 3000
});
const styles = {
    default: {
        kWkggS: "xrkmrrc",
        $$css: true
    },
    active: {
        kMwMTN: "xju2f9n",
        $$css: true
    }
};
stylex.props([
    styles.default,
    isActive && styles.active
]);
