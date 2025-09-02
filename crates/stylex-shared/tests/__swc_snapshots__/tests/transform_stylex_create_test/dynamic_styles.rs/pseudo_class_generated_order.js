import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x1qvlgnj:hover{color:var(--x-1113oo7)}", 3130);
_inject2(".xx746rz:active{color:var(--x-hxnnmm)}", 3170);
_inject2(".x152n5rj:focus{color:var(--x-8tbbve)}", 3150);
_inject2(".x126ychx:nth-child(2n){color:purple}", 3060);
_inject2('@property --x-1113oo7 { syntax: "*"; }', 0);
_inject2('@property --x-hxnnmm { syntax: "*"; }', 0);
_inject2('@property --x-8tbbve { syntax: "*"; }', 0);
export const styles = {
    root: (hover, active, focus)=>[
            {
                kMwMTN: (hover != null ? "x1qvlgnj" : hover) + (active != null ? "xx746rz" : active) + (focus != null ? "x152n5rj" : focus) + "x126ychx",
                $$css: true
            },
            {
                "--x-1113oo7": hover != null ? hover : undefined,
                "--x-hxnnmm": active != null ? active : undefined,
                "--x-8tbbve": focus != null ? focus : undefined
            }
        ]
};
