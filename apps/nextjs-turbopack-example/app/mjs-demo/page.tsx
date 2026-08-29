import * as stylex from '@stylexjs/stylex';

import { styles } from './animation.mjs';

// The page shows that StyleX in an .mjs module compiles correctly.
export default function MjsDemoPage() {
  return (
    <main {...stylex.props(pageStyles.main)}>
      <h1 {...stylex.props(pageStyles.title)}>StyleX in .mjs</h1>
      <div data-testid="mjs-badge" {...stylex.props(styles.badge)}>
        Compiled from an .mjs module
      </div>
    </main>
  );
}

const pageStyles = stylex.create({
  main: {
    alignItems: 'center',
    display: 'flex',
    flexDirection: 'column',
    gap: 24,
    justifyContent: 'center',
    minHeight: '100vh',
  },
  title: {
    fontFamily: 'sans-serif',
    fontSize: 24,
    fontWeight: 700,
  },
});
