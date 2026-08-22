import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    kMv6JI: "xk2v41j",
    $$css: true
};
_inject2({
    ltr: ".xk2v41j{font-family:var(--x-fontFamily)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-fontFamily { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    dyn: (h)=>[
            _temp,
            {
                "--x-fontFamily": `a${h}b` != null ? `a${h}b` : undefined
            }
        ]
};
