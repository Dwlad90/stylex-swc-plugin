import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1b8z93w{row-gap:10px}",
    priority: 3000
});
_inject2({
    ltr: ".x1r8uycs{row-gap:var(--x-rowGap)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-rowGap { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    wrapper: {
        khm7nJ: "x1b8z93w",
        $$css: true
    },
    dyn: (gap)=>[
            {
                khm7nJ: gap != null ? "x1r8uycs" : gap,
                $$css: true
            },
            {
                "--x-rowGap": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(gap)
            }
        ]
};
