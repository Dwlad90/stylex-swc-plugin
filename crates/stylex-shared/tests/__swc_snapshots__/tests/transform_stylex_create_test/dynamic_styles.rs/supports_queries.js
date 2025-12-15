import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x3d248p{color:var(--x-4xs81a)}",
    priority: 3000
});
_inject2({
    ltr: "@supports (hover: hover){.x1iuwwch.x1iuwwch{color:var(--x-b262sw)}}",
    priority: 3030
});
_inject2({
    ltr: "@supports not (hover: hover){.x5268pl.x5268pl{color:var(--x-wu2acw)}}",
    priority: 3030
});
_inject2({
    ltr: '@property --x-4xs81a { syntax: "*"; inherits: false; }',
    priority: 0
});
_inject2({
    ltr: '@property --x-b262sw { syntax: "*"; inherits: false; }',
    priority: 0
});
_inject2({
    ltr: '@property --x-wu2acw { syntax: "*"; inherits: false; }',
    priority: 0
});
export const styles = {
    root: (a, b, c)=>[
            {
                kMwMTN: (a != null ? "x3d248p " : a) + (b != null ? "x1iuwwch " : b) + (c != null ? "x5268pl" : c),
                $$css: true
            },
            {
                "--x-4xs81a": a != null ? a : undefined,
                "--x-b262sw": b != null ? b : undefined,
                "--x-wu2acw": c != null ? c : undefined
            }
        ]
};
