import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(":root{--xgck17p:blue;--xpegid5:grey;--xrqfjmn:10;--x4y59db:pink;}", 0);
_inject2("@media (prefers-color-scheme: dark){:root{--xgck17p:lightblue;--xpegid5:rgba(0, 0, 0, 0.8);}}", 0.1);
_inject2("@media print{:root{--xgck17p:white;}}", 0.1);
export const buttonTheme = {
    bgColor: "var(--xgck17p)",
    bgColorDisabled: "var(--xpegid5)",
    cornerRadius: "var(--xrqfjmn)",
    fgColor: "var(--x4y59db)",
    __themeName__: "x568ih9"
};
