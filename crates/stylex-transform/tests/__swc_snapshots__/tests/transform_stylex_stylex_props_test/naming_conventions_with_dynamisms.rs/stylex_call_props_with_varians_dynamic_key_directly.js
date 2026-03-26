import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x13rv2e4{color:hotpink}",
    priority: 3000
});
const styles = {
    title: {
        kMwMTN: "x13rv2e4",
        $$css: true
    }
};
const variant = {
    title: [
        styles.title
    ]
};
export function Text({ children, variant }) {
    return <div {...stylex.props(...variant.title)}>{children}</div>;
}
