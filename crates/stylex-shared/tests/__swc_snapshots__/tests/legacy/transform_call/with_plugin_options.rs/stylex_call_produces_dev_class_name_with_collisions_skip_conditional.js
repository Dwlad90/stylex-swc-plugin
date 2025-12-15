import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".color-xju2f9n{color:blue}",
    priority: 3000
});
const styles = {
    default: {
        "color-kMwMTN": "color-x1e2nbdu",
        $$css: "js/FooBar.react.js:3"
    },
    active: {
        "color-kMwMTN": "color-xju2f9n",
        $$css: "js/FooBar.react.js:6"
    }
};
stylex(styles.default, isActive && styles.active);
