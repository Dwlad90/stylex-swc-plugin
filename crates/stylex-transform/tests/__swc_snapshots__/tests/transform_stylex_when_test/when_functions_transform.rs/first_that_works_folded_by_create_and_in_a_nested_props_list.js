import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x12gg5we{color:b;color:a}",
    priority: 3000
});
_inject2({
    ltr: ".x1717udv{padding:0}",
    priority: 1000
});
const styles = {
    root: {
        kMwMTN: "x12gg5we",
        $$css: true
    },
    other: {
        kmVPX3: "x1717udv",
        $$css: true
    }
};
export function Foo({ active }) {
    return stylex.props([
        styles.other,
        active ? [
            styles.root,
            stylex.firstThatWorks('a', 'b')
        ] : null
    ]);
}
