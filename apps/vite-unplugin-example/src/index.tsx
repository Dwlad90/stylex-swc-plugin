import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { fonts } from '@stylexjs/open-props/lib/fonts.stylex';
import { sizes } from '@stylexjs/open-props/lib/sizes.stylex';
import * as stylex from '@stylexjs/stylex';
import { createRoot } from 'react-dom/client';

const styles = stylex.create({
  container: {
    alignItems: 'center',
    backgroundColor: colors.choco3,
    display: 'grid',
    height: '100dvh',
    justifyContent: 'center',
    width: '100dvw',
  },
  button: {
    alignItems: 'center',
    backgroundColor: colors.jungle6,
    borderRadius: sizes.spacing15,
    color: colors.gray2,
    display: 'flex',
    fontFamily: fonts.mono,
    justifyContent: 'center',
    paddingBlock: sizes.spacing5,
    paddingInline: sizes.spacing10,
    borderWidth: 0,
    fontWeight: 'bold',
    ':hover': {
      backgroundColor: colors.jungle8,
    },
    ':active': {
      backgroundColor: colors.jungle10,
    },
  },
});

function App() {
  return (
    <main {...stylex.props(styles.container)}>
      <button {...stylex.props(styles.button)}>Click Me</button>
    </main>
  );
}

const root = document.getElementById('root');

if (root) {
  createRoot(root).render(<App />);
}
