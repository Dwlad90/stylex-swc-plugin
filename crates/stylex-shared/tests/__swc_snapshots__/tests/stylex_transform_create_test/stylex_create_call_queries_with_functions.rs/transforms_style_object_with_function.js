//__stylex_metadata_start__[{"class_name":"xrkmrrc","style":{"rtl":null,"ltr":".xrkmrrc{background-color:red}"},"priority":3000},{"class_name":"xfx01vb","style":{"rtl":null,"ltr":".xfx01vb{color:var(--color)}"},"priority":3000},{"class_name":"--color","style":{"rtl":null,"ltr":"@property --color { syntax: \"*\"; inherits: false; initial-value: \"*\"; }"},"priority":0}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".xfx01vb{color:var(--color)}", 3000);
_inject2('@property --color { syntax: "*"; inherits: false; initial-value: "*"; }', 0);
export const styles = {
    default: (color)=>[
            {
                backgroundColor: "xrkmrrc",
                color: color == null ? null : "xfx01vb",
                $$css: true
            },
            {
                "--color": color != null ? color : undefined
            }
        ]
};
