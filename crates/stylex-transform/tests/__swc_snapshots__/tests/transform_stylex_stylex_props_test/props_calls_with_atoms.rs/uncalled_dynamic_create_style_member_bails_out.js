import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
import css from '@stylexjs/atoms';
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
const _temp = {
    k1xSpc: "x78zum5",
    $$css: true
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
stylex.props(_temp, styles.opacity);
