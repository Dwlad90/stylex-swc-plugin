import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ":root, .xir4if5{--xw1y7mb:red;}",
    priority: 0.1
});
_inject2({
    ltr: "@media (prefers-color-scheme: dark){:root, .xir4if5{--xw1y7mb:blue;}}",
    priority: 0.2
});
export const colors = {
    textMuted: "var(--xw1y7mb)",
    __varGroupHash__: "xir4if5"
};
