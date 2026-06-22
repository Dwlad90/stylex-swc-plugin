import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
import css from '@stylexjs/atoms';
_inject2({
    ltr: ".xe8ttls{padding:8px}",
    priority: 1000
});
_inject2({
    ltr: ".x12oqio5{border-radius:4px}",
    priority: 2000
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
    kmVPX3: "xe8ttls",
    $$css: true
};
const _temp3 = {
    kaIpWk: "x12oqio5",
    $$css: true
};
function Button({ isActive, color }) {
    return stylex.props(_temp2, _temp3, isActive && _temp.color(color));
}
