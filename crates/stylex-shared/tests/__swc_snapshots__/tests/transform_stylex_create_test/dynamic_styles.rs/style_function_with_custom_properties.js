import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".xwn82o0{--background-color:var(--x---background-color)}",
    priority: 1
});
_inject2({
    ltr: ".xp3hsad{--otherColor:var(--x---otherColor)}",
    priority: 1
});
_inject2({
    ltr: '@property --x---background-color { syntax: "*"; inherits: false; }',
    priority: 0
});
_inject2({
    ltr: '@property --x---otherColor { syntax: "*"; inherits: false; }',
    priority: 0
});
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
