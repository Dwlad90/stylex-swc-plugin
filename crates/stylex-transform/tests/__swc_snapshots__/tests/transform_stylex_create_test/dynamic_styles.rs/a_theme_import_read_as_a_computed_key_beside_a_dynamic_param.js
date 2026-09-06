import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "vars.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { vars } from 'vars.stylex.js';
_inject2({
    ltr: ".x16ofv5h{--xwx8imx:red}",
    priority: 1
});
_inject2({
    ltr: ".x194mhi7{--xwx8imx:var(--x---xwx8imx)}",
    priority: 1
});
_inject2({
    ltr: '@property --x---xwx8imx { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    wrapper: {
        "--xwx8imx": "x16ofv5h",
        $$css: true
    },
    dyn: (color)=>[
            {
                "--xwx8imx": color != null ? "x194mhi7" : color,
                $$css: true
            },
            {
                "--x---xwx8imx": color != null ? color : undefined
            }
        ]
};
