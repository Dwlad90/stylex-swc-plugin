import './reset.css';
import 'virtual:stylex.css';

import { createRoot } from 'react-dom/client';
import App from './App';

const root = document.getElementById('root');

if (root) {
  createRoot(root).render(<App />);
}
