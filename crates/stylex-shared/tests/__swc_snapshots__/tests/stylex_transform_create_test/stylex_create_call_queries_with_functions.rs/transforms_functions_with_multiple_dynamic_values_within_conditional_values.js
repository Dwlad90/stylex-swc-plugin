//__stylex_metadata_start__[{"class_name":"xrkmrrc","style":{"rtl":null,"ltr":".xrkmrrc{background-color:red}"},"priority":3000},{"class_name":"x1n25116","style":{"rtl":null,"ltr":".x1n25116{color:var(--4xs81a)}"},"priority":3000},{"class_name":"xtljkjt","style":{"rtl":null,"ltr":"@media (min-width: 1000px){.xtljkjt.xtljkjt:hover{color:green}}"},"priority":3330},{"class_name":"x1d4gdy3","style":{"rtl":null,"ltr":".x1d4gdy3:hover{color:var(--w5m4kq)}"},"priority":3130},{"class_name":"--4xs81a","style":{"rtl":null,"ltr":"@property --4xs81a { syntax: \"*\"; inherits: false; initial-value: \"*\"; }"},"priority":0},{"class_name":"--w5m4kq","style":{"rtl":null,"ltr":"@property --w5m4kq { syntax: \"*\"; inherits: false; initial-value: \"*\"; }"},"priority":0}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".x1n25116{color:var(--4xs81a)}", 3000);
_inject2("@media (min-width: 1000px){.xtljkjt.xtljkjt:hover{color:green}}", 3330);
_inject2(".x1d4gdy3:hover{color:var(--w5m4kq)}", 3130);
_inject2('@property --4xs81a { syntax: "*"; inherits: false; initial-value: "*"; }', 0);
_inject2('@property --w5m4kq { syntax: "*"; inherits: false; initial-value: "*"; }', 0);
export const styles = {
    default: (color)=>[
            {
                backgroundColor: "xrkmrrc",
                color: (color == null ? "" : "x1n25116 ") + "xtljkjt " + ('color-mix(' + color + ', blue)' == null ? "" : "x1d4gdy3"),
                $$css: true
            },
            {
                "--4xs81a": color != null ? color : undefined,
                "--w5m4kq": 'color-mix(' + color + ', blue)' != null ? 'color-mix(' + color + ', blue)' : undefined
            }
        ]
};
