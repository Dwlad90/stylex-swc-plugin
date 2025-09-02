import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x3d248p{color:var(--x-4xs81a)}", 3000);
_inject2("@supports (hover: hover){.x1iuwwch.x1iuwwch{color:var(--x-b262sw)}}", 3030);
_inject2("@supports not (hover: hover){.x5268pl.x5268pl{color:var(--x-wu2acw)}}", 3030);
_inject2('@property --x-4xs81a { syntax: "*"; inherits: false; }', 0);
_inject2('@property --x-b262sw { syntax: "*"; inherits: false; }', 0);
_inject2('@property --x-wu2acw { syntax: "*"; inherits: false; }', 0);
export const styles = {
    root: (a, b, c)=>[
            {
                kMwMTN: (a != null ? "x3d248p" : a) + (b != null ? "x1iuwwch" : b) + (c != null ? "x5268pl" : c),
                $$css: true
            },
            {
                "--x-4xs81a": a != null ? a : undefined,
                "--x-b262sw": b != null ? b : undefined,
                "--x-wu2acw": c != null ? c : undefined
            }
        ]
};
