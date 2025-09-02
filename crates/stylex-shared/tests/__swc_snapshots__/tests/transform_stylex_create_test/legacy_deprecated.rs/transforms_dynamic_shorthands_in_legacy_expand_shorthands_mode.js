import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
const _temp = {
    kWkggS: "xrkmrrc",
    keoZOQ: "x1gkbulp",
    $$css: true
};
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".x17e2bsb{margin-inline-end:var(--x-14mfytm)}", 3000);
_inject2(".xtcj1g9:hover{margin-inline-end:var(--x-yepcm9)}", 3130);
_inject2(".xg6eqc8{margin-bottom:var(--x-14mfytm)}", 4000);
_inject2(".xgrn1a3:hover{margin-bottom:var(--x-yepcm9)}", 4130);
_inject2(".x19ja4a5{margin-inline-start:var(--x-14mfytm)}", 3000);
_inject2(".x2tye95:hover{margin-inline-start:var(--x-yepcm9)}", 3130);
_inject2(".x1gkbulp{margin-top:var(--x-marginTop)}", 4000);
_inject2('@property --x-14mfytm { syntax: "*"; inherits: false; }', 0);
_inject2('@property --x-yepcm9 { syntax: "*"; }', 0);
_inject2('@property --x-marginTop { syntax: "*"; inherits: false; }', 0);
export const styles = {
    default: (margin)=>[
            _temp,
            {
                k71WvV: (margin != null ? "x17e2bsb" : margin) + "xtcj1g9",
                k1K539: (margin != null ? "xg6eqc8" : margin) + "xgrn1a3",
                keTefX: (margin != null ? "x19ja4a5" : margin) + "x2tye95",
                $$css: true
            },
            {
                "--x-14mfytm": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin),
                "--x-yepcm9": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin + 4),
                "--x-marginTop": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin - 4)
            }
        ]
};
