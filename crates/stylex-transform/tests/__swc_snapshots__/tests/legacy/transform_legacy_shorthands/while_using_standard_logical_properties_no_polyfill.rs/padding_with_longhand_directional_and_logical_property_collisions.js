import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".paddingTop-x123j3cw{padding-top:5px}",
    priority: 4000
});
_inject2({
    ltr: ".paddingBottom-xs9asl8{padding-bottom:5px}",
    priority: 4000
});
_inject2({
    ltr: ".paddingInlineStart-xaso8d8{padding-left:5px}",
    priority: 3000,
    rtl: ".paddingInlineStart-xaso8d8{padding-right:5px}"
});
_inject2({
    ltr: ".paddingInlineEnd-x2vl965{padding-right:10px}",
    priority: 3000,
    rtl: ".paddingInlineEnd-x2vl965{padding-left:10px}"
});
_inject2({
    ltr: ".paddingTop-x1nn3v0j{padding-top:2px}",
    priority: 4000
});
_inject2({
    ltr: ".paddingBottom-x1120s5i{padding-bottom:2px}",
    priority: 4000
});
_inject2({
    ltr: ".paddingLeft-xnljgj5{padding-left:22px}",
    priority: 4000
});
const styles = {
    foo: {
        "paddingTop-kLKAdn": "paddingTop-x123j3cw",
        "paddingBottom-kGO01o": "paddingBottom-xs9asl8",
        "paddingInlineStart-kZCmMZ": "paddingInlineStart-xaso8d8",
        "paddingInlineEnd-kwRFfy": "paddingInlineEnd-x2vl965",
        $$css: true
    },
    bar: {
        "paddingTop-kLKAdn": "paddingTop-x1nn3v0j",
        "paddingBottom-kGO01o": "paddingBottom-x1120s5i",
        "paddingLeft-kE3dHu": "paddingLeft-xnljgj5",
        "paddingInlineStart-kZCmMZ": null,
        "paddingInlineEnd-kwRFfy": null,
        $$css: true
    }
};
"paddingTop-x1nn3v0j paddingBottom-x1120s5i paddingLeft-xnljgj5";
export const string = stylex(styles.foo, styles.bar, xstyle);
