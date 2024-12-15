//__stylex_metadata_start__[{"class_name":"xrkmrrc","style":{"rtl":null,"ltr":".xrkmrrc{background-color:red}"},"priority":3000},{"class_name":"x1bl4301","style":{"rtl":null,"ltr":".x1bl4301{width:var(--width)}"},"priority":4000},{"class_name":"--width","style":{"rtl":null,"ltr":"@property --width { syntax: \"*\"; inherits: false; initial-value: \"*\"; }"},"priority":0}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".x1bl4301{width:var(--width)}", 4000);
_inject2('@property --width { syntax: "*"; inherits: false; initial-value: "*"; }', 0);
export const styles = {
    default: (width)=>[
            {
                backgroundColor: "xrkmrrc",
                width: width == null ? null : "x1bl4301",
                $$css: true
            },
            {
                "--width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width)
            }
        ]
};
