import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1e2nbdu{color:red}", 3000);
const styles = {
    red: {
        kMwMTN: "x1e2nbdu",
        $$css: true
    },
    blue: {
        kMwMTN: null,
        $$css: true
    }
};
stylex(styles.red, isActive && styles.blue);
