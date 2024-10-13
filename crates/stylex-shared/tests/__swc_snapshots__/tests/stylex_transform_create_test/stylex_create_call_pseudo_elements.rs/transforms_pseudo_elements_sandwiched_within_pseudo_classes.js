//__stylex_metadata_start__[{"class_name":"x16oeupf","style":{"rtl":null,"ltr":".x16oeupf::before{color:red}"},"priority":8000},{"class_name":"xeb2lg0","style":{"rtl":null,"ltr":".xeb2lg0:hover::before{color:blue}"},"priority":8130},{"class_name":"x18ezmze","style":{"rtl":null,"ltr":".x18ezmze:hover::before:hover{color:green}"},"priority":8260},{"class_name":"xnj3kot","style":{"rtl":null,"ltr":".xnj3kot:hover::before:active{color:purple}"},"priority":8300}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x16oeupf::before{color:red}", 8000);
_inject2(".xeb2lg0:hover::before{color:blue}", 8130);
_inject2(".x18ezmze:hover::before:hover{color:green}", 8260);
_inject2(".xnj3kot:hover::before:active{color:purple}", 8300);
export const styles = {
    foo: {
        "::before_color": "x16oeupf",
        ":hover_::before_color": "xeb2lg0 x18ezmze xnj3kot",
        $$css: true
    }
};
