import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "zIndex.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { zIndex } from 'zIndex.stylex.js';
const _temp = {
    kY2c9j: "x1nnvxg8",
    $$css: true
};
_inject2({
    ltr: ".x1nnvxg8{z-index:var(--x1t53vvn);z-index:1}",
    priority: 3000
});
_inject2({
    ltr: ".x16ye13r{height:var(--x-height)}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-height { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    dyn: (h)=>[
            _temp,
            {
                kZKoxP: h != null ? "x16ye13r" : h,
                $$css: true
            },
            {
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(h)
            }
        ]
};
