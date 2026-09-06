import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "spacing.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { spacing as ünïcödé } from 'spacing.stylex.js';
_inject2({
    ltr: ".xookrjq{padding:var(--x57iyp3)}",
    priority: 1000
});
_inject2({
    ltr: ".x1fozly0{padding:var(--x-padding)}",
    priority: 1000
});
_inject2({
    ltr: '@property --x-padding { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    plain: {
        kmVPX3: "xookrjq",
        $$css: true
    },
    escaped: (ünïcödé)=>[
            {
                kmVPX3: ünïcödé != null ? "x1fozly0" : ünïcödé,
                $$css: true
            },
            {
                "--x-padding": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(ünïcödé)
            }
        ]
};
