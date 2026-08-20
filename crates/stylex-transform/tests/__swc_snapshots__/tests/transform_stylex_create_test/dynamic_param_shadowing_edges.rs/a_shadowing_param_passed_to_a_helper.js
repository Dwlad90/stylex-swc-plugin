import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
import { firstThatWorks } from '@stylexjs/stylex';
import { zIndex } from 'zIndex.stylex.js';
_inject2({
    ltr: ".xk2v41j{font-family:var(--x-fontFamily)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-fontFamily { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    dyn: (zIndex)=>[
            {
                kMv6JI: firstThatWorks(zIndex, 'serif') != null ? "xk2v41j" : firstThatWorks(zIndex, 'serif'),
                $$css: true
            },
            {
                "--x-fontFamily": firstThatWorks(zIndex, 'serif') != null ? firstThatWorks(zIndex, 'serif') : undefined
            }
        ]
};
