export const PLUGIN_NAME = 'stylex';
export const VIRTUAL_CSS_PATH = require.resolve('./stylex.virtual.css');
export const VIRTUAL_CSS_PATTERN = /stylex\.virtual\.css/;
export const STYLEX_CHUNK_NAME = '_stylex-webpack-generated';
export const INCLUDE_REGEXP = /\.[cm]?[jt]sx?$/;
export const IS_DEV_ENV = process.env.NODE_ENV === 'development';
