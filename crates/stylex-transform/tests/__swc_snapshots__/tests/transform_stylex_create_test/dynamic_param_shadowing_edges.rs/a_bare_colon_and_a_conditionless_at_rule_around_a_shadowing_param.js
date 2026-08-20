import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
import { zIndex } from 'zIndex.stylex.js';
_inject2({
    ltr: ".xkrcnwa{z-index:var(--x-gsepj1)}",
    priority: 3000
});
_inject2({
    ltr: ".x1xt2tkc:{z-index:var(--x-9e86q5)}",
    priority: 3040
});
_inject2({
    ltr: "@media{.xhjt48w.xhjt48w{z-index:var(--x-1qzloiu)}}",
    priority: 3200
});
_inject2({
    ltr: '@property --x-gsepj1 { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-9e86q5 { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-1qzloiu { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    bare: (zIndex)=>[
            {
                kY2c9j: (zIndex != null ? "xkrcnwa " : zIndex) + (zIndex != null ? "x1xt2tkc" : zIndex),
                $$css: true
            },
            {
                "--x-gsepj1": zIndex != null ? zIndex : undefined,
                "--x-9e86q5": zIndex != null ? zIndex : undefined
            }
        ],
    conditionless: (zIndex)=>[
            {
                kY2c9j: (zIndex != null ? "xkrcnwa " : zIndex) + (zIndex != null ? "xhjt48w" : zIndex),
                $$css: true
            },
            {
                "--x-gsepj1": zIndex != null ? zIndex : undefined,
                "--x-1qzloiu": zIndex != null ? zIndex : undefined
            }
        ]
};
