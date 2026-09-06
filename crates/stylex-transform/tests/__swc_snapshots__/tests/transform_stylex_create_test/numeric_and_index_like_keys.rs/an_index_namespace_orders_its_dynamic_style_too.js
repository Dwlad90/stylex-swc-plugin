import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x5lhr3w{width:var(--x-width)}",
    priority: 4000
});
_inject2({
    ltr: ".x14rh7hd{color:var(--x-color)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-width { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-color { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    "0": (w)=>[
            {
                kzqmXN: w != null ? "x5lhr3w" : w,
                $$css: true
            },
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(w)
            }
        ],
    named: (c)=>[
            {
                kMwMTN: c != null ? "x14rh7hd" : c,
                $$css: true
            },
            {
                "--x-color": c != null ? c : undefined
            }
        ]
};
