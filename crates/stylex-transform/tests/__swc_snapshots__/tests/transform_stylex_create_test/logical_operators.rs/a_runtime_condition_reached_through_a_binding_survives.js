import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
_inject2({
    ltr: ".x1lliihq{display:block}",
    priority: 3000
});
_inject2({
    ltr: ".x76ihet{border-top:none}",
    priority: 2000
});
export function Component({ query }) {
    const lowerQuery = query.toLowerCase();
    const showAlternate = query.length > 0 && "documentation".startsWith(lowerQuery);
    return <section {...{
        0: {
            className: "x1lliihq"
        },
        1: {
            className: "x1lliihq x76ihet"
        }
    }[!!showAlternate << 0]}/>;
}
