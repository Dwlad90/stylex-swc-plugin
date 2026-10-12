import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "custom-marker.stylex";
import * as stylex from '@stylexjs/stylex';
import { customMarker } from 'custom-marker.stylex';
_inject2({
    ltr: ".x1t391ir{background-color:blue}",
    priority: 3000
});
_inject2({
    ltr: ".x7rpj1w.x7rpj1w:where(.x1lc2aw:hover *){background-color:red}",
    priority: 3011.3
});
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
const styles = {
    foo: {
        kWkggS: "x1t391ir x7rpj1w",
        $$css: true
    },
    bar: {
        kMwMTN: "x1e2nbdu",
        $$css: true
    }
};
function Foo({ active }) {
    return stylex.props([
        styles.foo,
        active ? [
            styles.bar,
            customMarker
        ] : null
    ]);
}
