import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const xs = [
    '1px'
];
_inject2({
    ltr: ".x16ye13r{height:var(--x-height)}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-height { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    dyn: (h)=>[
            {
                kZKoxP: [
                    ...xs,
                    '2px'
                ] != null ? "x16ye13r" : [
                    ...xs,
                    '2px'
                ],
                $$css: true
            },
            {
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)([
                    ...xs,
                    '2px'
                ])
            }
        ]
};
