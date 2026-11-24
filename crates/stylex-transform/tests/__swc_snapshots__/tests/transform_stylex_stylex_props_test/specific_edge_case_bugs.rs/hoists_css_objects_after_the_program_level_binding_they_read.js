import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
import { get } from './x';
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
const styles = {
    root: {
        k1xSpc: "x78zum5",
        $$css: true
    }
};
const cls = get();
const _temp = {
    $$css: true,
    __: cls
};
export function F(c) {
    return stylex.props(styles.root, c && _temp);
}
const other = get();
const _temp2 = {
    $$css: true,
    __: other
};
export const o = {
    f (c) {
        return stylex.props(c && _temp2);
    }
};
