import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1717udv{padding:0}", 1000);
_inject2(".x1ghz6dp{margin:0}", 1000);
_inject2(".xe8uvvx{list-style:none}", 2000);
_inject2(".xrvj5dj{display:grid}", 3000);
_inject2(".x1mt1orb{grid-auto-flow:column}", 3000);
_inject2(".xh8yej3{width:100%}", 4000);
_inject2(".x1nhvcw1{justify-content:flex-start}", 3000);
_inject2(".x1q0q8m5{border-bottom-style:solid}", 4000);
_inject2(".xso031l{border-bottom-width:1px}", 4000);
const styles = {
    primaryVariant: {
        kmVPX3: "x1717udv",
        kg3NbH: null,
        kuDDbn: null,
        kE3dHu: null,
        kP0aTx: null,
        kpe85a: null,
        k8WAf4: null,
        kLKAdn: null,
        kGO01o: null,
        kogj98: "x1ghz6dp",
        kUOVxO: null,
        keTefX: null,
        koQZXg: null,
        k71WvV: null,
        km5ZXQ: null,
        kqGvvJ: null,
        keoZOQ: null,
        k1K539: null,
        kB88ic: "xe8uvvx",
        khnUzm: null,
        kpqbRz: null,
        kH6xsr: null,
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
    const variantStyle = `${variant}Variant`;
    return <div {...stylex.props(styles[variantStyle])}/>;
}
