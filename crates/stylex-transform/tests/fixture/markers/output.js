import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "other-markers.stylex";
import * as stylex from "@stylexjs/stylex";
import { importedMarker } from "other-markers.stylex";
export const localMarker = {
    x1bawz77: "x1bawz77",
    $$css: true
};
_inject2({
    ltr: ".color-xkn7p67{color:gray}",
    priority: 3000
});
_inject2({
    ltr: ".color-xomp1nr.color-xomp1nr:where(.x1bawz77[data-open] *){color:white}",
    priority: 3040
});
_inject2({
    ltr: ".color-x1wgracu.color-x1wgracu:where(:has(.x183id7b:focus)){color:blue}",
    priority: 3016.5
});
_inject2({
    ltr: ".color-x1uatm7.color-x1uatm7:where(.x-default-marker:hover ~ *){color:black}",
    priority: 3031.3
});
export const styles = {
    label: {
        color: "color-xkn7p67 color-xomp1nr color-x1wgracu color-x1uatm7",
        $$css: "tests/fixture/markers/input.stylex.js:5"
    }
};
