import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".xaigonn::before{color:var(--x-1g451k2)}",
    priority: 8000
});
_inject2({
    ltr: ".x1p1099i::after{color:var(--x-19erzii)}",
    priority: 8000
});
_inject2({
    ltr: '@property --x-1g451k2 { syntax: "*"; }',
    priority: 0
});
_inject2({
    ltr: '@property --x-19erzii { syntax: "*"; }',
    priority: 0
});
export const styles = {
    foo: (a, b)=>[
            {
                kxBb7d: a != null ? "xaigonn" : a,
                kB1Fuz: b != null ? "x1p1099i" : b,
                $$css: true
            },
            {
                "--x-1g451k2": a != null ? a : undefined,
                "--x-19erzii": b != null ? b : undefined
            }
        ]
};
