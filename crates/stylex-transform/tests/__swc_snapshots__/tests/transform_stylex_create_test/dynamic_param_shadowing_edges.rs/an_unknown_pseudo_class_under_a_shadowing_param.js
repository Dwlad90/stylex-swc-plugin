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
    ltr: ".xkrcnwa{z-index:var(--x-gsepj1)}",
    priority: 3000
});
_inject2({
    ltr: ".x1ytmd8f:hoverr{z-index:var(--x-1ycp2ll)}",
    priority: 3040
});
_inject2({
    ltr: '@property --x-gsepj1 { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-1ycp2ll { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    wrapper: {
        kY2c9j: "x145lhke",
        $$css: true
    },
    dyn: (zIndex)=>[
            {
                kY2c9j: (zIndex != null ? "xkrcnwa " : zIndex) + (zIndex != null ? "x1ytmd8f" : zIndex),
                $$css: true
            },
            {
                "--x-gsepj1": zIndex != null ? zIndex : undefined,
                "--x-1ycp2ll": zIndex != null ? zIndex : undefined
            }
        ]
};
