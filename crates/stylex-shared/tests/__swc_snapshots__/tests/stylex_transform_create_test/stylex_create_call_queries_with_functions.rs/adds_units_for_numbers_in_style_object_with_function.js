import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".x1bl4301{width:var(--width)}", 4000);
_inject2('@property --width { syntax: "*"; inherits: false; }', 0);
export const styles = {
    default: (width)=>[
            {
                kWkggS: "xrkmrrc",
                kzqmXN: "x1bl4301",
                $$css: true
            },
            {
                "--width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width)
            }
        ]
};
