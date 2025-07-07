import * as stylex from '@stylexjs/stylex';
import { tokens } from './tokens.stylex';
import { breakpoints } from './consts.stylex';

import { type ReactNode } from 'react';

const styles = stylex.create({
  text: {
    color: {
      default: 'white',
      [breakpoints.mobile]: tokens.blue9,
      [breakpoints.desktop]: tokens.green7,
    },
  },
});

export interface TextProps {
  children: ReactNode;
}

export function Text({ children }: TextProps) {
  return <div {...stylex.props(styles.text)}>{children}</div>;
}
