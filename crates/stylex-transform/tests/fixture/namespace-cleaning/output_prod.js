import React from 'react';
import * as sx from '@stylexjs/stylex';
import { useMediaQuery } from '@hooks/useMediaQuery';
const c = {
    wrapper: {
        k1xSpc: "xjp7ctv",
        $$css: true
    },
    "p-2": {
        kGuDYH: "x1r6akvx",
        $$css: true
    },
    "p-1": {
        kGuDYH: "xfpxvwb",
        $$css: true
    },
    p: {
        kGuDYH: "xc57lpn",
        $$css: true
    },
    "p+1": {
        kGuDYH: "x10gvik0",
        $$css: true
    },
    "p+2": {
        kGuDYH: "xsqjmrb",
        $$css: true
    },
    "1": {
        kGuDYH: "x1unjt1s",
        $$css: true
    },
    "2": {
        kGuDYH: "xcmgggk",
        $$css: true
    },
    "p+3": {
        kGuDYH: "xwq55y6",
        $$css: true
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
    c["p+3"]
];
export default function NamespaceCleaning({ children }) {
    const [fontSizeIdx] = React.useState(2);
    const isMobile = useMediaQuery('(max-width: 1067px)');
    const props = sx.props(c.wrapper, isMobile && pClasses[fontSizeIdx]);
    return _jsxs("div", {
        ...props,
        children
    });
}
