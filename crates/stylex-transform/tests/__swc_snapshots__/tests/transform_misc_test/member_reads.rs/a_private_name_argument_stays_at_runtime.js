import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
const styles = {
    root: {
        kMwMTN: "x1e2nbdu",
        $$css: true
    }
};
export class Card {
    #root = 1;
    read() {
        return stylex.props(styles.#root);
    }
}
