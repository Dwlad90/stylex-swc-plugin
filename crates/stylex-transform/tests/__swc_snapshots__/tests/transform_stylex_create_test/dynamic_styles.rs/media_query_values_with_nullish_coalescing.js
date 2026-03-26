import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".xww4jgc{font-size:var(--x-19zvkyr)}",
    priority: 3000
});
_inject2({
    ltr: "@media (min-width: 800px) and (max-width: 1279.99px){.xqdov8i.xqdov8i{font-size:var(--x-1bks2es)}}",
    priority: 3200
});
_inject2({
    ltr: "@media (min-width: 1280px){.x1j86d60.x1j86d60{font-size:var(--x-q0n1i6)}}",
    priority: 3200
});
_inject2({
    ltr: '@property --x-19zvkyr { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-1bks2es { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-q0n1i6 { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    root: (a, b, c)=>[
            {
                kGuDYH: ((a ? '16px' : undefined) != null ? "xww4jgc " : a ? '16px' : undefined) + ((b ? '18px' : undefined) != null ? "xqdov8i " : b ? '18px' : undefined) + ((c ? '20px' : undefined) != null ? "x1j86d60" : c ? '20px' : undefined),
                $$css: true
            },
            {
                "--x-19zvkyr": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(a ? '16px' : undefined),
                "--x-1bks2es": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(b ? '18px' : undefined),
                "--x-q0n1i6": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(c ? '20px' : undefined)
            }
        ]
};
stylex.props(styles.root(true, false, true));
