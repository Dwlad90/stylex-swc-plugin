import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "tokens.stylex";
import * as stylex from '@stylexjs/stylex';
import { colors } from 'tokens.stylex';
const zero = 0;
_inject2({
    ltr: ".x14rh7hd{color:var(--x-color)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-color { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    a: (props)=>[
            {
                kMwMTN: [
                    zero ?? colors.glow,
                    props.x
                ].join('') != null ? "x14rh7hd" : [
                    zero ?? colors.glow,
                    props.x
                ].join(''),
                $$css: true
            },
            {
                "--x-color": [
                    zero ?? colors.glow,
                    props.x
                ].join('') != null ? [
                    zero ?? colors.glow,
                    props.x
                ].join('') : undefined
            }
        ]
};
