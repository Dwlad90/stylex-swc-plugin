'use client';
import * as stylex from '@stylexjs/stylex';
import { display } from '@styles/utils';
import foo from 'bar';
import { foo as baz } from 'bar';
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
    3,
    4
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
    return <div>{array.length > 0 ? <div {...stylex.props(s.div, display.flex)}>{array.map((_)=>null)}</div> : null}</div>;
};
const s = {
    div: {
        kWkggS: "xvto61e",
        $$css: true
    }
};
