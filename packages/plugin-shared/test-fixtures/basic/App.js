import * as stylex from '@stylexjs/stylex';

const styles = stylex.create({
  root: {
    color: 'red',
    backgroundColor: 'blue',
  },
});

export function appProps() {
  return stylex.props(styles.root);
}
