import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    kxBb7d: "x16oeupf" + "xndy4z1",
    $$css: true
};
_inject2(".x16oeupf::before{color:red}", 8000);
_inject2(".xndy4z1::before:hover{color:var(--x-6bge3v)}", 8130);
_inject2('@property --x-6bge3v { syntax: "*"; }', 0);
export const styles = {
    foo: (color)=>[
            _temp,
            {
                "--x-6bge3v": color != null ? color : undefined
            }
        ]
};
