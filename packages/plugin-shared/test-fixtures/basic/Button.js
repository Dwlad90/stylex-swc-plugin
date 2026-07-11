import * as stylex from '@stylexjs/stylex';

const styles = stylex.create({
  root: {
    paddingTop: '4px',
    fontSize: '16px',
  },
});

export function buttonProps() {
  return stylex.props(styles.root);
}
