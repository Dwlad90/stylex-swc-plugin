import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
import { zIndex } from 'zIndex.stylex.js';
const _temp = {
    kMv6JI: "xfib3io",
    $$css: true
};
_inject2({
    ltr: '.xfib3io{font-family:"My\\ Font"}',
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
    dyn: (zIndex)=>[
            _temp,
            {
                kY2c9j: zIndex != null ? "xr3buco" : zIndex,
                $$css: true
            },
            {
                "--x-zIndex": zIndex != null ? zIndex : undefined
            }
        ]
};
