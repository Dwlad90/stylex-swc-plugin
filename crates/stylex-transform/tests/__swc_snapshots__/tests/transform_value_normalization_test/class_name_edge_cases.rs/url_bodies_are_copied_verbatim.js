import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: '.x14wdgs6{background-image:url("a;b{c}d: e /* f */")}',
    priority: 3000
});
_inject2({
    ltr: ".xb71sbn{background-image:url(image.png?a=1&b=2)}",
    priority: 3000
});
_inject2({
    ltr: ".x19xh8vt{background-image:url(\"data:image/svg+xml;charset=utf8,%3Csvg xmlns='http://www.w3.org/2000/svg'%3E%3C/svg%3E\")}",
    priority: 3000
});
export const styles = {
    cssSyntaxInBody: {
        kKwaWg: "x14wdgs6",
        $$css: true
    },
    unquotedWithQuery: {
        kKwaWg: "xb71sbn",
        $$css: true
    },
    dataUri: {
        kKwaWg: "x19xh8vt",
        $$css: true
    }
};
