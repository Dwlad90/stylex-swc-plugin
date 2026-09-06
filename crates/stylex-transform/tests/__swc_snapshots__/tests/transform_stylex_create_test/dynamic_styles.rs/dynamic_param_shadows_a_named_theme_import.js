import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "zIndex.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { zIndex } from 'zIndex.stylex.js';
_inject2({
    ltr: ".x145lhke{z-index:var(--x1t53vvn)}",
    priority: 3000
});
_inject2({
    ltr: ".xr3buco{z-index:var(--x-zIndex)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-zIndex { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    wrapper: {
        kY2c9j: "x145lhke",
        $$css: true
    },
    zIndex: (zIndex)=>[
            {
                kY2c9j: zIndex != null ? "xr3buco" : zIndex,
                $$css: true
            },
            {
                "--x-zIndex": zIndex != null ? zIndex : undefined
            }
        ]
};
