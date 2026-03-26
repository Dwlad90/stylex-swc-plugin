import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".x1717udv{padding:0}",
    priority: 1000
});
_inject2({
    ltr: ".x1ghz6dp{margin:0}",
    priority: 1000
});
_inject2({
    ltr: ".xe8uvvx{list-style:none}",
    priority: 2000
});
_inject2({
    ltr: ".xrvj5dj{display:grid}",
    priority: 3000
});
_inject2({
    ltr: ".x1mt1orb{grid-auto-flow:column}",
    priority: 3000
});
_inject2({
    ltr: ".xh8yej3{width:100%}",
    priority: 4000
});
_inject2({
    ltr: ".x1nhvcw1{justify-content:flex-start}",
    priority: 3000
});
_inject2({
    ltr: ".x1q0q8m5{border-bottom-style:solid}",
    priority: 4000
});
_inject2({
    ltr: ".xso031l{border-bottom-width:1px}",
    priority: 4000
});
const styles = {
    "primary-variant": {
        kmVPX3: "x1717udv",
        kogj98: "x1ghz6dp",
        kB88ic: "xe8uvvx",
        k1xSpc: "xrvj5dj",
        kprqdN: "x1mt1orb",
        kzqmXN: "xh8yej3",
        kjj79g: "x1nhvcw1",
        kfdmCh: "x1q0q8m5",
        kt9PQ7: "xso031l",
        $$css: true
    }
};
function TestComponent({ variant }) {
    const variantStyle = `${variant}-variant`;
    return <div {...stylex.props(styles[variantStyle])}/>;
}
