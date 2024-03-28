import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from '@stylexjs/stylex';
_inject2(".x9f619{box-sizing:border-box}", 3000);
_inject2(".x1yc5d2u{grid-area:sidebar}", 1000);
_inject2(".x1fdo2jl{grid-area:content}", 1000);
_inject2(".xrvj5dj{display:grid}", 3000);
_inject2(".x7k18q3{grid-template-rows:100%}", 3000);
_inject2('.x5gp9wm{grid-template-areas:"content"}', 2000);
_inject2(".x1rkzygb{grid-template-columns:auto minmax(0,1fr)}", 3000);
_inject2('.x17lh93j{grid-template-areas:"sidebar content"}', 2000);
_inject2("@media (max-width: 640px){.xmr4b4k.xmr4b4k{grid-template-rows:minmax(0,1fr) auto}}", 3200);
_inject2('@media (max-width: 640px){.xesbpuc.xesbpuc{grid-template-areas:"content" "sidebar"}}', 2200);
_inject2("@media (max-width: 640px){.x15nfgh4.x15nfgh4{grid-template-columns:100%}}", 3200);
_inject2(".x1mkdm3x{grid-template-columns:minmax(0,1fr)}", 3000);
export const styles = {
    sidebar: {
        "UnknownFile__styles.sidebar": "UnknownFile__styles.sidebar",
        boxSizing: "x9f619",
        gridArea: "x1yc5d2u",
        gridRow: null,
        gridRowStart: null,
        gridRowEnd: null,
        gridColumn: null,
        gridColumnStart: null,
        gridColumnEnd: null,
        $$css: true
    },
    content: {
        "UnknownFile__styles.content": "UnknownFile__styles.content",
        gridArea: "x1fdo2jl",
        gridRow: null,
        gridRowStart: null,
        gridRowEnd: null,
        gridColumn: null,
        gridColumnStart: null,
        gridColumnEnd: null,
        $$css: true
    },
    root: {
        "UnknownFile__styles.root": "UnknownFile__styles.root",
        display: "xrvj5dj",
        gridTemplateRows: "x7k18q3",
        gridTemplateAreas: "x5gp9wm",
        $$css: true
    },
    withSidebar: {
        "UnknownFile__styles.withSidebar": "UnknownFile__styles.withSidebar",
        gridTemplateColumns: "x1rkzygb",
        gridTemplateRows: "x7k18q3",
        gridTemplateAreas: "x17lh93j",
        "@media (max-width: 640px)_gridTemplateRows": "xmr4b4k",
        "@media (max-width: 640px)_gridTemplateAreas": "xesbpuc",
        "@media (max-width: 640px)_gridTemplateColumns": "x15nfgh4",
        $$css: true
    },
    noSidebar: {
        "UnknownFile__styles.noSidebar": "UnknownFile__styles.noSidebar",
        gridTemplateColumns: "x1mkdm3x",
        $$css: true
    }
};
({
    0: {
        class: "UnknownFile__styles.root xrvj5dj UnknownFile__styles.withSidebar x1rkzygb x7k18q3 x17lh93j xmr4b4k xesbpuc x15nfgh4"
    },
    1: {
        class: "UnknownFile__styles.root xrvj5dj x7k18q3 x5gp9wm UnknownFile__styles.noSidebar x1mkdm3x"
    }
})[!!(sidebar == null) << 0];
