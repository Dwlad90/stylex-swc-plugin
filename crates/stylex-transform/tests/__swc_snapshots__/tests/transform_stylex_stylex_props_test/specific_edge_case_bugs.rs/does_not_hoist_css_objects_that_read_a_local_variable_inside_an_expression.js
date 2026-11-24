import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
const styles = {
    root: {
        k1xSpc: "x78zum5",
        $$css: true
    }
};
function Foo({ size, map }) {
    return <div {...stylex.props(styles.root, size && {
        $$css: true,
        a: `x${size}`
    }, map && {
        $$css: true,
        [map.key]: 'x'
    })}>Hello, foo!</div>;
}
