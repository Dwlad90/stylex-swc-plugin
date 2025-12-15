import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    kxBb7d: "x16oeupf " + "xndy4z1",
    $$css: true
};
_inject2({
    ltr: ".x16oeupf::before{color:red}",
    priority: 8000
});
_inject2({
    ltr: ".xndy4z1::before:hover{color:var(--x-6bge3v)}",
    priority: 8130
});
_inject2({
    ltr: '@property --x-6bge3v { syntax: "*"; }',
    priority: 0
});
export const styles = {
    foo: (color)=>[
            _temp,
            {
                "--x-6bge3v": color != null ? color : undefined
            }
        ]
};
