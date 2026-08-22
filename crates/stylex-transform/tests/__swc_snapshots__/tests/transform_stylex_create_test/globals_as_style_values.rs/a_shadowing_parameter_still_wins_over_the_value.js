import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x16ye13r{height:var(--x-height)}",
    priority: 4000
});
_inject2({
    ltr: ".x5lhr3w{width:var(--x-width)}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-height { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-width { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    shadowed: (NaN)=>[
            {
                kZKoxP: NaN != null ? "x16ye13r" : NaN,
                $$css: true
            },
            {
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(NaN)
            }
        ],
    alsoShadowed: (Infinity)=>[
            {
                kzqmXN: Infinity != null ? "x5lhr3w" : Infinity,
                $$css: true
            },
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(Infinity)
            }
        ]
};
