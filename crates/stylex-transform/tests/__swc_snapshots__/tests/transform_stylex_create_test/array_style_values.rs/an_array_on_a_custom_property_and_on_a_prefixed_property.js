import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    "--foo": "x1x1xuuh",
    kJFfOR: "x15iyhky",
    $$css: true
};
_inject2({
    ltr: ".x1x1xuuh{--foo:1px;--foo:2px}",
    priority: 1
});
_inject2({
    ltr: ".x15iyhky{-webkit-line-clamp:1;-webkit-line-clamp:2}",
    priority: 3000
});
_inject2({
    ltr: ".x16ye13r{height:var(--x-height)}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-height { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    dyn: (h)=>[
            _temp,
            {
                kZKoxP: h != null ? "x16ye13r" : h,
                $$css: true
            },
            {
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(h)
            }
        ]
};
