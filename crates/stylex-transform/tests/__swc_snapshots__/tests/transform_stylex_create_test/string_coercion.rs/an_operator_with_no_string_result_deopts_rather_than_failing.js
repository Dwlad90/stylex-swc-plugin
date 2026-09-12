import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    kzQI83: "xzftc1n",
    $$css: true
};
const _temp2 = {
    kzQI83: "xemlnqb",
    $$css: true
};
const _temp3 = {
    kzQI83: "xw36f2b",
    $$css: true
};
_inject2({
    ltr: ".xzftc1n{flex-grow:NaN}",
    priority: 3000
});
_inject2({
    ltr: ".xemlnqb{flex-grow:-1}",
    priority: 3000
});
_inject2({
    ltr: ".xw36f2b{flex-grow:var(--x-flexGrow)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-flexGrow { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    a: (props)=>[
            _temp,
            {}
        ],
    b: (props)=>[
            _temp2,
            {}
        ],
    c: (props)=>[
            _temp3,
            {
                "--x-flexGrow": [
                    1,
                    2
                ] * 2 != null ? [
                    1,
                    2
                ] * 2 : undefined
            }
        ]
};
