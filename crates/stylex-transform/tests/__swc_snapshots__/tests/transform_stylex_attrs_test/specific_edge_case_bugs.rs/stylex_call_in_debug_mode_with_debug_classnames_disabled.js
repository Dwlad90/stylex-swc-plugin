import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x9f619{box-sizing:border-box}",
    priority: 3000
});
_inject2({
    ltr: ".x1yc5d2u{grid-area:sidebar}",
    priority: 1000
});
_inject2({
    ltr: ".x1fdo2jl{grid-area:content}",
    priority: 1000
});
_inject2({
    ltr: ".xrvj5dj{display:grid}",
    priority: 3000
});
_inject2({
    ltr: ".x7k18q3{grid-template-rows:100%}",
    priority: 3000
});
_inject2({
    ltr: '.x5gp9wm{grid-template-areas:"content"}',
    priority: 2000
});
_inject2({
    ltr: ".x1rkzygb{grid-template-columns:auto minmax(0,1fr)}",
    priority: 3000
});
_inject2({
    ltr: '.x17lh93j{grid-template-areas:"sidebar content"}',
    priority: 2000
});
_inject2({
    ltr: "@media (max-width: 640px){.xmr4b4k.xmr4b4k{grid-template-rows:minmax(0,1fr) auto}}",
    priority: 3200
});
_inject2({
    ltr: '@media (max-width: 640px){.xesbpuc.xesbpuc{grid-template-areas:"content" "sidebar"}}',
    priority: 2200
});
_inject2({
    ltr: "@media (max-width: 640px){.x15nfgh4.x15nfgh4{grid-template-columns:100%}}",
    priority: 3200
});
_inject2({
    ltr: ".x1mkdm3x{grid-template-columns:minmax(0,1fr)}",
    priority: 3000
});
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
