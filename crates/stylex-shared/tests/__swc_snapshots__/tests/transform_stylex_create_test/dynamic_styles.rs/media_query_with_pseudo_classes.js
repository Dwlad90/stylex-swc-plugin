import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".xww4jgc{font-size:var(--x-19zvkyr)}",
    priority: 3000
});
_inject2({
    ltr: "@media (min-width: 800px){.xfqys7t.xfqys7t{font-size:var(--x-1xajcet)}}",
    priority: 3200
});
_inject2({
    ltr: "@media (min-width: 800px){.x13w7uki.x13w7uki:hover{font-size:var(--x-ke45ok)}}",
    priority: 3330
});
_inject2({
    ltr: '@property --x-19zvkyr { syntax: "*"; inherits: false; }',
    priority: 0
});
_inject2({
    ltr: '@property --x-1xajcet { syntax: "*"; inherits: false; }',
    priority: 0
});
_inject2({
    ltr: '@property --x-ke45ok { syntax: "*"; }',
    priority: 0
});
export const styles = {
    root: (a, b, c)=>[
            {
                kGuDYH: (a != null ? "xww4jgc " : a) + (b != null ? "xfqys7t " : b) + (c != null ? "x13w7uki" : c),
                $$css: true
            },
            {
                "--x-19zvkyr": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(a),
                "--x-1xajcet": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(b),
                "--x-ke45ok": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(c)
            }
        ]
};
