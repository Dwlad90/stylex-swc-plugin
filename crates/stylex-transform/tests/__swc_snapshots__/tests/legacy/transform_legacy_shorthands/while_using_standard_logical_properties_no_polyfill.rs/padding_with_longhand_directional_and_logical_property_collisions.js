import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".x123j3cw{padding-top:5px}",
    priority: 4000
});
_inject2({
    ltr: ".xs9asl8{padding-bottom:5px}",
    priority: 4000
});
_inject2({
    ltr: ".xaso8d8{padding-left:5px}",
    priority: 3000,
    rtl: ".xaso8d8{padding-right:5px}"
});
_inject2({
    ltr: ".x2vl965{padding-right:10px}",
    priority: 3000,
    rtl: ".x2vl965{padding-left:10px}"
});
_inject2({
    ltr: ".x1nn3v0j{padding-top:2px}",
    priority: 4000
});
_inject2({
    ltr: ".x1120s5i{padding-bottom:2px}",
    priority: 4000
});
_inject2({
    ltr: ".xnljgj5{padding-left:22px}",
    priority: 4000
});
const styles = {
    foo: {
        "paddingTop-kLKAdn": "x123j3cw",
        "paddingBottom-kGO01o": "xs9asl8",
        "paddingInlineStart-kZCmMZ": "xaso8d8",
        "paddingInlineEnd-kwRFfy": "x2vl965",
        $$css: true
    },
    bar: {
        "paddingTop-kLKAdn": "x1nn3v0j",
        "paddingBottom-kGO01o": "x1120s5i",
        "paddingLeft-kE3dHu": "xnljgj5",
        "paddingInlineStart-kZCmMZ": null,
        "paddingInlineEnd-kwRFfy": null,
        $$css: true
    }
};
"x1nn3v0j x1120s5i xnljgj5";
export const string = stylex(styles.foo, styles.bar, xstyle);
