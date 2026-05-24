import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ":root, .x1edtgoo{--x1lcj7dl:#2563eb;--x6sbfcu:#1d4ed8;}",
    priority: 0.1
});
_inject2({
    ltr: "@media (prefers-color-scheme: dark){:root, .x1edtgoo{--x1lcj7dl:#3b82f6;--x6sbfcu:#2563eb;}}",
    priority: 0.2
});
export const tokens = {
    button: {
        primary: {
            background: {
                default: "var(--x1lcj7dl)",
                hovered: "var(--x6sbfcu)"
            }
        }
    },
    __varGroupHash__: "x1edtgoo"
};
