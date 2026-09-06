import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "zIndex.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { zIndex } from 'zIndex.stylex.js';
const _temp = {
    kKO1pi: "x13ok0bc",
    $$css: true
};
_inject2({
    ltr: ".xr3buco{z-index:var(--x-zIndex)}",
    priority: 3000
});
_inject2({
    ltr: ".x13ok0bc:hover{z-index:1}",
    priority: 3130
});
_inject2({
    ltr: ".x145lhke{z-index:var(--x1t53vvn)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-zIndex { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    dyn: (zIndex)=>[
            _temp,
            {
                kY2c9j: zIndex != null ? "xr3buco" : zIndex,
                $$css: true
            },
            {
                "--x-zIndex": zIndex != null ? zIndex : undefined
            }
        ],
    raised: {
        kY2c9j: "x145lhke",
        $$css: true
    }
};
