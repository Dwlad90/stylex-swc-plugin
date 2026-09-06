import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    kZKoxP: "x16ye13r",
    $$css: true
};
const _temp2 = {
    kzqmXN: "x5lhr3w",
    $$css: true
};
_inject2({
    ltr: ".x16ye13r{height:var(--x-height)}",
    priority: 4000
});
_inject2({
    ltr: ".x5lhr3w{width:var(--x-width)}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-height { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-width { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    dyn: (flag)=>[
            _temp,
            {
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(flag ? '1px' : '2px')
            }
        ],
    negated: (flag)=>[
            _temp2,
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(!flag ? '1px' : '2px')
            }
        ]
};
