import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x1ttfofm:hover{background-color:var(--1e2mv7m)}", 3130);
_inject2(".x74ai9j:hover{color:var(--1113oo7)}", 3130);
_inject2('@property --1e2mv7m { syntax: "*"; }', 0);
_inject2('@property --1113oo7 { syntax: "*"; }', 0);
export const styles = {
    root: (color)=>[
            {
                kWkggS: color != null ? "x1ttfofm" : color,
                kMwMTN: color != null ? "x74ai9j" : color,
                $$css: true
            },
            {
                "--1e2mv7m": color != null ? color : undefined,
                "--1113oo7": color != null ? color : undefined
            }
        ]
};
