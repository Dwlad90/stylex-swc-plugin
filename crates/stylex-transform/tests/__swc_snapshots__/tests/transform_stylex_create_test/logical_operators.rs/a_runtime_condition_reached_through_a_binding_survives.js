import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
_inject2({
    ltr: ".x1lliihq{display:block}",
    priority: 3000
});
export function Component({ query }) {
    const lowerQuery = query.toLowerCase();
    const showAlternate = query.length > 0 && "documentation".startsWith(lowerQuery);
    return <section {...{
        0: {
            className: "x1lliihq"
        },
        1: {
            className: "x1lliihq"
        }
    }[!!showAlternate << 0]}/>;
}
