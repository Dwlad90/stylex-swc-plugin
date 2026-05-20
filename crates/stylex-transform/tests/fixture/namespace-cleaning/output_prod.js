import React from 'react';
import * as sx from '@stylexjs/stylex';
import { useMediaQuery } from '@hooks/useMediaQuery';
const c = {
    wrapper: {
        k1xSpc: "xjp7ctv",
        $$css: true
    },
    "p-2": {
        kGuDYH: "x10eyerh",
        $$css: true
    },
    "p-1": {
        kGuDYH: "x12gvsh5",
        $$css: true
    },
    p: {
        kGuDYH: "x17n2fls",
        $$css: true
    },
    "p+1": {
        kGuDYH: "x1wd262b",
        $$css: true
    },
    "p+2": {
        kGuDYH: "xnw33il",
        $$css: true
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
