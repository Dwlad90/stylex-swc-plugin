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
