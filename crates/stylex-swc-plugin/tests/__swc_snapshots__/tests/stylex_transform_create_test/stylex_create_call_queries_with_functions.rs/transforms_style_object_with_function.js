//__stylex_metadata_start__[{"class_name":"xrkmrrc","style":{"rtl":null,"ltr":".xrkmrrc{background-color:red}"},"priority":3000},{"class_name":"x19dipnz","style":{"rtl":null,"ltr":".x19dipnz{color:var(--color,revert)}"},"priority":3000}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".x19dipnz{color:var(--color,revert)}", 3000);
export const styles = {
    default: (color)=>[
            {
                backgroundColor: "xrkmrrc",
                color: "x19dipnz",
                $$css: true
            },
            {
                "--color": color != null ? color : "initial"
            }
        ]
};
