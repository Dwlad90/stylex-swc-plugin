//__stylex_metadata_start__[{"class_name":"xrkmrrc","style":{"rtl":null,"ltr":".xrkmrrc{background-color:red}"},"priority":3000},{"class_name":"x1n25116","style":{"rtl":null,"ltr":".x1n25116{color:var(--4xs81a)}"},"priority":3000},{"class_name":"xtljkjt","style":{"rtl":null,"ltr":"@media (min-width: 1000px){.xtljkjt.xtljkjt:hover{color:green}}"},"priority":3330},{"class_name":"x17z2mba","style":{"rtl":null,"ltr":".x17z2mba:hover{color:blue}"},"priority":3130}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".x1n25116{color:var(--4xs81a)}", 3000);
_inject2("@media (min-width: 1000px){.xtljkjt.xtljkjt:hover{color:green}}", 3330);
_inject2(".x17z2mba:hover{color:blue}", 3130);
export const styles = {
    default: (color)=>[
            {
                backgroundColor: "xrkmrrc",
                color: (color == null ? "" : "x1n25116 ") + "xtljkjt " + "x17z2mba",
                $$css: true
            },
            {
                "--4xs81a": color != null ? color : undefined
            }
        ]
};
