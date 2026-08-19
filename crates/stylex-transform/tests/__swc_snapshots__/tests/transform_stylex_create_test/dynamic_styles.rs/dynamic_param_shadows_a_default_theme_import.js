import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "tokens.stylex.js";
import * as stylex from '@stylexjs/stylex';
import tokens from 'tokens.stylex.js';
_inject2({
    ltr: ".x1w9t8p2{color:var(--xe7srj8)}",
    priority: 3000
});
_inject2({
    ltr: ".x14rh7hd{color:var(--x-color)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-color { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    wrapper: {
        kMwMTN: "x1w9t8p2",
        $$css: true
    },
    dyn: (tokens)=>[
            {
                kMwMTN: tokens != null ? "x14rh7hd" : tokens,
                $$css: true
            },
            {
                "--x-color": tokens != null ? tokens : undefined
            }
        ]
};
