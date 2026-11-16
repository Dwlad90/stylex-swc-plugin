import React from 'react';
import * as sx from '@stylexjs/stylex';
import { useMediaQuery } from '@hooks/useMediaQuery';
const c = {
    "1": {
        kGuDYH: "x14m1xnv",
        $$css: true
    },
    "2": {
        kGuDYH: "x116hk2i",
        $$css: true
    },
    wrapper: {
        k1xSpc: "xjp7ctv",
        $$css: true
    },
    "p-2": {
        kGuDYH: "x1xq5fe2",
        $$css: true
    },
    "p-1": {
        kGuDYH: "xqtq8vs",
        $$css: true
    },
    p: {
        kGuDYH: "xzhuj4x",
        $$css: true
    },
    "p+1": {
        kGuDYH: "x1b7weo0",
        $$css: true
    },
    "p+2": {
        kGuDYH: "x165g4x1",
        $$css: true
    },
    "p+3": {
        kGuDYH: "xfzzw43",
        $$css: true
    },
    "p+4": {
        kGuDYH: "x1fh0699",
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
