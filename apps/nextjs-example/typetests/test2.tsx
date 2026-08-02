/* oxlint-disable no-unused-vars */
/* oxlint-disable typescript/no-unused-vars */

import * as stylex from '@stylexjs/stylex';
import type { StaticStyles } from '@stylexjs/stylex';

type Props = {
  xstyle?: StaticStyles<{
    backgroundColor?: string;
  }>;
};

function Component({ xstyle }: Props) {
  return <div {...stylex.props(xstyle)} />;
}

const styles = stylex.create({
  valid: {
    backgroundColor: 'red',
  },
  invalid: {
    color: 'red',
  },
});

function OtherComponent() {
  <Component xstyle={styles.valid} />;

  // @ts-expect-error - `styles.invalid` contains `color` which is not allowed by Component's `xstyle` prop.
  <Component xstyle={styles.invalid} />;
}
/* oxlint-enable no-unused-vars */
/* oxlint-enable typescript/no-unused-vars */
