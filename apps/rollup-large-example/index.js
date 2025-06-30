'use strict';

import * as stylex from '@stylexjs/stylex';
import { lotsOfStyles } from './lotsOfStyles';
import { lotsOfStylesDynamic } from './lotsOfStylesDynamic.js';

const styles = lotsOfStyles.map((defs) => Object.values(defs));
const dynamicStyles = lotsOfStylesDynamic.map((defs) => Object.values(defs));

export default function App() {
  return stylex.props(styles, dynamicStyles);
}