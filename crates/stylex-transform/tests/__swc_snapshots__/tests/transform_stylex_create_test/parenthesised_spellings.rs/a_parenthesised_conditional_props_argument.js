import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
export const styles = {
    a: {
        kMwMTN: "x1e2nbdu",
        $$css: true
    },
    b: {
        k1xSpc: "x78zum5",
        $$css: true
    }
};
export const picked = (cond)=>({
        0: {
            className: "x78zum5"
        },
        1: {
            className: "x1e2nbdu"
        }
    })[!!cond << 0];
export const guarded = (cond)=>({
        0: {},
        1: {
            className: "x78zum5"
        }
    })[!!cond << 0];
