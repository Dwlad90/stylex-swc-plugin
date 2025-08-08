import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x1n25116{color:var(--4xs81a)}", 3000);
_inject2("@supports (hover: hover){.x1oeo35w.x1oeo35w{color:var(--b262sw)}}", 3030);
_inject2("@supports not (hover: hover){.x10db8fb.x10db8fb{color:var(--wu2acw)}}", 3030);
_inject2('@property --4xs81a { syntax: "*"; inherits: false; }', 0);
_inject2('@property --b262sw { syntax: "*"; inherits: false; }', 0);
_inject2('@property --wu2acw { syntax: "*"; inherits: false; }', 0);
export const styles = {
    root: (a, b, c)=>[
            {
                kMwMTN: (a != null ? "x1n25116" : a) + (b != null ? "x1oeo35w" : b) + (c != null ? "x10db8fb" : c),
                $$css: true
            },
            {
                "--4xs81a": a != null ? a : undefined,
                "--b262sw": b != null ? b : undefined,
                "--wu2acw": c != null ? c : undefined
            }
        ]
};
