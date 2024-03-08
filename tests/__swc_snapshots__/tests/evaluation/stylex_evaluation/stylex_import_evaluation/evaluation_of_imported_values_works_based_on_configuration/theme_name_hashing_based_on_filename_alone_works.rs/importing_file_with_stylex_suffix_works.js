import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1e2nbdu{color:red}", 3000);
// import { MyTheme } from 'otherFile.stylex';
const styles = {
    green: {
        color: "x1e2nbdu",
        $$css: true
    }
};
stylex(styles.green);
