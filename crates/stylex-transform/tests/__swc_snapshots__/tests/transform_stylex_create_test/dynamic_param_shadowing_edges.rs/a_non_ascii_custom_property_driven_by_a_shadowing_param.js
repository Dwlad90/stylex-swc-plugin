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
    ltr: ".x1xmayp6{--dépth:var(--x---dépth)}",
    priority: 1
});
_inject2({
    ltr: '@property --x---dépth { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    wrapper: {
        kY2c9j: "x145lhke",
        $$css: true
    },
    dyn: (zIndex)=>[
            {
                "--dépth": zIndex != null ? "x1xmayp6" : zIndex,
                $$css: true
            },
            {
                "--x---dépth": zIndex != null ? zIndex : undefined
            }
        ]
};
