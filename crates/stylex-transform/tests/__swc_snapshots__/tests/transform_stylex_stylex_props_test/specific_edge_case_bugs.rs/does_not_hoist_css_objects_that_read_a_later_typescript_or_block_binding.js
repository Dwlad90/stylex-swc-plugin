import * as stylex from '@stylexjs/stylex';
const _temp = {
    $$css: true,
    __: Ambient
};
const _temp2 = {
    $$css: true,
    __: ambientConst
};
const _temp3 = {
    $$css: true,
    __: ambientVar
};
const _temp4 = {
    $$css: true,
    __: AmbientClass
};
const _temp5 = {
    $$css: true,
    __: AmbientEnum
};
export function F(c) {
    return stylex.props(c && {
        $$css: true,
        __: Kind
    }, c && {
        $$css: true,
        __: Space
    }, c && {
        $$css: true,
        __: resource
    }, c && {
        $$css: true,
        __: inBlock
    }, c && {
        $$css: true,
        __: inLoop
    }, c && _temp, c && _temp2, c && _temp3, c && _temp4, c && _temp5);
}
enum Kind {
    A
}
namespace Space {
    export const a = 1;
}
using resource = open();
{
    var inBlock = 1;
}for (var inLoop of []){}
declare namespace Ambient {
}
declare const ambientConst: string;
declare var ambientVar: string;
declare class AmbientClass {
}
declare enum AmbientEnum {
    A
}
