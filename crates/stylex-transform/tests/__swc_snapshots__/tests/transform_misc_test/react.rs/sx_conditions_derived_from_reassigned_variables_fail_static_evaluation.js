import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
_inject2({
    ltr: ".x1jnr06f{gap:4px}",
    priority: 2000
});
export function Component({
    flag
}) {
    let items = [];
    if (flag) items = [1];
    const hasItems = items.length > 0;
    return <div {...{
        0: {
            className: "x78zum5"
        },
        1: {
            className: "x78zum5 x1jnr06f"
        }
    }[!!hasItems << 0]} />;
}
