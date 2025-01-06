import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1e2nbdu{color:red}", 3000);
const styles = {
    red: {
        color: "x1e2nbdu",
        $$css: true
    },
    blue: {
        color: null,
        $$css: true
    }
};
stylex.props([
    styles.red,
    isActive && styles.blue
]);
