import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".xfx01vb{color:var(--color)}", 3000);
_inject2(".x1mqxbix{color:black}", 3000);
_inject2('@property --color { syntax: "*"; inherits: false; }', 0);
export const styles = {
    default: (color)=>[
            {
                kWkggS: "xrkmrrc",
                kMwMTN: "xfx01vb",
                $$css: true
            },
            {
                "--color": color != null ? color : undefined
            }
        ],
    mono: {
        kMwMTN: "x1mqxbix",
        $$css: true
    }
};
