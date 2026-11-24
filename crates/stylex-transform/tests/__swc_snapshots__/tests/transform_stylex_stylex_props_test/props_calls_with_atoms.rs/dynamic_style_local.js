import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".x14rh7hd{color:var(--x-color)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-color { syntax: "*"; inherits: false;}',
    priority: 0
});
const styles = {
    color: (c)=>[
            c !== undefined && {
                kMwMTN: c != null ? "x14rh7hd" : "",
                $$css: true
            },
            {
                "--x-color": c != null ? c : undefined
            }
        ]
};
stylex.props(styles.color(color));
