import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: '@property --xw1y7mb { syntax: "<color>"; inherits: true; initial-value: red }',
    priority: 0
});
_inject2({
    ltr: ":root, .xir4if5{--x1gi4lgk:black;--xw1y7mb:red;}",
    priority: 0.1
});
export const colors = {
    text: "var(--x1gi4lgk)",
    textMuted: "var(--xw1y7mb)",
    __varGroupHash__: "xir4if5"
};
