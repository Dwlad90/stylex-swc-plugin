import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "zIndex.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { zIndex } from 'zIndex.stylex.js';
const _temp = {
    kZKoxP: "x5ozmkz",
    $$css: true
};
_inject2({
    ltr: ".x145lhke{z-index:var(--x1t53vvn)}",
    priority: 3000
});
_inject2({
    ltr: ".xr3buco{z-index:var(--x-zIndex)}",
    priority: 3000
});
_inject2({
    ltr: ".x5ozmkz{height:1px;height:2px}",
    priority: 4000
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
    dyn: (zIndex)=>[
            _temp,
            {
                kY2c9j: [
                    zIndex,
                    1
                ] != null ? "xr3buco" : [
                    zIndex,
                    1
                ],
                $$css: true
            },
            {
                "--x-zIndex": [
                    zIndex,
                    1
                ] != null ? [
                    zIndex,
                    1
                ] : undefined
            }
        ]
};
