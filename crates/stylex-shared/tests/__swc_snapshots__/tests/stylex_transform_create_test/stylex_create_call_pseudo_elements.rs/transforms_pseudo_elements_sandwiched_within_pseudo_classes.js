import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x16oeupf::before{color:red}", 8000);
_inject2(".xzzpreb:hover::before{color:blue}", 8130);
_inject2(".x1gobd9t:hover::before:hover{color:green}", 8260);
_inject2(".xs8jp5:hover::before:active{color:purple}", 8300);
export const styles = {
    foo: {
        "::before_color": "x16oeupf",
        ":hover_::before_color": "xzzpreb x1gobd9t xs8jp5",
        $$css: true
    }
};
