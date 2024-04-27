import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from "@stylexjs/stylex";
_inject2(".xq1mx2j{background-color:var(--backgroundColor,revert)}", 3000);
export const styles = {
    dynamic: (backgroundColor)=>[
            {
                backgroundColor: "xq1mx2j",
                $$css: true
            },
            {
                "--backgroundColor": backgroundColor != null ? backgroundColor : "initial"
            }
        ]
};
