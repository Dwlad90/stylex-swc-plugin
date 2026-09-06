import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "vars.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { "color-lg" as c } from 'vars.stylex.js';
_inject2({
    ltr: ".xqnu1qn{color:var(--x1vktwfk)}",
    priority: 3000
});
_inject2({
    ltr: ".x14rh7hd{color:var(--x-color)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-color { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    w: {
        kMwMTN: "xqnu1qn",
        $$css: true
    },
    dyn: (c)=>[
            {
                kMwMTN: c != null ? "x14rh7hd" : c,
                $$css: true
            },
            {
                "--x-color": c != null ? c : undefined
            }
        ]
};
