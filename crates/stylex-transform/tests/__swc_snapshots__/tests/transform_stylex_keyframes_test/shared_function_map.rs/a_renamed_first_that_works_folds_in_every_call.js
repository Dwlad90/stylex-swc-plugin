import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import { keyframes, positionTry, firstThatWorks as fallback } from '@stylexjs/stylex';
_inject2({
    ltr: "@keyframes x18duhyb-B{from{color:var(--a,red);}to{color:blue;}}",
    priority: 0
});
export const fadeIn = "x18duhyb-B";
_inject2({
    ltr: "@position-try --xg5t6hz {position-anchor:position-anchor;position-anchor:--anchor;top:top;top:var(--top,10px);}",
    priority: 0,
    rtl: "@position-try --xg5t6hz {position-anchor:--anchor;top:var(--top,10px);}"
});
export const anchor = "--xg5t6hz";
