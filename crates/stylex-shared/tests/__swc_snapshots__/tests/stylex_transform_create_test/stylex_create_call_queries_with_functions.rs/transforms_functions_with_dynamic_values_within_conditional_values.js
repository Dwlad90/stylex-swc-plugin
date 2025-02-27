import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".x1n25116{color:var(--4xs81a)}", 3000);
_inject2("@media (min-width: 1000px){.xtljkjt.xtljkjt:hover{color:green}}", 3330);
_inject2(".x17z2mba:hover{color:blue}", 3130);
_inject2('@property --4xs81a { syntax: "*"; inherits: false; }', 0);
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
