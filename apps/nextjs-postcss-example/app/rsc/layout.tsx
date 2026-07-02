import 'server-only';

import * as stylex from '@stylexjs/stylex';
import { globalTokens as $, spacing } from '@/app/globalTokens.stylex';

export const metadata = {
  title: 'RSC + StyleX',
  description: 'React Server Components and Client Components sharing one extracted stylesheet',
};

export default function RscLayout({ children }: { children: React.ReactNode }) {
  return (
    <main {...stylex.props(styles.main)}>
      <div {...stylex.props(styles.glow)} />
      {children}
    </main>
  );
}

const styles = stylex.create({
  main: {
    position: 'relative',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    minHeight: '100vh',
    padding: spacing.xl,
    overflow: 'hidden',
    fontFamily: $.fontSans,
  },
  glow: {
    position: 'absolute',
    zIndex: -1,
    width: 640,
    height: 480,
    top: -120,
    backgroundImage: $.primaryGlow,
    filter: 'blur(48px)',
    pointerEvents: 'none',
  },
});
