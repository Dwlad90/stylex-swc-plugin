import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import React from 'react';
import * as sx from '@stylexjs/stylex';
import { useMediaQuery } from '@hooks/useMediaQuery';
_inject2({
    ltr: ".display-xjp7ctv{display:contents}",
    priority: 3000
});
_inject2({
    ltr: "@media all and (max-width:1067px){.fontSize-x1r6akvx.fontSize-x1r6akvx{font-size:.75rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:1067px){.fontSize-xfpxvwb.fontSize-xfpxvwb{font-size:1rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:1067px){.fontSize-xc57lpn.fontSize-xc57lpn{font-size:1.25rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:1067px){.fontSize-x10gvik0.fontSize-x10gvik0{font-size:1.5rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:1067px){.fontSize-xsqjmrb.fontSize-xsqjmrb{font-size:1.75rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:1067px){.fontSize-x1unjt1s.fontSize-x1unjt1s{font-size:2rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:1067px){.fontSize-xcmgggk.fontSize-xcmgggk{font-size:2.25rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:1067px){.fontSize-xwq55y6.fontSize-xwq55y6{font-size:2.5rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:1067px){.fontSize-x1n9uav1.fontSize-x1n9uav1{font-size:2.75rem}}",
    priority: 3200
});
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
const c = {
    wrapper: {
        display: "display-xjp7ctv",
        $$css: "tests/fixture/namespace-cleaning-no-unused/input.stylex.js:6"
    },
    "p-2": {
        fontSize: "fontSize-x1r6akvx",
        $$css: "tests/fixture/namespace-cleaning-no-unused/input.stylex.js:9"
    },
    "p-1": {
        fontSize: "fontSize-xfpxvwb",
        $$css: "tests/fixture/namespace-cleaning-no-unused/input.stylex.js:15"
    },
    p: {
        fontSize: "fontSize-xc57lpn",
        $$css: "tests/fixture/namespace-cleaning-no-unused/input.stylex.js:21"
    },
    "p+1": {
        fontSize: "fontSize-x10gvik0",
        $$css: "tests/fixture/namespace-cleaning-no-unused/input.stylex.js:27"
    },
    "p+2": {
        fontSize: "fontSize-xsqjmrb",
        $$css: "tests/fixture/namespace-cleaning-no-unused/input.stylex.js:33"
    },
    "1": {
        fontSize: "fontSize-x1unjt1s",
        $$css: "tests/fixture/namespace-cleaning-no-unused/input.stylex.js:39"
    },
    "2": {
        fontSize: "fontSize-xcmgggk",
        $$css: "tests/fixture/namespace-cleaning-no-unused/input.stylex.js:45"
    },
    "p+3": {
        fontSize: "fontSize-xwq55y6",
        $$css: "tests/fixture/namespace-cleaning-no-unused/input.stylex.js:51"
    },
    "p+4": {
        fontSize: "fontSize-x1n9uav1",
        $$css: "tests/fixture/namespace-cleaning-no-unused/input.stylex.js:57"
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
    c[`p+4`]
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
