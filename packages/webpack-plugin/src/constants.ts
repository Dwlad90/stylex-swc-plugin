export const PLUGIN_NAME = 'stylex';
export const VIRTUAL_CSS_PATH = require.resolve('./stylex.virtual.css');
export const VIRTUAL_CSS_PATTERN = /stylex\.virtual\.css/;
export const STYLEX_CHUNK_NAME = '_stylex-webpack-generated';
export const INCLUDE_REGEXP = /\.[cm]?[jt]sx?$/;
export const IS_DEV_ENV = process.env.NODE_ENV === 'development';
/**
 * node_modules packages that ship untransformed StyleX source and must go
 * through the stylex-loader even though node_modules is excluded by default
 */
export const DEFAULT_STYLEX_PACKAGES = ['@stylexjs/'];
