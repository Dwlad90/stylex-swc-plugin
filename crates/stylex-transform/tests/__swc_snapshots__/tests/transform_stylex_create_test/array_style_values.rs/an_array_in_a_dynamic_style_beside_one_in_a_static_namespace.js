import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    kZKoxP: "xhvjh0r",
    $$css: true
};
_inject2({
    ltr: ".x5ozmkz{height:1px;height:2px}",
    priority: 4000
});
_inject2({
    ltr: ".xhvjh0r{height:3px;height:4px}",
    priority: 4000
});
_inject2({
    ltr: ".x5lhr3w{width:var(--x-width)}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-width { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    s: {
        kZKoxP: "x5ozmkz",
        $$css: true
    },
    dyn: (h)=>[
            _temp,
            {
                kzqmXN: h != null ? "x5lhr3w" : h,
                $$css: true
            },
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(h)
            }
        ]
};
