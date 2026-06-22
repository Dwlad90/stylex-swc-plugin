import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
import css from '@stylexjs/atoms';
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
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
const _temp2 = {
    k1xSpc: "x78zum5",
    $$css: true
};
stylex.props(_temp2, _temp.color(color));
