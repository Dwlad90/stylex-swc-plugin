import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from "@stylexjs/stylex";
/**
 * o--o o    o   o o-O-o o-o       o--o  o-o  o   o o-O-o  o-o
 * |    |    |   |   |   |  \      |    o   o |\  |   |   |
 * O-o  |    |   |   |   |   O     O-o  |   | | \ |   |    o-o
 * |    |    |   |   |   |  /      |    o   o |  \|   |       |
 * o    O---o o-o  o-O-o o-o       o     o-o  o   o   o   o--o
 *
 * Reference: https://utopia.fyi/type/calculator
 *
 * The following constants are used to calculate fluid typography.
 * Feel free to change these initial numbers to suit your needs.
 *
 * StyleX can compute all of this at compile time as all the information
 * is statically available in the same file and the only functions used are
 * the Math.pow and Math.round functions.
 *
 * NOTE: Any custom functions will not be able to be computed at compile time.
 */ const MIN_WIDTH = 320;
const MAX_WIDTH = 1240;
const MIN_SCALE = 1.2;
const MAX_SCALE = 1.333;
const MIN_BASE_SIZE = 16;
const MAX_BASE_SIZE = 20;
// Font sizes in `rem` units
const MIN_FONT = {
    xxs: Math.round(MIN_BASE_SIZE / Math.pow(MIN_SCALE, 3) / 0.16) / 100,
    xs: Math.round(MIN_BASE_SIZE / Math.pow(MIN_SCALE, 2) / 0.16) / 100,
    sm: Math.round(MIN_BASE_SIZE / MIN_SCALE / 0.16) / 100,
    p: Math.round(MIN_BASE_SIZE / 4) / 4,
    h5: Math.round(MIN_BASE_SIZE * MIN_SCALE / 0.16) / 100,
    h4: Math.round(MIN_BASE_SIZE * Math.pow(MIN_SCALE, 2) / 0.16) / 100,
    h3: Math.round(MIN_BASE_SIZE * Math.pow(MIN_SCALE, 3) / 0.16) / 100,
    h2: Math.round(MIN_BASE_SIZE * Math.pow(MIN_SCALE, 4) / 0.16) / 100,
    h1: Math.round(MIN_BASE_SIZE * Math.pow(MIN_SCALE, 5) / 0.16) / 100
};
// Font sizes in `rem` units
const MAX_FONT = {
    xxs: Math.round(MAX_BASE_SIZE / Math.pow(MAX_SCALE, 3) / 0.16) / 100,
    xs: Math.round(MAX_BASE_SIZE / Math.pow(MAX_SCALE, 2) / 0.16) / 100,
    sm: Math.round(MAX_BASE_SIZE / MAX_SCALE / 0.16) / 100,
    p: Math.round(MAX_BASE_SIZE / 4) / 4,
    h5: Math.round(MAX_BASE_SIZE * MAX_SCALE / 0.16) / 100,
    h4: Math.round(MAX_BASE_SIZE * Math.pow(MAX_SCALE, 2) / 0.16) / 100,
    h3: Math.round(MAX_BASE_SIZE * Math.pow(MAX_SCALE, 3) / 0.16) / 100,
    h2: Math.round(MAX_BASE_SIZE * Math.pow(MAX_SCALE, 4) / 0.16) / 100,
    h1: Math.round(MAX_BASE_SIZE * Math.pow(MAX_SCALE, 5) / 0.16) / 100
};
const SLOPE = {
    xxs: 16 * (MAX_FONT.xxs - MIN_FONT.xxs) / (MAX_WIDTH - MIN_WIDTH),
    xs: 16 * (MAX_FONT.xs - MIN_FONT.xs) / (MAX_WIDTH - MIN_WIDTH),
    sm: 16 * (MAX_FONT.sm - MIN_FONT.sm) / (MAX_WIDTH - MIN_WIDTH),
    p: 16 * (MAX_FONT.p - MIN_FONT.p) / (MAX_WIDTH - MIN_WIDTH),
    h5: 16 * (MAX_FONT.h5 - MIN_FONT.h5) / (MAX_WIDTH - MIN_WIDTH),
    h4: 16 * (MAX_FONT.h4 - MIN_FONT.h4) / (MAX_WIDTH - MIN_WIDTH),
    h3: 16 * (MAX_FONT.h3 - MIN_FONT.h3) / (MAX_WIDTH - MIN_WIDTH),
    h2: 16 * (MAX_FONT.h2 - MIN_FONT.h2) / (MAX_WIDTH - MIN_WIDTH),
    h1: 16 * (MAX_FONT.h1 - MIN_FONT.h1) / (MAX_WIDTH - MIN_WIDTH)
};
const INTERCEPT = {
    xxs: Math.round(100 * (MIN_FONT.xxs - SLOPE.xxs * (MIN_WIDTH / 16))) / 100,
    xs: Math.round(100 * (MIN_FONT.xs - SLOPE.xs * (MIN_WIDTH / 16))) / 100,
    sm: Math.round(100 * (MIN_FONT.sm - SLOPE.sm * (MIN_WIDTH / 16))) / 100,
    p: Math.round(100 * (MIN_FONT.p - SLOPE.p * (MIN_WIDTH / 16))) / 100,
    h5: Math.round(100 * (MIN_FONT.h5 - SLOPE.h5 * (MIN_WIDTH / 16))) / 100,
    h4: Math.round(100 * (MIN_FONT.h4 - SLOPE.h4 * (MIN_WIDTH / 16))) / 100,
    h3: Math.round(100 * (MIN_FONT.h3 - SLOPE.h3 * (MIN_WIDTH / 16))) / 100,
    h2: Math.round(100 * (MIN_FONT.h2 - SLOPE.h2 * (MIN_WIDTH / 16))) / 100,
    h1: Math.round(100 * (MIN_FONT.h1 - SLOPE.h1 * (MIN_WIDTH / 16))) / 100
};
_inject2(":root{--x1ql1w94:clamp(0.58rem, calc(0.6rem + -0.09vw), 0.53rem);--x1ogzt1a:clamp(0.69rem, calc(0.69rem + 0.02vw), 0.7rem);--x16zehmx:clamp(0.83rem, calc(0.79rem + 0.19vw), 0.94rem);--xhk4hdt:clamp(1rem, calc(0.91rem + 0.43vw), 1.25rem);--xwuz3e6:clamp(1.2rem, calc(1.04rem + 0.82vw), 1.67rem);--xcuma3z:clamp(1.44rem, calc(1.17rem + 1.36vw), 2.22rem);--x1d2707x:clamp(1.73rem, calc(1.3rem + 2.14vw), 2.96rem);--xvxqfsp:clamp(2.07rem, calc(1.42rem + 3.27vw), 3.95rem);--x1cypdqd:clamp(2.49rem, calc(1.53rem + 4.82vw), 5.26rem);}", 0);
export const text = {
    xxs: "var(--x1ql1w94)",
    xs: "var(--x1ogzt1a)",
    sm: "var(--x16zehmx)",
    p: "var(--xhk4hdt)",
    h5: "var(--xwuz3e6)",
    h4: "var(--xcuma3z)",
    h3: "var(--x1d2707x)",
    h2: "var(--xvxqfsp)",
    h1: "var(--x1cypdqd)",
    __themeName__: "x1lzvrc9"
};
/**
 * o--o o    o   o o-O-o o-o        o-o  o--o    O    o-o o--o
 * |    |    |   |   |   |  \      |     |   |  / \  /    |
 * O-o  |    |   |   |   |   O      o-o  O--o  o---oO     O-o
 * |    |    |   |   |   |  /          | |     |   | \    |
 * o    O---o o-o  o-O-o o-o       o--o  o     o   o  o-o o--o
 *
 * Reference: https://utopia.fyi/space/calculator
 *
 * Similar to the fluid typography, we can create fluid values for spacing.
 * Using similar formulas and similar scales.
 *
 * NOTE: It is common to have more varied needs for spacing than for font-size.
 * So feel free to add some more values by following the pattern below.
 *
 * EXCEPT: We are using `px` instead of `rem`
 * ------------------------------------------
 * When talking about font-size, it is the best practice to use
 * `rem` so that an end user can change the font-size using the
 * browser's font-size setting.
 *
 * However, when talking about spacing, it is the best practice to
 * use `px` because using `rems` here makes font-size behave like zoom.
 *
 * Users that prefer larger text, don't neccessarily want larger spacing as well.
 *
 */ const MULT = {
    xxxs: 0.25,
    xxs: 0.5,
    xs: 0.75,
    sm: 1,
    md: 1.5,
    lg: 2,
    xl: 3,
    xxl: 4,
    xxxl: 6,
    xxxxl: 8
};
const MIN_SPACE = {
    xxxs: MULT.xxxs * MIN_BASE_SIZE,
    xxs: MULT.xxs * MIN_BASE_SIZE,
    xs: MULT.xs * MIN_BASE_SIZE,
    sm: MULT.sm * MIN_BASE_SIZE,
    md: MULT.md * MIN_BASE_SIZE,
    lg: MULT.lg * MIN_BASE_SIZE,
    xl: MULT.xl * MIN_BASE_SIZE,
    xxl: MULT.xxl * MIN_BASE_SIZE,
    xxxl: MULT.xxxl * MIN_BASE_SIZE,
    xxxxl: MULT.xxxxl * MIN_BASE_SIZE
};
const MAX_SPACE = {
    xxxs: MULT.xxxs * MAX_BASE_SIZE,
    xxs: MULT.xxs * MAX_BASE_SIZE,
    xs: MULT.xs * MAX_BASE_SIZE,
    sm: MULT.sm * MAX_BASE_SIZE,
    md: MULT.md * MAX_BASE_SIZE,
    lg: MULT.lg * MAX_BASE_SIZE,
    xl: MULT.xl * MAX_BASE_SIZE,
    xxl: MULT.xxl * MAX_BASE_SIZE,
    xxxl: MULT.xxxl * MAX_BASE_SIZE,
    xxxxl: MULT.xxxxl * MAX_BASE_SIZE
};
const SLOPE_SPACE = {
    xxxs: (MAX_SPACE.xxxs - MIN_SPACE.xxxs) / (MAX_WIDTH - MIN_WIDTH),
    xxs: (MAX_SPACE.xxs - MIN_SPACE.xxs) / (MAX_WIDTH - MIN_WIDTH),
    xs: (MAX_SPACE.xs - MIN_SPACE.xs) / (MAX_WIDTH - MIN_WIDTH),
    sm: (MAX_SPACE.sm - MIN_SPACE.sm) / (MAX_WIDTH - MIN_WIDTH),
    md: (MAX_SPACE.md - MIN_SPACE.md) / (MAX_WIDTH - MIN_WIDTH),
    lg: (MAX_SPACE.lg - MIN_SPACE.lg) / (MAX_WIDTH - MIN_WIDTH),
    xl: (MAX_SPACE.xl - MIN_SPACE.xl) / (MAX_WIDTH - MIN_WIDTH),
    xxl: (MAX_SPACE.xxl - MIN_SPACE.xxl) / (MAX_WIDTH - MIN_WIDTH),
    xxxl: (MAX_SPACE.xxxl - MIN_SPACE.xxxl) / (MAX_WIDTH - MIN_WIDTH),
    xxxxl: (MAX_SPACE.xxxxl - MIN_SPACE.xxxxl) / (MAX_WIDTH - MIN_WIDTH)
};
// rounded to the nearest 0.25px
const INTERCEPT_SPACE = {
    xxxs: Math.round(4 * (MIN_SPACE.xxxs - SLOPE_SPACE.xxxs * MIN_WIDTH)) / 4,
    xxs: Math.round(4 * (MIN_SPACE.xxs - SLOPE_SPACE.xxs * MIN_WIDTH)) / 4,
    xs: Math.round(4 * (MIN_SPACE.xs - SLOPE_SPACE.xs * MIN_WIDTH)) / 4,
    sm: Math.round(4 * (MIN_SPACE.sm - SLOPE_SPACE.sm * MIN_WIDTH)) / 4,
    md: Math.round(4 * (MIN_SPACE.md - SLOPE_SPACE.md * MIN_WIDTH)) / 4,
    lg: Math.round(4 * (MIN_SPACE.lg - SLOPE_SPACE.lg * MIN_WIDTH)) / 4,
    xl: Math.round(4 * (MIN_SPACE.xl - SLOPE_SPACE.xl * MIN_WIDTH)) / 4,
    xxl: Math.round(4 * (MIN_SPACE.xxl - SLOPE_SPACE.xxl * MIN_WIDTH)) / 4,
    xxxl: Math.round(4 * (MIN_SPACE.xxxl - SLOPE_SPACE.xxxl * MIN_WIDTH)) / 4,
    xxxxl: Math.round(4 * (MIN_SPACE.xxxxl - SLOPE_SPACE.xxxxl * MIN_WIDTH)) / 4
};
_inject2(":root{--xe27369:clamp(4px, calc(3.75px - 0.11vw), 5px);--xbjetdn:clamp(8px, calc(7.25px - 0.22vw), 10px);--x1ixl80x:clamp(12px, calc(11px - 0.33vw), 15px);--x1kvcwuq:clamp(16px, calc(14.5px - 0.43vw), 20px);--xmdt6tw:clamp(24px, calc(22px - 0.65vw), 30px);--x1wksnfy:clamp(32px, calc(29.25px - 0.87vw), 40px);--xoxmq3b:clamp(48px, calc(43.75px - 1.3vw), 60px);--xdo4ik8:clamp(64px, calc(58.5px - 1.74vw), 80px);--x2u3u4d:clamp(96px, calc(87.75px - 2.61vw), 120px);--xmk1p5w:clamp(128px, calc(116.75px - 3.48vw), 160px);}", 0);
export const spacing = {
    xxxs: "var(--xe27369)",
    xxs: "var(--xbjetdn)",
    xs: "var(--x1ixl80x)",
    sm: "var(--x1kvcwuq)",
    md: "var(--xmdt6tw)",
    lg: "var(--x1wksnfy)",
    xl: "var(--xoxmq3b)",
    xxl: "var(--xdo4ik8)",
    xxxl: "var(--x2u3u4d)",
    xxxxl: "var(--xmk1p5w)",
    __themeName__: "x14ijk3f"
};
/**
 * Color Tokens
 */ const DARK_MODE = "@media (prefers-color-scheme: dark)";
