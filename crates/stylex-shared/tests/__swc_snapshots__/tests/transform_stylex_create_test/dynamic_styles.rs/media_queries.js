import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x11ymkkh{width:var(--x-1xmrurk)}", 4000);
_inject2("@media (min-width: 1000px){.x17gmrvw.x17gmrvw{width:var(--x-wm47pl)}}", 4200);
_inject2("@media (min-width: 2000px){.x1bai16n.x1bai16n{width:var(--x-1obb2yn)}}", 4200);
_inject2('@property --x-1xmrurk { syntax: "*"; inherits: false; }', 0);
_inject2('@property --x-wm47pl { syntax: "*"; inherits: false; }', 0);
_inject2('@property --x-1obb2yn { syntax: "*"; inherits: false; }', 0);
export const styles = {
    root: (a, b, c)=>[
            {
                kzqmXN: "x11ymkkh" + (b != null ? "x17gmrvw" : b) + (c != null ? "x1bai16n" : c),
                $$css: true
            },
            {
                "--x-1xmrurk": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)('color-mix(' + color + ', blue)'),
                "--x-wm47pl": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(b),
                "--x-1obb2yn": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(c)
            }
        ]
};
