import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { fonts } from '@stylexjs/open-props/lib/fonts.stylex';
import { sizes } from '@stylexjs/open-props/lib/sizes.stylex';
import * as stylex from '@stylexjs/stylex';
import { Text } from '@stylexswc/design-system';
import { tokens } from '@stylexswc/design-system/tokens.stylex';
import { lazy, Suspense } from 'react';
import { createRoot } from 'react-dom/client';

const LateCard = lazy(() => import('./LateCard.jsx'));

const styles = stylex.create({
  main: {
    width: stylex.env.tokens.layout.fullWidth,
    height: stylex.env.tokens.layout.fullHeight,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-around',
    flexDirection: 'column',
  },
  card: {
    padding: stylex.env.wrapper(sizes.spacing5),
    borderRadius: sizes.spacing2,
    justifyContent: 'center',
    display: 'flex',
    alignItems: 'center',
    fontFamily: fonts.mono,
  },
  blueBg: {
    backgroundColor: colors.blue3,
  },
  pinkBg: {
    backgroundColor: tokens.pink7,
  },
  orangeFg: {
    color: colors.orange7,
  },
});

function App() {
  return (
    <div {...stylex.props(styles.main)}>
      <div {...stylex.props(styles.card, styles.blueBg)}>
        <span {...stylex.props(styles.orangeFg)}>Blue rounded rectangle with orange text</span>
      </div>
      <div {...stylex.props(styles.card, styles.pinkBg)}>
        <Text>Pink rounded rectangle with blue text</Text>
      </div>
      <Suspense fallback={null}>
        <LateCard />
      </Suspense>
    </div>
  );
}

createRoot(document.getElementById('root')).render(<App />);
