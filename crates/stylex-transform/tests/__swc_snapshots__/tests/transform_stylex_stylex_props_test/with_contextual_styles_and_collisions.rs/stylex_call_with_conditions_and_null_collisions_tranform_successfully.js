import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
const styles = {
    red: {
        kMwMTN: "x1e2nbdu",
        $$css: true
    }
};
stylex.props(Math.random() > 1 ? styles.red : null);
stylex.props(true ? styles.red : null);
stylex.props(false ? styles.red : null);
stylex.props(Math.random() > 1 ? null : styles.red);
stylex.props(true ? null : styles.red);
stylex.props(false ? null : styles.red);
