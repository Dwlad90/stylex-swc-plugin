import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".boxSizing-x9f619{box-sizing:border-box}", 3000);
_inject2(".gridArea-x1yc5d2u{grid-area:sidebar}", 1000);
_inject2(".gridArea-x1fdo2jl{grid-area:content}", 1000);
_inject2(".display-xrvj5dj{display:grid}", 3000);
_inject2(".gridTemplateRows-x7k18q3{grid-template-rows:100%}", 3000);
_inject2('.gridTemplateAreas-x5gp9wm{grid-template-areas:"content"}', 2000);
_inject2(".gridTemplateColumns-x1rkzygb{grid-template-columns:auto minmax(0,1fr)}", 3000);
_inject2('.gridTemplateAreas-x17lh93j{grid-template-areas:"sidebar content"}', 2000);
_inject2("@media (max-width: 640px){.gridTemplateRows-xmr4b4k.gridTemplateRows-xmr4b4k{grid-template-rows:minmax(0,1fr) auto}}", 3200);
_inject2('@media (max-width: 640px){.gridTemplateAreas-xesbpuc.gridTemplateAreas-xesbpuc{grid-template-areas:"content" "sidebar"}}', 2200);
_inject2("@media (max-width: 640px){.gridTemplateColumns-x15nfgh4.gridTemplateColumns-x15nfgh4{grid-template-columns:100%}}", 3200);
_inject2(".gridTemplateColumns-x1mkdm3x{grid-template-columns:minmax(0,1fr)}", 3000);
export const styles = {
    sidebar: {
        "boxSizing-kB7OPa": "boxSizing-x9f619",
        "gridArea-kJuA4N": "gridArea-x1yc5d2u",
        $$css: true
    },
    content: {
        "gridArea-kJuA4N": "gridArea-x1fdo2jl",
        $$css: true
    },
    root: {
        "display-k1xSpc": "display-xrvj5dj",
        "gridTemplateRows-k9llMU": "gridTemplateRows-x7k18q3",
        "gridTemplateAreas-kC13JO": "gridTemplateAreas-x5gp9wm",
        $$css: true
    },
    withSidebar: {
        "gridTemplateColumns-kumcoG": "gridTemplateColumns-x1rkzygb",
        "gridTemplateRows-k9llMU": "gridTemplateRows-x7k18q3",
        "gridTemplateAreas-kC13JO": "gridTemplateAreas-x17lh93j",
        "@media (max-width: 640px)_gridTemplateRows-k9pwkU": "gridTemplateRows-xmr4b4k",
        "@media (max-width: 640px)_gridTemplateAreas-kOnEH4": "gridTemplateAreas-xesbpuc",
        "@media (max-width: 640px)_gridTemplateColumns-k1JLwA": "gridTemplateColumns-x15nfgh4",
        $$css: true
    },
    noSidebar: {
        "gridTemplateColumns-kumcoG": "gridTemplateColumns-x1mkdm3x",
        $$css: true
    }
};
stylex(styles.root, sidebar == null ? styles.noSidebar : styles.withSidebar);
