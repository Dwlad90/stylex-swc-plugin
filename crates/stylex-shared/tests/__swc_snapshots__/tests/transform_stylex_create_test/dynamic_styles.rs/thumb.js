import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x18fgbt0::-webkit-slider-thumb, .x18fgbt0::-moz-range-thumb, .x18fgbt0::-ms-thumb{width:var(--x-msahdu)}", 9000);
_inject2('@property --x-msahdu { syntax: "*"; }', 0);
export const styles = {
    foo: (width)=>[
            {
                k8pbKx: width != null ? "x18fgbt0" : width,
                $$css: true
            },
            {
                "--x-msahdu": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width)
            }
        ]
};
