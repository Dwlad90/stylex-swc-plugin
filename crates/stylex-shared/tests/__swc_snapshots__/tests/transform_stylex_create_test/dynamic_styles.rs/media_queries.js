import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x1svif2g{width:var(--1xmrurk)}", 4000);
_inject2("@media (min-width: 1000px){.x1a6pj3q.x1a6pj3q{width:var(--wm47pl)}}", 4200);
_inject2("@media (min-width: 2000px){.xf0apgt.xf0apgt{width:var(--1obb2yn)}}", 4200);
_inject2('@property --1xmrurk { syntax: "*"; inherits: false; }', 0);
_inject2('@property --wm47pl { syntax: "*"; inherits: false; }', 0);
_inject2('@property --1obb2yn { syntax: "*"; inherits: false; }', 0);
export const styles = {
    root: (a, b, c)=>[
            {
                kzqmXN: "x1svif2g" + (b != null ? "x1a6pj3q" : b) + (c != null ? "xf0apgt" : c),
                $$css: true
            },
            {
                "--1xmrurk": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)('color-mix(' + color + ', blue)'),
                "--wm47pl": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(b),
                "--1obb2yn": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(c)
            }
        ]
};
