//__stylex_metadata_start__[{"class_name":"xrkmrrc","style":{"rtl":null,"ltr":".xrkmrrc{background-color:red}"},"priority":3000},{"class_name":"x1ie72y1","style":{"rtl":".x1ie72y1{margin-left:var(--14mfytm)}","ltr":".x1ie72y1{margin-right:var(--14mfytm)}"},"priority":3000},{"class_name":"x128459","style":{"rtl":".x128459:hover{margin-left:var(--yepcm9)}","ltr":".x128459:hover{margin-right:var(--yepcm9)}"},"priority":3130},{"class_name":"x1hvr6ea","style":{"rtl":null,"ltr":".x1hvr6ea{margin-bottom:var(--14mfytm)}"},"priority":4000},{"class_name":"x3skgmg","style":{"rtl":null,"ltr":".x3skgmg:hover{margin-bottom:var(--yepcm9)}"},"priority":4130},{"class_name":"x1k44ad6","style":{"rtl":".x1k44ad6{margin-right:var(--14mfytm)}","ltr":".x1k44ad6{margin-left:var(--14mfytm)}"},"priority":3000},{"class_name":"x10ktymb","style":{"rtl":".x10ktymb:hover{margin-right:var(--yepcm9)}","ltr":".x10ktymb:hover{margin-left:var(--yepcm9)}"},"priority":3130},{"class_name":"x17zef60","style":{"rtl":null,"ltr":".x17zef60{margin-top:var(--marginTop)}"},"priority":4000}]__stylex_metadata_end__
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
