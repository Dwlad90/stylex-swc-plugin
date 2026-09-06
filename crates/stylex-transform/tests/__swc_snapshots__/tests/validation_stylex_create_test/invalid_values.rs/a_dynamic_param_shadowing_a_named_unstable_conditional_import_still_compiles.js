import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import { create, unstable_conditional } from '@stylexjs/stylex';
_inject2({
    ltr: ".x16ye13r{height:var(--x-height)}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-height { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    dyn: (unstable_conditional)=>[
            {
                kZKoxP: unstable_conditional != null ? "x16ye13r" : unstable_conditional,
                $$css: true
            },
            {
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(unstable_conditional)
            }
        ]
};
