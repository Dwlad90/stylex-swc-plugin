import React from 'react';
import * as sx from '@stylexjs/stylex';
import { useMediaQuery } from '@hooks/useMediaQuery';
const c = {
    wrapper: {
        k1xSpc: "xjp7ctv",
        $$css: true
    },
    "p-2": {
        kGuDYH: "x1f3yvym",
        $$css: true
    },
    "p-1": {
        kGuDYH: "x17vjwcc",
        $$css: true
    },
    p: {
        kGuDYH: "x19ppoyo",
        $$css: true
    },
    "p+1": {
        kGuDYH: "x3gzoht",
        $$css: true
    },
    "p+2": {
        kGuDYH: "xd310an",
        $$css: true
    },
    "1": {
        kGuDYH: "xeuu8e4",
        $$css: true
    },
    "2": {
        kGuDYH: "x1jbhjkf",
        $$css: true
    },
    "p+3": {
        kGuDYH: "x14h6vv3",
        $$css: true
    },
    "p+4": {
        kGuDYH: "x1eh3tls",
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
