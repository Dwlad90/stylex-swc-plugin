import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".x1555q52{margin-inline-end:var(--14mfytm)}", 3000);
_inject2(".x1bi16m7:hover{margin-inline-end:var(--yepcm9)}", 3130);
_inject2(".x1hvr6ea{margin-bottom:var(--14mfytm)}", 4000);
_inject2(".x3skgmg:hover{margin-bottom:var(--yepcm9)}", 4130);
_inject2(".x1feukp3{margin-inline-start:var(--14mfytm)}", 3000);
_inject2(".xgzim5p:hover{margin-inline-start:var(--yepcm9)}", 3130);
_inject2(".x17zef60{margin-top:var(--marginTop)}", 4000);
_inject2('@property --14mfytm { syntax: "*"; inherits: false; }', 0);
_inject2('@property --yepcm9 { syntax: "*"; }', 0);
_inject2('@property --marginTop { syntax: "*"; inherits: false; }', 0);
export const styles = {
    default: (margin)=>[
            {
                kWkggS: "xrkmrrc",
                k71WvV: "x1555q52 x1bi16m7",
                k1K539: "x1hvr6ea x3skgmg",
                keTefX: "x1feukp3 xgzim5p",
                keoZOQ: "x17zef60",
                $$css: true
            },
            {
                "--14mfytm": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin),
                "--yepcm9": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin + 4),
                "--marginTop": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin - 4)
            }
        ]
};
