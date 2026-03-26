import * as sx from '@stylexjs/stylex';
import * as React from 'react';
const c = {
    base: {
        k1xSpc: "xrvj5dj",
        $$css: true
    }
};
export default function CommentField({ type }) {
    const result = useHook();
    const nullable = null;
    const undef = undefined;
    return <div {...sx.props(nullable?.test && c.base, undef?.test && c.base, (()=>{
        const implementation = {
            foo: ()=>null,
            bar: ()=>c.base
        };
        return implementation[result?.value !== 'test' ? "foo" : result?.test] || implementation.foo;
    })()())}/>;
}
