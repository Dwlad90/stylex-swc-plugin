import react from '@vitejs/plugin-react-swc';
import removeTestIdAttribute from 'rollup-plugin-jsx-remove-attributes';
import externals from 'rollup-plugin-node-externals';
import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
import type { PluginOption, UserConfig } from 'vite';
import styleXRSPlugin from '@stylexswc/unplugin/vite';

export const plugins = [
  react({}),
  dts({
    entryRoot: 'stories/',
  }),
  removeTestIdAttribute({
    attributes: ['data-testid'],
    usage: 'vite',
  }),
  styleXRSPlugin({
    pageExtensions: ['tsx', 'jsx', 'js', 'ts', 'vue'],
    useCSSLayers: true,
    rsOptions: {
      dev: true,
      treeshakeCompensation: true,
      enableDebugClassNames: true,
    },
  }),
] as PluginOption[];

export const config: UserConfig = {
  plugins: [...plugins, externals()],
};

// https://vitejs.dev/config/
export default defineConfig(() => {
  return {
    ...config,
  };
});
