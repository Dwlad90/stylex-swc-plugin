import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    kxBb7d: "x16oeupf" + "x10u3axo",
    $$css: true
};
_inject2(".x16oeupf::before{color:red}", 8000);
_inject2(".x10u3axo::before:hover{color:var(--6bge3v)}", 8130);
_inject2('@property --6bge3v { syntax: "*"; }', 0);
export const styles = {
    foo: (color)=>[
            _temp,
            {
                "--6bge3v": color != null ? color : undefined
            }
        ]
};
