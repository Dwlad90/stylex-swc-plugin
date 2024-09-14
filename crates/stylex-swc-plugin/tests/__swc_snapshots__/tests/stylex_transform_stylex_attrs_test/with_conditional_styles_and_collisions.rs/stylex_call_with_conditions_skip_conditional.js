import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".xju2f9n{color:blue}", 3000);
const styles = {
    default: {
        backgroundColor: "xrkmrrc",
        $$css: true
    },
    active: {
        color: "xju2f9n",
        $$css: true
    }
};
stylex.attrs([
    styles.default,
    isActive && styles.active
]);
