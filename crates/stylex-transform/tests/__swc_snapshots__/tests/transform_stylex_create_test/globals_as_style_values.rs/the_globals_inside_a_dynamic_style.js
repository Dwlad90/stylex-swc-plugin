import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    kZKoxP: "x36qwtl",
    $$css: true
};
const _temp2 = {
    kZKoxP: "xkm8o77",
    $$css: true
};
_inject2({
    ltr: ".x36qwtl{height:2px}",
    priority: 4000
});
_inject2({
    ltr: ".x5lhr3w{width:var(--x-width)}",
    priority: 4000
});
_inject2({
    ltr: ".xkm8o77{height:Infinitypx;height:2px}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-width { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    notANumber: (h)=>[
            _temp,
            {
                kzqmXN: h != null ? "x5lhr3w" : h,
                $$css: true
            },
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(h)
            }
        ],
    infinite: (h)=>[
            _temp2,
            {
                kzqmXN: h != null ? "x5lhr3w" : h,
                $$css: true
            },
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(h)
            }
        ]
};
