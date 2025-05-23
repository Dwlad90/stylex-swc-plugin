import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
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
export const complex = {
    0: {
        class: "xrvj5dj x1rkzygb x7k18q3 x17lh93j xmr4b4k xesbpuc x15nfgh4"
    },
    4: {
        class: "xrvj5dj x7k18q3 x5gp9wm x1mkdm3x"
    },
    2: {
        class: "xrvj5dj x1rkzygb x7k18q3 x17lh93j xmr4b4k xesbpuc x15nfgh4 x9f619 x1yc5d2u"
    },
    6: {
        class: "xrvj5dj x7k18q3 x5gp9wm x1mkdm3x x9f619 x1yc5d2u"
    },
    1: {
        class: "xrvj5dj x1rkzygb x7k18q3 x17lh93j xmr4b4k xesbpuc x15nfgh4 x1fdo2jl"
    },
    5: {
        class: "xrvj5dj x7k18q3 x5gp9wm x1mkdm3x x1fdo2jl"
    },
    3: {
        class: "xrvj5dj x1rkzygb x7k18q3 x17lh93j xmr4b4k xesbpuc x15nfgh4 x9f619 x1fdo2jl"
    },
    7: {
        class: "xrvj5dj x7k18q3 x5gp9wm x1mkdm3x x9f619 x1fdo2jl"
    }
}[!!(sidebar == null && !isSidebar) << 2 | !!isSidebar << 1 | !!isContent << 0];
