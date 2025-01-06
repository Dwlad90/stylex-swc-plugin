import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1e2nbdu{color:red}", 3000);
const styles = {
    red: {
        color: "x1e2nbdu",
        $$css: true
    }
};
stylex.props(Math.random() > 1 ? styles.red : undefined);
stylex.props(true ? styles.red : undefined);
stylex.props(false ? styles.red : undefined);
stylex.props(Math.random() > 1 ? undefined : styles.red);
stylex.props(true ? undefined : styles.red);
stylex.props(false ? undefined : styles.red);
