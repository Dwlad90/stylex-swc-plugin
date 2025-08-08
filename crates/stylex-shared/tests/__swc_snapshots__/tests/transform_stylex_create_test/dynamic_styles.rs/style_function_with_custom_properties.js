import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x15mgraa{--background-color:var(----background-color)}", 1);
_inject2(".x1qph05k{--otherColor:var(----otherColor)}", 1);
_inject2('@property ----background-color { syntax: "*"; inherits: false; }', 0);
_inject2('@property ----otherColor { syntax: "*"; inherits: false; }', 0);
export const styles = {
    root: (bgColor, otherColor)=>[
            {
                "--background-color": bgColor != null ? "x15mgraa" : bgColor,
                "--otherColor": otherColor != null ? "x1qph05k" : otherColor,
                $$css: true
            },
            {
                "----background-color": bgColor != null ? bgColor : undefined,
                "----otherColor": otherColor != null ? otherColor : undefined
            }
        ]
};
