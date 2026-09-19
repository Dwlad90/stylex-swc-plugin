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
    default: {
        "FooBar__styles.default": "FooBar__styles.default",
        "color-kMwMTN": "x1e2nbdu",
        $$css: "js/FooBar.react.js:3"
    },
    active: {
        "FooBar__styles.active": "FooBar__styles.active",
        "color-kMwMTN": "xju2f9n",
        $$css: "js/FooBar.react.js:6"
    }
};
stylex(styles.default, isActive && styles.active);
