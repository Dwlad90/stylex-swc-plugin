import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x13rv2e4{color:hotpink}",
    priority: 3000
});
const styles = {
    defaultLink: {
        kMwMTN: "x13rv2e4",
        $$css: true
    }
};
export function Text({ children, variant }) {
    const variants = {
        default: {
            link: styles.defaultLink
        }
    };
    return <div {...stylex.props(variants[variant].link)}>{children}</div>;
}
