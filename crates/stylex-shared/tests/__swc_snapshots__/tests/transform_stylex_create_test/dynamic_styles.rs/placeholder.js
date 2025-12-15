import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1mzl164::placeholder{color:var(--x-163tekb)}",
    priority: 8000
});
_inject2({
    ltr: '@property --x-163tekb { syntax: "*"; }',
    priority: 0
});
export const styles = {
    foo: (color)=>[
            {
                k8Qsv1: color != null ? "x1mzl164" : color,
                $$css: true
            },
            {
                "--x-163tekb": color != null ? color : undefined
            }
        ]
};
