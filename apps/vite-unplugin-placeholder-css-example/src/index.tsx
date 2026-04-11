import '@stylexswc/design-system/tokens.stylex';
import '@stylexswc/design-system/consts.stylex';

import './reset.css';

import { createRoot } from 'react-dom/client';
import App from './App';

const root = document.getElementById('root');

if (root) {
  createRoot(root).render(<App />);
}
