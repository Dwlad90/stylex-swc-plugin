import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from "@stylexjs/stylex";
_inject2(".xr5ldyu{background-color:var(--backgroundColor)}", 3000);
_inject2('@property --backgroundColor { syntax: "*"; inherits: false; }', 0);
export const styles = {
    dynamic: (backgroundColor)=>[
            {
                backgroundColor: backgroundColor == null ? null : "xr5ldyu",
                $$css: true
            },
            {
                "--backgroundColor": backgroundColor != null ? backgroundColor : undefined
            }
        ]
};
