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
export const complex = {
    0: {
        className: "display-xrvj5dj gridTemplateColumns-x1rkzygb gridTemplateRows-x7k18q3 gridTemplateAreas-x17lh93j gridTemplateRows-xmr4b4k gridTemplateAreas-xesbpuc gridTemplateColumns-x15nfgh4",
        "data-style-src": "js/FooBar.react.js:10; js/FooBar.react.js:15"
    },
    4: {
        className: "display-xrvj5dj gridTemplateRows-x7k18q3 gridTemplateAreas-x5gp9wm gridTemplateColumns-x1mkdm3x",
        "data-style-src": "js/FooBar.react.js:10; js/FooBar.react.js:25"
    },
    2: {
        className: "display-xrvj5dj gridTemplateColumns-x1rkzygb gridTemplateRows-x7k18q3 gridTemplateAreas-x17lh93j gridTemplateRows-xmr4b4k gridTemplateAreas-xesbpuc gridTemplateColumns-x15nfgh4 boxSizing-x9f619 gridArea-x1yc5d2u",
        "data-style-src": "js/FooBar.react.js:10; js/FooBar.react.js:15; js/FooBar.react.js:3"
    },
    6: {
        className: "display-xrvj5dj gridTemplateRows-x7k18q3 gridTemplateAreas-x5gp9wm gridTemplateColumns-x1mkdm3x boxSizing-x9f619 gridArea-x1yc5d2u",
        "data-style-src": "js/FooBar.react.js:10; js/FooBar.react.js:25; js/FooBar.react.js:3"
    },
    1: {
        className: "display-xrvj5dj gridTemplateColumns-x1rkzygb gridTemplateRows-x7k18q3 gridTemplateAreas-x17lh93j gridTemplateRows-xmr4b4k gridTemplateAreas-xesbpuc gridTemplateColumns-x15nfgh4 gridArea-x1fdo2jl",
        "data-style-src": "js/FooBar.react.js:10; js/FooBar.react.js:15; js/FooBar.react.js:7"
    },
    5: {
        className: "display-xrvj5dj gridTemplateRows-x7k18q3 gridTemplateAreas-x5gp9wm gridTemplateColumns-x1mkdm3x gridArea-x1fdo2jl",
        "data-style-src": "js/FooBar.react.js:10; js/FooBar.react.js:25; js/FooBar.react.js:7"
    },
    3: {
        className: "display-xrvj5dj gridTemplateColumns-x1rkzygb gridTemplateRows-x7k18q3 gridTemplateAreas-x17lh93j gridTemplateRows-xmr4b4k gridTemplateAreas-xesbpuc gridTemplateColumns-x15nfgh4 boxSizing-x9f619 gridArea-x1fdo2jl",
        "data-style-src": "js/FooBar.react.js:10; js/FooBar.react.js:15; js/FooBar.react.js:3; js/FooBar.react.js:7"
    },
    7: {
        className: "display-xrvj5dj gridTemplateRows-x7k18q3 gridTemplateAreas-x5gp9wm gridTemplateColumns-x1mkdm3x boxSizing-x9f619 gridArea-x1fdo2jl",
        "data-style-src": "js/FooBar.react.js:10; js/FooBar.react.js:25; js/FooBar.react.js:3; js/FooBar.react.js:7"
    }
}[!!(sidebar == null && !isSidebar) << 2 | !!isSidebar << 1 | !!isContent << 0];
