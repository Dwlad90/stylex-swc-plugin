import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".color-x1e2nbdu{color:red}", 3000);
_inject2(".color-xju2f9n{color:blue}", 3000);
const styles = {
    default: {
        color: "color-x1e2nbdu",
        $$css: "js/FooBar.react.js:3"
    },
    active: {
        color: "color-xju2f9n",
        $$css: "js/FooBar.react.js:6"
    }
};
stylex(styles.default, isActive && styles.active);
