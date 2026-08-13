import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x14rh7hd{color:var(--x-color)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-color { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    root: (color)=>[
            {
                kMwMTN: String(color) != null ? "x14rh7hd" : String(color),
                $$css: true
            },
            {
                "--x-color": String(color) != null ? String(color) : undefined
            }
        ]
};
