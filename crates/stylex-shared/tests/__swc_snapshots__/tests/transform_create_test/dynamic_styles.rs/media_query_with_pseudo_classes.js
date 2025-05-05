import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x1cfcgx7{font-size:var(--19zvkyr)}", 3000);
_inject2("@media (min-width: 800px){.x956mei.x956mei{font-size:var(--1xajcet)}}", 3200);
_inject2("@media (min-width: 800px){.xarp7f8.xarp7f8:hover{font-size:var(--ke45ok)}}", 3330);
_inject2('@property --19zvkyr { syntax: "*"; inherits: false; }', 0);
_inject2('@property --1xajcet { syntax: "*"; inherits: false; }', 0);
_inject2('@property --ke45ok { syntax: "*"; }', 0);
export const styles = {
    root: (a, b, c)=>[
            {
                kGuDYH: "x1cfcgx7 x956mei xarp7f8",
                $$css: true
            },
            {
                "--19zvkyr": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(a),
                "--1xajcet": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(b),
                "--ke45ok": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(c)
            }
        ]
};
