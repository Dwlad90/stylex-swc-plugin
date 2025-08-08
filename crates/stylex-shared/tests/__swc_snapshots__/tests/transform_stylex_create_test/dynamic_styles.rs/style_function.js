import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".xfx01vb{color:var(--color)}", 3000);
_inject2('@property --color { syntax: "*"; inherits: false; }', 0);
export const styles = {
    root: (color)=>[
            {
                kWkggS: "xrkmrrc",
                kMwMTN: color != null ? "xfx01vb" : color,
                $$css: true
            },
            {
                "--color": color != null ? color : undefined
            }
        ]
};
