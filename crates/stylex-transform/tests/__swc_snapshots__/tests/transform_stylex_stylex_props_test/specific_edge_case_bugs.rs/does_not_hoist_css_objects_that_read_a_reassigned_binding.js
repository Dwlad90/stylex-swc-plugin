import * as stylex from '@stylexjs/stylex';
let v;
export function f(c) {
    return stylex.props(c && {
        $$css: true,
        __: v
    });
}
v = 'xa';
