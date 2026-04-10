import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1p1099i::after{color:var(--x-19erzii)}",
    priority: 8000
});
_inject2({
    ltr: '@property --x-19erzii { syntax: "*"; inherits: true;}',
    priority: 0
});
export const styles = {
    repro: (color)=>[
            {
                kB1Fuz: color != null ? "x1p1099i" : color,
                $$css: true
            },
            {
                "--x-19erzii": color != null ? color : undefined
            }
        ]
};
