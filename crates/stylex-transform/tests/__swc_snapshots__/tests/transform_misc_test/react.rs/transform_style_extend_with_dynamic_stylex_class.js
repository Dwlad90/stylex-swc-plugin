import * as sx from '@stylexjs/stylex';
import * as React from 'react';
const c = {
    base: {
        k1xSpc: "xrvj5dj",
        $$css: true
    },
    regularGrid: {
        k1xSpc: "xrvj5dj",
        $$css: true
    },
    irregularGrid: {
        k1xSpc: "xrvj5dj",
        $$css: true
    }
};
export default function CommentField({ type }) {
    let gridType = 'regular';
    if (type === 'irregular') {
        gridType = 'irregular';
    }
    const grid = `${gridType}Grid`;
    return <div styleExtend={[
        c.base,
        c[grid]
    ]}/>;
}
