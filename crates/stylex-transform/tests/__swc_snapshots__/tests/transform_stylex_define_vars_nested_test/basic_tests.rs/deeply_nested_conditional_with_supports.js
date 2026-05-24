import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ":root, .x1edtgoo{--xegmn9y:blue;}",
    priority: 0.1
});
_inject2({
    ltr: "@media (prefers-color-scheme: dark){:root, .x1edtgoo{--xegmn9y:lightblue;}}",
    priority: 0.2
});
_inject2({
    ltr: "@supports (color: oklch(0 0 0)){@media (prefers-color-scheme: dark){:root, .x1edtgoo{--xegmn9y:oklch(0.7 -0.3 -0.4);}}}",
    priority: 0.3
});
export const tokens = {
    color: {
        primary: "var(--xegmn9y)"
    },
    __varGroupHash__: "x1edtgoo"
};