_inject2(':root{--xdkvadk:1240px;--x1v9y61d:ui-monospace, Menlo, Monaco, "Cascadia Mono", "Segoe UI Mono", "Roboto Mono", "Oxygen Mono", "Ubuntu Monospace", "Source Code Pro", "Fira Mono", "Droid Sans Mono", "Courier New", monospace;--xu8xumw:-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";--x9q2m40:0;--xpzz690:0;--x16lcx6o:0;--xjk46kt:rgb(214, 219, 220);--x19cfreg:255;--x5f91dp:255;--xtrkg9t:255;--xrj4b28:rgb(238, 240, 241);--x13ytpr0:rgba(238, 240, 241, 0.5);--xjray:172;--x1ats3qd:175;--x12b45e3:176;--x1efhglm:180;--x1w81gmp:185;--x13v9q97:188;--x14edl43:131;--xdi7wre:134;--x1livm2j:135;--x1r7qzpr:conic-gradient(from 180deg at 50% 50%, #16abff33 0deg, #0885ff33 55deg, #54d6ff33 120deg, #0071ff33 160deg, transparent 360deg);--x1xmdc3p:radial-gradient(rgba(255, 255, 255, 1), rgba(255, 255, 255, 0));}', 0);
_inject2("@media (prefers-color-scheme: dark){:root{--x9q2m40:255;--xpzz690:255;--x16lcx6o:255;--xjk46kt:rgb(0, 0, 0);--x19cfreg:0;--x5f91dp:0;--xtrkg9t:0;--xrj4b28:rgb(20, 20, 20);--x13ytpr0:rgba(20, 20, 20, 0.5);--xjray:108;--x1ats3qd:108;--x12b45e3:108;--x1efhglm:100;--x1w81gmp:100;--x13v9q97:100;--x14edl43:200;--xdi7wre:200;--x1livm2j:200;--x1r7qzpr:radial-gradient(rgba(1, 65, 255, 0.4), rgba(1, 65, 255, 0));--x1xmdc3p:linear-gradient(to bottom right, rgba(1, 65, 255, 0), rgba(1, 65, 255, 0), rgba(1, 65, 255, 0.3));}}", 0.1);
export const globalTokens = {
    maxWidth: "var(--xdkvadk)",
    fontMono: "var(--x1v9y61d)",
    fontSans: "var(--xu8xumw)",
    foregroundR: "var(--x9q2m40)",
    foregroundG: "var(--xpzz690)",
    foregroundB: "var(--x16lcx6o)",
    bgStartRGB: "var(--xjk46kt)",
    bgEndR: "var(--x19cfreg)",
    bgEndG: "var(--x5f91dp)",
    bgEndB: "var(--xtrkg9t)",
    calloutRGB: "var(--xrj4b28)",
    calloutRGB50: "var(--x13ytpr0)",
    calloutBorderR: "var(--xjray)",
    calloutBorderG: "var(--x1ats3qd)",
    calloutBorderB: "var(--x12b45e3)",
    cardR: "var(--x1efhglm)",
    cardG: "var(--x1w81gmp)",
    cardB: "var(--x13v9q97)",
    cardBorderR: "var(--x14edl43)",
    cardBorderG: "var(--xdi7wre)",
    cardBorderB: "var(--x1livm2j)",
    primaryGlow: "var(--x1r7qzpr)",
    secondaryGlow: "var(--x1xmdc3p)",
    __themeName__: "xsbqktv"
};
