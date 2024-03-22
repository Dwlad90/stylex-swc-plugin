import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1e2nbdu{color:red}", 3000);
_inject2(".xju2f9n{color:blue}", 3000);
const styles = {
    default: {
        "FooBar__styles.default": "FooBar__styles.default",
        color: "x1e2nbdu",
        $$css: true
    },
    active: {
        "FooBar__styles.active": "FooBar__styles.active",
        color: "xju2f9n",
        $$css: true
    }
};
stylex(styles.default, isActive && styles.active);