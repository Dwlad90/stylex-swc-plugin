import stylexPlugin from '@toss/stylexswc-rollup-plugin';

const config = {
  input: './index.js',
  output: {
    file: './dist/bundle.js',
    format: 'es',
  },
  // See all options in the babel plugin configuration docs:
  // https://stylexjs.com/docs/api/configuration/babel-plugin/
  plugins: [stylexPlugin.default({ fileName: 'stylex.css' })],
};

export default config;
