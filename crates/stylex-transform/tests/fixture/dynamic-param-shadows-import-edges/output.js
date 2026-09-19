import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./vars/spacing.stylex.js";
import "./vars/zIndex.stylex.js";
import "./vars/grid.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { zIndex } from './vars/zIndex.stylex.js';
import { spacing as ünïcödé } from './vars/spacing.stylex.js';
import { firstThatWorks } from './vars/legacy.stylex.js';
import { grid } from './vars/grid.stylex.js';
const _temp = {
    "input__styles.unicodeParam": "input__styles.unicodeParam",
    $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:11"
};
const _temp2 = {
    "input__styles.escapedParam": "input__styles.escapedParam",
    $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:15"
};
const _temp3 = {
    "input__styles.helperName": "input__styles.helperName",
    $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:19"
};
const _temp4 = {
    "input__styles.shorthand": "input__styles.shorthand",
    $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:23"
};
const _temp5 = {
    "input__styles.customProperty": "input__styles.customProperty",
    $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:26"
};
const _temp6 = {
    "input__styles.prefixed": "input__styles.prefixed",
    $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:29"
};
const _temp7 = {
    "input__styles.deeplyNested": "input__styles.deeplyNested",
    $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:33"
};
const _temp8 = {
    "input__styles.computedFromParam": "input__styles.computedFromParam",
    zIndex: "xr3buco",
    content: "x1p70blb",
    width: "x5lhr3w",
    $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:60"
};
const _temp9 = {
    "input__styles.mixedParams": "input__styles.mixedParams",
    $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:67"
};
_inject2({
    ltr: ".x1rqil9o{padding:var(--xq1l1nf)}",
    priority: 1000
});
_inject2({
    ltr: ".x1fozly0{padding:var(--x-padding)}",
    priority: 1000
});
_inject2({
    ltr: ".xb9ncqk{margin:var(--x-margin)}",
    priority: 1000
});
_inject2({
    ltr: ".xk2v41j{font-family:var(--x-fontFamily)}",
    priority: 3000
});
_inject2({
    ltr: ".xccw97s{inset:var(--x-inset)}",
    priority: 1000
});
_inject2({
    ltr: ".xvlecxo{margin-inline:var(--x-marginInline)}",
    priority: 2000
});
_inject2({
    ltr: ".x5h8hlk{--depth:var(--x---depth)}",
    priority: 1
});
_inject2({
    ltr: ".x91d7kb{--nested-depth:var(--x---nested-depth)}",
    priority: 1
});
_inject2({
    ltr: ".x9pkiyq{user-select:var(--x-userSelect)}",
    priority: 3000
});
_inject2({
    ltr: ".xafmcc1{appearance:var(--x-appearance)}",
    priority: 3000
});
_inject2({
    ltr: ".xkrcnwa{z-index:var(--x-gsepj1)}",
    priority: 3000
});
_inject2({
    ltr: ".x141uv47:hover{z-index:var(--x-1ua3n7y)}",
    priority: 3130
});
_inject2({
    ltr: ".x140siia:focus:hover{z-index:var(--x-kat1qs)}",
    priority: 3280
});
_inject2({
    ltr: "@media (min-width: 600px){.xlzq18l.xlzq18l:focus:hover{z-index:var(--x-16ne5w5)}}",
    priority: 3480
});
_inject2({
    ltr: "@supports (display: grid){@media (min-width: 600px){.x103ewrf.x103ewrf.x103ewrf:focus:hover{z-index:var(--x-iny62n)}}}",
    priority: 3510
});
_inject2({
    ltr: "@supports (display: grid){@media (min-width: 600px){.x1gxqx9w.x1gxqx9w.x1gxqx9w:active:focus:hover{z-index:var(--x-1e2zpja)}}}",
    priority: 3680
});
_inject2({
    ltr: "@supports (display: grid){@media (prefers-color-scheme: dark){@media (min-width: 600px){.xqw1h1y.xqw1h1y.xqw1h1y.xqw1h1y:active:focus:hover{z-index:var(--x-gg2yv0)}}}}",
    priority: 3880
});
_inject2({
    ltr: "@supports (display: grid){@media (prefers-color-scheme: dark){@media (min-width: 600px){.x15stwyu.x15stwyu.x15stwyu.x15stwyu:active:first-child:focus:hover{z-index:var(--x-hsbtju)}}}}",
    priority: 3932
});
_inject2({
    ltr: ".xr3buco{z-index:var(--x-zIndex)}",
    priority: 3000
});
_inject2({
    ltr: ".x1p70blb{content:var(--x-content)}",
    priority: 3000
});
_inject2({
    ltr: ".x5lhr3w{width:var(--x-width)}",
    priority: 4000
});
_inject2({
    ltr: ".xuwbzjh{order:var(--x-order)}",
    priority: 3000
});
_inject2({
    ltr: ".x25bfn{z-index:var(--x19xkwqv)}",
    priority: 3000
});
_inject2({
    ltr: ".xvstzhk{grid-area:var(--xsx5c67)}",
    priority: 1000
});
_inject2({
    ltr: '@property --x-padding { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-margin { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-fontFamily { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-inset { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-marginInline { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x---depth { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x---nested-depth { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-userSelect { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-appearance { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-gsepj1 { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-1ua3n7y { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-kat1qs { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-16ne5w5 { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-iny62n { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-1e2zpja { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-gg2yv0 { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-hsbtju { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-zIndex { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-content { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-width { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-order { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    unicodeName: {
        "input__styles.unicodeName": "input__styles.unicodeName",
        padding: "x1rqil9o",
        $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:10"
    },
    unicodeParam: (ünïcödé)=>[
            _temp,
            {
                padding: ünïcödé != null ? "x1fozly0" : ünïcödé,
                $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:11"
            },
            {
                "--x-padding": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(ünïcödé)
            }
        ],
    escapedParam: (ünïcödé)=>[
            _temp2,
            {
                margin: ünïcödé != null ? "xb9ncqk" : ünïcödé,
                $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:15"
            },
            {
                "--x-margin": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(ünïcödé)
            }
        ],
    helperName: (firstThatWorks)=>[
            _temp3,
            {
                fontFamily: firstThatWorks != null ? "xk2v41j" : firstThatWorks,
                $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:19"
            },
            {
                "--x-fontFamily": firstThatWorks != null ? firstThatWorks : undefined
            }
        ],
    shorthand: (zIndex)=>[
            _temp4,
            {
                inset: zIndex != null ? "xccw97s" : zIndex,
                marginInline: zIndex != null ? "xvlecxo" : zIndex,
                $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:23"
            },
            {
                "--x-inset": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(zIndex),
                "--x-marginInline": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(zIndex)
            }
        ],
    customProperty: (zIndex)=>[
            _temp5,
            {
                "--depth": zIndex != null ? "x5h8hlk" : zIndex,
                "--nested-depth": zIndex != null ? "x91d7kb" : zIndex,
                $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:26"
            },
            {
                "--x---depth": zIndex != null ? zIndex : undefined,
                "--x---nested-depth": zIndex != null ? zIndex : undefined
            }
        ],
    prefixed: (zIndex)=>[
            _temp6,
            {
                userSelect: zIndex != null ? "x9pkiyq" : zIndex,
                appearance: zIndex != null ? "xafmcc1" : zIndex,
                $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:29"
            },
            {
                "--x-userSelect": zIndex != null ? zIndex : undefined,
                "--x-appearance": zIndex != null ? zIndex : undefined
            }
        ],
    deeplyNested: (zIndex)=>[
            _temp7,
            {
                zIndex: (zIndex != null ? "xkrcnwa " : zIndex) + (zIndex != null ? "x141uv47 " : zIndex) + (zIndex != null ? "x140siia " : zIndex) + (zIndex != null ? "xlzq18l " : zIndex) + (zIndex != null ? "x103ewrf " : zIndex) + (zIndex != null ? "x1gxqx9w " : zIndex) + (zIndex != null ? "xqw1h1y " : zIndex) + (zIndex != null ? "x15stwyu" : zIndex),
                $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:33"
            },
            {
                "--x-gsepj1": zIndex != null ? zIndex : undefined,
                "--x-1ua3n7y": zIndex != null ? zIndex : undefined,
                "--x-kat1qs": zIndex != null ? zIndex : undefined,
                "--x-16ne5w5": zIndex != null ? zIndex : undefined,
                "--x-iny62n": zIndex != null ? zIndex : undefined,
                "--x-1e2zpja": zIndex != null ? zIndex : undefined,
                "--x-gg2yv0": zIndex != null ? zIndex : undefined,
                "--x-hsbtju": zIndex != null ? zIndex : undefined
            }
        ],
    computedFromParam: (zIndex)=>[
            _temp8,
            {
                "--x-zIndex": zIndex + 1 != null ? zIndex + 1 : undefined,
                "--x-content": `"${zIndex}"` != null ? `"${zIndex}"` : undefined,
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(`calc(100% - ${zIndex}px)`)
            }
        ],
    mixedParams: (zIndex, level)=>[
            _temp9,
            {
                zIndex: zIndex != null ? "xr3buco" : zIndex,
                order: level != null ? "xuwbzjh" : level,
                $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:67"
            },
            {
                "--x-zIndex": zIndex != null ? zIndex : undefined,
                "--x-order": level != null ? level : undefined
            }
        ],
    "static": {
        "input__styles.static": "input__styles.static",
        zIndex: "x25bfn",
        gridArea: "xvstzhk",
        $$css: "tests/fixture/dynamic-param-shadows-import-edges/input.stylex.js:71"
    }
};
