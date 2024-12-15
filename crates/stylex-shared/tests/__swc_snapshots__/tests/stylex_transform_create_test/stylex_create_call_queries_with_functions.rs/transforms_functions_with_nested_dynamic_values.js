//__stylex_metadata_start__[{"class_name":"x1gykpug","style":{"rtl":null,"ltr":".x1gykpug:hover{background-color:red}"},"priority":3130},{"class_name":"xtyu0qe","style":{"rtl":null,"ltr":".xtyu0qe:hover{color:var(--1ijzsae)}"},"priority":3130},{"class_name":"--1ijzsae","style":{"rtl":null,"ltr":"@property --1ijzsae { syntax: \"*\"; inherits: false; initial-value: \"*\"; }"},"priority":0}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1gykpug:hover{background-color:red}", 3130);
_inject2(".xtyu0qe:hover{color:var(--1ijzsae)}", 3130);
_inject2('@property --1ijzsae { syntax: "*"; inherits: false; initial-value: "*"; }', 0);
export const styles = {
    default: (color)=>[
            {
                ":hover_backgroundColor": "x1gykpug",
                ":hover_color": color == null ? null : "xtyu0qe",
                $$css: true
            },
            {
                "--1ijzsae": color != null ? color : undefined
            }
        ]
};
