import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".xfx01vb{color:var(--color)}", 3000);
_inject2(".x1mqxbix{color:black}", 3000);
_inject2('@property --color { syntax: "*"; inherits: false; }', 0);
export const styles = {
    one: (color)=>[
            {
                kMwMTN: color != null ? "xfx01vb" : color,
                $$css: true
            },
            {
                "--color": color != null ? color : undefined
            }
        ],
    two: {
        kMwMTN: "x1mqxbix",
        $$css: true
    }
};
