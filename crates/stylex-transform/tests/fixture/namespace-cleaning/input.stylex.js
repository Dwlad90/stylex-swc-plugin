import React from 'react';
import * as sx from '@stylexjs/stylex';
import { useMediaQuery } from '@hooks/useMediaQuery';

const c = sx.create({
    wrapper: {
        display: 'contents'
    },
    'p-2': {
        fontSize: {
            default: null,
            ["@media all and (max-width:37.4375em)"]: "0.75rem"
        },
    },
    'p-1': {
        fontSize: {
            default: null,
            ["@media all and (max-width:37.4375em)"]: "1rem"
        },
    },
    p: {
        fontSize: {
            default: null,
            ["@media all and (max-width:37.4375em)"]: "1.25rem"
        },
    },
    'p+1': {
        fontSize: {
            default: null,
            ["@media all and (max-width:37.4375em)"]: "1.5rem"
        },
    },
    'p+2': {
        fontSize: {
            default: null,
            ["@media all and (max-width:37.4375em)"]: "1.75rem"
        },
    }
});
const pClasses = [
    c['p-2'],
    c['p-1'],
    c.p,
    c['p+1'],
    c['p+2']
];

export default function NamespaceCleaning({ children }) {
  const [fontSizeIdx] = React.useState(2);
  const isMobile = useMediaQuery('(max-width: 37.4375em)');

  const props = sx.props(c.wrapper, isMobile && pClasses[fontSizeIdx]);

  return /*#__PURE__*/ _jsxs("div", {
      ...props,
      children
  });
}
