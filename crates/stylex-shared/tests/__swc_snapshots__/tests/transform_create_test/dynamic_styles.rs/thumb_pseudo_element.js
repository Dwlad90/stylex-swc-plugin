import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x3j4sww::-webkit-slider-thumb, .x3j4sww::-moz-range-thumb, .x3j4sww::-ms-thumb{width:var(--msahdu)}", 9000);
_inject2('@property --msahdu { syntax: "*"; inherits: false; }', 0);
export const styles = {
    foo: (width)=>[
            {
                k8pbKx: "x3j4sww",
                $$css: true
            },
            {
                "--msahdu": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width)
            }
        ]
};
