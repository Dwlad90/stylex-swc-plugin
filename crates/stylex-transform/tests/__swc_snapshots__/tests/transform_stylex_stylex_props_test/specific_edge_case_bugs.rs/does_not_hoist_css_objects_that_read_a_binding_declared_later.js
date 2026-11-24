import * as stylex from '@stylexjs/stylex';
import { get } from './x';
export function F(c) {
    return stylex.props(c && {
        $$css: true,
        __: later
    });
}
const later = get();
export function G(c) {
    return stylex.props(c && {
        $$css: true,
        __: laterVar
    });
}
var laterVar = get();
export const Bar = (c)=>stylex.props(c && {
        $$css: true,
        __: Bar
    });
const _temp = {
    $$css: true,
    __: later,
    a: hoisted
};
export function H(c) {
    return stylex.props(c && _temp);
}
function hoisted() {}
