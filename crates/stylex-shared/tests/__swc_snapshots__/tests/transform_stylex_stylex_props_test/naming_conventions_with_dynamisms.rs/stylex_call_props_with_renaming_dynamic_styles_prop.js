import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "@stylexjs/open-props/lib/fonts.stylex";
import * as stylex from '@stylexjs/stylex';
import { fonts as f } from '@stylexjs/open-props/lib/fonts.stylex';
_inject2({
    ltr: ".x13rv2e4{color:hotpink}",
    priority: 3000
});
const styles = {
    text: {
        kMwMTN: "x13rv2e4",
        $$css: true
    }
};
_inject2({
    ltr: ".x3b68l4{font-size:var(--x1cjnt43)}",
    priority: 3000
});
_inject2({
    ltr: ".x59qt1o{font-size:var(--xw8ib4r)}",
    priority: 3000
});
const variants = {
    small: {
        kGuDYH: "x3b68l4",
        $$css: true
    },
    big: {
        kGuDYH: "x59qt1o",
        $$css: true
    }
};
export function Text2({ children, size: s }) {
    return <div {...stylex.props(styles.text, variants[s])}>{children}</div>;
}
