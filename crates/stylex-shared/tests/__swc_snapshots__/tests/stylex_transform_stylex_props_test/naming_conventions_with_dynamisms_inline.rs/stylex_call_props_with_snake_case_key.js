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
    primary_variant: {
        padding: "x1717udv",
        paddingInline: null,
        paddingStart: null,
        paddingLeft: null,
        paddingEnd: null,
        paddingRight: null,
        paddingBlock: null,
        paddingTop: null,
        paddingBottom: null,
        margin: "x1ghz6dp",
        marginInline: null,
        marginInlineStart: null,
        marginLeft: null,
        marginInlineEnd: null,
        marginRight: null,
        marginBlock: null,
        marginTop: null,
        marginBottom: null,
        listStyle: "xe8uvvx",
        listStyleImage: null,
        listStylePosition: null,
        listStyleType: null,
        display: "xrvj5dj",
        gridAutoFlow: "x1mt1orb",
        width: "xh8yej3",
        justifyContent: "x1nhvcw1",
        borderBottomStyle: "x1q0q8m5",
        borderBottomWidth: "xso031l",
        $$css: true
    }
};
function TestComponent({ variant }) {
    return <div {...stylex.props(styles[`${variant}_variant`])}/>;
}
