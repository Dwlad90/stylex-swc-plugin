import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1gykpug:hover{background-color:red}", 3130);
_inject2(".x11bf1mc:hover{color:var(--1ijzsae,revert)}", 3130);
export const styles = {
    default: color => [{
        ":hover_backgroundColor": "x1gykpug",
        ":hover_color": "x11bf1mc",
        $$css: true
    }, {
        "--1ijzsae": color != null ? color : "initial"
    }]
};
