import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".xfx01vb{color:var(--color)}", 3000);
_inject2('@property --color { syntax: "*"; inherits: false; initial-value: "*"; }', 0);
export const styles = {
    default: (color)=>[
            {
                backgroundColor: "xrkmrrc",
                color: color == null ? null : "xfx01vb",
                $$css: true
            },
            {
                "--color": color != null ? color : undefined
            }
        ]
};
