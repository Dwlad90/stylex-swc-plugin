// StyleX carrier stylesheet — replaced with the extracted CSS at build time
import '@stylexswc/webpack-plugin/stylex.css';
import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';

const domNode = document.getElementById('app');
const root = createRoot(domNode);
root.render(<App />);
