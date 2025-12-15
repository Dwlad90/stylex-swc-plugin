import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
const _temp = {
    kWkggS: "xrkmrrc",
    keoZOQ: "x1gkbulp",
    $$css: true
};
_inject2({
    ltr: ".xrkmrrc{background-color:red}",
    priority: 3000
});
_inject2({
    ltr: ".x17e2bsb{margin-inline-end:var(--x-14mfytm)}",
    priority: 3000
});
_inject2({
    ltr: ".xtcj1g9:hover{margin-inline-end:var(--x-yepcm9)}",
    priority: 3130
});
_inject2({
    ltr: ".xg6eqc8{margin-bottom:var(--x-14mfytm)}",
    priority: 4000
});
_inject2({
    ltr: ".xgrn1a3:hover{margin-bottom:var(--x-yepcm9)}",
    priority: 4130
});
_inject2({
    ltr: ".x19ja4a5{margin-inline-start:var(--x-14mfytm)}",
    priority: 3000
});
_inject2({
    ltr: ".x2tye95:hover{margin-inline-start:var(--x-yepcm9)}",
    priority: 3130
});
_inject2({
    ltr: ".x1gkbulp{margin-top:var(--x-marginTop)}",
    priority: 4000
});
_inject2({
    ltr: '@property --x-14mfytm { syntax: "*"; inherits: false; }',
    priority: 0
});
_inject2({
    ltr: '@property --x-yepcm9 { syntax: "*"; }',
    priority: 0
});
_inject2({
    ltr: '@property --x-marginTop { syntax: "*"; inherits: false; }',
    priority: 0
});
export const styles = {
    default: (margin)=>[
            _temp,
            {
                k71WvV: (margin != null ? "x17e2bsb " : margin) + "xtcj1g9",
                k1K539: (margin != null ? "xg6eqc8 " : margin) + "xgrn1a3",
                keTefX: (margin != null ? "x19ja4a5 " : margin) + "x2tye95",
                $$css: true
            },
            {
                "--x-14mfytm": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin),
                "--x-yepcm9": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin + 4),
                "--x-marginTop": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin - 4)
            }
        ]
};
