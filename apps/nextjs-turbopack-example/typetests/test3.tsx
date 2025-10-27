/* eslint-disable no-unused-vars */
/* eslint-disable @typescript-eslint/no-unused-vars */

import * as stylex from '@stylexjs/stylex';
import type { StaticStyles, StyleXStyles } from '@stylexjs/stylex';

type Props = {
  xstyle?: StyleXStyles;
  staticXstyle?: StaticStyles;
};

function Component({ xstyle, staticXstyle }: Props): null {
  <div {...stylex.props(xstyle)} />;

  <div {...stylex.props([staticXstyle])} />;

  return null;
}

const styles = stylex.create({
  base: {
    color: 'red',
  },
});

function OtherComponent() {
  <Component xstyle={styles.base} />;
}
/* eslint-enable no-unused-vars */
/* eslint-enable @typescript-eslint/no-unused-vars */
