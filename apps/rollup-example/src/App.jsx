'use strict';

import * as stylex from '@stylexjs/stylex';

const styles = stylex.create({
  main: {
    width: '100vw',
    height: '100vh',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    // @ts-expect-error - env is not correctly typed
    backgroundColor: stylex.env.wrapper(stylex.env.tokens.colors.background),
  },
  card: {
    backgroundColor: '#fefefe',
    // @ts-expect-error - env is not correctly typed
    padding: stylex.env.wrapper('1rem'),
    borderRadius: 10,
    justifyContent: 'center',
    display: 'flex',
    alignItems: 'center',
    // @ts-expect-error - env is not correctly typed
    color: stylex.env.wrapper(stylex.env.tokens.colors.text),
    fontFamily: 'Arial',
  },
});

export default function App() {
    return (
    <div sx={styles.main}>
      <div {...stylex.props(styles.card)}>Content</div>
    </div>
  );
}
