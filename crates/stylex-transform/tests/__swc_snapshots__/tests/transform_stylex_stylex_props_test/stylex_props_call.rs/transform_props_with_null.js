import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
import { useState } from 'react';
_inject2({
    ltr: ".backgroundColor-x1t391ir{background-color:blue}",
    priority: 3000
});
_inject2({
    ltr: ".right-x3m8u43{right:0}",
    priority: 4000
});
_inject2({
    ltr: ".left-xu96u03{left:0}",
    priority: 4000
});
_inject2({
    ltr: ".right-x131sewu{right:10px}",
    priority: 4000
});
_inject2({
    ltr: ".left-x12lbrt0{left:10px}",
    priority: 4000
});
const styles = {
    base: {
        backgroundColor: "backgroundColor-x1t391ir",
        $$css: true
    },
    active: {
        right: "right-x3m8u43",
        insetInlineStart: null,
        insetInlineEnd: null,
        $$css: true
    },
    inactive: {
        left: "left-xu96u03",
        insetInlineStart: null,
        insetInlineEnd: null,
        $$css: true
    },
    answered: {
        right: "right-x131sewu",
        insetInlineStart: null,
        insetInlineEnd: null,
        $$css: true
    },
    unanswered: {
        left: "left-x12lbrt0",
        insetInlineStart: null,
        insetInlineEnd: null,
        $$css: true
    }
};
export function Props_With_Null(isActive, isInactive, items) {
    const isAnswered = items[isActive] !== null;
    const [isFirst, setIsFirst] = useState(false);
    return <>
  <button {...stylex.props(styles.base, ...isFirst === true ? [
        styles.active
    ] : [], ...isFirst === true ? [
        styles.answered,
        styles.active
    ] : [
        styles.base
    ], isAnswered ? styles.answered : null, isAnswered ? styles.answered : isInactive ? styles.inactive : null, isAnswered ? styles.answered : styles.unanswered)}/>
    <button {...stylex.props(styles.base, ...isFirst === true ? [
        styles.active
    ] : [])}>Active</button>
      <button {...stylex.props(styles.base, ...isFirst === true ? [
        styles.active
    ] : [])}>Inactive</button>
      <button {...stylex.props(styles.base, ...isFirst === true ? [
        styles.active
    ] : [])}>Answered</button>
      <button {...stylex.props(styles.base, ...isFirst === true ? [
        styles.active
    ] : [])}>Unanswered</button>
      </>;
}
