import React from 'react';
import * as sx from '@stylexjs/stylex';
import { useMediaQuery } from '@hooks/useMediaQuery';

const MULTIPLIER = 5;

const c = sx.create({
  wrapper: {
    display: 'contents'
  },
  'p-2': {
    fontSize: {
      default: null,
      ["@media all and (max-width:1067px)"]: "0.75rem"
    },
  },
  'p-1': {
    fontSize: {
      default: null,
      ["@media all and (max-width:1067px)"]: "1rem"
    },
  },
  p: {
    fontSize: {
      default: null,
      ["@media all and (max-width:1067px)"]: "1.25rem"
    },
  },
  'p+1': {
    fontSize: {
      default: null,
      ["@media all and (max-width:1067px)"]: "1.5rem"
    },
  },
  'p+2': {
    fontSize: {
      default: null,
      ["@media all and (max-width:1067px)"]: "1.75rem"
    },
  },
  1: {
    fontSize: {
      default: null,
      ["@media all and (max-width:1067px)"]: "2rem"
    },
  },
  2n: {
    fontSize: {
      default: null,
      ["@media all and (max-width:1067px)"]: "2.25rem"
    },
  },
  ["p+3"]: {
    fontSize: {
      default: null,
      ["@media all and (max-width:1067px)"]: "2.5rem"
    },
  },
  [`p+4`]: {
    fontSize: {
      default: null,
      ["@media all and (max-width:1067px)"]: "2.75rem"
    },
  },
  unused: {
    color: 'red'
  }
});
const pClasses = [
  c['p-2'],
  c['p-1'],
  c.p,
  c['p+1'],
  c['p+2'],
  c[1],
  c[2n],
  c["p+3"],
  c[`p+4`],
];

export default function NamespaceCleaning({ children }) {
  const [fontSizeIdx] = React.useState(2);
  const isMobile = useMediaQuery('(max-width: 1067px)');

  const props = sx.props(c.wrapper, isMobile && pClasses[fontSizeIdx]);

  return /*#__PURE__*/ _jsxs("div", {
    ...props,
    children
  });
}
