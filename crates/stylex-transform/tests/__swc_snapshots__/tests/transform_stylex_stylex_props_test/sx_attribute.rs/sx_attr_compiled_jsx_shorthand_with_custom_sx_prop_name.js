import stylex from 'stylex';
export function Leaf({ css }) {
    return _jsx("div", {
        ...stylex.props(css),
        children: "Hello World"
    });
}
