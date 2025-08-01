import * as stylex from 'stylex';
import otherStyles from './otherStyles';
import npmStyles from './npmStyles';

const fadeAnimation = stylex.keyframes({
  '0%': {
    opacity: 0.25,
  },
  '100%': {
    opacity: 1,
  },
});

const styles = stylex.create({
  foo: {
    animationName: fadeAnimation,
    backgroundColor: {
      default: null,
      ':hover': 'red',
    },
    borderStartStartRadius: 7.5,
    display: 'flex',
    height: 500,
    marginInlineStart: 10,
    marginBlockStart: 99,
  },
});

export default function App() {
  return stylex.props(otherStyles.bar, styles.foo, npmStyles.baz);
}
