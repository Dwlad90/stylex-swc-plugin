//__stylex_metadata_start__[{"class_name":"x15mgraa","style":{"rtl":null,"ltr":".x15mgraa{--background-color:var(----background-color)}"},"priority":1},{"class_name":"----background-color","style":{"rtl":null,"ltr":"@property ----background-color { syntax: \"*\"; inherits: false; initial-value: \"*\"; }"},"priority":0}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x15mgraa{--background-color:var(----background-color)}", 1);
_inject2('@property ----background-color { syntax: "*"; inherits: false; initial-value: "*"; }', 0);
export const styles = {
    default: (bgColor)=>[
            {
                "--background-color": bgColor == null ? null : "x15mgraa",
                $$css: true
            },
            {
                "----background-color": bgColor != null ? bgColor : undefined
            }
        ]
};
