import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1h0pinx{--header-height:0px}",
    priority: 1
});
_inject2({
    ltr: ".x1pw20mz{min-height:calc(100dvh - var(--header-height,0px))}",
    priority: 4000
});
export default function Example() {
    return <div className={"x1h0pinx x1pw20mz"}>Content</div>;
}
