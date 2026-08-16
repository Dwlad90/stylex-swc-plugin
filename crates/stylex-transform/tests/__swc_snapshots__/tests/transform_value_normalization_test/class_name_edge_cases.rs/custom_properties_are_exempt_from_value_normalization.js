import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".x1sgzfop{--myVar:backgroundColor}",
    priority: 1
});
_inject2({
    ltr: ".x3592ib{--myVar:0px}",
    priority: 1
});
_inject2({
    ltr: ".x19srcev{color:var(--x,var(--y,#ABCDEF))}",
    priority: 3000
});
_inject2({
    ltr: ".x1tjm4ty{width:var(--x)px}",
    priority: 4000
});
export const styles = {
    camelCaseValue: {
        "--myVar": "x1sgzfop",
        $$css: true
    },
    zeroLength: {
        "--myVar": "x3592ib",
        $$css: true
    },
    nestedFallback: {
        kMwMTN: "x19srcev",
        $$css: true
    },
    flushUnit: {
        kzqmXN: "x1tjm4ty",
        $$css: true
    }
};
