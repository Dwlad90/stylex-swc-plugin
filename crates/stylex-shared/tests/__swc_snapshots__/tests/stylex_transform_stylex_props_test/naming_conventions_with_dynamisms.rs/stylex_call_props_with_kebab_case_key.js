//__stylex_metadata_start__[{"class_name":"x1717udv","style":{"rtl":null,"ltr":".x1717udv{padding:0}"},"priority":1000},{"class_name":"x1ghz6dp","style":{"rtl":null,"ltr":".x1ghz6dp{margin:0}"},"priority":1000},{"class_name":"xe8uvvx","style":{"rtl":null,"ltr":".xe8uvvx{list-style:none}"},"priority":2000},{"class_name":"xrvj5dj","style":{"rtl":null,"ltr":".xrvj5dj{display:grid}"},"priority":3000},{"class_name":"x1mt1orb","style":{"rtl":null,"ltr":".x1mt1orb{grid-auto-flow:column}"},"priority":3000},{"class_name":"xh8yej3","style":{"rtl":null,"ltr":".xh8yej3{width:100%}"},"priority":4000},{"class_name":"x1nhvcw1","style":{"rtl":null,"ltr":".x1nhvcw1{justify-content:flex-start}"},"priority":3000},{"class_name":"x1q0q8m5","style":{"rtl":null,"ltr":".x1q0q8m5{border-bottom-style:solid}"},"priority":4000},{"class_name":"xso031l","style":{"rtl":null,"ltr":".xso031l{border-bottom-width:1px}"},"priority":4000}]__stylex_metadata_end__
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
    "primary-variant": {
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
    const variantStyle = `${variant}-variant`;
    return <div {...stylex.props(styles[variantStyle])}/>;
}
