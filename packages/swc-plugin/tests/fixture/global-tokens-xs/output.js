import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from "@stylexjs/stylex";
const MIN_WIDTH = 320;
const MAX_WIDTH = 1240;
const MIN_SCALE = 1.2;
const MAX_SCALE = 1.333;
const MIN_BASE_SIZE = 16;
const MAX_BASE_SIZE = 20;
// Font sizes in `rem` units
const MIN_FONT = {
    xs: Math.round(MIN_BASE_SIZE / Math.pow(MIN_SCALE, 2) / 0.16) / 100
};
// Font sizes in `rem` units
const MAX_FONT = {
    xs: Math.round(MAX_BASE_SIZE / Math.pow(MAX_SCALE, 2) / 0.16) / 100
};
const SLOPE = {
    xs: 16 * (MAX_FONT.xs - MIN_FONT.xs) / (MAX_WIDTH - MIN_WIDTH)
};
const INTERCEPT = {
    xs: Math.round(100 * (MIN_FONT.xs - SLOPE.xs * (MIN_WIDTH / 16))) / 100
};
_inject2(":root{--x1ogzt1a:0.69rem;}", 0);
export const text = {
    xs: "var(--x1ogzt1a)",
    __themeName__: "x1lzvrc9"
};
