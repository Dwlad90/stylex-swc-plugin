import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x13rv2e4{color:hotpink}",
    priority: 3000
});
export function Text({ children, variant: { title: t } }) {
    return <div {...stylex.props(...t)}>{children}</div>;
}
