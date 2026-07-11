// Resolved per-suite via `resolve.alias` to the bundler package's carrier
// stylesheet (e.g. @stylexswc/webpack-plugin/stylex.css)
import 'stylex-carrier.css';

import { appProps } from './App';
import { buttonProps } from './Button';

console.log(appProps(), buttonProps());
