import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
const styles = {};
export const text = stylex.props('a string');
export const guarded = {
    0: {},
    1: {
        className: "x1e2nbdu"
    }
}[!!1 << 0];
export const missing = stylex.props(styles.notAStyle);
