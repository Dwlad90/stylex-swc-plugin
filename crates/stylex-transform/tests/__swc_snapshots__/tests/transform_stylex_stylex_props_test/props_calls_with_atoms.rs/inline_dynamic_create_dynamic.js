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
_inject2({
    ltr: ".xb4nw82{opacity:var(--x-opacity)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-opacity { syntax: "*"; inherits: false;}',
    priority: 0
});
const styles = {
    opacity: (o)=>[
            {
                kSiTet: o != null ? "xb4nw82" : o,
                $$css: true
            },
            {
                "--x-opacity": o != null ? o : undefined
            }
        ]
};
stylex.props(_temp.color(color), styles.opacity(0.5));
