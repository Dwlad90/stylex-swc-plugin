import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ":root, .x1edtgoo{--xi1hctn:blue;}",
    priority: 0.1
});
_inject2({
    ltr: "@media (prefers-color-scheme: dark){:root, .x1edtgoo{--xi1hctn:lightblue;}}",
    priority: 0.2
});
export const tokens = {
    button: {
        color: "var(--xi1hctn)"
    },
    __varGroupHash__: "x1edtgoo"
};
