import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
export function Component({ flag }) {
    if (flag) {
        let tokens = {
            color: "blue"
        };
        tokens = {
            color: "green"
        };
    }
    return <div className="x1e2nbdu"/>;
}
