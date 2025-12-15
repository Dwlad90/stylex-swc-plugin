import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1t391ir{background-color:blue}",
    priority: 3000
});
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".x1prwzq3{color:green}",
    priority: 3000
});
export function Props_With_Conditional_Array(status) {
    const isActive = /status/.test(status);
    return <>
      <button {...{
        0: {
            className: "x1t391ir x1prwzq3"
        },
        1: {
            className: "x1t391ir x1e2nbdu"
        }
    }[!!isActive << 0]}/>
      {isActive ? <div className="x1e2nbdu">Active</div> : <div className="x1prwzq3">Inactive</div>}
      <div {...{
        0: {},
        1: {
            className: "x1e2nbdu"
        }
    }[!!isActive << 0]}>Active</div>
      </>;
}
