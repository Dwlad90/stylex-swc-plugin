import * as stylex from 'stylex';
import { label } from './plain';

const styles = stylex.create({
  root: {
    color: 'red',
  },
});

export default function App() {
  return [label, stylex.props(styles.root)];
}
