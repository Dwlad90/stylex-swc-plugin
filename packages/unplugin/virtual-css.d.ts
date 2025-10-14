/**
 * TypeScript declaration for StyleX virtual CSS module
 *
 * When using `useViteCssPipeline: true`, you can import the virtual CSS module:
 *
 * @example
 * ```ts
 * import 'virtual:stylex.css';
 * ```
 */
declare module 'virtual:stylex.css' {
  const css: string;
  export default css;
}

