import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ":root, .xsg933n{--xr1piqu:red;--xbbvm0f:white;}",
    priority: 0.1
});
_inject2({
    ltr: "@media (prefers-color-scheme: dark){:root, .xsg933n{--xr1piqu:blue;}}",
    priority: 0.2
});
export const vars = {
    button: {
        background: "var(--xr1piqu)",
        color: "var(--xbbvm0f)"
    },
    __varGroupHash__: "xsg933n"
};
