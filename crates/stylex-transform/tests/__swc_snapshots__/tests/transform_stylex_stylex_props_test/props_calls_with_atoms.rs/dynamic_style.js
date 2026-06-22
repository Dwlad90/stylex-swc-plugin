import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
import css from '@stylexjs/atoms';
_inject2({
    ltr: ".x14rh7hd{color:var(--x-color)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-color { syntax: "*"; inherits: false;}',
    priority: 0
});
const _temp = {
    color: (_v)=>[
            {
                "kMwMTN": _v != null ? "x14rh7hd" : _v,
                "$$css": true
            },
            {
                "--x-color": _v != null ? _v : undefined
            }
        ]
};
stylex.props(_temp.color(color));
