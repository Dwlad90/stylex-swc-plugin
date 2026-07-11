import * as stylex from '@stylexjs/stylex';

const styles = stylex.create({
  root: {
    color: 'red',
    backgroundColor: 'blue',
  },
});

export default function Button(props) {
  return <button {...stylex.props(styles.root)}>{props.children}</button>;
}
