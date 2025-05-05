import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x74ai9j:hover{color:var(--1113oo7)}", 3130);
_inject2(".x19c4yy1:active{color:var(--hxnnmm)}", 3170);
_inject2(".x10peeyq:focus{color:var(--8tbbve)}", 3150);
_inject2(".x126ychx:nth-child(2n){color:purple}", 3060);
_inject2('@property --1113oo7 { syntax: "*"; }', 0);
_inject2('@property --hxnnmm { syntax: "*"; }', 0);
_inject2('@property --8tbbve { syntax: "*"; }', 0);
export const styles = {
    root: (hover, active, focus)=>[
            {
                kMwMTN: "x74ai9j x19c4yy1 x10peeyq x126ychx",
                $$css: true
            },
            {
                "--1113oo7": hover != null ? hover : undefined,
                "--hxnnmm": active != null ? active : undefined,
                "--8tbbve": focus != null ? focus : undefined
            }
        ]
};
