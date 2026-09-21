import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./vars/zIndex.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { zIndex } from './vars/zIndex.stylex.js';
const _temp = {
    "input__styles.zIndex": "input__styles.zIndex",
    $$css: "tests/fixture/dynamic-param-shadows-import/input.stylex.js:6"
};
_inject2({
    ltr: ".x25bfn{z-index:var(--x19xkwqv)}",
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
        "input__styles.wrapper": "input__styles.wrapper",
        zIndex: "x25bfn",
        $$css: "tests/fixture/dynamic-param-shadows-import/input.stylex.js:5"
    },
    zIndex: (zIndex)=>[
            _temp,
            {
                zIndex: zIndex != null ? "xr3buco" : zIndex,
                $$css: "tests/fixture/dynamic-param-shadows-import/input.stylex.js:6"
            },
            {
                "--x-zIndex": zIndex != null ? zIndex : undefined
            }
        ]
};
