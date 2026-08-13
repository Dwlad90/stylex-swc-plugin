import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".xadwnm5:hover{color:var(--x-1ijzsae)}",
    priority: 3130
});
_inject2({
    ltr: '@property --x-1ijzsae { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    repro: (color)=>[
            {
                kDPRdz: color != null ? "xadwnm5" : color,
                $$css: true
            },
            {
                "--x-1ijzsae": color != null ? color : undefined
            }
        ]
};
