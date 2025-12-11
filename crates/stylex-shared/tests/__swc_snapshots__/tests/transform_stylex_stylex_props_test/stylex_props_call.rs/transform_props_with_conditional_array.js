import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x1t391ir{background-color:blue}", 3000);
_inject2(".x1e2nbdu{color:red}", 3000);
_inject2(".x1prwzq3{color:green}", 3000);
export function Props_With_Conditional_Array(status) {
    const isActive = status === 'active';
    return <button {...{
        0: {
            className: "x1t391ir x1prwzq3"
        },
        1: {
            className: "x1t391ir x1e2nbdu"
        }
    }[!!isActive << 0]}/>;
}
