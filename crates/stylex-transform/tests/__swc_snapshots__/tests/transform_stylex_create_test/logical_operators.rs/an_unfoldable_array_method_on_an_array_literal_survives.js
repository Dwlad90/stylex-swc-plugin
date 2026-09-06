import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1mqxbix{color:black}",
    priority: 3000
});
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
export function View({ hView }) {
    const isHView = hView && [
        'grid',
        'list'
    ].includes(hView);
    return <div {...{
        0: {
            className: "x1mqxbix"
        },
        1: {
            className: "x1e2nbdu"
        }
    }[!!isHView << 0]}/>;
}
