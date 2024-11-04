//__stylex_metadata_start__[{"class_name":"x1sm8rlu","style":{"rtl":null,"ltr":"@property --x1sm8rlu { syntax: \"<color>\"; inherits: true; initial-value: blue }"},"priority":0},{"class_name":"xxncinc","style":{"rtl":null,"ltr":"@property --xxncinc { syntax: \"<color>\"; inherits: true; initial-value: grey }"},"priority":0},{"class_name":"x4e1236","style":{"rtl":null,"ltr":"@property --x4e1236 { syntax: \"<length>\"; inherits: true; initial-value: 10px }"},"priority":0},{"class_name":"xv9uic","style":{"rtl":null,"ltr":"@property --xv9uic { syntax: \"<color>\"; inherits: true; initial-value: pink }"},"priority":0},{"class_name":"xmpye33","style":{"rtl":null,"ltr":":root, .xmpye33{--x1sm8rlu:blue;--xxncinc:grey;--x4e1236:10px;--xv9uic:pink;}"},"priority":0},{"class_name":"xmpye33-1lveb7","style":{"rtl":null,"ltr":"@media (prefers-color-scheme: dark){:root, .xmpye33{--x1sm8rlu:lightblue;--xxncinc:rgba(0, 0, 0, 0.8);}}"},"priority":0.1},{"class_name":"xmpye33-bdddrq","style":{"rtl":null,"ltr":"@media print{:root, .xmpye33{--x1sm8rlu:white;}}"},"priority":0.1}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2('@property --x1sm8rlu { syntax: "<color>"; inherits: true; initial-value: blue }', 0);
_inject2('@property --xxncinc { syntax: "<color>"; inherits: true; initial-value: grey }', 0);
_inject2('@property --x4e1236 { syntax: "<length>"; inherits: true; initial-value: 10px }', 0);
_inject2('@property --xv9uic { syntax: "<color>"; inherits: true; initial-value: pink }', 0);
_inject2(":root, .xmpye33{--x1sm8rlu:blue;--xxncinc:grey;--x4e1236:10px;--xv9uic:pink;}", 0);
_inject2("@media (prefers-color-scheme: dark){:root, .xmpye33{--x1sm8rlu:lightblue;--xxncinc:rgba(0, 0, 0, 0.8);}}", 0.1);
_inject2("@media print{:root, .xmpye33{--x1sm8rlu:white;}}", 0.1);
export const buttonTheme = {
    bgColor: "var(--x1sm8rlu)",
    bgColorDisabled: "var(--xxncinc)",
    cornerRadius: "var(--x4e1236)",
    fgColor: "var(--xv9uic)",
    __themeName__: "xmpye33"
};
