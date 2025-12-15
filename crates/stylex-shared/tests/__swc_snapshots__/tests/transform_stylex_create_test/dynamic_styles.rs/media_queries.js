import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x11ymkkh{width:var(--x-1xmrurk)}",
    priority: 4000
});
_inject2({
    ltr: "@media (min-width: 1000px) and (max-width: 1999.99px){.x38mdg9.x38mdg9{width:var(--x-wm47pl)}}",
    priority: 4200
});
_inject2({
    ltr: "@media (min-width: 2000px){.x1bai16n.x1bai16n{width:var(--x-1obb2yn)}}",
    priority: 4200
});
_inject2({
    ltr: '@property --x-1xmrurk { syntax: "*"; inherits: false; }',
    priority: 0
});
_inject2({
    ltr: '@property --x-wm47pl { syntax: "*"; inherits: false; }',
    priority: 0
});
_inject2({
    ltr: '@property --x-1obb2yn { syntax: "*"; inherits: false; }',
    priority: 0
});
export const styles = {
    root: (a, b, c)=>[
            {
                kzqmXN: "x11ymkkh " + "x38mdg9 " + (c != null ? "x1bai16n" : c),
                $$css: true
            },
            {
                "--x-1xmrurk": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)('color-mix(' + color + ', blue)'),
                "--x-wm47pl": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(b),
                "--x-1obb2yn": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(c)
            }
        ]
};
