import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x5lhr3w{width:var(--x-width)}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-width { syntax: "*"; inherits: false; }',
    priority: 0
});
export const styles = {
    root: (width)=>[
            {
                kzqmXN: width != null ? "x5lhr3w" : width,
                $$css: true
            },
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width)
            }
        ]
};
