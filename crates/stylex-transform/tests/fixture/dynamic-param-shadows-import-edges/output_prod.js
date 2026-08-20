import * as stylex from '@stylexjs/stylex';
import { zIndex } from './vars/zIndex.stylex.js';
import { spacing as ünïcödé } from './vars/spacing.stylex.js';
import { firstThatWorks } from './vars/legacy.stylex.js';
import { grid } from './vars/grid.stylex.js';
const _temp2 = {
    kY2c9j: "xr3buco",
    kah6P1: "x1p70blb",
    kzqmXN: "x5lhr3w",
    $$css: true
};
export const styles = {
    unicodeName: {
        kmVPX3: "x1rqil9o",
        $$css: true
    },
    unicodeParam: (ünïcödé)=>[
            {
                kmVPX3: ünïcödé != null ? "x1fozly0" : ünïcödé,
                $$css: true
            },
            {
                "--x-padding": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(ünïcödé)
            }
        ],
    escapedParam: (ünïcödé)=>[
            {
                kogj98: ünïcödé != null ? "xb9ncqk" : ünïcödé,
                $$css: true
            },
            {
                "--x-margin": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(ünïcödé)
            }
        ],
    helperName: (firstThatWorks)=>[
            {
                kMv6JI: firstThatWorks != null ? "xk2v41j" : firstThatWorks,
                $$css: true
            },
            {
                "--x-fontFamily": firstThatWorks != null ? firstThatWorks : undefined
            }
        ],
    shorthand: (zIndex)=>[
            {
                kpwlN0: zIndex != null ? "xccw97s" : zIndex,
                kUOVxO: zIndex != null ? "xvlecxo" : zIndex,
                $$css: true
            },
            {
                "--x-inset": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(zIndex),
                "--x-marginInline": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(zIndex)
            }
        ],
    customProperty: (zIndex)=>[
            {
                "--depth": zIndex != null ? "x5h8hlk" : zIndex,
                "--nested-depth": zIndex != null ? "x91d7kb" : zIndex,
                $$css: true
            },
            {
                "--x---depth": zIndex != null ? zIndex : undefined,
                "--x---nested-depth": zIndex != null ? zIndex : undefined
            }
        ],
    prefixed: (zIndex)=>[
            {
                kfSwDN: zIndex != null ? "x9pkiyq" : zIndex,
                kysU6D: zIndex != null ? "xafmcc1" : zIndex,
                $$css: true
            },
            {
                "--x-userSelect": zIndex != null ? zIndex : undefined,
                "--x-appearance": zIndex != null ? zIndex : undefined
            }
        ],
    deeplyNested: (zIndex)=>[
            {
                kY2c9j: (zIndex != null ? "xkrcnwa " : zIndex) + (zIndex != null ? "x141uv47 " : zIndex) + (zIndex != null ? "x140siia " : zIndex) + (zIndex != null ? "xlzq18l " : zIndex) + (zIndex != null ? "x103ewrf " : zIndex) + (zIndex != null ? "xd94ota " : zIndex) + (zIndex != null ? "xfmfnbh " : zIndex) + (zIndex != null ? "x1b7ijjw" : zIndex),
                $$css: true
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
            _temp2,
            {
                "--x-zIndex": zIndex + 1 != null ? zIndex + 1 : undefined,
                "--x-content": `"${zIndex}"` != null ? `"${zIndex}"` : undefined,
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(`calc(100% - ${zIndex}px)`)
            }
        ],
    mixedParams: (zIndex, level)=>[
            {
                kY2c9j: zIndex != null ? "xr3buco" : zIndex,
                kayTVb: level != null ? "xuwbzjh" : level,
                $$css: true
            },
            {
                "--x-zIndex": zIndex != null ? zIndex : undefined,
                "--x-order": level != null ? level : undefined
            }
        ],
    "static": {
        kY2c9j: "x25bfn",
        kJuA4N: "xvstzhk",
        $$css: true
    }
};
