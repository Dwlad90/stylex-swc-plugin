//__stylex_metadata_start__[{"class_name":"xrkmrrc","style":{"rtl":null,"ltr":".xrkmrrc{background-color:red}"},"priority":3000},{"class_name":"xfx01vb","style":{"rtl":null,"ltr":".xfx01vb{color:var(--color)}"},"priority":3000},{"class_name":"x1mqxbix","style":{"rtl":null,"ltr":".x1mqxbix{color:black}"},"priority":3000}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".xfx01vb{color:var(--color)}", 3000);
_inject2(".x1mqxbix{color:black}", 3000);
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
        ],
    mono: {
        color: "x1mqxbix",
        $$css: true
    }
};
