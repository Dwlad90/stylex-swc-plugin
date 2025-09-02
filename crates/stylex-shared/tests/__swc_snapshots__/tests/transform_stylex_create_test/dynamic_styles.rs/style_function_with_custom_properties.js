import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".xwn82o0{--background-color:var(--x---background-color)}", 1);
_inject2(".xp3hsad{--otherColor:var(--x---otherColor)}", 1);
_inject2('@property --x---background-color { syntax: "*"; inherits: false; }', 0);
_inject2('@property --x---otherColor { syntax: "*"; inherits: false; }', 0);
export const styles = {
    root: (bgColor, otherColor)=>[
            {
                "--background-color": bgColor != null ? "xwn82o0" : bgColor,
                "--otherColor": otherColor != null ? "xp3hsad" : otherColor,
                $$css: true
            },
            {
                "--x---background-color": bgColor != null ? bgColor : undefined,
                "--x---otherColor": otherColor != null ? otherColor : undefined
            }
        ]
};
