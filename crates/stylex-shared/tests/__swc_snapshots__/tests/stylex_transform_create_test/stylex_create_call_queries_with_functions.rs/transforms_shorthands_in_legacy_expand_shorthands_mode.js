import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".x1ie72y1{margin-right:var(--14mfytm)}", 3000, ".x1ie72y1{margin-left:var(--14mfytm)}");
_inject2(".x128459:hover{margin-right:var(--yepcm9)}", 3130, ".x128459:hover{margin-left:var(--yepcm9)}");
_inject2(".x1hvr6ea{margin-bottom:var(--14mfytm)}", 4000);
_inject2(".x3skgmg:hover{margin-bottom:var(--yepcm9)}", 4130);
_inject2(".x1k44ad6{margin-left:var(--14mfytm)}", 3000, ".x1k44ad6{margin-right:var(--14mfytm)}");
_inject2(".x10ktymb:hover{margin-left:var(--yepcm9)}", 3130, ".x10ktymb:hover{margin-right:var(--yepcm9)}");
_inject2(".x17zef60{margin-top:var(--marginTop)}", 4000);
_inject2('@property --14mfytm { syntax: "*"; inherits: false; initial-value: "*"; }', 0);
_inject2('@property --yepcm9 { syntax: "*"; inherits: false; initial-value: "*"; }', 0);
_inject2('@property --marginTop { syntax: "*"; inherits: false; initial-value: "*"; }', 0);
export const styles = {
    default: (margin)=>[
            {
                backgroundColor: "xrkmrrc",
                marginEnd: (margin == null ? "" : "x1ie72y1 ") + (margin + 4 == null ? "" : "x128459"),
                marginBottom: (margin == null ? "" : "x1hvr6ea ") + (margin + 4 == null ? "" : "x3skgmg"),
                marginStart: (margin == null ? "" : "x1k44ad6 ") + (margin + 4 == null ? "" : "x10ktymb"),
                marginTop: margin - 4 == null ? null : "x17zef60",
                $$css: true
            },
            {
                "--14mfytm": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin),
                "--yepcm9": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin + 4),
                "--marginTop": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(margin - 4)
            }
        ]
};
