import * as stylex from '@stylexjs/stylex';

// This module is an .mjs file that contains StyleX code. The bundler must send
// .mjs files to the StyleX loader. If it does not, the keyframes call stays in
// the bundle and it fails when the page runs.
const pulse = stylex.keyframes({
  from: { opacity: 0.35 },
  to: { opacity: 1 },
});

export const styles = stylex.create({
  badge: {
    animationDuration: '1.4s',
    animationIterationCount: 'infinite',
    animationName: pulse,
    animationTimingFunction: 'ease-in-out',
    backgroundColor: '#0ca678',
    borderRadius: 8,
    color: 'white',
    fontWeight: 700,
    paddingBlock: 8,
    paddingInline: 16,
  },
});
