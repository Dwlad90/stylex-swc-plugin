import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    kzQI83: "xw36f2b",
    $$css: true
};
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
            {
                "--x-flexGrow": {} * props.x != null ? {} * props.x : undefined
            }
        ]
};
