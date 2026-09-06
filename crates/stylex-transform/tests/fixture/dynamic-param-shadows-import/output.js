import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./vars/zIndex.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { zIndex } from './vars/zIndex.stylex.js';
_inject2({
    ltr: ".zIndex-x1bsllxr{z-index:var(--_10-x19xkwqv)}",
    priority: 3000
});
_inject2({
    ltr: ".zIndex-xr3buco{z-index:var(--x-zIndex)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-zIndex { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    wrapper: {
        zIndex: "zIndex-x1bsllxr",
        $$css: "tests/fixture/dynamic-param-shadows-import/input.stylex.js:4"
    },
    zIndex: (zIndex)=>[
            {
                zIndex: zIndex != null ? "zIndex-xr3buco" : zIndex,
                $$css: "tests/fixture/dynamic-param-shadows-import/input.stylex.js:7"
            },
            {
                "--x-zIndex": zIndex != null ? zIndex : undefined
            }
        ]
};
