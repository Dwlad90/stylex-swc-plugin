import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".paddingTop-x123j3cw{padding-top:5px}", 4000);
_inject2(".paddingBottom-xs9asl8{padding-bottom:5px}", 4000);
_inject2(".paddingInlineStart-xaso8d8{padding-left:5px}", 3000, ".paddingInlineStart-xaso8d8{padding-right:5px}");
_inject2(".paddingInlineEnd-x2vl965{padding-right:10px}", 3000, ".paddingInlineEnd-x2vl965{padding-left:10px}");
_inject2(".paddingTop-x1nn3v0j{padding-top:2px}", 4000);
_inject2(".paddingBottom-x1120s5i{padding-bottom:2px}", 4000);
_inject2(".paddingLeft-xnljgj5{padding-left:22px}", 4000);
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
