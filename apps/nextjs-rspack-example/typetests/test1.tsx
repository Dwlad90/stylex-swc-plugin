/* eslint-disable no-unused-vars */
/* eslint-disable typescript/no-unused-vars */
import * as stylex from '@stylexjs/stylex';
import type { StaticStyles } from '@stylexjs/stylex';

type Props = {
  xstyle?: StaticStyles;
};

function Component({ xstyle }: Props) {
  return <div {...stylex.props(xstyle)} />;
}

const styles = stylex.create({
  base: {
    color: 'red',
  },
});

function OtherComponent() {
  return <Component xstyle={styles.base} />;
}

function OtherComponent2() {
  return <Component xstyle={[styles.base, undefined]} />;
}
/* eslint-enable no-unused-vars */
/* eslint-enable typescript/no-unused-vars */
