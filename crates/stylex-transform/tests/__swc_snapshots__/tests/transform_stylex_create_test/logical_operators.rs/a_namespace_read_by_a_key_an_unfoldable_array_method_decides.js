import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const WIDE_VIEWS = [
    'grid',
    'list'
];
_inject2({
    ltr: ".x1mqxbix{color:black}",
    priority: 3000
});
_inject2({
    ltr: ".xrvj5dj{display:grid}",
    priority: 3000
});
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
const styles = {
    base: {
        kMwMTN: "x1mqxbix",
        $$css: true
    },
    regularGrid: {
        k1xSpc: "xrvj5dj",
        $$css: true
    },
    wideGrid: {
        k1xSpc: "x78zum5",
        $$css: true
    }
};
export function View({ hView }) {
    let gridType = 'regular';
    if (hView && WIDE_VIEWS.includes(hView)) {
        gridType = 'wide';
    }
    const grid = `${gridType}Grid`;
    return <div {...stylex.props([
        styles.base,
        styles[grid]
    ])}/>;
}
