import * as stylex from '@stylexjs/stylex';
import { tokens } from './tokens.stylex';

import { type ReactNode } from 'react';

const styles = stylex.create({
  text: {
    color: tokens.blue9,
  },
});

export interface TextProps {
  children: ReactNode;
}

export function Text({ children }: TextProps) {
  return <div {...stylex.props(styles.text)}>{children}</div>;
}
