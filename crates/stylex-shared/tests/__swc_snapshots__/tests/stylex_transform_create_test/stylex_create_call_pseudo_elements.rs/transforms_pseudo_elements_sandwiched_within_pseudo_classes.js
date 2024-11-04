//__stylex_metadata_start__[{"class_name":"x16oeupf","style":{"rtl":null,"ltr":".x16oeupf::before{color:red}"},"priority":8000},{"class_name":"xzzpreb","style":{"rtl":null,"ltr":".xzzpreb:hover::before{color:blue}"},"priority":8130},{"class_name":"x1gobd9t","style":{"rtl":null,"ltr":".x1gobd9t:hover::before:hover{color:green}"},"priority":8260},{"class_name":"xs8jp5","style":{"rtl":null,"ltr":".xs8jp5:hover::before:active{color:purple}"},"priority":8300}]__stylex_metadata_end__
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
