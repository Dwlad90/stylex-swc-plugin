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
    ltr: "@media all and (max-width:37.4375em){.fontSize-x10eyerh.fontSize-x10eyerh{font-size:.75rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:37.4375em){.fontSize-x12gvsh5.fontSize-x12gvsh5{font-size:1rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:37.4375em){.fontSize-x17n2fls.fontSize-x17n2fls{font-size:1.25rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:37.4375em){.fontSize-x1wd262b.fontSize-x1wd262b{font-size:1.5rem}}",
    priority: 3200
});
_inject2({
    ltr: "@media all and (max-width:37.4375em){.fontSize-xnw33il.fontSize-xnw33il{font-size:1.75rem}}",
    priority: 3200
});
const c = {
    wrapper: {
        display: "display-xjp7ctv",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:5"
    },
    "p-2": {
        fontSize: "fontSize-x10eyerh",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:8"
    },
    "p-1": {
        fontSize: "fontSize-x12gvsh5",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:14"
    },
    p: {
        fontSize: "fontSize-x17n2fls",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:20"
    },
    "p+1": {
        fontSize: "fontSize-x1wd262b",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:26"
    },
    "p+2": {
        fontSize: "fontSize-xnw33il",
        $$css: "tests/fixture/namespace-cleaning/input.stylex.js:32"
    }
};
const pClasses = [
    c['p-2'],
    c['p-1'],
    c.p,
    c['p+1'],
    c['p+2']
];
export default function NamespaceCleaning({ children }) {
    const [fontSizeIdx] = React.useState(2);
    const isMobile = useMediaQuery('(max-width: 37.4375em)');
    const props = sx.props(c.wrapper, isMobile && pClasses[fontSizeIdx]);
    return _jsxs("div", {
        ...props,
        children
    });
}
