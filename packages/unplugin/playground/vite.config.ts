// @ts-expect-error - Vite does not have types for this plugin
import { defineConfig } from 'vite';
import Inspect from 'vite-plugin-inspect';
import Unplugin from '../src/vite';

export default defineConfig({
  plugins: [Inspect(), Unplugin()],
});
