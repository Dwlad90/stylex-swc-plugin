//__stylex_metadata_start__[{"class_name":"xyv4n8w","style":{"rtl":null,"ltr":".xyv4n8w{--background-color:var(----background-color,revert)}"},"priority":1}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xyv4n8w{--background-color:var(----background-color,revert)}", 1);
export const styles = {
    default: (bgColor)=>[
            {
                "--background-color": "xyv4n8w",
                $$css: true
            },
            {
                "----background-color": bgColor != null ? bgColor : "initial"
            }
        ]
};
