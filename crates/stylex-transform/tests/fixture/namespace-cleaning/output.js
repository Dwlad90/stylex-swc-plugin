import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import React from 'react';
import * as sx from '@stylexjs/stylex';
import { useMediaQuery } from '@hooks/useMediaQuery';
const MULTIPLIER = 5;
_inject2({
    ltr: ".display-xjp7ctv{display:contents}",
    priority: 3000
});
_inject2({
    ltr: "@media (all) and (max-width: 1067px){.fontSize-x1f3yvym.fontSize-x1f3yvym{font-size:.75rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (all) and (max-width: 1067px){.fontSize-x17vjwcc.fontSize-x17vjwcc{font-size:1rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (all) and (max-width: 1067px){.fontSize-x19ppoyo.fontSize-x19ppoyo{font-size:1.25rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (all) and (max-width: 1067px){.fontSize-x3gzoht.fontSize-x3gzoht{font-size:1.5rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (all) and (max-width: 1067px){.fontSize-xd310an.fontSize-xd310an{font-size:1.75rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (all) and (max-width: 1067px){.fontSize-xeuu8e4.fontSize-xeuu8e4{font-size:2rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (all) and (max-width: 1067px){.fontSize-x1jbhjkf.fontSize-x1jbhjkf{font-size:2.25rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (all) and (max-width: 1067px){.fontSize-x14h6vv3.fontSize-x14h6vv3{font-size:2.5rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (all) and (max-width: 1067px){.fontSize-x1eh3tls.fontSize-x1eh3tls{font-size:2.75rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media (all) and (max-width: 1067px){.fontSize-x8rl4l3.fontSize-x8rl4l3{font-size:3rem}}",
    priority: 3200
});
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
const c = {
    wrapper: {
        display: "display-xjp7ctv",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:6"
    },
    "p-2": {
        fontSize: "fontSize-x1f3yvym",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:9"
    },
    "p-1": {
        fontSize: "fontSize-x17vjwcc",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:15"
    },
    p: {
        fontSize: "fontSize-x19ppoyo",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:21"
    },
    "p+1": {
        fontSize: "fontSize-x3gzoht",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:27"
    },
    "p+2": {
        fontSize: "fontSize-xd310an",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:33"
    },
    "1": {
        fontSize: "fontSize-xeuu8e4",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:39"
    },
    "2": {
        fontSize: "fontSize-x1jbhjkf",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:45"
    },
    "p+3": {
        fontSize: "fontSize-x14h6vv3",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:51"
    },
    "p+4": {
        fontSize: "fontSize-x1eh3tls",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:57"
    },
    "p+5": {
        fontSize: "fontSize-x8rl4l3",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:63"
    },
    unused: {
        color: "color-x1e2nbdu",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:69"
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
