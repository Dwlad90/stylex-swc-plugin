import * as stylex from '@stylexjs/stylex';
import { zIndex } from './vars/zIndex.stylex.js';

export const styles = stylex.create({
  wrapper: { zIndex: zIndex._10 },
  zIndex: (zIndex) => ({ zIndex }),
});
