import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2('@property --x1sm8rlu { syntax: "<color>"; inherits: true; initial-value: blue }', 0);
_inject2('@property --xxncinc { syntax: "<color>"; inherits: true; initial-value: grey }', 0);
_inject2('@property --x4e1236 { syntax: "<length>"; inherits: true; initial-value: 10px }', 0);
_inject2('@property --xv9uic { syntax: "<color>"; inherits: true; initial-value: pink }', 0);
_inject2(":root{--x1sm8rlu:blue;--xxncinc:grey;--x4e1236:10px;--xv9uic:pink;}", 0);
_inject2("@media (prefers-color-scheme: dark){:root{--x1sm8rlu:lightblue;--xxncinc:rgba(0, 0, 0, 0.8);}}", 0.1);
_inject2("@media print{:root{--x1sm8rlu:white;}}", 0.1);
export const buttonTheme = {
    bgColor: "var(--x1sm8rlu)",
    bgColorDisabled: "var(--xxncinc)",
    cornerRadius: "var(--x4e1236)",
    fgColor: "var(--xv9uic)",
    __themeName__: "xmpye33"
};
