'use client';
import * as stylex from '@stylexjs/stylex';
import { display } from '@styles/utils';
const fn = ()=>({
        arg: ()=>{}
    });
function func() {}
export const Component = ()=>{
    const display = null;
    return display;
};
const array = [
    1,
    2,
    3
];
export const ComponentWithCallings = ()=>{
    array.forEach((item)=>{
        if (fn(item).arg('str', 1, null, undefined, NaN, {
            foo: 'bar'
        }, [
            1,
            2,
            3
        ], func())) {
            fn(item);
        }
    });
    return <div>{array.length > 0 ? <div {...stylex.props(s.div, display.flex)}>{array.map((_)=>null)}</div> : o}</div>;
};
const s = {
    div: {
        background: "x1s5p4n8",
        $$css: true
    }
};
