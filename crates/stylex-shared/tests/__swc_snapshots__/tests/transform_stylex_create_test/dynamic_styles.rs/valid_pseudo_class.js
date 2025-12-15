import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1j2k28p:hover{background-color:var(--x-1e2mv7m)}",
    priority: 3130
});
_inject2({
    ltr: ".x1qvlgnj:hover{color:var(--x-1113oo7)}",
    priority: 3130
});
_inject2({
    ltr: '@property --x-1e2mv7m { syntax: "*"; }',
    priority: 0
});
_inject2({
    ltr: '@property --x-1113oo7 { syntax: "*"; }',
    priority: 0
});
export const styles = {
    root: (color)=>[
            {
                kWkggS: color != null ? "x1j2k28p" : color,
                kMwMTN: color != null ? "x1qvlgnj" : color,
                $$css: true
            },
            {
                "--x-1e2mv7m": color != null ? color : undefined,
                "--x-1113oo7": color != null ? color : undefined
            }
        ]
};
