/* oxlint-disable no-unused-vars */
/* oxlint-disable typescript/no-unused-vars */
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
/* oxlint-enable no-unused-vars */
/* oxlint-enable typescript/no-unused-vars */
