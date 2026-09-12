import * as stylex from "@stylexjs/stylex";
export function Leaf({ id, sx }) {
    return _jsx("div", {
        id,
        ...stylex.props(sx),
        children: "Hello World"
    });
}
