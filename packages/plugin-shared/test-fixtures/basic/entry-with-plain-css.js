// Variant of entry.js that also pulls a plain (non-StyleX) stylesheet through
// the CSS pipeline — used by the cache-group funnel integration tests to prove
// foreign CSS funneled into the stylex chunk survives the asset finalization.
import 'stylex-carrier.css';
import './plain.css';

import { appProps } from './App';
import { buttonProps } from './Button';

console.log(appProps(), buttonProps());
