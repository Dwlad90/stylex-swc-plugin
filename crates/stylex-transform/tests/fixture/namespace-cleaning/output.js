import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import React from 'react';
import * as sx from '@stylexjs/stylex';
import { useMediaQuery } from '@hooks/useMediaQuery';
const MULTIPLIER = 5;
_inject2({
    ltr: "@media (max-width: 1067px){.x14m1xnv.x14m1xnv{font-size:2rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 1067px){.x116hk2i.x116hk2i{font-size:2.25rem}}",
    priority: 3200
});
_inject2({
    ltr: ".xjp7ctv{display:contents}",
    priority: 3000
});
_inject2({
    ltr: "@media (max-width: 1067px){.x1xq5fe2.x1xq5fe2{font-size:.75rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 1067px){.xqtq8vs.xqtq8vs{font-size:1rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 1067px){.xzhuj4x.xzhuj4x{font-size:1.25rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 1067px){.x1b7weo0.x1b7weo0{font-size:1.5rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 1067px){.x165g4x1.x165g4x1{font-size:1.75rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 1067px){.xfzzw43.xfzzw43{font-size:2.5rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 1067px){.x1fh0699.x1fh0699{font-size:2.75rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 1067px){.xxkvdpn.xxkvdpn{font-size:3rem}}",
    priority: 3200
});
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
const c = {
    "1": {
        "input__c.1": "input__c.1",
        fontSize: "x14m1xnv",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:41"
    },
    "2": {
        "input__c.2": "input__c.2",
        fontSize: "x116hk2i",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:47"
    },
    wrapper: {
        "input__c.wrapper": "input__c.wrapper",
        display: "xjp7ctv",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:8"
    },
    "p-2": {
        "input__c.p-2": "input__c.p-2",
        fontSize: "x1xq5fe2",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:11"
    },
    "p-1": {
        "input__c.p-1": "input__c.p-1",
        fontSize: "xqtq8vs",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:17"
    },
    p: {
        "input__c.p": "input__c.p",
        fontSize: "xzhuj4x",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:23"
    },
    "p+1": {
        "input__c.p1": "input__c.p1",
        fontSize: "x1b7weo0",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:29"
    },
    "p+2": {
        "input__c.p2": "input__c.p2",
        fontSize: "x165g4x1",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:35"
    },
    "p+3": {
        "input__c.p3": "input__c.p3",
        fontSize: "xfzzw43",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:53"
    },
    "p+4": {
        "input__c.p4": "input__c.p4",
        fontSize: "x1fh0699",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:59"
    },
    "p+5": {
        "input__c.p5": "input__c.p5",
        fontSize: "xxkvdpn",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:65"
    },
    unused: {
        "input__c.unused": "input__c.unused",
        color: "x1e2nbdu",
        $$css: "@stylexswc/transform:tests/fixture/namespace-cleaning/input.stylex.js:71"
    }
};
const pClasses = [
    c['p-2'],
    c['p-1'],
    c.p,
    c['p+1'],
    c['p+2'],
    c[1],
    c[2n],
    c["p+3"],
    c[`p+4`],
    c[`p+${MULTIPLIER}`]
];
export default function NamespaceCleaning({ children }) {
    const [fontSizeIdx] = React.useState(2);
    const isMobile = useMediaQuery('(max-width: 1067px)');
    const props = sx.props(c.wrapper, isMobile && pClasses[fontSizeIdx]);
    return /*#__PURE__*/ _jsxs("div", {
        ...props,
        children
    });
}
