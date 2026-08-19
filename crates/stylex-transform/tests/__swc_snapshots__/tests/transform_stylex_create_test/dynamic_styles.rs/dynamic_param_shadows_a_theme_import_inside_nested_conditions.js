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
    ltr: ".x141uv47:hover{z-index:var(--x-1ua3n7y)}",
    priority: 3130
});
_inject2({
    ltr: "@media (min-width: 600px){.xrxvmrx.xrxvmrx:hover{z-index:var(--x-wb6suc)}}",
    priority: 3330
});
_inject2({
    ltr: "@media (min-width: 600px){.xsaaf0t.xsaaf0t:focus:hover{z-index:var(--x-frh1i3)}}",
    priority: 3480
});
_inject2({
    ltr: '@property --x-gsepj1 { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-1ua3n7y { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-wb6suc { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-frh1i3 { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    wrapper: {
        kY2c9j: "x145lhke",
        $$css: true
    },
    dyn: (zIndex)=>[
            {
                kY2c9j: (zIndex != null ? "xkrcnwa " : zIndex) + (zIndex != null ? "x141uv47 " : zIndex) + (zIndex != null ? "xrxvmrx " : zIndex) + (zIndex != null ? "xsaaf0t" : zIndex),
                $$css: true
            },
            {
                "--x-gsepj1": zIndex != null ? zIndex : undefined,
                "--x-1ua3n7y": zIndex != null ? zIndex : undefined,
                "--x-wb6suc": zIndex != null ? zIndex : undefined,
                "--x-frh1i3": zIndex != null ? zIndex : undefined
            }
        ]
};
