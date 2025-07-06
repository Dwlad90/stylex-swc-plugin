'use client';
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
import { display } from '@styles/utils';
import foo from 'bar';
import { foo as baz } from 'bar';
const fn = ()=>({
        arg: ()=>{}
    });
function func() {}
{
    const display = null;
}{
    {
        const display = null;
    }
}{
    const { display } = {
        display: null
    };
}{
    const [display] = [
        null
    ];
}export const Component = ()=>{
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
_inject2(".backgroundColor-xvto61e{background-color:#F7F5F6}", 3000);
const s = {
    div: {
        "backgroundColor-kWkggS": "backgroundColor-xvto61e",
        $$css: true
    }
};
