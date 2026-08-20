import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import { create, types } from '@stylexjs/stylex';
_inject2({
    ltr: ".x16ye13r{height:var(--x-height)}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-height { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    dyn: (types)=>[
            {
                kZKoxP: types != null ? "x16ye13r" : types,
                $$css: true
            },
            {
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(types)
            }
        ]
};
